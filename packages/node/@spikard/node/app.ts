/**
 * Ergonomic typed-handler App wrapper for spikard.
 *
 * This module provides a high-level `App` interface with verb decorators
 * (@app.post, @app.get, etc.) that support typed request/response handling via zod schemas.
 *
 * Request validation happens in the Rust core and returns 422 ProblemDetails on validation
 * failure. Zod schemas are converted to JSON Schema and attached to routes. Handlers receive
 * validated, typed request data.
 *
 * The low-level service App (supporting raw handler registration) is available via
 * `import { App } from 'spikard/service'` for advanced use cases.
 */

import * as service from "./service.js";
import type {
  ApiKeyAuthConfig,
  AuthorizationConfig,
  CompressionConfig,
  CorsConfig,
  JsonValue,
  JwtAuthConfig,
  LifecycleHooksConfig,
  RateLimitConfig,
} from "./index.js";
import type { z } from "zod";
import { zodToJsonSchema } from "zod-to-json-schema";

/**
 * Request object passed to typed handlers.
 * Contains validated request data + extracted parameters from RequestData.
 */
export interface TypedRequest<T = unknown> {
  /** Validated and parsed request body (type-safe, matches schema) */
  body: T;
  /** Path parameters (e.g., /users/:id → { id: "123" }) */
  params: Record<string, string>;
  /** Query string parameters */
  query: Record<string, string>;
  /** HTTP headers (case-insensitive access recommended) */
  headers: Record<string, string>;
  /** Cookies */
  cookies: Record<string, string>;
  /** HTTP method (GET, POST, etc.) */
  method: string;
  /** Request path */
  path: string;
  /** Content-Type header value, if present */
  contentType?: string;
}

/**
 * Response object returned by typed handlers.
 * The handler can omit fields; defaults are applied.
 */
export interface TypedResponse<T = unknown> {
  /** HTTP status code (default: 200) */
  statusCode?: number;
  /** Response content (will be JSON-serialized if not a string/buffer) */
  content?: T;
  /** Response headers */
  headers?: Record<string, string>;
}

/**
 * Handler function signature: receives a typed request, returns a response.
 */
export type Handler<ReqBody = unknown, ResBody = unknown> = (
  req: TypedRequest<ReqBody>,
) => Promise<TypedResponse<ResBody>>;

/**
 * Per-route middleware configuration.
 *
 * Every field maps one-to-one onto a `RouteBuilder` setter; the middleware itself is applied by
 * the Rust core, so these options only carry configuration across the FFI boundary. Omitting a
 * field leaves the server-global default in place, which is why every option is optional rather
 * than nullable: the core distinguishes "unset" from an explicitly empty configuration. ~keep
 */
export interface RouteMiddlewareConfig {
  /** Per-route CORS configuration, overriding the server-global default */
  cors?: CorsConfig;
  /** Per-route response compression configuration, overriding the server-global default */
  compression?: CompressionConfig;
  /** Per-route maximum request body size in bytes, overriding the server-global default */
  bodyLimit?: number;
  /** Per-route request timeout in seconds, overriding the server-global default */
  requestTimeout?: number;
  /** Per-route rate limiting configuration, overriding the server-global default */
  rateLimit?: RateLimitConfig;
  /** Force per-route request-id generation on or off, overriding the server-global default */
  requestId?: boolean;
  /** Require JWT authentication for this route */
  jwtAuth?: JwtAuthConfig;
  /** Require API key authentication for this route */
  apiKeyAuth?: ApiKeyAuthConfig;
  /** Roles/scopes/permissions authorization requirement for this route */
  authorization?: AuthorizationConfig;
  /** Registered lifecycle hooks to run for this route */
  lifecycleHooks?: LifecycleHooksConfig;
  /** Literal OpenRPC method spec document for this route */
  openrpcSpec?: JsonValue;
}

/**
 * Route configuration with optional schema validation and per-route middleware.
 * Schemas are converted to JSON Schema and attached to the Rust core for validation.
 */
export interface RouteConfig<ReqBody = unknown> extends RouteMiddlewareConfig {
  /** Zod schema to validate the request body (converted to JSON Schema in Rust core) */
  body?: z.ZodType<ReqBody>;
  /** Optional query parameters schema (for future use) */
  query?: z.ZodType<unknown>;
  /** Optional path parameters schema (for future use) */
  params?: z.ZodType<unknown>;
}

