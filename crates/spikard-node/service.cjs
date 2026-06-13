// Auto-generated service API class

const { ServerConfig } = require("./index");
const { Method, RouteBuilder } = require("./index");
const { appIntoRouter, appRun } = require("./index");
/**
 * Spikard application builder.
 */
class App {
  readonly _app;
  _config;

  /**
   * Create a new App instance.
   */
  static new() {
    return new App();
  }
  /**
   * Create a new application with the default server configuration.
   */
  constructor() {
    this._app = new JsApp();
  }
  /**
   * Set the server configuration.
   */
  config(config) {
    this._config = config;
    this._app.config(JSON.stringify(config));

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
      this._app.route(builder, fn);
      return fn;
    };
  }
  /**
   * Register a route callback directly.
   */
  registerRoute(builder, handler) {
    this._app.route(builder, handler);
    return this;
  }
  /**
   * Register a GET route at the given path.
   */
  get(path, handler) {
    const builder = RouteBuilder.new(Method.Get, path);
    if (handler !== undefined) {
      this._app.get(path, handler);
      return this;
    }
    return (fn) => {
      this._app.get(path, fn);
      return fn;
    };
  }
  /**
   * Register a POST route at the given path.
   */
  post(path, handler) {
    const builder = RouteBuilder.new(Method.Post, path);
    if (handler !== undefined) {
      this._app.post(path, handler);
      return this;
    }
    return (fn) => {
      this._app.post(path, fn);
      return fn;
    };
  }
  /**
   * Register a PUT route at the given path.
   */
  put(path, handler) {
    const builder = RouteBuilder.new(Method.Put, path);
    if (handler !== undefined) {
      this._app.put(path, handler);
      return this;
    }
    return (fn) => {
      this._app.put(path, fn);
      return fn;
    };
  }
  /**
   * Register a PATCH route at the given path.
   */
  patch(path, handler) {
    const builder = RouteBuilder.new(Method.Patch, path);
    if (handler !== undefined) {
      this._app.patch(path, handler);
      return this;
    }
    return (fn) => {
      this._app.patch(path, fn);
      return fn;
    };
  }
  /**
   * Register a DELETE route at the given path.
   */
  delete(path, handler) {
    const builder = RouteBuilder.new(Method.Delete, path);
    if (handler !== undefined) {
      this._app.delete(path, handler);
      return this;
    }
    return (fn) => {
      this._app.delete(path, fn);
      return fn;
    };
  }
  /**
   * Register a HEAD route at the given path.
   */
  head(path, handler) {
    const builder = RouteBuilder.new(Method.Head, path);
    if (handler !== undefined) {
      this._app.head(path, handler);
      return this;
    }
    return (fn) => {
      this._app.head(path, fn);
      return fn;
    };
  }
  /**
   * Register an OPTIONS route at the given path.
   */
  options(path, handler) {
    const builder = RouteBuilder.new(Method.Options, path);
    if (handler !== undefined) {
      this._app.options(path, handler);
      return this;
    }
    return (fn) => {
      this._app.options(path, fn);
      return fn;
    };
  }
  /**
   * Register a CONNECT route at the given path.
   */
  connect(path, handler) {
    const builder = RouteBuilder.new(Method.Connect, path);
    if (handler !== undefined) {
      this._app.connect(path, handler);
      return this;
    }
    return (fn) => {
      this._app.connect(path, fn);
      return fn;
    };
  }
  /**
   * Register a TRACE route at the given path.
   */
  trace(path, handler) {
    const builder = RouteBuilder.new(Method.Trace, path);
    if (handler !== undefined) {
      this._app.trace(path, handler);
      return this;
    }
    return (fn) => {
      this._app.trace(path, fn);
      return fn;
    };
  }
  /**
   * Run the HTTP server using the configured routes.
   *
   * # Errors
   *
   * Returns an error if server construction or execution fails.
   */
  async run() {
    return await this._app.run();
  }
  /**
   * Build the underlying Axum router.
   *
   * # Errors
   *
   * Returns an error if server or router construction fails.
   */
  into_router() {
    return this._app.intoRouter();
  }
  /**
   * Called before any other processing for each inbound request.
   */
  onRequest(handler) {
    this._app.onRequest(handler);
    return this;
  }
  /**
   * Called after parsing but before parameter validation.
   */
  preValidation(handler) {
    this._app.preValidation(handler);
    return this;
  }
  /**
   * Called after validation but before invoking the route handler.
   */
  preHandler(handler) {
    this._app.preHandler(handler);
    return this;
  }
  /**
   * Called after the handler returns but before the response is serialized.
   */
  onResponse(handler) {
    this._app.onResponse(handler);
    return this;
  }
  /**
   * Called when a handler returns an error.
   */
  onError(handler) {
    this._app.onError(handler);
    return this;
  }
}

module.exports = { App };
