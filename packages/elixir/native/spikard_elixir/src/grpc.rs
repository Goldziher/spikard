//! Elixir gRPC handler bridge.
//!
//! This module connects Elixir gRPC handler functions to Spikard's shared Rust
//! gRPC runtime. The Rust side owns transport, framing, and HTTP/2 concerns;
//! Elixir receives raw protobuf payload bytes and returns raw protobuf payloads.

#![deny(clippy::unwrap_used)]

use bytes::Bytes;
use futures::StreamExt;
use once_cell::sync::Lazy;
use rustler::{Atom, Binary, Encoder, Env, LocalPid, MapIterator, NifResult, OwnedBinary, OwnedEnv, Term};
use spikard_http::grpc::streaming::{MessageStream, StreamingRequest};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRegistry, GrpcRequestData, GrpcResponseData, RpcMode};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::oneshot;
use tonic::metadata::MetadataMap;
use tracing::{debug, warn};

use crate::atoms;

const MAX_METADATA_ENTRIES: usize = 128;
const MAX_METADATA_KEY_SIZE: usize = 1024;
const MAX_METADATA_VALUE_SIZE: usize = 8192;
const MAX_PAYLOAD_BYTES: usize = 100 * 1024 * 1024;
const MAX_STREAM_MESSAGES: usize = 10_000;
const MAX_STREAM_TOTAL_BYTES: usize = 500 * 1024 * 1024;
const HANDLER_TIMEOUT_SECS: u64 = 30;

static PENDING_GRPC_REQUESTS: Lazy<Mutex<HashMap<u64, oneshot::Sender<ElixirGrpcReply>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static GRPC_REQUEST_ID_COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

#[derive(Debug, Clone)]
enum ElixirGrpcReply {
    Unary {
        payload: Vec<u8>,
        metadata: HashMap<String, String>,
    },
    Stream {
        messages: Vec<Vec<u8>>,
    },
    Error {
        code: String,
        message: String,
    },
}

#[derive(Clone)]
pub struct ElixirGrpcHandler {
    handler_runner_pid: LocalPid,
    service_name: Arc<str>,
    timeout: Duration,
}

impl ElixirGrpcHandler {
    pub fn new(handler_runner_pid: LocalPid, service_name: String) -> Self {
        Self {
            handler_runner_pid,
            service_name: Arc::from(service_name.as_str()),
            timeout: Duration::from_secs(HANDLER_TIMEOUT_SECS),
        }
    }
}

pub fn build_registry(
    grpc_service_defs: Term,
    handler_runner_pid: LocalPid,
) -> Result<Option<Arc<GrpcRegistry>>, String> {
    let Some(service_iter) = MapIterator::new(grpc_service_defs) else {
        return Ok(None);
    };

    let mut registry = GrpcRegistry::new();

    for (service_term, methods_term) in service_iter {
        let service_name =
            decode_term_string(service_term).map_err(|_| "gRPC service names must be strings or atoms".to_string())?;
        let methods_iter = MapIterator::new(methods_term)
            .ok_or_else(|| format!("gRPC service {service_name} must map to a method map"))?;

        let handler = Arc::new(ElixirGrpcHandler::new(handler_runner_pid, service_name.clone()));

        for (method_term, mode_term) in methods_iter {
            let method_name = decode_term_string(method_term)
                .map_err(|_| format!("gRPC method names for {service_name} must be strings or atoms"))?;
            let mode_name = decode_term_string(mode_term)
                .map_err(|_| format!("gRPC rpc mode for {service_name}/{method_name} must be a string or atom"))?;
            let rpc_mode = parse_rpc_mode(&mode_name)?;
            registry.register(service_name.clone(), method_name, handler.clone(), rpc_mode);
        }
    }

    if registry.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Arc::new(registry)))
    }
}

#[rustler::nif]
pub fn deliver_grpc_response<'a>(env: Env<'a>, request_id: u64, response_term: Term<'a>) -> NifResult<Term<'a>> {
    let response = decode_grpc_reply(env, response_term)
        .map_err(|err| rustler::Error::Term(Box::new(format!("invalid gRPC response: {err}"))))?;

    if deliver_response(request_id, response) {
        Ok(atoms::ok().encode(env))
    } else {
        Ok((atoms::error(), atoms::not_implemented()).encode(env))
    }
}

