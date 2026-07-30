#![allow(clippy::too_many_arguments, clippy::unused_async, non_snake_case)]

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jint, jlong};
use jni::{AttachGuard, Env, EnvUnowned};
use serde_json;
use std::sync::Arc;
use std::sync::OnceLock;
/// Opaque handle to a App service instance.
/// Allocated by constructor_app(), freed by free_app().
/// Thread safety: this handle wraps the Rust owner, which may not be Send/Sync.
/// The JNI binding layer is responsible for thread synchronization via JVM thread attachment.
#[repr(C)]
pub struct AppOpaque {
    pub inner: spikard::App,
}

/// Allocate a new App instance.
///
/// Returns the address as a jlong pointer. This pointer must be freed via free_app().
/// Never dereference this pointer after freeing it.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppNew() -> jlong {
    let owner = spikard::App::new();
    let opaque = Box::new(AppOpaque { inner: owner });
    Box::into_raw(opaque) as jlong
}

/// Free a App instance allocated by constructor_app().
///
/// # Safety
/// - handle must have been allocated by constructor_app().
/// - After this call, handle is invalid and must not be dereferenced.
/// - Calling this twice on the same handle causes undefined behavior.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppFree(
    _env: EnvUnowned,
    _class: JClass,
    handle: jlong,
) {
    if handle != 0 {
        // SAFETY: handle was allocated by into_raw above; we are the sole owner
        // and this is the final drop.
        unsafe {
            let _ = Box::from_raw(handle as *mut AppOpaque);
        }
    }
}
/// Generated JNI bridge for the `Handler` contract.
///
/// Wraps a global JVM reference to a Java handler object so it can be used
/// as `Arc<dyn Handler>` from Rust async code.
pub struct JniHandlerBridge {
    /// Global JVM reference to the Java handler object.
    global_ref: jni::refs::Global<jni::objects::JObject<'static>>,
    /// The JavaVM pointer for thread attachment.
    jvm: jni::JavaVM,
    /// Method ID for the dispatch method (cached for performance).
    method_id: jni::sys::jmethodID,
}