/**
 * Ergonomic, typed-handler App wrapper.
 *
 * Provides a clean interface for defining routes with type-safe handlers.
 * Automatically converts zod schemas to JSON Schema and attaches them to routes.
 * The Rust core validates all inputs and returns 422 ProblemDetails on validation failure.
 * Handlers receive typed, validated request data.
 */
export class App {
  private serviceApp: service.App;

  constructor() {
    this.serviceApp = new service.App();
  }

  /**
   * Register a GET route with optional typed request validation.
   */
  get<ReqBody = unknown, ResBody = unknown>(
    path: string,
    config: RouteConfig<ReqBody> = {},
    handler: Handler<ReqBody, ResBody>,
  ): void {
    this.registerRoute("GET", path, config, handler);
  }

  /**
   * Register a POST route with optional typed request validation.
   */
  post<ReqBody = unknown, ResBody = unknown>(
    path: string,
    config: RouteConfig<ReqBody> = {},
    handler: Handler<ReqBody, ResBody>,
  ): void {
    this.registerRoute("POST", path, config, handler);
  }

  /**
   * Register a PUT route with optional typed request validation.
   */
  put<ReqBody = unknown, ResBody = unknown>(
    path: string,
    config: RouteConfig<ReqBody> = {},
    handler: Handler<ReqBody, ResBody>,
  ): void {
    this.registerRoute("PUT", path, config, handler);
  }

  /**
   * Register a PATCH route with optional typed request validation.
   */
  patch<ReqBody = unknown, ResBody = unknown>(
    path: string,
    config: RouteConfig<ReqBody> = {},
    handler: Handler<ReqBody, ResBody>,
  ): void {
    this.registerRoute("PATCH", path, config, handler);
  }

  /**
   * Register a DELETE route with optional typed request validation.
   */
  delete<ReqBody = unknown, ResBody = unknown>(
    path: string,
    config: RouteConfig<ReqBody> = {},
    handler: Handler<ReqBody, ResBody>,
  ): void {
    this.registerRoute("DELETE", path, config, handler);
  }

  /**
   * Register a HEAD route with optional typed request validation.
   */
  head<ReqBody = unknown, ResBody = unknown>(
    path: string,
    config: RouteConfig<ReqBody> = {},
    handler: Handler<ReqBody, ResBody>,
  ): void {
    this.registerRoute("HEAD", path, config, handler);
  }

  /**
   * Register an OPTIONS route with optional typed request validation.
   */
  options<ReqBody = unknown, ResBody = unknown>(
    path: string,
    config: RouteConfig<ReqBody> = {},
    handler: Handler<ReqBody, ResBody>,
  ): void {
    this.registerRoute("OPTIONS", path, config, handler);
  }

