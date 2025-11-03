/**
 * Spikard - High-performance HTTP framework for Node.js/TypeScript
 *
 * Type-safe routing, validation, and testing powered by Rust core.
 */

// @ts-expect-error - Native module will be available after build
export { process } from "../spikard.node";
export { TestClient, type TestResponse } from "./testing";

/**
 * Route metadata for defining HTTP routes
 */
export interface RouteMetadata {
	method: string;
	path: string;
	handler_name: string;
	request_schema?: any;
	response_schema?: any;
	parameter_schema?: any;
	file_params?: any;
	is_async: boolean;
	cors?: any;
}

/**
 * Spikard application interface
 */
export interface SpikardApp {
	routes: RouteMetadata[];
	handlers: Record<string, (...args: any[]) => any | Promise<any>>;
}