impl GrpcHandler for ElixirGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send + '_>> {
        let pid = self.handler_runner_pid;
        let timeout = self.timeout;

        Box::pin(async move {
            let (tx, rx) = oneshot::channel();
            let request_id = register_pending_request(tx);

            send_unary_request(pid, request_id, atoms::unary(), &request).map_err(tonic::Status::internal)?;
            match wait_for_reply(request_id, timeout, rx).await? {
                ElixirGrpcReply::Unary { payload, metadata } => Ok(GrpcResponseData {
                    payload: Bytes::from(payload),
                    metadata: hashmap_to_metadata_map(&metadata)?,
                }),
                ElixirGrpcReply::Stream { .. } => Err(tonic::Status::internal(
                    "Elixir gRPC unary handler returned a stream response",
                )),
                ElixirGrpcReply::Error { code, message } => Err(grpc_error_to_status(&code, &message)),
            }
        })
    }

    fn service_name(&self) -> &str {
        self.service_name.as_ref()
    }

    fn call_server_stream(
        &self,
        request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send + '_>> {
        let pid = self.handler_runner_pid;
        let timeout = self.timeout;

        Box::pin(async move {
            let (tx, rx) = oneshot::channel();
            let request_id = register_pending_request(tx);

            send_unary_request(pid, request_id, atoms::server_stream(), &request).map_err(tonic::Status::internal)?;
            match wait_for_reply(request_id, timeout, rx).await? {
                ElixirGrpcReply::Unary { .. } => Err(tonic::Status::internal(
                    "Elixir gRPC server-stream handler returned a unary response",
                )),
                ElixirGrpcReply::Stream { messages } => {
                    let stream = futures::stream::iter(messages.into_iter().map(|msg| Ok(Bytes::from(msg))));
                    Ok(Box::pin(stream) as MessageStream)
                }
                ElixirGrpcReply::Error { code, message } => Err(grpc_error_to_status(&code, &message)),
            }
        })
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send + '_>> {
        let pid = self.handler_runner_pid;
        let timeout = self.timeout;

        Box::pin(async move {
            let messages = collect_request_stream(request.message_stream).await?;
            let (tx, rx) = oneshot::channel();
            let request_id = register_pending_request(tx);

            send_stream_request(
                pid,
                request_id,
                atoms::client_stream(),
                &request.service_name,
                &request.method_name,
                &request.metadata,
                &messages,
            )
            .map_err(tonic::Status::internal)?;

            match wait_for_reply(request_id, timeout, rx).await? {
                ElixirGrpcReply::Unary { payload, metadata } => Ok(GrpcResponseData {
                    payload: Bytes::from(payload),
                    metadata: hashmap_to_metadata_map(&metadata)?,
                }),
                ElixirGrpcReply::Stream { .. } => Err(tonic::Status::internal(
                    "Elixir gRPC client-stream handler returned a stream response",
                )),
                ElixirGrpcReply::Error { code, message } => Err(grpc_error_to_status(&code, &message)),
            }
        })
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send + '_>> {
        let pid = self.handler_runner_pid;
        let timeout = self.timeout;

        Box::pin(async move {
            let messages = collect_request_stream(request.message_stream).await?;
            let (tx, rx) = oneshot::channel();
            let request_id = register_pending_request(tx);

            send_stream_request(
                pid,
                request_id,
                atoms::bidi_stream(),
                &request.service_name,
                &request.method_name,
                &request.metadata,
                &messages,
            )
            .map_err(tonic::Status::internal)?;

            match wait_for_reply(request_id, timeout, rx).await? {
                ElixirGrpcReply::Unary { .. } => Err(tonic::Status::internal(
                    "Elixir gRPC bidi handler returned a unary response",
                )),
                ElixirGrpcReply::Stream { messages } => {
                    let stream = futures::stream::iter(messages.into_iter().map(|msg| Ok(Bytes::from(msg))));
                    Ok(Box::pin(stream) as MessageStream)
                }
                ElixirGrpcReply::Error { code, message } => Err(grpc_error_to_status(&code, &message)),
            }
        })
    }
}

fn next_request_id() -> u64 {
    let mut counter = GRPC_REQUEST_ID_COUNTER.lock().unwrap_or_else(|err| err.into_inner());
    *counter = counter.wrapping_add(1);
    *counter
}

fn register_pending_request(sender: oneshot::Sender<ElixirGrpcReply>) -> u64 {
    let id = next_request_id();
    let mut pending = PENDING_GRPC_REQUESTS.lock().unwrap_or_else(|err| err.into_inner());
    pending.insert(id, sender);
    id
}

