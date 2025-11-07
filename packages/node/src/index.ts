/**
 * Spikard - High-performance HTTP framework for Node.js/TypeScript
 *
 * Type-safe routing, validation, and testing powered by Rust core.
 */

export { process } from "..";

// Parameter types
export type { Body, Path, Query, QueryDefault } from "./params";

// Request interface
export type { Request } from "./request";

// Routing
export { del, get, patch, post, put, type RouteOptions, route } from "./routing";

// Testing utilities
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
