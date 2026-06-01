// Auto-generated service API class

const { app_run } = require('./index');

/**
 * Spikard application builder.
 */
class App {
  constructor() {
    this._registrations = [];
    // Constructor initialization (parameters stored for future use)
  }

  /**
   * Set the server configuration.
   */
  config(config) {
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
  register_route(builder, handler) {
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register a GET route at the given path.
   */
  get(path, handler) {
    const { RouteBuilder, Method } = require('./index');
    const builder = new RouteBuilder(Method.Get, path);
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register a POST route at the given path.
   */
  post(path, handler) {
    const { RouteBuilder, Method } = require('./index');
    const builder = new RouteBuilder(Method.Post, path);
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register a PUT route at the given path.
   */
  put(path, handler) {
    const { RouteBuilder, Method } = require('./index');
    const builder = new RouteBuilder(Method.Put, path);
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register a PATCH route at the given path.
   */
  patch(path, handler) {
    const { RouteBuilder, Method } = require('./index');
    const builder = new RouteBuilder(Method.Patch, path);
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register a DELETE route at the given path.
   */
  delete(path, handler) {
    const { RouteBuilder, Method } = require('./index');
    const builder = new RouteBuilder(Method.Delete, path);
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register a HEAD route at the given path.
   */
  head(path, handler) {
    const { RouteBuilder, Method } = require('./index');
    const builder = new RouteBuilder(Method.Head, path);
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register an OPTIONS route at the given path.
   */
  options(path, handler) {
    const { RouteBuilder, Method } = require('./index');
    const builder = new RouteBuilder(Method.Options, path);
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register a CONNECT route at the given path.
   */
  connect(path, handler) {
    const { RouteBuilder, Method } = require('./index');
    const builder = new RouteBuilder(Method.Connect, path);
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register a TRACE route at the given path.
   */
  trace(path, handler) {
    const { RouteBuilder, Method } = require('./index');
    const builder = new RouteBuilder(Method.Trace, path);
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Run the HTTP server using the configured routes.
   *
   * # Errors
   *
   * Returns an error if server construction or execution fails.
   */
  async run() {
    return await app_run(this._registrations);
  }

  /**
   * Build the underlying Axum router.
   *
   * # Errors
   *
   * Returns an error if server or router construction fails.
   */
  into_router() {
    const { app_into_router } = require('./index');
    return app_into_router(this._registrations);
  }
}

module.exports = { App };