fn remove_pending_request(request_id: u64) {
    let mut pending = PENDING_GRPC_REQUESTS.lock().unwrap_or_else(|err| err.into_inner());
    pending.remove(&request_id);
}

fn deliver_response(request_id: u64, response: ElixirGrpcReply) -> bool {
    let sender = {
        let mut pending = PENDING_GRPC_REQUESTS.lock().unwrap_or_else(|err| err.into_inner());
        pending.remove(&request_id)
    };

    if let Some(tx) = sender {
        tx.send(response).is_ok()
    } else {
        debug!("No pending gRPC request found for ID {}", request_id);
        false
    }
}

async fn wait_for_reply(
    request_id: u64,
    timeout: Duration,
    rx: oneshot::Receiver<ElixirGrpcReply>,
) -> Result<ElixirGrpcReply, tonic::Status> {
    match tokio::time::timeout(timeout, rx).await {
        Ok(Ok(reply)) => Ok(reply),
        Ok(Err(_)) => Err(tonic::Status::internal(
            "Elixir gRPC response channel closed unexpectedly",
        )),
        Err(_) => {
            remove_pending_request(request_id);
            Err(tonic::Status::deadline_exceeded("Elixir gRPC handler timed out"))
        }
    }
}

fn send_unary_request(
    handler_runner_pid: LocalPid,
    request_id: u64,
    rpc_mode: Atom,
    request: &GrpcRequestData,
) -> Result<(), String> {
    let metadata_map = metadata_map_to_hashmap(&request.metadata);
    let owned_env = OwnedEnv::new();

    owned_env.run(|env| {
        let execute_atom = atoms::grpc_execute();
        let payload_term = binary_term_from_slice(env, &request.payload)?;
        let metadata_term = crate::conversion::map_to_elixir_map(env, &metadata_map)
            .map_err(|err| format!("failed to encode gRPC metadata: {:?}", err))?;
        let message = (
            execute_atom,
            request_id,
            rpc_mode,
            request.service_name.clone(),
            request.method_name.clone(),
            payload_term,
            metadata_term,
        )
            .encode(env);

        env.send(&handler_runner_pid, message)
            .map_err(|err| format!("failed to send gRPC request to Elixir: {:?}", err))
    })
}

fn send_stream_request(
    handler_runner_pid: LocalPid,
    request_id: u64,
    rpc_mode: Atom,
    service_name: &str,
    method_name: &str,
    metadata: &MetadataMap,
    messages: &[Vec<u8>],
) -> Result<(), String> {
    let metadata_map = metadata_map_to_hashmap(metadata);
    let owned_env = OwnedEnv::new();

    owned_env.run(|env| {
        let execute_atom = atoms::grpc_execute();
        let mut payload_terms = Vec::with_capacity(messages.len());
        for message in messages {
            payload_terms.push(binary_term_from_slice(env, message)?);
        }
        let metadata_term = crate::conversion::map_to_elixir_map(env, &metadata_map)
            .map_err(|err| format!("failed to encode gRPC metadata: {:?}", err))?;
        let message = (
            execute_atom,
            request_id,
            rpc_mode,
            service_name.to_string(),
            method_name.to_string(),
            payload_terms,
            metadata_term,
        )
            .encode(env);

        env.send(&handler_runner_pid, message)
            .map_err(|err| format!("failed to send gRPC stream request to Elixir: {:?}", err))
    })
}

async fn collect_request_stream(mut message_stream: MessageStream) -> Result<Vec<Vec<u8>>, tonic::Status> {
    let mut messages = Vec::new();
    let mut total_bytes = 0usize;

    while let Some(result) = message_stream.next().await {
        if messages.len() >= MAX_STREAM_MESSAGES {
            return Err(tonic::Status::resource_exhausted(format!(
                "Client stream exceeded maximum {} messages",
                MAX_STREAM_MESSAGES
            )));
        }

        match result {
            Ok(bytes) => {
                if bytes.len() > MAX_PAYLOAD_BYTES {
                    return Err(tonic::Status::resource_exhausted(format!(
                        "Message size {} exceeds maximum {}",
                        bytes.len(),
                        MAX_PAYLOAD_BYTES
                    )));
                }

                total_bytes = total_bytes
                    .checked_add(bytes.len())
                    .ok_or_else(|| tonic::Status::resource_exhausted("Stream total size overflow"))?;

                if total_bytes > MAX_STREAM_TOTAL_BYTES {
                    return Err(tonic::Status::resource_exhausted(format!(
                        "Stream total bytes {} exceeds maximum {}",
                        total_bytes, MAX_STREAM_TOTAL_BYTES
                    )));
                }

                messages.push(bytes.to_vec());
            }
            Err(status) => return Err(status),
        }
    }

    Ok(messages)
}

