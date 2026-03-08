/**
 * gRPC support for Spikard
 *
 * This module provides TypeScript bindings for implementing gRPC service handlers
 * that connect to Spikard's Rust-based gRPC runtime.
 *
 * # Architecture
 *
 * The gRPC binding follows the FFI pattern:
 * 1. TypeScript handler implements GrpcHandler interface
 * 2. Rust side (NodeGrpcHandler) calls JavaScript via napi-rs ThreadsafeFunction
 * 3. Protobuf serialization/deserialization happens in JavaScript using protobufjs
 *
 * # Current Limitations
 *
 * - Public TypeScript helpers cover unary, server-streaming, client-streaming,
 *   and bidirectional-streaming registration
 * - Binary metadata values are not accessible (only ASCII string metadata)
 *
 * # Example
 *
 * ```typescript
 * import {
 *   GrpcHandler,
 *   GrpcRequest,
 *   GrpcResponse,
 *   GrpcService,
 *   GrpcServerStreamingHandler,
 * } from 'spikard';
 * import * as $protobuf from 'protobufjs';
 *
 * // Generated protobuf types (using protobufjs)
 * interface User {
 *   id: number;
 *   name: string;
 *   email?: string;
 * }
 *
 * interface GetUserRequest {
 *   id: number;
 * }
 *
 * class UserServiceHandler implements GrpcHandler {
 *   async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
 *     if (request.methodName === 'GetUser') {
 *       // Deserialize request using protobufjs
 *       const req = UserService.GetUserRequest.decode(request.payload);
 *
 *       // Process request
 *       const user: User = {
 *         id: req.id,
 *         name: 'John Doe',
 *         email: 'john@example.com'
 *       };
 *
 *       // Serialize response
 *       const encoded = UserService.User.encode(user).finish();
 *       return {
 *         payload: Buffer.from(encoded)
 *       };
 *     }
 *
 *     throw new Error(`Unknown method: ${request.methodName}`);
 *   }
 * }
 *
 * // Register handlers in a service registry
 * const service = new GrpcService();
 * service.registerUnary('mypackage.UserService', 'GetUser', new UserServiceHandler());
 * service.registerServerStreaming('mypackage.UserService', 'ListUsers', {
 *   async handleServerStream(request) {
 *     return { messages: [Buffer.from([])] };
 *   }
 * } satisfies GrpcServerStreamingHandler);
 *
 * const app = new Spikard();
 * app.useGrpc(service);
 * ```
 */

/**
 * gRPC request received by handlers
 *
 * Contains the parsed gRPC request with all components extracted from
 * the gRPC wire format.
 */
export type GrpcMetadata = Record<string, string>;

export interface GrpcRequest {
	/**
	 * Fully qualified service name
	 *
	 * Example: "mypackage.UserService"
	 */
	serviceName: string;

	/**
	 * Method name being called
	 *
	 * Example: "GetUser", "ListUsers", "CreateUser"
	 */
	methodName: string;

	/**
	 * Serialized protobuf message payload
	 *
	 * This is the raw bytes of the protobuf message. You must deserialize
	 * it using protobufjs or another protobuf library.
	 *
	 * Example:
	 * ```typescript
	 * const req = UserService.GetUserRequest.decode(request.payload);
	 * ```
	 */
	payload: Buffer;

	/**
	 * gRPC metadata (similar to HTTP headers)
	 *
	 * Key-value pairs of metadata sent with the request.
	 *
	 * Example: { "authorization": "Bearer token123", "x-request-id": "abc" }
	 */
	metadata: GrpcMetadata;
}

/**
 * gRPC response returned by handlers
 *
 * Contains the serialized protobuf response and optional metadata to
 * include in the gRPC response headers.
 */
