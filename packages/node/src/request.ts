import { convertHandlerBody } from "./converters";
import type { JsonValue } from "./types";

export interface Request {
	method: string;
	path: string;
	params: Record<string, string>;
	/** Alias for params (for HandlerInput compatibility) */
	pathParams: Record<string, string>;
	query: Record<string, string>;
	/** Alias for query (for HandlerInput compatibility) */
	queryParams: Record<string, string>;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	body: Buffer | null;
	dependencies: Record<string, unknown> | undefined;
	json<T = JsonValue>(): T;
	form(): Record<string, string>;
}

export interface NativeRequestData {
	method: string;
	path: string;
	// Legacy JSON serialization uses params/query
	params?: Record<string, string>;
	query?: Record<string, string>;
	// Direct napi object uses pathParams/queryParams (from HandlerInput)
	pathParams?: Record<string, string>;
	queryParams?: unknown;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	body: number[] | unknown | null;
	dependencies?: Record<string, unknown>;
}

class RequestImpl implements Request {
	method: string;
	path: string;
	params: Record<string, string>;
	pathParams: Record<string, string>;
	query: Record<string, string>;
	queryParams: Record<string, string>;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	body: Buffer | null;
	dependencies: Record<string, unknown> | undefined;

	#jsonCache: unknown | undefined;
	#formCache: Record<string, string> | undefined;

	constructor(data: NativeRequestData) {
		this.method = data.method;
		this.path = data.path;
		// Support both field naming conventions (legacy JSON: params/query, napi object: pathParams/queryParams)
		const pathParams = data.params ?? data.pathParams ?? {};
		const queryParams = data.query ?? extractQueryParams(data.queryParams) ?? {};
		this.params = pathParams;
		this.pathParams = pathParams; // Alias for HandlerInput compatibility
		this.query = queryParams;
		this.queryParams = queryParams; // Alias for HandlerInput compatibility
		this.headers = normalizeHeaders(data.headers ?? {});
		this.cookies = data.cookies ?? {};
		this.body = convertBodyToBuffer(data.body);
		this.dependencies = data.dependencies;
	}

	json<T = unknown>(): T {
		if (this.#jsonCache !== undefined) {
			return this.#jsonCache as T;
		}

		if (!this.body || this.body.length === 0) {
			throw new Error("No body available to parse as JSON");
		}

		const raw = this.body.toString("utf-8");
		const parsed = JSON.parse(raw) as JsonValue;
		const converted = convertHandlerBody(parsed) as T;
		this.#jsonCache = converted;
		return converted;
	}

	form(): Record<string, string> {
		if (this.#formCache !== undefined) {
			return this.#formCache;
		}

		if (!this.body || this.body.length === 0) {
			throw new Error("No body available to parse as form data");
		}

		const text = this.body.toString("utf-8");
		const params = new URLSearchParams(text);
		const form: Record<string, string> = {};
		for (const [key, value] of params.entries()) {
			form[key] = value;
		}
		this.#formCache = form;
		return form;
	}
}

const normalizeHeaders = (headers: Record<string, string>): Record<string, string> =>
	Object.fromEntries(Object.entries(headers).map(([key, value]) => [key.toLowerCase(), value]));

/**
 * Extract query parameters from various formats.
 * When coming from napi directly, queryParams might be a serde_json::Value object.
 */
const extractQueryParams = (queryParams: unknown): Record<string, string> | undefined => {
	if (!queryParams || typeof queryParams !== "object") {
		return undefined;
	}
	// Convert object to string map (handles serde_json::Value from Rust)
	const result: Record<string, string> = {};
	for (const [key, value] of Object.entries(queryParams)) {
		result[key] = String(value);
	}
	return result;
};

/**
 * Convert body from various formats to Buffer.
 * Handles both legacy number[] arrays and direct serde_json::Value from napi.
 */
const convertBodyToBuffer = (body: number[] | unknown | null): Buffer | null => {
	// Null/undefined check
	if (body === null || body === undefined) {
		return null;
	}
	// Legacy format: array of byte numbers (from JSON serialization)
	if (Array.isArray(body)) {
		// If it's an array of numbers, convert to Buffer (legacy byte array format)
		if (body.length === 0 || body.every((b) => typeof b === "number")) {
			return Buffer.from(body as number[]);
		}
		// Otherwise serialize the array as JSON
		return Buffer.from(JSON.stringify(body), "utf-8");
	}
	// Direct napi object: serialize to JSON bytes
	if (typeof body === "object") {
		return Buffer.from(JSON.stringify(body), "utf-8");
	}
	// String body
	if (typeof body === "string") {
		return Buffer.from(body, "utf-8");
	}
	return null;
};

export function createRequest(data: NativeRequestData): Request {
	return new RequestImpl(data);
}
