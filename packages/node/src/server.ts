/**
 * Server runtime for Spikard Node
 */

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

export interface ServerOptions {
	host?: string;
	port?: number;
}

/**
 * Run the Spikard server
 *
 * @param app - The Spikard application instance
 * @param options - Server configuration options
 *
 * @example
 * ```typescript
 * const app = new Spikard();
 *
 * // Register routes...
 *
 * runServer(app, { host: '0.0.0.0', port: 8000 });
 * ```
 */
export function runServer(app: SpikardApp, options: ServerOptions = {}): void {
	const { host = "127.0.0.1", port = 8000 } = options;

	// Call the native Rust function
	nativeBinding.runServer(app, host, port);
}