export interface GrpcResponse {
	/**
	 * Serialized protobuf message payload
	 *
	 * This must be the raw bytes of a serialized protobuf message.
	 * Use protobufjs to serialize your response object.
	 *
	 * Example:
	 * ```typescript
	 * const user = UserService.User.create({ id: 1, name: 'John' });
	 * const payload = Buffer.from(UserService.User.encode(user).finish());
	 * ```
	 */
	payload: Buffer;

	/**
	 * Optional gRPC metadata to include in response
	 *
	 * Key-value pairs of metadata to send back to the client.
	 *
	 * Example: { "x-server-id": "server-1", "x-cache-status": "hit" }
	 */
	metadata?: GrpcMetadata;
}

export interface GrpcClientStreamRequest {
	serviceName: string;
	methodName: string;
	metadata: GrpcMetadata;
	messages: Buffer[];
}

export interface GrpcServerStreamResponse {
	messages: Buffer[];
}

export interface GrpcBidiStreamRequest {
	serviceName: string;
	methodName: string;
	metadata: GrpcMetadata;
	messages: Buffer[];
}

export interface GrpcBidiStreamResponse {
	messages: Buffer[];
	metadata?: GrpcMetadata;
}

/**
 * gRPC handler interface
 *
 * Implement this interface to handle gRPC requests for a service.
 * The handler is responsible for:
 * 1. Deserializing the request payload
 * 2. Processing the request
 * 3. Serializing the response payload
 * 4. Optionally setting response metadata
 */
export interface GrpcHandler {
	/**
	 * Handle a gRPC request
	 *
	 * This method is called for each incoming gRPC request to the service.
	 * It should:
	 * 1. Deserialize request.payload using protobufjs
	 * 2. Process the request based on request.methodName
	 * 3. Serialize the response using protobufjs
	 * 4. Return a GrpcResponse with the serialized payload
	 *
	 * # Error Handling
	 *
	 * If you throw an error, it will be converted to a gRPC status:
	 * - Regular Error: INTERNAL (code 13)
	 * - GrpcError: Uses the specified status code
	 *
	 * # Example
	 *
	 * ```typescript
	 * async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
	 *   switch (request.methodName) {
	 *     case 'GetUser': {
	 *       const req = UserService.GetUserRequest.decode(request.payload);
	 *       const user = await db.getUser(req.id);
	 *       if (!user) {
	 *         throw new GrpcError(GrpcStatusCode.NOT_FOUND, 'User not found');
	 *       }
	 *       return {
	 *         payload: Buffer.from(UserService.User.encode(user).finish())
	 *       };
	 *     }
	 *     default:
	 *       throw new GrpcError(GrpcStatusCode.UNIMPLEMENTED, 'Method not implemented');
	 *   }
	 * }
	 * ```
	 *
	 * @param request - The gRPC request to handle
	 * @returns Promise resolving to a GrpcResponse
	 * @throws GrpcError for specific gRPC status codes
	 * @throws Error for internal server errors (code 13)
	 */
	handleRequest(request: GrpcRequest): Promise<GrpcResponse>;
}

export interface GrpcServerStreamingHandler {
	/**
	 * Handle a server-streaming request and return ordered response messages.
	 *
	 * Each Buffer in `messages` is emitted as one gRPC response frame.
	 */
	handleServerStream(request: GrpcRequest): Promise<GrpcServerStreamResponse>;
}

export interface GrpcClientStreamingHandler {
	handleClientStream(request: GrpcClientStreamRequest): Promise<GrpcResponse>;
}

export interface GrpcBidirectionalStreamingHandler {
	handleBidiStream(request: GrpcBidiStreamRequest): Promise<GrpcBidiStreamResponse>;
}

/**
 * gRPC status codes
 *
 * Standard gRPC status codes as defined in the gRPC specification.
 * Use these with GrpcError to return specific error statuses.
 *
 * @see https://grpc.io/docs/guides/status-codes/
 */
