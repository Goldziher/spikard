/**
 * Ergonomic typed-handler App wrapper for spikard (CommonJS).
 *
 * This module provides a high-level `App` interface with verb decorators
 * (app.post, app.get, etc.) that support typed request/response handling via zod schemas.
 *
 * Request validation happens in the Rust core and returns 422 ProblemDetails on validation
 * failure. Zod schemas are converted to JSON Schema and attached to routes. Handlers receive
 * validated, typed request data.
 *
 * The low-level service App (supporting raw handler registration) is available via
 * `const { App } = require('@spikard/node/service.cjs')` for advanced use cases.
 */

// Pull the low-level primitives straight from their source modules — NOT from
// `index-wrapper.cjs`. The wrapper is the package `main` and re-exports this
// ergonomic `App` last (so `require('@spikard/node').App` is ergonomic); reading
// `App` back from the wrapper here would resolve to this class and recurse
// infinitely. `service.cjs` owns the low-level service `App`; the native addon
// (`./index`) owns `Method`/`RouteBuilder`. Loaded lazily to avoid load-time cycles.
let _cachedService = null;

function getServiceAndBindings() {
  if (_cachedService !== null) {
    return _cachedService;
  }

  const service = require("./service.cjs");
  const native = require("./index");
  _cachedService = {
    App: service.App,
    Method: native.Method,
    RouteBuilder: native.RouteBuilder,
  };
  return _cachedService;
}

// zod ships a native JSON Schema converter. In zod v4 it is a top-level export
// (`require('zod').toJSONSchema`); in zod 3.25.x the v4 API lives under the
// `zod/v4` subpath. Resolve whichever is present.
function resolveZodToJsonSchema() {
  const zod = require("zod");
  if (typeof zod.toJSONSchema === "function") return zod.toJSONSchema;
  try {
    const v4 = require("zod/v4");
    if (typeof v4.toJSONSchema === "function") return v4.toJSONSchema;
  } catch (_e) {
    // no zod/v4 subpath — fall through to the error below
  }
  throw new Error("spikard: the ergonomic App requires zod >= 3.25 (with toJSONSchema)");
}
const zodToJsonSchema = resolveZodToJsonSchema();

/**
 * Ergonomic, typed-handler App wrapper.
 *
 * Provides a clean interface for defining routes with type-safe handlers.
 * Automatically converts zod schemas to JSON Schema and attaches them to routes.
 * The Rust core validates all inputs and returns 422 ProblemDetails on validation failure.
 * Handlers receive typed, validated request data.
 */
class App {
  constructor() {
    const service = getServiceAndBindings();
    this.serviceApp = new service.App();
  }

  /**
   * Register a GET route with optional typed request validation.
   */
  get(path, config = {}, handler) {
    // Handle overloads: get(path, handler) or get(path, config, handler)
    if (typeof config === "function") {
      handler = config;
      config = {};
    }
    this.registerRoute("GET", path, config, handler);
  }

  /**
   * Register a POST route with optional typed request validation.
   */
  post(path, config = {}, handler) {
    // Handle overloads: post(path, handler) or post(path, config, handler)
    if (typeof config === "function") {
      handler = config;
      config = {};
    }
    this.registerRoute("POST", path, config, handler);
  }

  /**
   * Register a PUT route with optional typed request validation.
   */
  put(path, config = {}, handler) {
    // Handle overloads: put(path, handler) or put(path, config, handler)
    if (typeof config === "function") {
      handler = config;
      config = {};
    }
    this.registerRoute("PUT", path, config, handler);
  }

  /**
   * Register a PATCH route with optional typed request validation.
   */
  patch(path, config = {}, handler) {
    // Handle overloads: patch(path, handler) or patch(path, config, handler)
    if (typeof config === "function") {
      handler = config;
      config = {};
    }
    this.registerRoute("PATCH", path, config, handler);
  }

  /**
   * Register a DELETE route with optional typed request validation.
   */
  delete(path, config = {}, handler) {
    // Handle overloads: delete(path, handler) or delete(path, config, handler)
    if (typeof config === "function") {
      handler = config;
      config = {};
    }
    this.registerRoute("DELETE", path, config, handler);
  }

  /**
   * Register a HEAD route with optional typed request validation.
   */
  head(path, config = {}, handler) {
    // Handle overloads: head(path, handler) or head(path, config, handler)
    if (typeof config === "function") {
      handler = config;
      config = {};
    }
    this.registerRoute("HEAD", path, config, handler);
  }

