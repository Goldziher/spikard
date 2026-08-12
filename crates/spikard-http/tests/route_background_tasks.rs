//! `ServerConfig.background_tasks` starts an executor during router construction, and every
//! request carries a `BackgroundHandle` extension so a handler can enqueue work that outlives the
//! response. Modelled on `fixtures/background.json` (`background_event_logging` and
//! `background_event_logging_second_payload`): POST an event, get 202 back immediately, and see
//! the event recorded by a job that ran after the response was produced.

use axum::http::StatusCode;
use serde_json::json;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{
    BackgroundHandle, BackgroundTaskConfig, Handler, HandlerResult, Method, RequestData, Route, ServerConfig,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Events recorded by background jobs, shared between the handler and the assertions.
type EventLog = Arc<Mutex<Vec<String>>>;

/// Enqueues the posted event onto the background executor reached through the request extension,
/// and answers 202 without waiting for the job to run.
struct EnqueueEventHandler {
    events: EventLog,
}

impl Handler for EnqueueEventHandler {
    fn call(
        &self,
        request: axum::http::Request<axum::body::Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let events = Arc::clone(&self.events);
        let background = request.extensions().get::<BackgroundHandle>().cloned();

        Box::pin(async move {
            let Some(background) = background else {
                return Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    json!({"error": "no background handle"}).to_string(),
                ));
            };

            let event = request_data
                .body
                .get("event")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_string();

            background
                .spawn(move || async move {
                    events.lock().expect("event log lock").push(event);
                    Ok(())
                })
                .map_err(|error| (StatusCode::SERVICE_UNAVAILABLE, error.to_string()))?;

            Ok(axum::http::Response::builder()
                .status(StatusCode::ACCEPTED)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(json!({"status": "queued"}).to_string()))
                .unwrap())
        })
    }
}

/// Enqueues `SLOW_JOB_COUNT` slow jobs and answers with the number the executor refused, so a
/// test can assert the configured `max_queue_size` is really in force.
struct QueueFloodingHandler;

const SLOW_JOB_COUNT: usize = 64;

impl Handler for QueueFloodingHandler {
    fn call(
        &self,
        request: axum::http::Request<axum::body::Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let background = request.extensions().get::<BackgroundHandle>().cloned();
        Box::pin(async move {
            let background = background.expect("background handle");
            let mut rejected = 0_usize;
            for _ in 0..SLOW_JOB_COUNT {
                if background
                    .spawn(|| async {
                        tokio::time::sleep(Duration::from_millis(200)).await;
                        Ok(())
                    })
                    .is_err()
                {
                    rejected += 1;
                }
            }
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .body(axum::body::Body::from(rejected.to_string()))
                .unwrap())
        })
    }
}

fn events_route() -> Route {
    Route {
        method: Method::Post,
        path: "/background/events".to_string(),
        handler_name: "log_event".to_string(),
        expects_json_body: true,
        ..Default::default()
    }
}

