/**
 * gRPC Test Client for executing fixtures against running gRPC server.
 *
 * This module provides a wrapper for executing gRPC streaming fixtures
 * in integration tests with support for:
 * - All four streaming modes (unary, server, client, bidirectional)
 * - Metadata headers (authentication, tracing, etc.)
 * - Timeouts per request
 * - JSON-encoded messages (compatible with Spikard's gRPC implementation)
 *
 * Usage:
 *     const client = new GrpcTestClient('localhost:50051');
 *     const responses = await client.executeServerStreaming(
 *         'example.v1.StreamService',
 *         'GetStream',
 *         { request_id: 'test-001' },
 *         { authorization: 'Bearer token' },
 *         5.0,
 *     );
 */

/**
 * Test client for gRPC streaming operations.
 *
 * Provides methods to execute all four gRPC streaming modes:
 * - Unary: single request, single response
 * - Server streaming: single request, stream of responses
 * - Client streaming: stream of requests, single response
 * - Bidirectional: stream of requests, stream of responses
 *
 * Note: This is a fixture test helper that uses grpc-js for testing.
 * The actual implementation should be installed as a dev dependency
 * for integration testing only.
 */
export class GrpcTestClient {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	private grpc: any = null;
	private serverAddress: string;

	/**
	 * Initialize gRPC test client.
	 *
	 * @param serverAddress - Server address in format "host:port"
	 */
	constructor(serverAddress: string = "localhost:50051") {
		const normalized = typeof serverAddress === "string" ? serverAddress.trim() : "";
		this.serverAddress = normalized.length > 0 ? normalized : "localhost:50051";
		this.initializeGrpc();
	}

	/**
	 * Create a gRPC client bound to the configured server address.
	 */
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	private createClient(): any {
		if (!this.grpc) {
			throw new Error(
				`gRPC module not initialized. Cannot connect to ${this.serverAddress}. Is @grpc/grpc-js installed?`,
			);
		}

		const credentials = this.grpc.credentials?.createInsecure?.();
		if (!credentials) {
			throw new Error("gRPC credentials factory unavailable in @grpc/grpc-js");
		}

		const address = typeof this.serverAddress === "string" ? this.serverAddress.trim() : "";
		const target = address.length > 0 ? address : "localhost:50051";
		return new this.grpc.Client(target, credentials);
	}

	/**
	 * Initialize the gRPC module using @grpc/grpc-js.
	 *
	 * This is called automatically in the constructor.
	 * Stores a reference to the grpc module for making dynamic RPC calls.
	 */
	private initializeGrpc(): void {
		try {
			// eslint-disable-next-line @typescript-eslint/no-require-imports, @typescript-eslint/no-var-requires
			this.grpc = require("@grpc/grpc-js");
		} catch (error) {
			// If grpc-js is not available, the grpc reference will be null
			// and API calls will fail with proper error
			console.error(`Failed to initialize gRPC module:`, error instanceof Error ? error.message : String(error));
		}
	}

	/**
	 * Prepare metadata from dictionary format to gRPC format.
	 *
	 * @param metadata - Metadata dictionary from fixture
	 * @returns Metadata object for gRPC calls or null
	 */
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	private prepareMetadata(metadata?: Record<string, string>): any | null {
		if (!metadata || Object.keys(metadata).length === 0) {
			return null;
		}

		// Create a proper Metadata object using @grpc/grpc-js
		const grpcMetadata = new this.grpc.Metadata();
		for (const [key, value] of Object.entries(metadata)) {
			grpcMetadata.set(key, value);
		}
		return grpcMetadata;
	}

	/**
	 * Execute unary RPC from fixture.
	 *
	 * @param serviceName - Fully qualified service name (e.g., "example.v1.Service")
	 * @param methodName - Method name
	 * @param request - Request data as object
	 * @param metadata - Optional metadata headers
	 * @param timeout - Optional timeout in seconds
	 * @returns Response data as object
	 */
	async executeUnary(
		serviceName: string,
		methodName: string,
		request: Record<string, unknown>,
		metadata?: Record<string, string>,
		timeout?: number,
	): Promise<Record<string, unknown>> {
		const method = `/${serviceName}/${methodName}`;

		// Create a dynamic client for making unary RPC calls
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const client = this.createClient() as any;

		return new Promise((resolve, reject) => {
			try {
				// Make the unary RPC call directly
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				client.makeUnaryRequest(
					method,
					(value: Record<string, unknown>) => {
						return Buffer.from(JSON.stringify(value));
					},
					(value: Buffer) => {
						return JSON.parse(value.toString("utf-8"));
					},
					request,
					this.prepareMetadata(metadata),
					{
						deadline: timeout ? new Date(Date.now() + timeout * 1000) : undefined,
					},
					(err: Error | null, value: Record<string, unknown> | undefined) => {
						try {
							if (err) {
								reject(err);
							} else {
								resolve(value || {});
							}
						} finally {
							client.close();
						}
					},
				);
			} catch (error) {
				client.close();
				reject(error);
			}
		});
	}

