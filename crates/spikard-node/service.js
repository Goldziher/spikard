// Auto-generated service API class
// Delegates to native App (JsApp in Rust) for server operations via
// nativeRegisterRoute, setConfig, and nativeRun methods added in
// the e2e/node CORS+compression fix.

/**
 * Spikard application builder.
 */
class App {
  constructor() {
    // Access the native App class via _nativeApp — index.js saves this
    // BEFORE overriding module.exports.App with this service App class.
    try {
      const pkg = require("./index");
      const NativeApp = pkg._nativeApp;
      this._native = NativeApp ? NativeApp.new() : null;
    } catch (e) {
      this._native = null;
    }
    this._registrations = [];
  }

  /**
   * Create a new App instance.
   */
  static new() {
    return new App();
  }

  /**
   * Set the server configuration.
   * Delegates to native setConfig when available (supports compression etc).
   */
  config(config) {
    if (this._native && typeof this._native.setConfig === "function") {
      this._native.setConfig(config);
    }
    return this;
  }

  /**
   * Register a route using the provided builder and handler function.
   *
   * # Errors
   *
   * Returns an error if route construction fails or if the handler registration fails.
   */
  route(builder) {
    return (fn) => {
      this.registerRoute(builder, fn);
      return fn;
    };
  }

  /**
   * Register a route callback directly (snake_case form).
   */
  register_route(builder, handler) {
    return this.registerRoute(builder, handler);
  }

  /**
   * Register a route callback directly (camelCase alias).
   * Delegates to native nativeRegisterRoute when available.
   */
  registerRoute(builder, handler) {
    if (this._native && typeof this._native.nativeRegisterRoute === "function") {
      this._native.nativeRegisterRoute(builder, handler);
    } else {
      this._registrations.push(["route", [builder], handler]);
    }
    return this;
  }

  /**
   * Register a GET route at the given path.
   */
  get(path, handler) {
    const { RouteBuilder, Method } = require("./index");
    const builder = new RouteBuilder(Method.Get, path);
    return this.registerRoute(builder, handler);
  }

  /**
   * Register a POST route at the given path.
   */
  post(path, handler) {
    const { RouteBuilder, Method } = require("./index");
    const builder = new RouteBuilder(Method.Post, path);
    return this.registerRoute(builder, handler);
  }

  /**
   * Register a PUT route at the given path.
   */
  put(path, handler) {
    const { RouteBuilder, Method } = require("./index");
    const builder = new RouteBuilder(Method.Put, path);
    return this.registerRoute(builder, handler);
  }

  /**
   * Register a PATCH route at the given path.
   */
  patch(path, handler) {
    const { RouteBuilder, Method } = require("./index");
    const builder = new RouteBuilder(Method.Patch, path);
    return this.registerRoute(builder, handler);
  }

  /**
   * Register a DELETE route at the given path.
   */
  delete(path, handler) {
    const { RouteBuilder, Method } = require("./index");
    const builder = new RouteBuilder(Method.Delete, path);
    return this.registerRoute(builder, handler);
  }

  /**
   * Register a HEAD route at the given path.
   */
  head(path, handler) {
    const { RouteBuilder, Method } = require("./index");
    const builder = new RouteBuilder(Method.Head, path);
    return this.registerRoute(builder, handler);
  }

  /**
   * Register an OPTIONS route at the given path.
   */
  options(path, handler) {
    const { RouteBuilder, Method } = require("./index");
    const builder = new RouteBuilder(Method.Options, path);
    return this.registerRoute(builder, handler);
  }

  /**
   * Register a CONNECT route at the given path.
   */
  connect(path, handler) {
    const { RouteBuilder, Method } = require("./index");
    const builder = new RouteBuilder(Method.Connect, path);
    return this.registerRoute(builder, handler);
  }

  /**
   * Register a TRACE route at the given path.
   */
  trace(path, handler) {
    const { RouteBuilder, Method } = require("./index");
    const builder = new RouteBuilder(Method.Trace, path);
    return this.registerRoute(builder, handler);
  }

  /**
   * Run the HTTP server using the configured routes.
   * Delegates to native nativeRun when available.
   */
  async run() {
    if (this._native && typeof this._native.nativeRun === "function") {
      return await this._native.nativeRun();
    }
    // Fallback: try app_run (legacy path)
    const { app_run } = require("./index");
    if (typeof app_run === "function") {
      return await app_run(this._registrations);
    }
    throw new Error("No run method available: native nativeRun missing and app_run not exported");
  }

  /**
   * Build the underlying Axum router.
   */
  into_router() {
    const { app_into_router } = require("./index");
    return app_into_router(this._registrations);
  }
}

module.exports = { App };
