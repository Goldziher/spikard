// Auto-generated service API class

import type { JsObject, RequestData, Response } from '../index';
import { app_run } from '../index';

/**
 * Spikard application builder.
 */
export class App {
  private _registrations: Array<[string, any[], (...args: any[]) => any]> = [];

  /**
   * Create a new application with the default server configuration.
   */
  constructor() {
    // Constructor initialization (parameters stored for future use)
  }

  /**
   * Set the server configuration.
   */
  config(config: ServerConfig): this {
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
      this._registrations.push(["route", [builder], fn]);
      return fn;
    };
  }

  /**
   * Register a route callback directly.
   */
  register_route(builder: RouteBuilder, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [builder], handler]);
    return this;
  }

  /**
   * Register a GET route at the given path.
   */
  get(path: string, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [path], handler]);
    return this;
  }

  /**
   * Register a POST route at the given path.
   */
  post(path: string, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [path], handler]);
    return this;
  }

  /**
   * Register a PUT route at the given path.
   */
  put(path: string, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [path], handler]);
    return this;
  }

  /**
   * Register a PATCH route at the given path.
   */
  patch(path: string, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [path], handler]);
    return this;
  }

  /**
   * Register a DELETE route at the given path.
   */
  delete(path: string, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [path], handler]);
    return this;
  }

  /**
   * Register a HEAD route at the given path.
   */
  head(path: string, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [path], handler]);
    return this;
  }

  /**
   * Register an OPTIONS route at the given path.
   */
  options(path: string, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [path], handler]);
    return this;
  }

  /**
   * Register a CONNECT route at the given path.
   */
  connect(path: string, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [path], handler]);
    return this;
  }

  /**
   * Register a TRACE route at the given path.
   */
  trace(path: string, handler: (...args: any[]) => any): this {
    this._registrations.push(["route", [path], handler]);
    return this;
  }

  /**
   * Run the HTTP server using the configured routes.
   *
   * # Errors
   *
   * Returns an error if server construction or execution fails.
   */
  async run(): Promise<void> {
    return await app_run(this._registrations);
  }

  /**
   * Build the underlying Axum router.
   *
   * # Errors
   *
   * Returns an error if server or router construction fails.
   */
  into_router(): string {
    return app_into_router(this._registrations);
  }

}