export enum GrpcStatusCode {
	/** Not an error; returned on success */
	OK = 0,
	/** The operation was cancelled */
	CANCELLED = 1,
	/** Unknown error */
	UNKNOWN = 2,
	/** Client specified an invalid argument */
	INVALID_ARGUMENT = 3,
	/** Deadline expired before operation could complete */
	DEADLINE_EXCEEDED = 4,
	/** Some requested entity was not found */
	NOT_FOUND = 5,
	/** Some entity that we attempted to create already exists */
	ALREADY_EXISTS = 6,
	/** The caller does not have permission to execute the specified operation */
	PERMISSION_DENIED = 7,
	/** Some resource has been exhausted */
	RESOURCE_EXHAUSTED = 8,
	/** Operation was rejected because the system is not in a state required for the operation's execution */
	FAILED_PRECONDITION = 9,
	/** The operation was aborted */
	ABORTED = 10,
	/** Operation was attempted past the valid range */
	OUT_OF_RANGE = 11,
	/** Operation is not implemented or not supported/enabled */
	UNIMPLEMENTED = 12,
	/** Internal errors */
	INTERNAL = 13,
	/** The service is currently unavailable */
	UNAVAILABLE = 14,
	/** Unrecoverable data loss or corruption */
	DATA_LOSS = 15,
	/** The request does not have valid authentication credentials */
	UNAUTHENTICATED = 16,
}

/**
 * gRPC error with status code
 *
 * Throw this error from your handler to return a specific gRPC status code
 * to the client.
 *
 * # Example
 *
 * ```typescript
 * if (!user) {
 *   throw new GrpcError(GrpcStatusCode.NOT_FOUND, 'User not found');
 * }
 *
 * if (!hasPermission) {
 *   throw new GrpcError(GrpcStatusCode.PERMISSION_DENIED, 'Access denied');
 * }
 * ```
 */
export class GrpcError extends Error {
	/**
	 * gRPC status code
	 */
	public readonly code: GrpcStatusCode;

	/**
	 * Create a new gRPC error
	 *
	 * @param code - gRPC status code
	 * @param message - Error message
	 */
	constructor(code: GrpcStatusCode, message: string) {
		super(message);
		this.code = code;
		this.name = "GrpcError";
	}
}

/**
 * gRPC service configuration
 *
 * Configuration for a gRPC service registration.
 */
export type GrpcRpcMode = "unary" | "serverStreaming" | "clientStreaming" | "bidirectionalStreaming";

export type GrpcMethodHandler =
	| GrpcHandler
	| GrpcServerStreamingHandler
	| GrpcClientStreamingHandler
	| GrpcBidirectionalStreamingHandler;

export interface GrpcMethodConfig {
	serviceName: string;
	methodName: string;
	rpcMode: GrpcRpcMode;
	handler: GrpcMethodHandler;
}

/**
 * Service registry for gRPC handlers
 *
 * Mirrors the registry helpers exposed by the other language bindings.
 * A single registry can route requests for multiple fully-qualified gRPC services.
 */
export class GrpcService {
	private readonly methods = new Map<string, GrpcMethodConfig>();

	private methodKey(serviceName: string, methodName: string): string {
		return `${serviceName}/${methodName}`;
	}

	private registerMethod(config: GrpcMethodConfig): this {
		if (!config.serviceName) {
			throw new Error("Service name cannot be empty");
		}
		if (!config.methodName) {
			throw new Error("Method name cannot be empty");
		}

		switch (config.rpcMode) {
			case "unary":
				if (typeof (config.handler as GrpcHandler)?.handleRequest !== "function") {
					throw new TypeError("Unary handler must implement handleRequest(request)");
				}
				break;
			case "serverStreaming":
				if (typeof (config.handler as GrpcServerStreamingHandler)?.handleServerStream !== "function") {
					throw new TypeError("Server-streaming handler must implement handleServerStream(request)");
				}
				break;
			case "clientStreaming":
				if (typeof (config.handler as GrpcClientStreamingHandler)?.handleClientStream !== "function") {
					throw new TypeError("Client-streaming handler must implement handleClientStream(request)");
				}
				break;
			case "bidirectionalStreaming":
				if (typeof (config.handler as GrpcBidirectionalStreamingHandler)?.handleBidiStream !== "function") {
					throw new TypeError("Bidirectional-streaming handler must implement handleBidiStream(request)");
				}
				break;
		}

		this.methods.set(this.methodKey(config.serviceName, config.methodName), config);
		return this;
	}

