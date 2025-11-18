/**
 * Spikard - High-performance HTTP framework for Node.js/TypeScript
 *
 * Type-safe routing, validation, and testing powered by Rust core.
 */

import type { HandlerFunction, JsonValue } from "./types";

// Application
export { type LifecycleHookFunction, type LifecycleHooks, Spikard } from "./app";
export * as background from "./background";
// Configuration types
export type {
	ApiKeyConfig,
	CompressionConfig,
	ContactInfo,
	JwtConfig,
	LicenseInfo,
	OpenApiConfig,
	RateLimitConfig,
	SecuritySchemeInfo,
	ServerConfig,
	ServerInfo,
	StaticFilesConfig,
} from "./config";
// Parameter types
export type { Body, Path, Query, QueryDefault } from "./params";
// Request interface
export type { Request } from "./request";
// Routing
export { del, get, patch, post, put, type RouteOptions, route } from "./routing";
// Server
export { runServer, type ServerOptions } from "./server";
export { StreamingResponse, type StreamingResponseInit } from "./streaming";
// Testing utilities
export { TestClient, type TestResponse } from "./testing";
export type {
	Base64EncodedBody,
	HandlerFunction,
	HandlerPayload,
	HandlerResult,
	JsonPrimitive,
	JsonRecord,
	JsonValue,
	MaybePromise,
	StructuredHandlerResponse,
} from "./types";

/**
 * JSON schema definition for validation
 */
export interface JsonSchema {
	type?: string | string[];
	properties?: Record<string, JsonSchema>;
	required?: string[];
	items?: JsonSchema | JsonSchema[];
	enum?: JsonValue[];
	[key: string]: JsonSchema | JsonSchema[] | JsonValue | JsonValue[] | string | number | boolean | undefined;
}

/**
 * CORS configuration
 */
export interface CorsConfig {
	allow_origin?: string | string[];
	allow_methods?: string[];
	allow_headers?: string[];
	allow_credentials?: boolean;
	max_age?: number;
}

/**
 * File upload parameter configuration
 */
export interface FileParam {
	name: string;
	required?: boolean;
	max_size?: number;
	allowed_types?: string[];
}

/**
 * Route metadata for defining HTTP routes
 */
export interface RouteMetadata {
	/** HTTP method (GET, POST, etc.) */
	method: string;
	/** URL path pattern (may include path parameters like /users/:id) */
	path: string;
	/** Name of the handler function */
	handler_name: string;
	/** JSON schema for request body validation */
	request_schema?: JsonSchema;
	/** JSON schema for response validation */
	response_schema?: JsonSchema;
	/** JSON schema for path/query parameter validation */
	parameter_schema?: JsonSchema;
	/** File upload parameter configurations */
	file_params?: FileParam[];
	/** Whether the handler is async (almost always true for Node.js) */
	is_async: boolean;
	/** CORS configuration for this route */
	cors?: CorsConfig;
}

/**
 * Spikard application interface
 */
export interface SpikardApp {
	/** Route metadata array */
	routes: RouteMetadata[];
	/** Handler functions mapped by handler_name */
	handlers: Record<string, HandlerFunction>;
	/** Optional server configuration (for middleware, auth, etc.) */
	config?: import("./config").ServerConfig;
	/** Optional lifecycle hooks */
	lifecycleHooks?: Partial<import("./app").LifecycleHooks>;
}
