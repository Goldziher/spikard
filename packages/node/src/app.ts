/**
 * Spikard application class
 */

import type { HandlerFunction, RouteMetadata, SpikardApp } from "./index";
import { runServer, type ServerOptions } from "./server";

/**
 * Spikard application
 *
 * @example
 * ```typescript
 * import { Spikard, get, post } from '@spikard/node';
 *
 * const app = new Spikard();
 *
 * get('/')(async function root() {
 *   return { message: 'Hello' };
 * });
 *
 * if (require.main === module) {
 *   app.run({ port: 8000 });
 * }
 * ```
 */
export class Spikard implements SpikardApp {
	routes: RouteMetadata[] = [];
	handlers: Record<string, HandlerFunction> = {};

	/**
	 * Add a route to the application
	 *
	 * @param metadata - Route configuration metadata
	 * @param handler - Handler function (sync or async)
	 */
	addRoute(metadata: RouteMetadata, handler: HandlerFunction): void {
		this.routes.push(metadata);
		this.handlers[metadata.handler_name] = handler;
	}

	/**
	 * Run the server
	 *
	 * @param options - Server configuration
	 */
	run(options: ServerOptions = {}): void {
		runServer(this, options);
	}
}
