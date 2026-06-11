import Foundation
import RustBridge

@_silgen_name("spikard_server_config_from_json")
private func _spikard_server_config_from_json(_ json: UnsafePointer<CChar>)
  -> UnsafeMutableRawPointer?

@_silgen_name("spikard_server_config_free")
private func _spikard_server_config_free(_ ptr: UnsafeMutableRawPointer?)

@_silgen_name("spikard_app_config")
private func _spikard_app_config(
  _ app: UnsafeMutablePointer<OpaquePointer>, _ config: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer?

/// Errors thrown by service wrapper methods.
public enum ServiceError: Error {
  /// The service handle was already consumed or never initialised.
  case invalidHandle
  /// The C-side registration call returned a non-zero status code.
  case registrationFailed
  /// The service runtime returned the given error envelope.
  case runtime(String)
}

// C function pointer for app.route callback registration
// This function is defined as a plain extern "C" in the Rust crate (outside the swift-bridge module).
// Swift calls it via this @_silgen_name import at module scope.
@_silgen_name("app_route_via_callback")
private func _app_route_via_callback(
  _ app: UnsafeMutablePointer<OpaquePointer>, _ builder: RustBridge.RouteBuilder,
  _ ctx: UnsafeMutableRawPointer?,
  _ callback:
    @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<UInt8>?, Int) -> UnsafeMutablePointer<
      UInt8
    >?
) -> Int32
/// Spikard application builder.
public final class App {

  private var inner: RustBridge.App?

  /// Retained handler boxes. Each box is passed to the C layer as the
  /// trampoline context pointer and released in `deinit` to avoid leaks.
  private var handlerBoxes: [UnsafeMutableRawPointer] = []

  /// Boxes a handler closure so it can travel through a C context pointer.
  private final class HandlerBox {
    let handler: (String) -> String
    init(_ handler: @escaping (String) -> String) { self.handler = handler }
  }
  /// Create a new service instance.
  public init() {
    // swift-bridge generates `class App` with a `convenience init()`
    // wrapping `__swift_bridge__$App$new`. Call the constructor directly
    // instead of looking for a free `appNew()` module function.
    self.inner = RustBridge.App()
  }
  /// Free the service instance.
  deinit {
    inner = nil
    // Release every retained handler box.
    for contextPtr in handlerBoxes {
      Unmanaged<HandlerBox>.fromOpaque(contextPtr).release()
    }
    handlerBoxes.removeAll()
  }
  /// Set the server configuration.
  @discardableResult
  public func config() -> Self {
    guard let inner = inner else { return self }
    // swift-bridge emits `fn config(client: &mut App);` as a free Swift function
    // `RustBridge.config(_ client: AppRefMut)`; the wrapper class itself
    // does not gain `inner.config()`. Call through the module function instead.
    RustBridge.config(inner)
    return self
  }
  /// Configure server host and port.
  ///
  /// Creates a ServerConfig with the given host and port,
  /// and applies it to the server via the FFI layer.
  @discardableResult
  public func config(host: String, port: UInt16) throws -> Self {
    guard let inner = inner else { throw ServiceError.invalidHandle }

    // Build ServerConfig JSON with host and port
    let configDict: [String: Any] = [
      "host": host,
      "port": port,
    ]
    guard let configData = try? JSONSerialization.data(withJSONObject: configDict, options: []),
      let configJson = String(data: configData, encoding: .utf8)
    else {
      throw ServiceError.runtime("Failed to serialize server config")
    }

    // Call FFI to create ServerConfig from JSON (null-terminated C string)
    let serverConfig = configJson.withCString { cStr in
      _spikard_server_config_from_json(cStr)
    }
    guard serverConfig != nil else {
      throw ServiceError.runtime("Failed to create ServerConfig from JSON")
    }
    defer {
      // Free the ServerConfig after applying it
      _spikard_server_config_free(serverConfig)
    }

    // Apply the config to the app via the FFI bridge.
    // swift-bridge wraps the opaque `App` type; we need its raw pointer.
    let rawAddr = RustBridge.appRawPtr(inner)
    var innerPtr = OpaquePointer(bitPattern: rawAddr)!
    let _ = _spikard_app_config(&innerPtr, serverConfig)

    return self
  }
  /// Register a route using the provided builder and handler function.
  ///
  /// # Errors
  ///
  /// Returns an error if route construction fails or if the handler registration fails.
  public func route(_ handler: @escaping (String) -> String, builder: RustBridge.RouteBuilder)
    throws
  {
    // Box the handler and retain it; the box pointer is passed to the
    // C layer as the trampoline context and released in deinit.
    let handlerBox = HandlerBox(handler)
    let contextPtr = Unmanaged.passRetained(handlerBox).toOpaque()
    handlerBoxes.append(contextPtr)

    // Create a C-compatible callback wrapper
    let trampolineFunc:
      @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<UInt8>?, Int) -> UnsafeMutablePointer<
        UInt8
      >? = { contextPtr, requestPtr, requestLen in
        guard let contextPtr = contextPtr else { return nil }
        guard let requestPtr = requestPtr else { return nil }

        // Recover the boxed handler closure from the context pointer
        let handlerBox = Unmanaged<HandlerBox>.fromOpaque(contextPtr).takeUnretainedValue()
        let requestData = Data(bytes: requestPtr, count: requestLen)
        let requestJSON = String(data: requestData, encoding: .utf8) ?? ""
        let responseJSON = handlerBox.handler(requestJSON)

        // Allocate response string on heap (Rust side frees via extern "C" { fn free })
        let responseBytes = responseJSON.utf8CString
        let responsePtr = UnsafeMutablePointer<UInt8>.allocate(capacity: responseBytes.count)
        for (i, byte) in responseBytes.enumerated() {
          responsePtr[i] = UInt8(bitPattern: byte)
        }
        return responsePtr
      }