  /**
   * Register an OPTIONS route with optional typed request validation.
   */
  options(path, config = {}, handler) {
    // Handle overloads: options(path, handler) or options(path, config, handler)
    if (typeof config === "function") {
      handler = config;
      config = {};
    }
    this.registerRoute("OPTIONS", path, config, handler);
  }

  /**
   * Register a generic route with the given method.
   *
   * Zod schemas are converted to JSON Schema and attached to the RouteBuilder.
   * The Rust core validates the request and returns 422 ProblemDetails on validation failure.
   * Handlers receive already-validated request data.
   */
  registerRoute(method, path, config, handler) {
    // Get the method enum value
    const methodEnum = this.getMethodEnum(method);
    if (!methodEnum) {
      throw new Error(`Unknown HTTP method: ${method}`);
    }

    // Create the route builder
    const service = getServiceAndBindings();
    let builder = new service.RouteBuilder(methodEnum, path);

    // Convert zod schema to JSON Schema and attach to builder
    // The Rust core will validate the request body against this schema
    // and return 422 ProblemDetails if validation fails
    if (config.body) {
      // The core validates the request body against this schema and returns 422
      // ProblemDetails before dispatch. The napi `requestSchemaJson(schema: any)`
      // param is converted straight into a serde_json::Value, so pass the derived
      // schema OBJECT (not a JSON string — a string would deserialize to a
      // Value::String and fail schema compilation).
      const jsonSchema = zodToJsonSchema(config.body);
      builder = builder.requestSchemaJson(jsonSchema);
    }

    // Map a user handler return value onto the low-level wire envelope. The
    // native contract deserializes into `spikard::Response { content, status_code,
    // headers }` (snake_case `status_code`; the payload field is `content`).
    const toEnvelope = (response) => ({
      status_code: response.statusCode ?? 200,
      content: response.content,
      headers: response.headers ?? {},
    });

    const toErrorEnvelope = (error) => {
      console.error("Handler error:", error);
      return {
        status_code: 500,
        content: {
          error: "Internal server error",
          details: error instanceof Error ? error.message : String(error),
        },
        headers: { "content-type": "application/json" },
      };
    };

    // Bridge the typed interface to the raw contract. At this point the request
    // body has already been validated by the Rust core (invalid bodies never
    // reach here — they return 422 ProblemDetails before dispatch).
    //
    // This is intentionally NOT declared `async`: the native ThreadsafeFunction
    // consumes the return value synchronously. For a synchronous user handler we
    // must return the envelope object directly; for an async handler we return
    // the pending Promise so the native side can await it.
    // The native ThreadsafeFunction is callee-handled: the JS callback is invoked
    // as `(err, requestData)`, so the request payload is the SECOND argument.
    const bridgeHandler = (_err, requestData) => {
      try {
        let body = requestData.body;
        if (config.body) {
          // Body is already valid per JSON Schema; parse only to recover types.
          const result = config.body.safeParse(body);
          if (result.success) {
            body = result.data;
          }
        }

        const typedRequest = {
          body,
          params: requestData.path_params || {},
          query: requestData.query_params || {},
          headers: requestData.headers || {},
          cookies: requestData.cookies || {},
          method: requestData.method || "GET",
          path: requestData.path || "/",
          contentType: requestData.content_type,
        };

        const response = handler(typedRequest);
        if (response && typeof response.then === "function") {
          return response.then(toEnvelope).catch(toErrorEnvelope);
        }
        return toEnvelope(response);
      } catch (error) {
        return toErrorEnvelope(error);
      }
    };

    // Register with the low-level service App
    this.serviceApp.registerRoute(builder, bridgeHandler);
  }

  /**
   * Convert HTTP method string to the service Method enum.
   */
  getMethodEnum(method) {
    const service = getServiceAndBindings();
    const methodMap = {
      GET: service.Method.Get,
      POST: service.Method.Post,
      PUT: service.Method.Put,
      PATCH: service.Method.Patch,
      DELETE: service.Method.Delete,
      HEAD: service.Method.Head,
      OPTIONS: service.Method.Options,
      CONNECT: service.Method.Connect,
      TRACE: service.Method.Trace,
    };
    return methodMap[method.toUpperCase()];
  }

  /**
   * Configure the server (host, port, etc.).
   */
  config(config) {
    this.serviceApp.config(config);
  }

  /**
   * Start the HTTP server.
   */
  async run() {
    await this.serviceApp.run();
  }

  /**
   * Build a router (returns the underlying router, advanced use case).
   */
  async intoRouter() {
    await this.serviceApp.intoRouter();
  }

  /**
   * Get the underlying low-level service App (for advanced use cases).
   */
  getServiceApp() {
    return this.serviceApp;
  }
}

module.exports = { App };