// SAFETY: GlobalRef is Send+Sync once obtained in JVM context.
// JavaVM is Send+Sync per jni crate semantics (one global VM per process).
// jmethodID is stable for the method lifetime.
unsafe impl Send for JniHandlerBridge {}
unsafe impl Sync for JniHandlerBridge {}
impl spikard::Handler for JniHandlerBridge {
    fn call(
        &self,
        _request: spikard::Request<spikard::Body>,
        request_data: spikard::RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard::HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let outcome: Result<spikard::Response, Box<dyn std::error::Error + Send + Sync>> = async move {
                let req_json = serde_json::to_string(&request_data)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                let result_json = {
                    let env = self.jvm.attach_current_thread().map_err(|e| {
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("failed to attach JVM thread: {}", e),
                        )) as Box<dyn std::error::Error + Send + Sync>
                    })?;

                    let req_jni = env.new_string(&req_json).map_err(|e| {
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("failed to create JNI string: {}", e),
                        )) as Box<dyn std::error::Error + Send + Sync>
                    })?;

                    let result: jni::sys::jstring = unsafe {
                        // SAFETY: method_id was validated when bridge was created.
                        // self.global_ref is valid for the JVM's lifetime.
                        env.call_method_unchecked(
                            self.global_ref.as_obj(),
                            self.method_id,
                            jni::sys::JNI_ABORT,
                            &[jni::objects::JValue::from(&req_jni)],
                        )?
                        .l()?
                        .as_raw()
                    };

                    let result_obj = unsafe {
                        // SAFETY: result is a valid jstring from the JNI call.
                        jni::objects::JString::from_raw(result)
                    };
                    env.get_string(&result_obj)?.into_string().map_err(|e| {
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("response is not valid UTF-8: {}", e),
                        )) as Box<dyn std::error::Error + Send + Sync>
                    })?
                };

                let response: spikard::Response = serde_json::from_str(&result_json)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                Ok(response)
            }
            .await;

            spikard::handler_result_from_response(outcome)
        })
    }
}
/// Register a Java handler for `App::Route`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRoute(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    builder: jni::objects::JObject,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque).inner.route(builder, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Register a Java handler for `App::get`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRouteGet(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    path: jni::objects::JString,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    let builder = spikard::RouteBuilder::new(spikard::Method::Get, path);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque).inner.route(builder, spikard::Method::Get, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Register a Java handler for `App::post`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRoutePost(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    path: jni::objects::JString,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    let builder = spikard::RouteBuilder::new(spikard::Method::Post, path);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque).inner.route(builder, spikard::Method::Post, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Register a Java handler for `App::put`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRoutePut(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    path: jni::objects::JString,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    let builder = spikard::RouteBuilder::new(spikard::Method::Put, path);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque).inner.route(builder, spikard::Method::Put, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Register a Java handler for `App::patch`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRoutePatch(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    path: jni::objects::JString,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    let builder = spikard::RouteBuilder::new(spikard::Method::Patch, path);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque)
            .inner
            .route(builder, spikard::Method::Patch, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Register a Java handler for `App::delete`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRouteDelete(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    path: jni::objects::JString,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    let builder = spikard::RouteBuilder::new(spikard::Method::Delete, path);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque)
            .inner
            .route(builder, spikard::Method::Delete, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Register a Java handler for `App::head`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRouteHead(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    path: jni::objects::JString,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    let builder = spikard::RouteBuilder::new(spikard::Method::Head, path);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque).inner.route(builder, spikard::Method::Head, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Register a Java handler for `App::options`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRouteOptions(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    path: jni::objects::JString,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    let builder = spikard::RouteBuilder::new(spikard::Method::Options, path);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque)
            .inner
            .route(builder, spikard::Method::Options, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Register a Java handler for `App::connect`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRouteConnect(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    path: jni::objects::JString,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    let builder = spikard::RouteBuilder::new(spikard::Method::Connect, path);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque)
            .inner
            .route(builder, spikard::Method::Connect, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Register a Java handler for `App::trace`.
///
/// Called from Java/Kotlin to provide a handler implementation.
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   handler: the Java handler object
///   metadata params: route pattern, HTTP method, etc.
///
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRegisterRouteTrace(
    env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
    handler: JObject,
    path: jni::objects::JString,
) -> jint {
    // Validate owner handle
    if owner_handle == 0 {
        return 1; // Error: null pointer
    }

    let jvm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(_) => return 2, // Error: failed to get JavaVM
    };

    let global_ref = match env.new_global_ref(&handler) {
        Ok(g) => g,
        Err(_) => return 3, // Error: failed to create global reference
    };

    let method_id = match env.get_method_id(&handler, "call", "(Ljava/lang/String;)Ljava/lang/String;") {
        Ok(id) => id,
        Err(_) => return 4, // Error: failed to find method
    };

    let bridge = JniHandlerBridge {
        global_ref,
        jvm,
        method_id,
    };
    let handler_arc: Arc<dyn spikard::Handler> = Arc::new(bridge);

    let builder = spikard::RouteBuilder::new(spikard::Method::Trace, path);

    // SAFETY: owner_handle was returned by the service constructor and
    // is valid until freed. The caller is responsible for ensuring no use-after-free.
    match unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        (*owner_opaque)
            .inner
            .route(builder, spikard::Method::Trace, handler_arc)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 5, // Error: registration failed
    }
}
/// Drive `App::Run` from Java/Kotlin.
///
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   ep params: as defined in the service entrypoint signature
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppRun(
    _env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
) {
    // Validate owner handle
    if owner_handle == 0 {
        return;
    }

    // SAFETY: owner_handle was allocated by the constructor and is valid
    // until freed. The caller is responsible for not using after free.
    unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        let owner_ref = &mut (*owner_opaque).inner;
        let rt = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(_) => return, // Failed to create tokio runtime
        };

        let _ = rt.block_on(owner_ref.run());
    }
}
/// Drive `App::IntoRouter` from Java/Kotlin.
///
/// Parameters:
///   owner_handle: jlong returned by the service constructor entry point
///   ep params: as defined in the service entrypoint signature
#[no_mangle]
pub extern "system" fn Java_dev_spikard_AppServiceBridge_nativeAppIntoRouter(
    _env: EnvUnowned,
    _class: JClass,
    owner_handle: jlong,
) -> jlong {
    // Validate owner handle
    if owner_handle == 0 {
        return 0; // Error: null pointer
    }

    // SAFETY: owner_handle was allocated by the constructor and is valid
    // until freed. The caller is responsible for not using after free.
    unsafe {
        let owner_opaque = owner_handle as *mut AppOpaque;
        let owner_ref = &mut (*owner_opaque).inner;
        let rt = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(_) => return 0, // Error: failed to create tokio runtime
        };

        let _result = rt.block_on(owner_ref.into_router());
        // Finalize returns the transformed result; caller decides what to do with it
        owner_handle
    }
}