	/**
	 * Register a unary handler for a fully-qualified service method.
	 *
	 * @param serviceName - Service name such as `mypackage.UserService`
	 * @param methodName - Method name such as `GetUser`
	 * @param handler - Handler implementation for that method
	 * @returns The registry for chaining
	 */
	registerUnary(serviceName: string, methodName: string, handler: GrpcHandler): this {
		return this.registerMethod({ serviceName, methodName, rpcMode: "unary", handler });
	}

	registerServerStreaming(serviceName: string, methodName: string, handler: GrpcServerStreamingHandler): this {
		return this.registerMethod({ serviceName, methodName, rpcMode: "serverStreaming", handler });
	}

	registerClientStreaming(serviceName: string, methodName: string, handler: GrpcClientStreamingHandler): this {
		return this.registerMethod({ serviceName, methodName, rpcMode: "clientStreaming", handler });
	}

	registerBidirectionalStreaming(
		serviceName: string,
		methodName: string,
		handler: GrpcBidirectionalStreamingHandler,
	): this {
		return this.registerMethod({ serviceName, methodName, rpcMode: "bidirectionalStreaming", handler });
	}

	/**
	 * Remove a handler from the registry.
	 *
	 * @param serviceName - Fully-qualified service name
	 * @param methodName - Method name
	 */
	unregister(serviceName: string, methodName: string): void {
		if (!this.methods.delete(this.methodKey(serviceName, methodName))) {
			throw new Error(`No handler registered for method: ${serviceName}/${methodName}`);
		}
	}

	/**
	 * Get the registration for a service method.
	 *
	 * @param serviceName - Fully-qualified service name
	 * @param methodName - Method name
	 * @returns The registered method configuration, if present
	 */
	getMethod(serviceName: string, methodName: string): GrpcMethodConfig | undefined {
		return this.methods.get(this.methodKey(serviceName, methodName));
	}

	/**
	 * List all registered service names.
	 *
	 * @returns Fully-qualified service names
	 */
	serviceNames(): string[] {
		return Array.from(new Set(Array.from(this.methods.values(), (entry) => entry.serviceName)));
	}

	methodNames(serviceName: string): string[] {
		return Array.from(this.methods.values())
			.filter((entry) => entry.serviceName === serviceName)
			.map((entry) => entry.methodName);
	}

	/**
	 * Check whether a specific service method is registered.
	 *
	 * @param serviceName - Fully-qualified service name
	 * @param methodName - Method name
	 * @returns True when a handler is registered for the method
	 */
	hasMethod(serviceName: string, methodName: string): boolean {
		return this.methods.has(this.methodKey(serviceName, methodName));
	}

	/**
	 * Return registered method entries.
	 */
	entries(): GrpcMethodConfig[] {
		return Array.from(this.methods.values());
	}

	/**
	 * Route a unary request to the registered method handler.
	 *
	 * @param request - Incoming gRPC request
	 * @returns Promise resolving to the handler response
	 * @throws GrpcError when no service is registered
	 */
	async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
		const method = this.getMethod(request.serviceName, request.methodName);
		if (!method) {
			throw new GrpcError(
				GrpcStatusCode.UNIMPLEMENTED,
				`No handler registered for method: ${request.serviceName}/${request.methodName}`,
			);
		}
		if (method.rpcMode !== "unary") {
			throw new GrpcError(
				GrpcStatusCode.UNIMPLEMENTED,
				`Method ${request.serviceName}/${request.methodName} is registered as ${method.rpcMode}`,
			);
		}