  /**
   * Register a generic route with the given method.
   *
   * Zod schemas are converted to JSON Schema and attached to the RouteBuilder.
   * The Rust core validates the request and returns 422 ProblemDetails on validation failure.
   * Handlers receive already-validated request data.
   */
  private registerRoute<ReqBody = unknown, ResBody = unknown>(
    method: string,
    path: string,
    config: RouteConfig<ReqBody>,
    handler: Handler<ReqBody, ResBody>,
  ): void {
    // Get the method enum value
    const methodEnum = this.getMethodEnum(method);
    if (!methodEnum) {
      throw new Error(`Unknown HTTP method: ${method}`);
    }

    // Create the route builder
    let builder = new service.RouteBuilder(methodEnum, path);

    // Convert zod schema to JSON Schema and attach to builder
    // The Rust core will validate the request body against this schema
    // and return 422 ProblemDetails if validation fails
    if (config.body) {
      const jsonSchema = zodToJsonSchema(config.body);
      builder = builder.requestSchemaJson(jsonSchema);
    }

    builder = this.applyMiddleware(builder, config);

    // Create the low-level handler that bridges the typed interface to the raw contract
    // At this point, the request body is already validated by the Rust core
    const bridgeHandler = async (requestData: Record<string, any>): Promise<Record<string, any>> => {
      try {
        // Extract and parse the request body
        // The body is already validated by the Rust core, so we just parse it
        // with zod to get the typed value (type coercion, not validation)
        let body = requestData.body;
        if (config.body) {
          // Body is already valid per JSON Schema, just parse for typing
          body = config.body.parse(body);
        }

        // Extract path and query parameters
        const params = requestData.path_params || {};
        const query = requestData.query_params || {};

        // Build the TypedRequest object
        const typedRequest: TypedRequest<ReqBody> = {
          body,
          params,
          query,
          headers: requestData.headers || {},
          cookies: requestData.cookies || {},
          method: requestData.method || "GET",
          path: requestData.path || "/",
          contentType: requestData.content_type,
        };

        // Call the user's handler
        const response = await handler(typedRequest);

        // Convert the TypedResponse to the low-level contract
        // Note: field name is 'content', not 'body'
        return {
          status_code: response.statusCode ?? 200,
          content: response.content,
          headers: response.headers ?? {},
        };
      } catch (error: any) {
        // Unexpected error: return 500 Internal Server Error
        // (validation errors are already caught by Rust core and return 422)
        return {
          status_code: 500,
          content: {
            error: "Internal server error",
            details: error instanceof Error ? error.message : String(error),
          },
          headers: { "content-type": "application/json" },
        };
      }
    };

    // Register with the low-level service App
    this.serviceApp.registerRoute(builder, bridgeHandler);
  }

  /**
   * Forward the per-route middleware options onto the RouteBuilder.
   *
   * Each setter is called only when the option is present, so an unset option leaves the
   * server-global default untouched rather than overriding it with an empty configuration. ~keep
   */
  private applyMiddleware(builder: service.RouteBuilder, config: RouteMiddlewareConfig): service.RouteBuilder {
    let result = builder;
    if (config.cors !== undefined) {
      result = result.cors(config.cors);
    }
    if (config.compression !== undefined) {
      result = result.compression(config.compression);
    }
    if (config.bodyLimit !== undefined) {
      result = result.bodyLimit(config.bodyLimit);
    }
    if (config.requestTimeout !== undefined) {
      result = result.requestTimeout(config.requestTimeout);
    }
    if (config.rateLimit !== undefined) {
      result = result.rateLimit(config.rateLimit);
    }
    if (config.requestId !== undefined) {
      result = result.requestId(config.requestId);
    }
    if (config.jwtAuth !== undefined) {
      result = result.jwtAuth(config.jwtAuth);
    }
    if (config.apiKeyAuth !== undefined) {
      result = result.apiKeyAuth(config.apiKeyAuth);
    }
    if (config.authorization !== undefined) {
      result = result.authorization(config.authorization);
    }
    if (config.lifecycleHooks !== undefined) {
      result = result.lifecycleHooks(config.lifecycleHooks);
    }
    if (config.openrpcSpec !== undefined) {
      result = result.openrpcSpec(config.openrpcSpec);
    }
    return result;
  }

  /**
   * Convert HTTP method string to the service Method enum.
   */
  private getMethodEnum(method: string): (typeof service.Method)[keyof typeof service.Method] | undefined {
    const methodMap: Record<string, (typeof service.Method)[keyof typeof service.Method]> = {
      GET: service.Method.Get,
      POST: service.Method.Post,
      PUT: service.Method.Put,
      PATCH: service.Method.Patch,
      DELETE: service.Method.Delete,
      HEAD: service.Method.Head,
      OPTIONS: service.Method.Options,
      CONNECT: service.Method.Connect,
      TRACE: service.Method.Trace,
    };
    return methodMap[method.toUpperCase()];
  }

  /**
   * Configure the server (host, port, etc.).
   */
  config(config: service.ServerConfig): void {
    this.serviceApp.config(config);
  }

  /**
   * Start the HTTP server.
   */
  async run(): Promise<void> {
    await this.serviceApp.run();
  }

  /**
   * Build a router (returns the underlying router, advanced use case).
   */
  async intoRouter(): Promise<void> {
    await this.serviceApp.intoRouter();
  }

  /**
   * Get the underlying low-level service App (for advanced use cases).
   */
  getServiceApp(): service.App {
    return this.serviceApp;
  }
}
