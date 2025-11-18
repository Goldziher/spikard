import type { JsonValue } from "./types";

/**
 * Request type for Spikard handlers
 *
 * At runtime, the Request object is provided by the Rust/Axum backend.
 * This module provides type definitions for static type checking.
 */

/**
 * HTTP Request object provided to route handlers
 *
 * This is a type definition for type checking. At runtime, the actual Request
 * object is provided by the Rust backend through Axum.
 */
export interface Request {
	/**
	 * HTTP method (GET, POST, etc.)
	 */
	method: string;

	/**
	 * Request path
	 */
	path: string;

	/**
	 * Query string
	 */
	queryString: string;

	/**
	 * Request headers
	 */
	headers: Record<string, string>;

	/**
	 * Request body (if any)
	 */
	body: Buffer | null;

	/**
	 * Parse request body as JSON
	 *
	 * @returns Parsed JSON data
	 * @throws Error if body is not valid JSON
	 */
	json<T extends JsonValue = JsonValue>(): T;

	/**
	 * Parse request body as form data
	 *
	 * @returns Form data as a record of key-value pairs
	 * @throws Error if body is not valid form data
	 */
	form(): Record<string, string>;
}
