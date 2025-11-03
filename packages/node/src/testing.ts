/**
 * Testing utilities for Spikard applications
 */

// Native module is built into this package directory
// @ts-expect-error - Native module will be available after build
import { TestClient as NativeTestClient } from "../spikard.node";
import type { SpikardApp, RouteMetadata } from "./index";

/**
 * HTTP response from test client
 */
export interface TestResponse {
	statusCode: number;
	headers(): any;
	text(): string;
	json(): any;
	bytes(): Buffer;
}

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
	private nativeClient: NativeTestClient;

	/**
	 * Create a new test client
	 *
	 * @param app - Spikard application with routes and handlers
	 */
	constructor(app: SpikardApp) {
		const routesJson = JSON.stringify(app.routes);
		const handlersMap = app.handlers || {};

		this.nativeClient = NativeTestClient.new(routesJson, handlersMap);
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
	async post(
		path: string,
		options?: { headers?: Record<string, string>; json?: any }
	): Promise<TestResponse> {
		return this.nativeClient.post(
			path,
			options?.headers || null,
			options?.json || null
		);
	}

	/**
	 * Make a PUT request
	 *
	 * @param path - Request path
	 * @param options - Request options
	 * @returns Response promise
	 */
	async put(
		path: string,
		options?: { headers?: Record<string, string>; json?: any }
	): Promise<TestResponse> {
		return this.nativeClient.put(
			path,
			options?.headers || null,
			options?.json || null
		);
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
	async patch(
		path: string,
		options?: { headers?: Record<string, string>; json?: any }
	): Promise<TestResponse> {
		return this.nativeClient.patch(
			path,
			options?.headers || null,
			options?.json || null
		);
	}
}