fn decode_grpc_reply(_env: Env<'_>, response_term: Term<'_>) -> Result<ElixirGrpcReply, String> {
    if let Ok((tag, payload_term, metadata_term)) = response_term.decode::<(Atom, Term, Term)>()
        && tag == atoms::ok()
    {
        let payload = decode_binary(payload_term)?;
        let metadata = decode_string_map(metadata_term)?;
        validate_payload(&payload)?;
        validate_metadata(&metadata)?;
        return Ok(ElixirGrpcReply::Unary { payload, metadata });
    }

    if let Ok((tag, messages)) = response_term.decode::<(Atom, Vec<Term>)>()
        && tag == atoms::stream()
    {
        let mut decoded_messages = Vec::with_capacity(messages.len());
        for message_term in messages {
            let message = decode_binary(message_term)?;
            validate_payload(&message)?;
            decoded_messages.push(message);
        }
        return Ok(ElixirGrpcReply::Stream {
            messages: decoded_messages,
        });
    }

    if let Ok((tag, code_term, message_term, _metadata_term)) = response_term.decode::<(Atom, Term, Term, Term)>()
        && tag == atoms::error()
    {
        let code = decode_term_string(code_term)
            .map_err(|_| "gRPC error codes must be strings, atoms, or integers".to_string())?;
        let message =
            decode_term_string(message_term).map_err(|_| "gRPC error messages must be strings".to_string())?;
        return Ok(ElixirGrpcReply::Error { code, message });
    }

    Err(format!("unsupported gRPC response term: {:?}", response_term))
}

fn validate_payload(payload: &[u8]) -> Result<(), String> {
    if payload.len() > MAX_PAYLOAD_BYTES {
        Err(format!(
            "payload size {} exceeds maximum {}",
            payload.len(),
            MAX_PAYLOAD_BYTES
        ))
    } else {
        Ok(())
    }
}

fn validate_metadata(metadata: &HashMap<String, String>) -> Result<(), String> {
    if metadata.len() > MAX_METADATA_ENTRIES {
        return Err(format!(
            "metadata entries {} exceeds maximum {}",
            metadata.len(),
            MAX_METADATA_ENTRIES
        ));
    }

    for (key, value) in metadata {
        if key.len() > MAX_METADATA_KEY_SIZE {
            return Err(format!("metadata key exceeds maximum size {}", MAX_METADATA_KEY_SIZE));
        }
        if value.len() > MAX_METADATA_VALUE_SIZE {
            return Err(format!(
                "metadata value exceeds maximum size {}",
                MAX_METADATA_VALUE_SIZE
            ));
        }
    }

    Ok(())
}

fn parse_rpc_mode(mode_name: &str) -> Result<RpcMode, String> {
    match mode_name.trim().to_ascii_lowercase().as_str() {
        "unary" => Ok(RpcMode::Unary),
        "server_stream" => Ok(RpcMode::ServerStreaming),
        "client_stream" => Ok(RpcMode::ClientStreaming),
        "bidi_stream" => Ok(RpcMode::BidirectionalStreaming),
        other => Err(format!("unsupported gRPC rpc mode '{other}'")),
    }
}

fn grpc_error_to_status(code: &str, message: &str) -> tonic::Status {
    let normalized = code.trim().to_ascii_lowercase();
    let tonic_code = match normalized.as_str() {
        "0" | "ok" => tonic::Code::Ok,
        "1" | "cancelled" | "canceled" => tonic::Code::Cancelled,
        "2" | "unknown" => tonic::Code::Unknown,
        "3" | "invalid_argument" => tonic::Code::InvalidArgument,
        "4" | "deadline_exceeded" => tonic::Code::DeadlineExceeded,
        "5" | "not_found" => tonic::Code::NotFound,
        "6" | "already_exists" => tonic::Code::AlreadyExists,
        "7" | "permission_denied" => tonic::Code::PermissionDenied,
        "8" | "resource_exhausted" => tonic::Code::ResourceExhausted,
        "9" | "failed_precondition" => tonic::Code::FailedPrecondition,
        "10" | "aborted" => tonic::Code::Aborted,
        "11" | "out_of_range" => tonic::Code::OutOfRange,
        "12" | "unimplemented" => tonic::Code::Unimplemented,
        "13" | "internal" => tonic::Code::Internal,
        "14" | "unavailable" => tonic::Code::Unavailable,
        "15" | "data_loss" => tonic::Code::DataLoss,
        "16" | "unauthenticated" => tonic::Code::Unauthenticated,
        other => {
            warn!("Unknown Elixir gRPC error code '{}', mapping to INTERNAL", other);
            tonic::Code::Internal
        }
    };

    tonic::Status::new(tonic_code, message.to_string())
}

