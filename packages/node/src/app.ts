/**
 * Spikard application class
 */

import { isNativeHandler, wrapHandler } from "./handler-wrapper";
import type { HandlerFunction, NativeHandlerFunction, RouteMetadata, SpikardApp } from "./index";
import type { Request } from "./request";
import { runServer, type ServerOptions } from "./server";
import type { MaybePromise, StructuredHandlerResponse, WebSocketHandler, WebSocketOptions } from "./types";

/**
 * Payload type provided to lifecycle hooks
 */
export type LifecycleHookPayload = Request | StructuredHandlerResponse;

/**
 * Lifecycle hook function type
 *
 * Hooks can return:
 * - The (possibly modified) request to continue processing
 * - A Response object to short-circuit the request pipeline
 */
export type LifecycleHookFunction = (payload: LifecycleHookPayload) => MaybePromise<LifecycleHookPayload>;

/**
 * Container for lifecycle hooks
 */
export interface LifecycleHooks {
	onRequest: LifecycleHookFunction[];
	preValidation: LifecycleHookFunction[];
	preHandler: LifecycleHookFunction[];
	onResponse: LifecycleHookFunction[];
	onError: LifecycleHookFunction[];
}

/**
 * Spikard application
 *
 * @example
 * ```typescript
 * import { Spikard, get, post } from '@spikard/node';
 *
 * const app = new Spikard();
 *
 * get('/')(async function root() {
 *   return { message: 'Hello' };
 * });
 *
 * if (require.main === module) {
 *   app.run({ port: 8000 });
 * }
 * ```
 */
export class Spikard implements SpikardApp {
	routes: RouteMetadata[] = [];
	handlers: Record<string, NativeHandlerFunction> = {};
	websocketRoutes: RouteMetadata[] = [];
	websocketHandlers: Record<string, Record<string, unknown>> = {};
	lifecycleHooks: LifecycleHooks = {
		onRequest: [],
		preValidation: [],
		preHandler: [],
		onResponse: [],
		onError: [],
	};

	/**
	 * Add a route to the application
	 *
	 * @param metadata - Route configuration metadata
	 * @param handler - Handler function (sync or async)
	 */
	addRoute(metadata: RouteMetadata, handler: HandlerFunction | NativeHandlerFunction): void {
		this.routes.push(metadata);
		const nativeHandler = isNativeHandler(handler) ? handler : wrapHandler(handler as HandlerFunction);
		this.handlers[metadata.handler_name] = nativeHandler;
	}

	/**
	 * Register a WebSocket route (message-based)
	 */
	websocket(path: string, handler: WebSocketHandler, options: WebSocketOptions = {}): void {
		const handlerName =
			options.handlerName ?? `ws_${this.websocketRoutes.length}_${path}`.replace(/[^a-zA-Z0-9_]/g, "_");
		const route: RouteMetadata = {
			method: "GET",
			path,
			handler_name: handlerName,
			request_schema: options.messageSchema as never,
			response_schema: options.responseSchema as never,
			parameter_schema: undefined,
			file_params: undefined,
			is_async: true,
		};

		this.websocketRoutes.push(route);
		this.websocketHandlers[handlerName] = handler;
	}

	/**
	 * Run the server
	 *
	 * @param options - Server configuration
	 */
	run(options: ServerOptions = {}): void {
		runServer(this, options);
	}

	/**
	 * Register an onRequest lifecycle hook
	 *
	 * Runs before routing. Can inspect/modify the request or short-circuit with a response.
	 *
	 * @param hook - Async function that receives a request and returns either:
	 *               - The (possibly modified) request to continue processing
	 *               - A Response object to short-circuit the request pipeline
	 * @returns The hook function (for decorator usage)
	 *
	 * @example
	 * ```typescript
	 * app.onRequest(async (request) => {
	 *   console.log(`Request: ${request.method} ${request.path}`);
	 *   return request;
	 * });
	 * ```
	 */
	onRequest(hook: LifecycleHookFunction): LifecycleHookFunction {
		this.lifecycleHooks.onRequest.push(hook);
		return hook;
	}