fn build_router(background_enabled: bool, events: EventLog) -> Result<axum::Router, String> {
    let config = ServerConfig {
        background_tasks: BackgroundTaskConfig {
            enabled: background_enabled,
            max_queue_size: 16,
            max_concurrent_tasks: 4,
            drain_timeout_secs: 5,
        },
        ..ServerConfig::default()
    };

    build_router_with_handlers_and_config(
        vec![(
            events_route(),
            Arc::new(EnqueueEventHandler { events }) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
}

/// Background jobs complete after their response, so the assertion has to wait for one. Polls
/// rather than sleeping a fixed interval so a slow machine does not turn into a flake. ~keep
async fn await_events(events: &EventLog, expected: usize) -> Vec<String> {
    const POLL_TIMEOUT: Duration = Duration::from_secs(5);
    const POLL_INTERVAL: Duration = Duration::from_millis(10);

    let deadline = Instant::now() + POLL_TIMEOUT;
    loop {
        {
            let recorded = events.lock().expect("event log lock");
            if recorded.len() >= expected {
                return recorded.clone();
            }
        }
        if Instant::now() >= deadline {
            return events.lock().expect("event log lock").clone();
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

#[tokio::test]
async fn handler_enqueues_a_background_job_that_runs_after_the_response() {
    let events: EventLog = Arc::new(Mutex::new(Vec::new()));
    let server = axum_test::TestServer::new(build_router(true, Arc::clone(&events)).expect("router"));

    let response = server.post("/background/events").json(&json!({"event": "alpha"})).await;

    assert_eq!(response.status_code(), StatusCode::ACCEPTED);
    assert_eq!(
        response.header("content-type").to_str().expect("content-type"),
        "application/json"
    );
    assert_eq!(
        await_events(&events, 1).await,
        vec!["alpha".to_string()],
        "the enqueued job must actually execute on the background executor started by the router"
    );
}

#[tokio::test]
async fn background_jobs_from_successive_requests_all_execute() {
    let events: EventLog = Arc::new(Mutex::new(Vec::new()));
    let server = axum_test::TestServer::new(build_router(true, Arc::clone(&events)).expect("router"));

    for event in ["alpha", "beta"] {
        let response = server.post("/background/events").json(&json!({"event": event})).await;
        assert_eq!(response.status_code(), StatusCode::ACCEPTED);
    }

    let mut recorded = await_events(&events, 2).await;
    recorded.sort();
    assert_eq!(recorded, vec!["alpha".to_string(), "beta".to_string()]);
}

/// Without this, "the handle is reachable" could be true of every router ever built, which would
/// make the tests above pass whether or not the config was ever read.
#[tokio::test]
async fn no_background_handle_reaches_handlers_when_background_tasks_are_disabled() {
    let events: EventLog = Arc::new(Mutex::new(Vec::new()));
    let server = axum_test::TestServer::new(build_router(false, Arc::clone(&events)).expect("router"));

    let response = server.post("/background/events").json(&json!({"event": "alpha"})).await;

    assert_eq!(
        response.status_code(),
        StatusCode::SERVICE_UNAVAILABLE,
        "with background tasks disabled the handler must find no BackgroundHandle extension"
    );
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(
        events.lock().expect("event log lock").is_empty(),
        "nothing can have been executed when no executor was started"
    );
}

/// `BackgroundRuntime` spawns onto the ambient Tokio runtime, and router construction is
/// synchronous, so a caller outside a runtime must get an error rather than a panic.
#[test]
fn enabling_background_tasks_outside_a_tokio_runtime_fails_the_router_build() {
    let events: EventLog = Arc::new(Mutex::new(Vec::new()));

    let error = build_router(true, events).expect_err("router build must fail with no Tokio runtime");

    assert!(
        error.contains("outside a Tokio runtime"),
        "the build error must explain why no executor could be spawned; got: {error}"
    );
}

#[tokio::test]
async fn queue_overflow_is_reported_to_the_handler_rather_than_dropped() {
    let events: EventLog = Arc::new(Mutex::new(Vec::new()));
    let config = ServerConfig {
        background_tasks: BackgroundTaskConfig {
            enabled: true,
            max_queue_size: 1,
            max_concurrent_tasks: 1,
            drain_timeout_secs: 5,
        },
        ..ServerConfig::default()
    };

    let router = build_router_with_handlers_and_config(
        vec![(events_route(), Arc::new(QueueFloodingHandler) as Arc<dyn Handler>)],
        config,
        Vec::new(),
    )
    .expect("router");
    let server = axum_test::TestServer::new(router);

    let response = server.post("/background/events").json(&json!({"event": "alpha"})).await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let rejected: usize = response.text().parse().expect("rejection count");
    assert!(
        rejected > 0,
        "a queue of 1 must reject some of the immediately-enqueued slow jobs, so the configured \
         max_queue_size is demonstrably in force"
    );
    drop(events);
}
