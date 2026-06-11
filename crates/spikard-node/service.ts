// Auto-generated service API class

import type { ServerConfig } from "./index";
import { Method, RouteBuilder } from "./index";
import { appIntoRouter, appRun } from "./index";
/**
 * Spikard application builder.
 */
export class App {
  private readonly _app: JsApp;
  /**
   * Create a new App instance.
   */
  static new(): App {
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
  config(_config: ServerConfig): this {
    return this;
  }
  /**
   * Register a route using the provided builder and handler function.
   *
   * # Errors
   *
   * Returns an error if route construction fails or if the handler registration fails.
   */
  route(builder: RouteBuilder): (fn: (...args: any[]) => any) => (...args: any[]) => any {
    return (fn: (...args: any[]) => any) => {
      this._app.route(builder, fn);
      return fn;
    };
  }
  /**
   * Register a route callback directly.
   */
  registerRoute(builder: RouteBuilder, handler: (...args: any[]) => any): this {
    this._app.route(builder, handler);
    return this;
  }
  /**
   * Register a GET route at the given path.
   */
  get(path: string, handler: (...args: any[]) => any): this;
  get(path: string): (fn: (...args: any[]) => any) => (...args: any[]) => any;
  get(path: string, handler?: (...args: any[]) => any): this | ((fn: (...args: any[]) => any) => (...args: any[]) => any) {
    const builder = RouteBuilder.new(Method.Get, path);
    if (handler !== undefined) {
      this._app.get(path, handler);
      return this;
    }
    return (fn: (...args: any[]) => any) => {
      this._app.get(path, fn);
      return fn;
    };
  }
  /**
   * Register a POST route at the given path.
   */
  post(path: string, handler: (...args: any[]) => any): this;
  post(path: string): (fn: (...args: any[]) => any) => (...args: any[]) => any;
  post(path: string, handler?: (...args: any[]) => any): this | ((fn: (...args: any[]) => any) => (...args: any[]) => any) {
    const builder = RouteBuilder.new(Method.Post, path);
    if (handler !== undefined) {
      this._app.post(path, handler);
      return this;
    }
    return (fn: (...args: any[]) => any) => {
      this._app.post(path, fn);
      return fn;
    };
  }
  /**
   * Register a PUT route at the given path.
   */
  put(path: string, handler: (...args: any[]) => any): this;
  put(path: string): (fn: (...args: any[]) => any) => (...args: any[]) => any;
  put(path: string, handler?: (...args: any[]) => any): this | ((fn: (...args: any[]) => any) => (...args: any[]) => any) {
    const builder = RouteBuilder.new(Method.Put, path);
    if (handler !== undefined) {
      this._app.put(path, handler);
      return this;
    }
    return (fn: (...args: any[]) => any) => {
      this._app.put(path, fn);
      return fn;
    };
  }
  /**
   * Register a PATCH route at the given path.
   */
  patch(path: string, handler: (...args: any[]) => any): this;
  patch(path: string): (fn: (...args: any[]) => any) => (...args: any[]) => any;
  patch(path: string, handler?: (...args: any[]) => any): this | ((fn: (...args: any[]) => any) => (...args: any[]) => any) {
    const builder = RouteBuilder.new(Method.Patch, path);
    if (handler !== undefined) {
      this._app.patch(path, handler);
      return this;
    }
    return (fn: (...args: any[]) => any) => {
      this._app.patch(path, fn);
      return fn;
    };
  }
  /**
   * Register a DELETE route at the given path.
   */
  delete(path: string, handler: (...args: any[]) => any): this;
  delete(path: string): (fn: (...args: any[]) => any) => (...args: any[]) => any;
  delete(path: string, handler?: (...args: any[]) => any): this | ((fn: (...args: any[]) => any) => (...args: any[]) => any) {
    const builder = RouteBuilder.new(Method.Delete, path);
    if (handler !== undefined) {
      this._app.delete(path, handler);
      return this;
    }
    return (fn: (...args: any[]) => any) => {
      this._app.delete(path, fn);
      return fn;
    };
  }
  /**
   * Register a HEAD route at the given path.
   */
  head(path: string, handler: (...args: any[]) => any): this;
  head(path: string): (fn: (...args: any[]) => any) => (...args: any[]) => any;
  head(path: string, handler?: (...args: any[]) => any): this | ((fn: (...args: any[]) => any) => (...args: any[]) => any) {
    const builder = RouteBuilder.new(Method.Head, path);
    if (handler !== undefined) {
      this._app.head(path, handler);
      return this;
    }
    return (fn: (...args: any[]) => any) => {
      this._app.head(path, fn);
      return fn;
    };
  }
  /**
   * Register an OPTIONS route at the given path.
   */
  options(path: string, handler: (...args: any[]) => any): this;
  options(path: string): (fn: (...args: any[]) => any) => (...args: any[]) => any;
  options(path: string, handler?: (...args: any[]) => any): this | ((fn: (...args: any[]) => any) => (...args: any[]) => any) {
    const builder = RouteBuilder.new(Method.Options, path);
    if (handler !== undefined) {
      this._app.options(path, handler);
      return this;
    }
    return (fn: (...args: any[]) => any) => {
      this._app.options(path, fn);
      return fn;
    };
  }
  /**
   * Register a CONNECT route at the given path.
   */
  connect(path: string, handler: (...args: any[]) => any): this;
  connect(path: string): (fn: (...args: any[]) => any) => (...args: any[]) => any;
  connect(path: string, handler?: (...args: any[]) => any): this | ((fn: (...args: any[]) => any) => (...args: any[]) => any) {
    const builder = RouteBuilder.new(Method.Connect, path);
    if (handler !== undefined) {
      this._app.connect(path, handler);
      return this;
    }
    return (fn: (...args: any[]) => any) => {
      this._app.connect(path, fn);
      return fn;
    };
  }
  /**
   * Register a TRACE route at the given path.
   */
  trace(path: string, handler: (...args: any[]) => any): this;
  trace(path: string): (fn: (...args: any[]) => any) => (...args: any[]) => any;
  trace(path: string, handler?: (...args: any[]) => any): this | ((fn: (...args: any[]) => any) => (...args: any[]) => any) {
    const builder = RouteBuilder.new(Method.Trace, path);
    if (handler !== undefined) {
      this._app.trace(path, handler);
      return this;
    }
    return (fn: (...args: any[]) => any) => {
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
  async run(): Promise<void> {
    return await this._app.run();
  }
  /**
   * Build the underlying Axum router.
   *
   * # Errors
   *
   * Returns an error if server or router construction fails.
   */
  into_router(): Promise<void> {
    return this._app.intoRouter();
  }
}

export { App };