	/**
	 * Register a preValidation lifecycle hook
	 *
	 * Runs after routing but before validation. Useful for rate limiting.
	 *
	 * @param hook - Async function that receives a request and returns either:
	 *               - The (possibly modified) request to continue processing
	 *               - A Response object to short-circuit the request pipeline
	 * @returns The hook function (for decorator usage)
	 *
	 * @example
	 * ```typescript
	 * app.preValidation(async (request) => {
	 *   if (tooManyRequests()) {
	 *     return { error: "Rate limit exceeded", status: 429 };
	 *   }
	 *   return request;
	 * });
	 * ```
	 */
	preValidation(hook: LifecycleHookFunction): LifecycleHookFunction {
		this.lifecycleHooks.preValidation.push(hook);
		return hook;
	}

	/**
	 * Register a preHandler lifecycle hook
	 *
	 * Runs after validation but before the handler. Ideal for authentication/authorization.
	 *
	 * @param hook - Async function that receives a request and returns either:
	 *               - The (possibly modified) request to continue processing
	 *               - A Response object to short-circuit the request pipeline
	 * @returns The hook function (for decorator usage)
	 *
	 * @example
	 * ```typescript
	 * app.preHandler(async (request) => {
	 *   if (!validToken(request.headers.authorization)) {
	 *     return { error: "Unauthorized", status: 401 };
	 *   }
	 *   return request;
	 * });
	 * ```
	 */
	preHandler(hook: LifecycleHookFunction): LifecycleHookFunction {
		this.lifecycleHooks.preHandler.push(hook);
		return hook;
	}

	/**
	 * Register an onResponse lifecycle hook
	 *
	 * Runs after the handler executes. Can modify the response.
	 *
	 * @param hook - Async function that receives a response and returns the (possibly modified) response
	 * @returns The hook function (for decorator usage)
	 *
	 * @example
	 * ```typescript
	 * app.onResponse(async (response) => {
	 *   response.headers["X-Frame-Options"] = "DENY";
	 *   return response;
	 * });
	 * ```
	 */
	onResponse(hook: LifecycleHookFunction): LifecycleHookFunction {
		this.lifecycleHooks.onResponse.push(hook);
		return hook;
	}

	/**
	 * Register an onError lifecycle hook
	 *
	 * Runs when an error occurs. Can customize error responses.
	 *
	 * @param hook - Async function that receives an error response and returns a (possibly modified) response
	 * @returns The hook function (for decorator usage)
	 *
	 * @example
	 * ```typescript
	 * app.onError(async (response) => {
	 *   response.headers["Content-Type"] = "application/json";
	 *   return response;
	 * });
	 * ```
	 */
	onError(hook: LifecycleHookFunction): LifecycleHookFunction {
		this.lifecycleHooks.onError.push(hook);
		return hook;
	}

	/**
	 * Get all registered lifecycle hooks
	 *
	 * @returns Dictionary of hook lists by type
	 */
	getLifecycleHooks(): LifecycleHooks {
		return {
			onRequest: [...this.lifecycleHooks.onRequest],
			preValidation: [...this.lifecycleHooks.preValidation],
			preHandler: [...this.lifecycleHooks.preHandler],
			onResponse: [...this.lifecycleHooks.onResponse],
			onError: [...this.lifecycleHooks.onError],
		};
	}
}

function wrapWebSocketHandler(handler: WebSocketHandler, options: WebSocketOptions): Record<string, unknown> {
	return {
		handleMessage: async (message: unknown): Promise<string> => {
			const result = await handler(message);
			if (result === undefined) {
				return "null";
			}
			return JSON.stringify(result);
		},
		onConnect: options.onConnect,
		onDisconnect: options.onDisconnect,
		_messageSchema: options.messageSchema,
		_responseSchema: options.responseSchema,
	};
}
