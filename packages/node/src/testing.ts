/**
 * Testing utilities for Spikard applications
 */

import {
	TestClient as NativeTestClient,
	type TestResponse as NativeTestResponse,
	type WebSocketTestConnection,
} from "../index.js";
import type { ServerConfig } from "./config";
import type { HandlerFunction, SpikardApp } from "./index";
import type { JsonValue } from "./types";

/**
 * HTTP response from test client
 */
export type TestResponse = NativeTestResponse;

interface MultipartFile {
	name: string;
	filename?: string;
	content: string;
	contentType?: string;
}

export interface RequestOptions {
	headers?: Record<string, string>;
	json?: JsonValue;
	form?: Record<string, JsonValue>;
	multipart?: {
		fields?: Record<string, JsonValue>;
		files?: MultipartFile[];
	};
}

type MultipartPayload = {
	__spikard_multipart__: {
		fields: Record<string, JsonValue>;
		files: MultipartFile[];
	};
};

type FormPayload = {
	__spikard_form__: Record<string, JsonValue>;
};

type NativeBody = MultipartPayload | FormPayload | JsonValue | null;

interface NativeClient {
	get(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	post(path: string, headers: Record<string, string> | null, body: NativeBody): Promise<TestResponse>;
	put(path: string, headers: Record<string, string> | null, body: NativeBody): Promise<TestResponse>;
	delete(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	patch(path: string, headers: Record<string, string> | null, body: NativeBody): Promise<TestResponse>;
	head(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	options(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	trace(path: string, headers: Record<string, string> | null): Promise<TestResponse>;
	websocket(path: string): Promise<WebSocketTestConnection>;
}

type NativeClientConstructor = new (
	routesJson: string,
	handlers: Record<string, HandlerFunction>,
	config: ServerConfig | null,
) => NativeClient;

type NativeClientFactory = (
	routesJson: string,
	handlers: Record<string, HandlerFunction>,
	config: ServerConfig | null,
) => NativeClient;

const defaultNativeClientFactory: NativeClientFactory = (routesJson, handlers, config) => {
	const Ctor = NativeTestClient as NativeClientConstructor;
	return new Ctor(routesJson, handlers, config);
};

let nativeClientFactory: NativeClientFactory = defaultNativeClientFactory;

export const __setNativeClientFactory = (factory?: NativeClientFactory): void => {
	nativeClientFactory = factory ?? defaultNativeClientFactory;
};

/**
 * Test client for making HTTP requests to Spikard applications
 *
 * Provides a high-level API for testing HTTP endpoints without
 * starting an actual server.
 *
 * @example
 * ```typescript
 * import { TestClient } from '@spikard/node';
 *
 * const app = {
 *   routes: [
 *     {
 *       method: 'GET',
 *       path: '/users/:id',
 *       handler_name: 'getUser',
 *       is_async: true
 *     }
 *   ],
 *   handlers: {
 *     getUser: async (req) => ({ id: req.params.id, name: 'Alice' })
 *   }
 * };
 *
 * const client = new TestClient(app);
 * const response = await client.get('/users/123');
 * console.log(response.json()); // { id: '123', name: 'Alice' }
 * ```
 */
export class TestClient {
	private nativeClient: NativeClient;

	/**
	 * Create a new test client
	 *
	 * @param app - Spikard application with routes and handlers
	 */
	constructor(app: SpikardApp) {
		if (!app || !Array.isArray(app.routes)) {
			throw new Error("Invalid Spikard app: missing routes array");
		}
		this.app = app;
		const routesJson = JSON.stringify(app.routes);
		const handlersMap = app.handlers || {};
		const config = app.config ?? null;

		this.nativeClient = nativeClientFactory(routesJson, handlersMap, config);
	}

	/**
	 * Make a GET request
	 *
	 * @param path - Request path
	 * @param headers - Optional request headers
	 * @returns Response promise
	 */
	async get(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		return this.nativeClient.get(path, headers || null);
	}

	/**
	 * Make a POST request
	 *
	 * @param path - Request path
	 * @param options - Request options
	 * @returns Response promise
	 */
	async post(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.post(path, this.buildHeaders(options), this.buildBody(options));
	}

	/**
	 * Make a PUT request
	 *
	 * @param path - Request path
	 * @param options - Request options
	 * @returns Response promise
	 */
	async put(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.put(path, this.buildHeaders(options), this.buildBody(options));
	}

	/**
	 * Make a DELETE request
	 *
	 * @param path - Request path
	 * @param headers - Optional request headers
	 * @returns Response promise
	 */
	async delete(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		return this.nativeClient.delete(path, headers || null);
	}

	/**
	 * Make a PATCH request
	 *
	 * @param path - Request path
	 * @param options - Request options
	 * @returns Response promise
	 */
	async patch(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.patch(path, this.buildHeaders(options), this.buildBody(options));
	}

	/**
	 * Make a HEAD request
	 *
	 * @param path - Request path
	 * @param headers - Optional request headers
	 * @returns Response promise
	 */
	async head(path: string, headers?: Record<string, string>): Promise<TestResponse> {
		return this.nativeClient.head(path, headers || null);
	}

	/**
	 * Make an OPTIONS request
	 *
	 * @param path - Request path
	 * @param options - Request options
	 * @returns Response promise
	 */
	async options(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.options(path, this.buildHeaders(options));
	}

	/**
	 * Make a TRACE request
	 *
	 * @param path - Request path
	 * @param headers - Optional request headers
	 * @returns Response promise
	 */
	async trace(path: string, options?: RequestOptions): Promise<TestResponse> {
		return this.nativeClient.trace(path, this.buildHeaders(options));
	}

	private buildHeaders(options?: RequestOptions): Record<string, string> | null {
		if (!options?.headers || Object.keys(options.headers).length === 0) {
			return null;
		}
		return options.headers;
	}

	private buildBody(options?: RequestOptions): NativeBody {
		if (!options) {
			return null;
		}

		if (options.multipart) {
			return {
				__spikard_multipart__: {
					fields: options.multipart.fields ?? {},
					files: options.multipart.files ?? [],
				},
			};
		}

		if (options.form) {
			return {
				__spikard_form__: options.form,
			};
		}

		if ("json" in options) {
			return options.json ?? null;
		}

		return null;
	}

	/**
	 * Connect to a WebSocket endpoint
	 *
	 * Uses the native test client to create an in-memory WebSocket connection.
	 *
	 * @param path - WebSocket path
	 * @returns WebSocket test connection
	 */
	async websocketConnect(path: string): Promise<WebSocketTestConnection> {
		return this.nativeClient.websocket(path);
	}

	/**
	 * Cleanup resources when test client is done
	 */
	async cleanup(): Promise<void> {}
}