	/**
	 * Execute server streaming RPC from fixture.
	 *
	 * @param serviceName - Fully qualified service name
	 * @param methodName - Method name
	 * @param request - Request data as object
	 * @param metadata - Optional metadata headers
	 * @param timeout - Optional timeout in seconds
	 * @returns List of response messages
	 */
	async executeServerStreaming(
		serviceName: string,
		methodName: string,
		request: Record<string, unknown>,
		metadata?: Record<string, string>,
		timeout?: number,
	): Promise<Array<Record<string, unknown>>> {
		const method = `/${serviceName}/${methodName}`;

		// Create a dynamic client for making server streaming RPC calls
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const client = this.createClient() as any;

		return new Promise((resolve, reject) => {
			const responses: Array<Record<string, unknown>> = [];
			let settled = false;
			let finalStatus: Record<string, unknown> | null = null;

			const finishWithError = (err: unknown): void => {
				if (settled) {
					return;
				}
				settled = true;
				client.close();
				reject(err);
			};

			const finishWithSuccess = (): void => {
				if (settled) {
					return;
				}
				settled = true;
				client.close();
				resolve(responses);
			};

			try {
				// Make the server streaming RPC call
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				const call = client.makeServerStreamRequest(
					method,
					(value: Record<string, unknown>) => {
						return Buffer.from(JSON.stringify(value));
					},
					(value: Buffer) => {
						return JSON.parse(value.toString("utf-8"));
					},
					request,
					this.prepareMetadata(metadata),
					{
						deadline: timeout ? new Date(Date.now() + timeout * 1000) : undefined,
					},
				);

				// Register error handler FIRST to catch errors early
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				call.on("error", (err: any) => {
					if (err && typeof err === "object") {
						(err as Record<string, unknown>).responses = responses;
					}
					finishWithError(err);
				});

				// Then register data and end handlers
				call.on("data", (response: Record<string, unknown>) => {
					responses.push(response);
				});

				call.on("status", (status: Record<string, unknown>) => {
					finalStatus = status;
					const code = status.code;
					if (typeof code === "number" && code !== 0) {
						const details = status.details ?? status.message ?? "gRPC error";
						const statusError = new Error(String(details));
						(statusError as Record<string, unknown>).code = code;
						(statusError as Record<string, unknown>).details = details;
						(statusError as Record<string, unknown>).metadata = status.metadata;
						(statusError as Record<string, unknown>).responses = responses;
						finishWithError(statusError);
					}
				});

				call.on("end", () => {
					const code = finalStatus?.code;
					if (typeof code === "number" && code !== 0) {
						const details = finalStatus?.details ?? finalStatus?.message ?? "gRPC error";
						const statusError = new Error(String(details));
						(statusError as Record<string, unknown>).code = code;
						(statusError as Record<string, unknown>).details = details;
						(statusError as Record<string, unknown>).metadata = finalStatus?.metadata;
						(statusError as Record<string, unknown>).responses = responses;
						finishWithError(statusError);
						return;
					}

					finishWithSuccess();
				});
			} catch (error) {
				finishWithError(error);
			}
		});
	}

