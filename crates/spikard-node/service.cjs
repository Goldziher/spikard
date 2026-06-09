// Auto-generated service API class

const { ServerConfig } = require("./index");
const { Method, RouteBuilder } = require("./index");
const { appIntoRouter, appRun } = require("./index");
/**
 * Spikard application builder.
 */
class App {
  _registrations = [];
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
    // Constructor initialization (parameters stored for future use)
  }
  /**
   * Set the server configuration.
   */
  config(_config) {
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
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }
  /**
   * Register a route callback directly.
   */
  registerRoute(builder, handler) {
    this._registrations.push(["route", [builder], handler]);
    return this;
  }
  /**
   * Register a GET route at the given path.
   */
  get(path, handler) {
    const builder = RouteBuilder.new(Method.Get, path);
    if (handler !== undefined) {
      this._registrations.push(["route", [builder], handler]);
      return this;
    }
    return (fn) => {
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }
  /**
   * Register a POST route at the given path.
   */
  post(path, handler) {
    const builder = RouteBuilder.new(Method.Post, path);
    if (handler !== undefined) {
      this._registrations.push(["route", [builder], handler]);
      return this;
    }
    return (fn) => {
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }
  /**
   * Register a PUT route at the given path.
   */
  put(path, handler) {
    const builder = RouteBuilder.new(Method.Put, path);
    if (handler !== undefined) {
      this._registrations.push(["route", [builder], handler]);
      return this;
    }
    return (fn) => {
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }
  /**
   * Register a PATCH route at the given path.
   */
  patch(path, handler) {
    const builder = RouteBuilder.new(Method.Patch, path);
    if (handler !== undefined) {
      this._registrations.push(["route", [builder], handler]);
      return this;
    }
    return (fn) => {
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }
  /**
   * Register a DELETE route at the given path.
   */
  delete(path, handler) {
    const builder = RouteBuilder.new(Method.Delete, path);
    if (handler !== undefined) {
      this._registrations.push(["route", [builder], handler]);
      return this;
    }
    return (fn) => {
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }
  /**
   * Register a HEAD route at the given path.
   */
  head(path, handler) {
    const builder = RouteBuilder.new(Method.Head, path);
    if (handler !== undefined) {
      this._registrations.push(["route", [builder], handler]);
      return this;
    }
    return (fn) => {
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }
  /**
   * Register an OPTIONS route at the given path.
   */
  options(path, handler) {
    const builder = RouteBuilder.new(Method.Options, path);
    if (handler !== undefined) {
      this._registrations.push(["route", [builder], handler]);
      return this;
    }
    return (fn) => {
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }
  /**
   * Register a CONNECT route at the given path.
   */
  connect(path, handler) {
    const builder = RouteBuilder.new(Method.Connect, path);
    if (handler !== undefined) {
      this._registrations.push(["route", [builder], handler]);
      return this;
    }
    return (fn) => {
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }
  /**
   * Register a TRACE route at the given path.
   */
  trace(path, handler) {
    const builder = RouteBuilder.new(Method.Trace, path);
    if (handler !== undefined) {
      this._registrations.push(["route", [builder], handler]);
      return this;
    }
    return (fn) => {
      this._registrations.push(["route", [builder], fn]);
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
    return await appRun(this._registrations);
  }
  /**
   * Build the underlying Axum router.
   *
   * # Errors
   *
   * Returns an error if server or router construction fails.
   */
  into_router() {
    return appIntoRouter(this._registrations);
  }
}

module.exports = { App };