		return (method.handler as GrpcHandler).handleRequest(request);
	}
}

/**
 * Result type for unary handlers that can include response metadata
 */
export type UnaryHandlerResult<TResponse> =
	| TResponse
	| {
			response: TResponse;
			metadata?: Record<string, string>;
	  };

/**
 * Helper function to create a simple unary gRPC handler
 *
 * This is a convenience function for creating handlers that only implement
 * a single unary method.
 *
 * # Example
 *
 * ```typescript
 * // Simple response
 * const getUserHandler = createUnaryHandler<GetUserRequest, User>(
 *   'GetUser',
 *   async (req) => {
 *     const user = await db.getUser(req.id);
 *     if (!user) {
 *       throw new GrpcError(GrpcStatusCode.NOT_FOUND, 'User not found');
 *     }
 *     return user;
 *   },
 *   UserService.GetUserRequest,
 *   UserService.User
 * );
 *
 * // With response metadata
 * const getUserHandler = createUnaryHandler<GetUserRequest, User>(
 *   'GetUser',
 *   async (req) => {
 *     const user = await db.getUser(req.id);
 *     return {
 *       response: user,
 *       metadata: { 'x-cache-status': 'hit' }
 *     };
 *   },
 *   UserService.GetUserRequest,
 *   UserService.User
 * );
 * ```
 *
 * @param methodName - The gRPC method name
 * @param handler - Function to process the request
 * @param requestType - protobufjs Type for request deserialization
 * @param responseType - protobufjs Type for response serialization
 * @returns A GrpcHandler implementation
 */
export function createUnaryHandler<TRequest, TResponse>(
	methodName: string,
	handler: (request: TRequest, metadata: Record<string, string>) => Promise<UnaryHandlerResult<TResponse>>,
	requestType: { decode(buffer: Uint8Array): TRequest },
	responseType: { encode(message: TResponse): { finish(): Uint8Array } },
): GrpcHandler {
	return {
		async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
			if (request.methodName !== methodName) {
				throw new GrpcError(GrpcStatusCode.UNIMPLEMENTED, `Method ${request.methodName} not implemented`);
			}

			// Deserialize request
			const req = requestType.decode(request.payload);

			// Process request
			const result = await handler(req, request.metadata);

			// Extract response and metadata
			let response: TResponse;
			let responseMetadata: Record<string, string> | undefined;

			if (result && typeof result === "object" && "response" in result) {
				// Result includes metadata
				response = result.response;
				responseMetadata = result.metadata;
			} else {
				// Simple response without metadata
				response = result as TResponse;
			}

			// Serialize response
			const encoded = responseType.encode(response).finish();

			if (responseMetadata) {
				return {
					payload: Buffer.from(encoded),
					metadata: responseMetadata,
				};
			}

			return {
				payload: Buffer.from(encoded),
			};
		},
	};
}

/**
 * Helper function to create a multi-method gRPC handler
 *
 * This is a convenience function for creating handlers that implement
 * multiple methods in a single service.
 *
 * # Example
 *
 * ```typescript
 * const userServiceHandler = createServiceHandler({
 *   GetUser: createUnaryHandler(...),
 *   ListUsers: createUnaryHandler(...),
 *   CreateUser: createUnaryHandler(...),
 * });
 * ```
 *
 * @param methods - Map of method names to handlers
 * @returns A GrpcHandler implementation that routes to the appropriate method
 */
export function createServiceHandler(methods: Record<string, GrpcHandler>): GrpcHandler {
	return {
		async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
			const handler = methods[request.methodName];
			if (!handler) {
				throw new GrpcError(GrpcStatusCode.UNIMPLEMENTED, `Method ${request.methodName} not implemented`);
			}
			return handler.handleRequest(request);
		},
	};
}
