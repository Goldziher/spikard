/**
 * Server runtime for Spikard Node
 */

import { createRequire } from "node:module";
import type { ServerConfig } from "./config";
import { isNativeHandler, wrapHandler } from "./handler-wrapper";
import type { HandlerFunction, NativeHandlerFunction, SpikardApp } from "./index";

interface NativeServerBinding {
	runServer(app: SpikardApp, config: ServerConfig | ServerOptions): void;
}

let nativeBinding: NativeServerBinding;

function loadBinding(): NativeServerBinding {
	try {
		const require = createRequire(import.meta.url);
		return require("../index.js") as NativeServerBinding;
	} catch {
		console.warn("[spikard-node] Native binding not found. Please run: pnpm build:native");
		return {
			runServer: () => {
				throw new Error("Native binding not built. Run: pnpm build:native");
			},
		};
	}
}

nativeBinding = loadBinding();

/**
 * @deprecated Use ServerConfig instead
 */
export interface ServerOptions {
	host?: string;
	port?: number;
}

/**
 * Run the Spikard server
 *
 * @param app - The Spikard application instance
 * @param config - Server configuration options (supports full ServerConfig or legacy ServerOptions)
 *
 * @example
 * ```typescript
 * import { Spikard, runServer } from 'spikard';
 * import type { ServerConfig } from 'spikard';
 *
 * const app = new Spikard();
 * // Register routes...
 *
 * // Simple usage (backwards compatible)
 * runServer(app, { host: '0.0.0.0', port: 8000 });
 *
 * // Full configuration with middleware
 * const config: ServerConfig = {
 *   host: '0.0.0.0',
 *   port: 8080,
 *   workers: 4,
 *   compression: { quality: 9 },
 *   rateLimit: { perSecond: 100, burst: 200 },
 *   jwtAuth: { secret: 'your-secret', algorithm: 'HS256' },
 *   openapi: { enabled: true, title: 'My API', version: '1.0.0' },
 * };
 * runServer(app, config);
 * ```
 */
export function runServer(app: SpikardApp, config: ServerConfig | ServerOptions = {}): void {
	const handlers: Record<string, NativeHandlerFunction> = {};
	const routes = (app.routes || []).map((route) => {
		const handler = app.handlers?.[route.handler_name];
		if (!handler) return route;
		const nativeHandler = isNativeHandler(handler) ? handler : wrapHandler(handler as HandlerFunction);
		handlers[route.handler_name] = nativeHandler;
		const isAsync = nativeHandler.constructor.name === "AsyncFunction";
		return { ...route, is_async: isAsync };
	});

	nativeBinding.runServer({ ...app, handlers, routes }, config);
}