fn metadata_map_to_hashmap(metadata: &MetadataMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for key_value in metadata.iter() {
        if let tonic::metadata::KeyAndValueRef::Ascii(key, value) = key_value
            && let Ok(value_str) = value.to_str()
        {
            map.insert(key.as_str().to_string(), value_str.to_string());
        }
    }
    map
}

fn hashmap_to_metadata_map(map: &HashMap<String, String>) -> Result<MetadataMap, tonic::Status> {
    let mut metadata = MetadataMap::new();
    for (key, value) in map {
        let metadata_key = key
            .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
            .map_err(|err| tonic::Status::invalid_argument(format!("Invalid metadata key '{}': {}", key, err)))?;
        let metadata_value = value
            .parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
            .map_err(|err| {
                tonic::Status::invalid_argument(format!("Invalid metadata value for key '{}': {}", key, err))
            })?;
        metadata.insert(metadata_key, metadata_value);
    }
    Ok(metadata)
}

fn decode_term_string(term: Term<'_>) -> Result<String, rustler::Error> {
    if let Ok(value) = term.decode::<String>() {
        return Ok(value);
    }
    if let Ok(value) = term.decode::<i64>() {
        return Ok(value.to_string());
    }
    if let Ok(atom) = term.decode::<Atom>() {
        return Ok(format!("{:?}", atom).trim_start_matches(':').to_string());
    }
    Err(rustler::Error::BadArg)
}

fn decode_string_map(term: Term<'_>) -> Result<HashMap<String, String>, String> {
    let Some(iter) = MapIterator::new(term) else {
        return Err("expected metadata map".to_string());
    };

    let mut map = HashMap::new();
    for (key_term, value_term) in iter {
        let key = decode_term_string(key_term).map_err(|_| "metadata keys must be strings or atoms".to_string())?;
        let value =
            decode_term_string(value_term).map_err(|_| format!("metadata value for key '{}' must be a string", key))?;
        map.insert(key, value);
    }
    Ok(map)
}

fn decode_binary(term: Term<'_>) -> Result<Vec<u8>, String> {
    if let Ok(bytes) = term.decode::<Vec<u8>>() {
        return Ok(bytes);
    }
    if let Ok(binary) = term.decode::<Binary<'_>>() {
        return Ok(binary.as_slice().to_vec());
    }
    Err("expected binary payload".to_string())
}

fn binary_term_from_slice<'a>(env: Env<'a>, bytes: &[u8]) -> Result<Term<'a>, String> {
    let mut binary = OwnedBinary::new(bytes.len()).ok_or_else(|| "failed to allocate Elixir binary".to_string())?;
    binary.as_mut_slice().copy_from_slice(bytes);
    Ok(Binary::from_owned(binary, env).encode(env))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_rpc_modes() {
        assert_eq!(parse_rpc_mode("unary").unwrap(), RpcMode::Unary);
        assert_eq!(parse_rpc_mode("server_stream").unwrap(), RpcMode::ServerStreaming);
        assert_eq!(parse_rpc_mode("client_stream").unwrap(), RpcMode::ClientStreaming);
        assert_eq!(parse_rpc_mode("bidi_stream").unwrap(), RpcMode::BidirectionalStreaming);
    }

    #[test]
    fn maps_grpc_error_codes() {
        assert_eq!(
            grpc_error_to_status("unimplemented", "x").code(),
            tonic::Code::Unimplemented
        );
        assert_eq!(grpc_error_to_status("14", "x").code(), tonic::Code::Unavailable);
        assert_eq!(grpc_error_to_status("bogus", "x").code(), tonic::Code::Internal);
    }
}
