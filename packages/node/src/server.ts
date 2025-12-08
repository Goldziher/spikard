/**
 * Server runtime for Spikard Node
 */

import type { ServerConfig } from "./config";
import { isNativeHandler, wrapHandler } from "./handler-wrapper";
import type { HandlerFunction, NativeHandlerFunction, SpikardApp } from "./index";
import { createRequire } from "module";

interface NativeServerBinding {
	runServer(app: SpikardApp, config: ServerConfig | ServerOptions): void;
}

let nativeBinding: NativeServerBinding;

function loadBinding(): NativeServerBinding {
	try {
		// createRequire allows us to require CommonJS modules from ESM context
		// This is necessary to load the NAPI binding which is a .node file loaded via CommonJS
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
	const handlers: Record<string, NativeHandlerFunction> = Object.fromEntries(
		Object.entries(app.handlers || {}).map(([name, handler]) => {
			const nativeHandler = isNativeHandler(handler) ? handler : wrapHandler(handler as HandlerFunction);
			return [name, nativeHandler];
		}),
	);

	nativeBinding.runServer({ ...app, handlers }, config);
}
