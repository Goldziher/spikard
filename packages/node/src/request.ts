import { convertHandlerBody } from "./converters";
import type { JsonValue } from "./types";

export interface Request {
	method: string;
	path: string;
	params: Record<string, string>;
	query: Record<string, string>;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	body: Buffer | null;
	dependencies: Record<string, unknown> | undefined;
	json<T = unknown>(): T;
	form(): Record<string, string>;
}

export interface NativeRequestData {
	method: string;
	path: string;
	params: Record<string, string>;
	query: Record<string, string>;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	body: number[] | null;
	dependencies?: Record<string, unknown>;
}

class RequestImpl implements Request {
	method: string;
	path: string;
	params: Record<string, string>;
	query: Record<string, string>;
	headers: Record<string, string>;
	cookies: Record<string, string>;
	body: Buffer | null;
	dependencies: Record<string, unknown> | undefined;

	#jsonCache: unknown | undefined;
	#formCache: Record<string, string> | undefined;

	constructor(data: NativeRequestData) {
		this.method = data.method;
		this.path = data.path;
		this.params = data.params ?? {};
		this.query = data.query ?? {};
		this.headers = normalizeHeaders(data.headers ?? {});
		this.cookies = data.cookies ?? {};
		this.body = data.body ? Buffer.from(data.body) : null;
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

export function createRequest(data: NativeRequestData): Request {
	return new RequestImpl(data);
}
