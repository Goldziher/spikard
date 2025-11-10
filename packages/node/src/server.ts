/**
 * Server runtime for Spikard Node
 */

import type { ServerConfig } from "./config";
import type { SpikardApp } from "./index";

// This will be the native binding once built
let nativeBinding: any;

try {
	// Try to load the native module from the package root
	nativeBinding = require("../spikard-node.darwin-arm64.node");
} catch (_e) {
	try {
		// Fallback to platform-agnostic name
		nativeBinding = require("../spikard-node.node");
	} catch (_e2) {
		console.warn("[spikard-node] Native binding not found. Please run: pnpm build:native");
		nativeBinding = {
			runServer: () => {
				throw new Error("Native binding not built. Run: pnpm build:native");
			},
		};
	}
}

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
 * import { Spikard, runServer } from '@spikard/node';
 * import type { ServerConfig } from '@spikard/node';
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
	// The Rust binding (crates/spikard-node/src/lib.rs) already handles
	// full ServerConfig extraction. We just pass the config object through.
	// Backwards compatibility: if only host/port provided, Rust will use defaults.
	nativeBinding.runServer(app, config);
}
