/**
 * Routing decorators and utilities for Spikard
 */

import type { RouteMetadata } from "./index";

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
	 * Request body schema (Zod schema)
	 */
	bodySchema?: unknown;

	/**
	 * Response schema (Zod schema)
	 */
	responseSchema?: unknown;

	/**
	 * Parameter schema (Zod schema for path and query parameters)
	 */
	parameterSchema?: unknown;

	/**
	 * CORS configuration
	 */
	cors?: unknown;
}

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
export function route(path: string, options: RouteOptions = {}): (handler: Function) => Function {
	return (handler: Function) => {
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
		(handler as any).__route_metadata__ = metadata;

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
export function get(path: string, options: Omit<RouteOptions, "methods"> = {}): (handler: Function) => Function {
	return route(path, { ...options, methods: ["GET"] });
}

/**
 * POST route decorator
 */
export function post(path: string, options: Omit<RouteOptions, "methods"> = {}): (handler: Function) => Function {
	return route(path, { ...options, methods: ["POST"] });
}

/**
 * PUT route decorator
 */
export function put(path: string, options: Omit<RouteOptions, "methods"> = {}): (handler: Function) => Function {
	return route(path, { ...options, methods: ["PUT"] });
}

/**
 * DELETE route decorator
 */
export function del(path: string, options: Omit<RouteOptions, "methods"> = {}): (handler: Function) => Function {
	return route(path, { ...options, methods: ["DELETE"] });
}

/**
 * PATCH route decorator
 */
export function patch(path: string, options: Omit<RouteOptions, "methods"> = {}): (handler: Function) => Function {
	return route(path, { ...options, methods: ["PATCH"] });
}