    guard let inner = inner else { throw ServiceError.invalidHandle }

    // Call the C function via @_silgen_name. The callback registration is defined
    // OUTSIDE the swift-bridge module as a plain extern "C" function. swift-bridge
    // hides the wrapper's raw pointer behind an `internal` field, so call through
    // the alef-emitted `RustBridge.appRawPtr` shim which returns
    // the App address as a `usize` we can reconstitute into an OpaquePointer.
    let rawAddr = RustBridge.appRawPtr(inner)
    var innerPtr = OpaquePointer(bitPattern: rawAddr)!
    let result = _app_route_via_callback(
      &innerPtr,
      builder,
      contextPtr,
      trampolineFunc
    )
    guard result == 0 else { throw ServiceError.registrationFailed }
  }
  /// Register a GET route at the given path.
  public func get(_ handler: @escaping (String) -> String, path: String) throws {
    // Construct the wrapper metadata param (e.g. RouteBuilder) from the fixed enum arg
    // and any free args. swift-bridge generates enums as opaque classes — there are no
    // static member constants (e.g. `RustBridge.Method.Get` is invalid). Instead,
    // each opaque enum type has a `<type>FromJson` factory that parses a serde JSON
    // string into an opaque instance. The wrapper constructor factory `routeBuilderNew`
    // is a bridge-declared free function that calls the Rust wrapper's `new` method.
    let __builder = RustBridge.routeBuilderNew(try methodFromJson("\"Get\""), path)
    // Delegate to the base registration method, passing the constructed wrapper as the
    // `builder` metadata param. This reuses all the handler boxing and
    // trampoline logic from the base method without duplicating it.
    try route(handler, builder: __builder)
  }
  /// Register a POST route at the given path.
  public func post(_ handler: @escaping (String) -> String, path: String) throws {
    // Construct the wrapper metadata param (e.g. RouteBuilder) from the fixed enum arg
    // and any free args. swift-bridge generates enums as opaque classes — there are no
    // static member constants (e.g. `RustBridge.Method.Get` is invalid). Instead,
    // each opaque enum type has a `<type>FromJson` factory that parses a serde JSON
    // string into an opaque instance. The wrapper constructor factory `routeBuilderNew`
    // is a bridge-declared free function that calls the Rust wrapper's `new` method.
    let __builder = RustBridge.routeBuilderNew(try methodFromJson("\"Post\""), path)
    // Delegate to the base registration method, passing the constructed wrapper as the
    // `builder` metadata param. This reuses all the handler boxing and
    // trampoline logic from the base method without duplicating it.
    try route(handler, builder: __builder)
  }
  /// Register a PUT route at the given path.
  public func put(_ handler: @escaping (String) -> String, path: String) throws {
    // Construct the wrapper metadata param (e.g. RouteBuilder) from the fixed enum arg
    // and any free args. swift-bridge generates enums as opaque classes — there are no
    // static member constants (e.g. `RustBridge.Method.Get` is invalid). Instead,
    // each opaque enum type has a `<type>FromJson` factory that parses a serde JSON
    // string into an opaque instance. The wrapper constructor factory `routeBuilderNew`
    // is a bridge-declared free function that calls the Rust wrapper's `new` method.
    let __builder = RustBridge.routeBuilderNew(try methodFromJson("\"Put\""), path)
    // Delegate to the base registration method, passing the constructed wrapper as the
    // `builder` metadata param. This reuses all the handler boxing and
    // trampoline logic from the base method without duplicating it.
    try route(handler, builder: __builder)
  }
  /// Register a PATCH route at the given path.
  public func patch(_ handler: @escaping (String) -> String, path: String) throws {
    // Construct the wrapper metadata param (e.g. RouteBuilder) from the fixed enum arg
    // and any free args. swift-bridge generates enums as opaque classes — there are no
    // static member constants (e.g. `RustBridge.Method.Get` is invalid). Instead,
    // each opaque enum type has a `<type>FromJson` factory that parses a serde JSON
    // string into an opaque instance. The wrapper constructor factory `routeBuilderNew`
    // is a bridge-declared free function that calls the Rust wrapper's `new` method.
    let __builder = RustBridge.routeBuilderNew(try methodFromJson("\"Patch\""), path)
    // Delegate to the base registration method, passing the constructed wrapper as the
    // `builder` metadata param. This reuses all the handler boxing and
    // trampoline logic from the base method without duplicating it.
    try route(handler, builder: __builder)
  }
  /// Register a DELETE route at the given path.
  public func delete(_ handler: @escaping (String) -> String, path: String) throws {
    // Construct the wrapper metadata param (e.g. RouteBuilder) from the fixed enum arg
    // and any free args. swift-bridge generates enums as opaque classes — there are no
    // static member constants (e.g. `RustBridge.Method.Get` is invalid). Instead,
    // each opaque enum type has a `<type>FromJson` factory that parses a serde JSON
    // string into an opaque instance. The wrapper constructor factory `routeBuilderNew`
    // is a bridge-declared free function that calls the Rust wrapper's `new` method.
    let __builder = RustBridge.routeBuilderNew(try methodFromJson("\"Delete\""), path)
    // Delegate to the base registration method, passing the constructed wrapper as the
    // `builder` metadata param. This reuses all the handler boxing and
    // trampoline logic from the base method without duplicating it.
    try route(handler, builder: __builder)
  }
  /// Register a HEAD route at the given path.
  public func head(_ handler: @escaping (String) -> String, path: String) throws {
    // Construct the wrapper metadata param (e.g. RouteBuilder) from the fixed enum arg
    // and any free args. swift-bridge generates enums as opaque classes — there are no
    // static member constants (e.g. `RustBridge.Method.Get` is invalid). Instead,
    // each opaque enum type has a `<type>FromJson` factory that parses a serde JSON
    // string into an opaque instance. The wrapper constructor factory `routeBuilderNew`
    // is a bridge-declared free function that calls the Rust wrapper's `new` method.
    let __builder = RustBridge.routeBuilderNew(try methodFromJson("\"Head\""), path)
    // Delegate to the base registration method, passing the constructed wrapper as the
    // `builder` metadata param. This reuses all the handler boxing and
    // trampoline logic from the base method without duplicating it.
    try route(handler, builder: __builder)
  }
  /// Register an OPTIONS route at the given path.
  public func options(_ handler: @escaping (String) -> String, path: String) throws {
    // Construct the wrapper metadata param (e.g. RouteBuilder) from the fixed enum arg
    // and any free args. swift-bridge generates enums as opaque classes — there are no
    // static member constants (e.g. `RustBridge.Method.Get` is invalid). Instead,
    // each opaque enum type has a `<type>FromJson` factory that parses a serde JSON
    // string into an opaque instance. The wrapper constructor factory `routeBuilderNew`
    // is a bridge-declared free function that calls the Rust wrapper's `new` method.
    let __builder = RustBridge.routeBuilderNew(try methodFromJson("\"Options\""), path)
    // Delegate to the base registration method, passing the constructed wrapper as the
    // `builder` metadata param. This reuses all the handler boxing and
    // trampoline logic from the base method without duplicating it.
    try route(handler, builder: __builder)
  }
  /// Register a CONNECT route at the given path.
  public func connect(_ handler: @escaping (String) -> String, path: String) throws {
    // Construct the wrapper metadata param (e.g. RouteBuilder) from the fixed enum arg
    // and any free args. swift-bridge generates enums as opaque classes — there are no
    // static member constants (e.g. `RustBridge.Method.Get` is invalid). Instead,
    // each opaque enum type has a `<type>FromJson` factory that parses a serde JSON
    // string into an opaque instance. The wrapper constructor factory `routeBuilderNew`
    // is a bridge-declared free function that calls the Rust wrapper's `new` method.
    let __builder = RustBridge.routeBuilderNew(try methodFromJson("\"Connect\""), path)
    // Delegate to the base registration method, passing the constructed wrapper as the
    // `builder` metadata param. This reuses all the handler boxing and
    // trampoline logic from the base method without duplicating it.
    try route(handler, builder: __builder)
  }
  /// Register a TRACE route at the given path.
  public func trace(_ handler: @escaping (String) -> String, path: String) throws {
    // Construct the wrapper metadata param (e.g. RouteBuilder) from the fixed enum arg
    // and any free args. swift-bridge generates enums as opaque classes — there are no
    // static member constants (e.g. `RustBridge.Method.Get` is invalid). Instead,
    // each opaque enum type has a `<type>FromJson` factory that parses a serde JSON
    // string into an opaque instance. The wrapper constructor factory `routeBuilderNew`
    // is a bridge-declared free function that calls the Rust wrapper's `new` method.
    let __builder = RustBridge.routeBuilderNew(try methodFromJson("\"Trace\""), path)
    // Delegate to the base registration method, passing the constructed wrapper as the
    // `builder` metadata param. This reuses all the handler boxing and
    // trampoline logic from the base method without duplicating it.
    try route(handler, builder: __builder)
  }
  /// Run the HTTP server using the configured routes.
  ///
  /// # Errors
  ///
  /// Returns an error if server construction or execution fails.
  public func run() throws {
    guard let inner = inner else { throw ServiceError.invalidHandle }

    // swift-bridge emits `fn run(client: &mut App) -> String;` as a free
    // module function — the wrapper class itself does not gain `inner.run()`.
    let result = RustBridge.run(inner).toString()

    // The Rust wrapper flattens `Result<T, E>` into a String envelope:
    // empty string means success, non-empty carries the formatted error.
    // swift-bridge 0.1.59 does not bridge native Swift `Result`s.
    if !result.isEmpty {
      throw ServiceError.runtime(result)
    }
  }
}
