/**
 * Routing decorators and utilities for Spikard
 */

import type { CorsConfig, HandlerFunction, JsonSchema, RouteMetadata } from "./index";
import type { Request } from "./request";
import type { HandlerResult, JsonValue, MaybePromise } from "./types";

/**
 * Route configuration options
 */
export interface RouteOptions {
	/**
	 * HTTP method(s) for this route
	 * Can be a single method or an array of methods
	 */
	methods?: string | string[];

	/**
	 * Request body schema (JSON Schema or Zod schema)
	 */
	bodySchema?: JsonSchema;

	/**
	 * Response schema (JSON Schema or Zod schema)
	 */
	responseSchema?: JsonSchema;

	/**
	 * Parameter schema (JSON Schema or Zod schema for path and query parameters)
	 */
	parameterSchema?: JsonSchema;

	/**
	 * CORS configuration
	 */
	cors?: CorsConfig;
}

type RouteArgument = Request | JsonValue | string | number | boolean | null | undefined;

type RouteHandler = (...args: RouteArgument[]) => MaybePromise<HandlerResult>;

/**
 * Route decorator for defining HTTP routes
 *
 * @param path - The route path (e.g., "/users/:id")
 * @param options - Route configuration options
 * @returns A decorator function
 *
 * @example
 * ```typescript
 * import { route, Spikard } from '@spikard/node';
 * import { z } from 'zod';
 *
 * const app = new Spikard();
 *
 * const UserSchema = z.object({
 *   name: z.string(),
 *   email: z.string().email(),
 * });
 *
 * route("/users", { methods: ["POST"], bodySchema: UserSchema})
 * async function createUser() {
 *   return { created: true };
 * }
 *
 * app.run();
 * ```
 */
export function route(path: string, options: RouteOptions = {}): (handler: RouteHandler) => RouteHandler {
	return (handler: RouteHandler) => {
		// Extract methods, defaulting to GET if not specified
		const methods = options.methods ? (Array.isArray(options.methods) ? options.methods : [options.methods]) : ["GET"];

		// Store route metadata
		const metadata: RouteMetadata = {
			method: methods.join(","),
			path,
			handler_name: handler.name || "anonymous",
			request_schema: options.bodySchema,
			response_schema: options.responseSchema,
			parameter_schema: options.parameterSchema,
			cors: options.cors,
			is_async: true, // Assume async by default
		};

		// Store metadata on the function
		(handler as HandlerFunction & { __route_metadata__?: RouteMetadata }).__route_metadata__ = metadata;

		return handler;
	};
}

/**
 * GET route decorator
 *
 * @param path - The route path
 * @param options - Route configuration options (excluding methods)
 * @returns A decorator function
 *
 * @example
 * ```typescript
 * import { get } from '@spikard/node';
 *
 * get("/users/:id")
 * function getUser(request: Request, id: Path<string>) {
 *   // Implementation
 * }
 * ```
 */
export function get(
	path: string,
	options: Omit<RouteOptions, "methods"> = {},
): (handler: RouteHandler) => RouteHandler {
	return route(path, { ...options, methods: ["GET"] });
}

/**
 * POST route decorator
 */
export function post(
	path: string,
	options: Omit<RouteOptions, "methods"> = {},
): (handler: RouteHandler) => RouteHandler {
	return route(path, { ...options, methods: ["POST"] });
}

/**
 * PUT route decorator
 */
export function put(
	path: string,
	options: Omit<RouteOptions, "methods"> = {},
): (handler: RouteHandler) => RouteHandler {
	return route(path, { ...options, methods: ["PUT"] });
}

/**
 * DELETE route decorator
 */
export function del(
	path: string,
	options: Omit<RouteOptions, "methods"> = {},
): (handler: RouteHandler) => RouteHandler {
	return route(path, { ...options, methods: ["DELETE"] });
}

/**
 * PATCH route decorator
 */
export function patch(
	path: string,
	options: Omit<RouteOptions, "methods"> = {},
): (handler: RouteHandler) => RouteHandler {
	return route(path, { ...options, methods: ["PATCH"] });
}
