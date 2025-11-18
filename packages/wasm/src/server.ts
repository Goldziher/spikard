import type { ServerConfig } from "./config";
import type { SpikardApp } from "./index";

export interface ServerOptions {
	host?: string;
	port?: number;
}

export function runServer(_app: SpikardApp, _config: ServerConfig | ServerOptions = {}): never {
	throw new Error("Spikard WASM bindings do not support running an HTTP server yet");
}
