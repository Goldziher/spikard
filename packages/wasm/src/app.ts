/**
 * Spikard application class
 */

import type { HandlerFunction, RouteMetadata, SpikardApp } from "./index";
import type { Request } from "./request";
import { runServer, type ServerOptions } from "./server";
import type { MaybePromise, StructuredHandlerResponse } from "./types";

/**
 * Dependency value or factory configuration
 */
export type DependencyValue = unknown;

/**
 * Factory function for creating dependencies
 */
export type DependencyFactory = (...args: unknown[]) => MaybePromise<unknown>;

/**
 * Dependency registration options
 */
export interface DependencyOptions {
	/** List of dependency keys this factory depends on */
	dependsOn?: string[];
	/** Whether this is a singleton (resolved once globally) */
	singleton?: boolean;
	/** Whether to cache per-request (default: false for factories, true for values) */
	cacheable?: boolean;
}

interface DependencyDescriptor {
	isFactory: boolean;
	value?: DependencyValue | undefined;
	factory?: ((dependenciesJson: string) => Promise<string>) | undefined;
	dependsOn: string[];
	singleton: boolean;
	cacheable: boolean;
}

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
export type LifecycleHookFunction = (payload: unknown) => MaybePromise<unknown>;

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
 * import { Spikard, get, post } from 'spikard';
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
	handlers: Record<string, HandlerFunction> = {};
	lifecycleHooks: LifecycleHooks = {
		onRequest: [],
		preValidation: [],
		preHandler: [],
		onResponse: [],
		onError: [],
	};
	dependencies: Record<string, DependencyDescriptor> = {};

	/**
	 * Add a route to the application
	 *
	 * @param metadata - Route configuration metadata
	 * @param handler - Handler function (sync or async)
	 */
	addRoute(metadata: RouteMetadata, handler: HandlerFunction): void {
		this.routes.push(metadata);
		this.handlers[metadata.handler_name] = handler;
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
	 * Register a dependency value or factory
	 *
	 * Dependencies are normalized to `snake_case` so fixtures and cross-language
	 * bindings stay aligned.
	 */
	provide(key: string, valueOrFactory: DependencyValue | DependencyFactory, options?: DependencyOptions): this {
		const normalizedKey = normalizeDependencyKey(key);
		const isFactory = typeof valueOrFactory === "function";

		const dependsOn =
			options?.dependsOn?.map((depKey) => normalizeDependencyKey(depKey)) ??
			(isFactory ? inferFactoryDependencies(valueOrFactory as DependencyFactory) : []);

		this.dependencies[normalizedKey] = {
			isFactory,
			value: isFactory ? undefined : valueOrFactory,
			factory: isFactory ? wrapFactory(valueOrFactory as DependencyFactory) : undefined,
			dependsOn,
			singleton: options?.singleton ?? false,
			cacheable: options?.cacheable ?? !isFactory,
		};

		return this;
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

function normalizeDependencyKey(key: string): string {
	if (key.includes("_")) {
		return key.toLowerCase();
	}
	return key
		.replace(/([a-z0-9])([A-Z])/g, "$1_$2")
		.replace(/([A-Z])([A-Z][a-z])/g, "$1_$2")
		.toLowerCase();
}

function inferFactoryDependencies(factory: DependencyFactory): string[] {
	const params = getFactoryParameterNames(factory);
	return params.map((name) => normalizeDependencyKey(name));
}

function getFactoryParameterNames(factory: DependencyFactory): string[] {
	const source = factory.toString().trim();
	const openParen = source.indexOf("(");
	if (openParen === -1) {
		return [];
	}
	const closeParen = source.indexOf(")", openParen + 1);
	if (closeParen === -1) {
		return [];
	}
	const raw = source.slice(openParen + 1, closeParen).trim();
	if (!raw) {
		return [];
	}

	return raw
		.split(",")
		.map((entry) => entry.trim())
		.filter((entry) => entry.length > 0)
		.map(
			(entry) =>
				entry
					.replace(/^\.\.\./, "")
					.split("=")[0]
					?.trim() ?? "",
		)
		.filter((entry) => entry.length > 0 && !entry.startsWith("{") && !entry.startsWith("["));
}

function wrapFactory(factory: DependencyFactory): (dependenciesJson: string) => Promise<string> {
	return async (dependenciesJson) => {
		const depsUnknown = JSON.parse(dependenciesJson) as unknown;
		const deps = typeof depsUnknown === "object" && depsUnknown ? (depsUnknown as Record<string, unknown>) : {};

		const paramNames = getFactoryParameterNames(factory);
		const args = paramNames.map((name) => deps[normalizeDependencyKey(name)]);

		const result = await factory(...args);

		if (result && typeof result === "object" && Symbol.asyncIterator in (result as object)) {
			const generator = result as AsyncGenerator<unknown, unknown, unknown> & { return?: (value?: unknown) => unknown };
			const first = await generator.next();
			const value = first.value;
			try {
				if (typeof generator.return === "function") {
					await generator.return(undefined);
				}
			} catch {}
			return JSON.stringify(value ?? null);
		}

		return JSON.stringify(result ?? null);
	};
}