	/**
	 * Execute client streaming RPC from fixture.
	 *
	 * @param serviceName - Fully qualified service name
	 * @param methodName - Method name
	 * @param requests - List of request messages
	 * @param metadata - Optional metadata headers
	 * @param timeout - Optional timeout in seconds
	 * @returns Response data as object
	 */
	async executeClientStreaming(
		serviceName: string,
		methodName: string,
		requests: Array<Record<string, unknown>>,
		metadata?: Record<string, string>,
		timeout?: number,
	): Promise<Record<string, unknown>> {
		const method = `/${serviceName}/${methodName}`;

		// Create a dynamic client for making client streaming RPC calls
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const client = this.createClient() as any;

		return new Promise((resolve, reject) => {
			try {
				// Make the client streaming RPC call
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				const call = client.makeClientStreamRequest(
					method,
					(value: Record<string, unknown>) => {
						return Buffer.from(JSON.stringify(value));
					},
					(value: Buffer) => {
						return JSON.parse(value.toString("utf-8"));
					},
					this.prepareMetadata(metadata),
					{
						deadline: timeout ? new Date(Date.now() + timeout * 1000) : undefined,
					},
					(err: Error | null, value: Record<string, unknown> | undefined) => {
						try {
							if (err) {
								reject(err);
							} else {
								resolve(value || {});
							}
						} finally {
							client.close();
						}
					},
				);

				// Register error handler FIRST to catch errors early
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				call.on("error", (err: any) => {
					client.close();
					reject(err);
				});

				// Write all requests after error handler is registered
				for (const request of requests) {
					call.write(request);
				}
				call.end();
			} catch (error) {
				client.close();
				reject(error);
			}
		});
	}

	/**
	 * Execute bidirectional streaming RPC from fixture.
	 *
	 * @param serviceName - Fully qualified service name
	 * @param methodName - Method name
	 * @param requests - List of request messages
	 * @param metadata - Optional metadata headers
	 * @param timeout - Optional timeout in seconds
	 * @returns List of response messages
	 */
	async executeBidirectional(
		serviceName: string,
		methodName: string,
		requests: Array<Record<string, unknown>>,
		metadata?: Record<string, string>,
		timeout?: number,
	): Promise<Array<Record<string, unknown>>> {
		const method = `/${serviceName}/${methodName}`;

		// Create a dynamic client for making bidirectional streaming RPC calls
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const client = this.createClient() as any;

		return new Promise((resolve, reject) => {
			const responses: Array<Record<string, unknown>> = [];
			let settled = false;
			let finalStatus: Record<string, unknown> | null = null;

			const finishWithError = (err: unknown): void => {
				if (settled) {
					return;
				}
				settled = true;
				client.close();
				reject(err);
			};

			const finishWithSuccess = (): void => {
				if (settled) {
					return;
				}
				settled = true;
				client.close();
				resolve(responses);
			};

			try {
				// Make the bidirectional streaming RPC call
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				const call = client.makeBidiStreamRequest(
					method,
					(value: Record<string, unknown>) => {
						return Buffer.from(JSON.stringify(value));
					},
					(value: Buffer) => {
						return JSON.parse(value.toString("utf-8"));
					},
					this.prepareMetadata(metadata),
					{
						deadline: timeout ? new Date(Date.now() + timeout * 1000) : undefined,
					},
				);

				// Register error handler FIRST to catch errors early
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				call.on("error", (err: any) => {
					if (err && typeof err === "object") {
						(err as Record<string, unknown>).responses = responses;
					}
					finishWithError(err);
				});

				// Then register data and end handlers
				call.on("data", (response: Record<string, unknown>) => {
					responses.push(response);
				});

				call.on("status", (status: Record<string, unknown>) => {
					finalStatus = status;
					const code = status.code;
					if (typeof code === "number" && code !== 0) {
						const details = status.details ?? status.message ?? "gRPC error";
						const statusError = new Error(String(details));
						(statusError as Record<string, unknown>).code = code;
						(statusError as Record<string, unknown>).details = details;
						(statusError as Record<string, unknown>).metadata = status.metadata;
						(statusError as Record<string, unknown>).responses = responses;
						finishWithError(statusError);
					}
				});

				call.on("end", () => {
					const code = finalStatus?.code;
					if (typeof code === "number" && code !== 0) {
						const details = finalStatus?.details ?? finalStatus?.message ?? "gRPC error";
						const statusError = new Error(String(details));
						(statusError as Record<string, unknown>).code = code;
						(statusError as Record<string, unknown>).details = details;
						(statusError as Record<string, unknown>).metadata = finalStatus?.metadata;
						(statusError as Record<string, unknown>).responses = responses;
						finishWithError(statusError);
						return;
					}

					finishWithSuccess();
				});

				// Write all requests after error handler is registered
				for (const request of requests) {
					call.write(request);
				}
				call.end();
			} catch (error) {
				finishWithError(error);
			}
		});
	}

	/**
	 * Set the gRPC module (internal use for fixture tests).
	 *
	 * Allows injection of a custom gRPC module for testing purposes.
	 *
	 * @param grpcModule - The gRPC module to use
	 */
	setGrpcModule(grpcModule: unknown): void {
		this.grpc = grpcModule;
	}
}
