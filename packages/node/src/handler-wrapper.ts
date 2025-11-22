/**
 * Handler wrapper that provides ergonomic API for route handlers
 *
 * Automatically converts file metadata to UploadFile instances,
 * providing the same zero-boilerplate experience as Python bindings.
 */

import { convertHandlerBody } from "./converters";
import type { HandlerFunction, JsonValue } from "./types";

/**
 * Request data passed from Rust handler
 */
export interface RequestData {
	path: string;
	method: string;
	path_params: Record<string, string>;
	query_params: Record<string, string>;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	body: JsonValue;
}

/**
 * Wrapped handler payload with converted body
 */
export interface HandlerPayload {
	path: string;
	method: string;
	pathParams: Record<string, string>;
	queryParams: Record<string, string>;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	body: unknown;
}

/**
 * Handler function signature that receives converted body
 */
export type TypedHandler<TBody = unknown> = (payload: {
	body: TBody;
	pathParams: Record<string, string>;
	queryParams: Record<string, string>;
	headers: Record<string, string>;
	cookies: Record<string, string>;
}) => unknown | Promise<unknown>;

/**
 * Wraps a handler function to automatically convert file metadata to UploadFile instances
 *
 * This provides the ergonomic API where users write:
 * ```typescript
 * async function handler({ body }: { body: UploadRequest }) {
 *   return { filename: body.file.filename };
 * }
 * ```
 *
 * Instead of manually parsing JSON with file metadata.
 *
 * @param handler - Handler function that receives converted body
 * @returns Raw handler function compatible with Rust FFI
 */
export function wrapHandler<TBody = unknown>(handler: TypedHandler<TBody>): HandlerFunction {
	return async (requestJson: string): Promise<string> => {
		// Parse request JSON from Rust
		const requestData = JSON.parse(requestJson) as RequestData;

		// Convert file metadata in body to UploadFile instances
		const convertedBody = convertHandlerBody(requestData.body);

		// Create ergonomic payload with camelCase properties
		const payload = {
			body: convertedBody as TBody,
			pathParams: requestData.path_params,
			queryParams: requestData.query_params,
			headers: requestData.headers,
			cookies: requestData.cookies,
		};

		// Call the user's handler
		const result = await handler(payload);

		// Serialize result back to JSON for Rust
		return JSON.stringify(result);
	};
}

/**
 * Alternative wrapper for handlers that want full request context
 *
 * @param handler - Handler function that receives full request context
 * @returns Raw handler function compatible with Rust FFI
 */
export function wrapHandlerWithContext<TBody = unknown>(
	handler: (payload: HandlerPayload & { body: TBody }) => unknown | Promise<unknown>,
): HandlerFunction {
	return async (requestJson: string): Promise<string> => {
		const requestData = JSON.parse(requestJson) as RequestData;

		const convertedBody = convertHandlerBody(requestData.body);

		const payload = {
			path: requestData.path,
			method: requestData.method,
			body: convertedBody as TBody,
			pathParams: requestData.path_params,
			queryParams: requestData.query_params,
			headers: requestData.headers,
			cookies: requestData.cookies,
		};

		const result = await handler(payload);

		return JSON.stringify(result);
	};
}

/**
 * Body-only wrapper for simple handlers that only need the request body
 *
 * This provides the most ergonomic API:
 * ```typescript
 * async function handler(body: UploadRequest) {
 *   return { filename: body.file.filename };
 * }
 * ```
 *
 * @param handler - Handler function that receives only the converted body
 * @returns Raw handler function compatible with Rust FFI
 */
export function wrapBodyHandler<TBody = unknown>(
	handler: (body: TBody) => unknown | Promise<unknown>,
): HandlerFunction {
	return async (requestJson: string): Promise<string> => {
		const requestData = JSON.parse(requestJson) as RequestData;

		const convertedBody = convertHandlerBody(requestData.body);

		const result = await handler(convertedBody as TBody);

		return JSON.stringify(result);
	};
}
