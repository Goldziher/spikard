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
 * - Only unary (request-response) calls are currently supported
 * - Streaming (client, server, or bidirectional) is not yet implemented
 * - Binary metadata values are not accessible (only ASCII string metadata)
 *
 * # Example
 *
 * ```typescript
 * import { GrpcHandler, GrpcRequest, GrpcResponse } from 'spikard';
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
 * // Register handler with Spikard
 * const handler = new UserServiceHandler();
 * app.registerGrpcHandler('mypackage.UserService', handler);
 * ```
 */

/**
 * gRPC request received by handlers
 *
 * Contains the parsed gRPC request with all components extracted from
 * the gRPC wire format.
 */
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
	metadata: Record<string, string>;
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
	metadata?: Record<string, string>;
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
export interface GrpcServiceConfig {
	/**
	 * Fully qualified service name
	 *
	 * This must match the service name in your .proto file.
	 *
	 * Example: "mypackage.UserService"
	 */
	serviceName: string;

	/**
	 * Handler implementation
	 *
	 * An object implementing the GrpcHandler interface.
	 */
	handler: GrpcHandler;
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

			return {
				payload: Buffer.from(encoded),
				metadata: responseMetadata,
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
