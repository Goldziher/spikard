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
	private channel: any = null;
	private serverAddress: string;

	/**
	 * Initialize gRPC test client.
	 *
	 * @param serverAddress - Server address in format "host:port"
	 */
	constructor(serverAddress: string = "localhost:50051") {
		this.serverAddress = serverAddress;
		this.initializeChannel();
	}

	/**
	 * Initialize the gRPC channel using @grpc/grpc-js.
	 *
	 * This is called automatically in the constructor.
	 * The channel is created with insecure credentials for test purposes.
	 */
	private initializeChannel(): void {
		try {
			// eslint-disable-next-line @typescript-eslint/no-require-imports, @typescript-eslint/no-var-requires
			const grpc = require("@grpc/grpc-js");

			// Create insecure channel for testing
			this.channel = new grpc.Channel(
				this.serverAddress,
				grpc.ChannelCredentials.createInsecure(),
			);
		} catch (error) {
			// If grpc-js is not available or channel creation fails,
			// the channel will be null and API calls will fail with proper error
			console.error(
				`Failed to initialize gRPC channel to ${this.serverAddress}:`,
				error instanceof Error ? error.message : String(error),
			);
		}
	}

	/**
	 * Prepare metadata from dictionary format to gRPC format.
	 *
	 * @param metadata - Metadata dictionary from fixture
	 * @returns List of [key, value] tuples or null
	 */
	private prepareMetadata(metadata?: Record<string, string>): Array<[string, string]> | null {
		if (!metadata || Object.keys(metadata).length === 0) {
			return null;
		}

		return Object.entries(metadata).map(([key, value]) => [key, value]);
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
		if (!this.channel) {
			throw new Error(
				`Channel not initialized. Cannot connect to ${this.serverAddress}. Is the gRPC server running?`,
			);
		}

		const method = `/${serviceName}/${methodName}`;

		// Create stub for unary RPC
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const stub = this.channel.makeUnaryRequest(
			method,
			(value: Record<string, unknown>) => {
				return Buffer.from(JSON.stringify(value));
			},
			(value: Buffer) => {
				return JSON.parse(value.toString("utf-8"));
			},
			request,
			{
				metadata: this.prepareMetadata(metadata),
				timeout: timeout ? timeout * 1000 : undefined,
			},
		);

		return new Promise((resolve, reject) => {
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			stub((err: any, value: any) => {
				if (err) {
					reject(err);
				} else {
					resolve(value);
				}
			});
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
		if (!this.channel) {
			throw new Error(
				`Channel not initialized. Cannot connect to ${this.serverAddress}. Is the gRPC server running?`,
			);
		}

		const method = `/${serviceName}/${methodName}`;

		// Create stub for server streaming RPC
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const call = this.channel.makeServerStreamRequest(
			method,
			(value: Record<string, unknown>) => {
				return Buffer.from(JSON.stringify(value));
			},
			(value: Buffer) => {
				return JSON.parse(value.toString("utf-8"));
			},
			request,
			{
				metadata: this.prepareMetadata(metadata),
				timeout: timeout ? timeout * 1000 : undefined,
			},
		);

		return new Promise((resolve, reject) => {
			const responses: Array<Record<string, unknown>> = [];

			call.on("data", (response: Record<string, unknown>) => {
				responses.push(response);
			});

			call.on("end", () => {
				resolve(responses);
			});

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			call.on("error", (err: any) => {
				reject(err);
			});
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
		if (!this.channel) {
			throw new Error(
				`Channel not initialized. Cannot connect to ${this.serverAddress}. Is the gRPC server running?`,
			);
		}

		const method = `/${serviceName}/${methodName}`;

		// Create stub for client streaming RPC
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const call = this.channel.makeClientStreamRequest(
			method,
			(value: Record<string, unknown>) => {
				return Buffer.from(JSON.stringify(value));
			},
			(value: Buffer) => {
				return JSON.parse(value.toString("utf-8"));
			},
			{
				metadata: this.prepareMetadata(metadata),
				timeout: timeout ? timeout * 1000 : undefined,
			},
		);

		return new Promise((resolve, reject) => {
			// Write all requests
			const writeRequests = () => {
				for (const request of requests) {
					call.write(request);
				}
				call.end();
			};

			call.on("data", (response: Record<string, unknown>) => {
				resolve(response);
			});

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			call.on("error", (err: any) => {
				reject(err);
			});

			writeRequests();
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
		if (!this.channel) {
			throw new Error(
				`Channel not initialized. Cannot connect to ${this.serverAddress}. Is the gRPC server running?`,
			);
		}

		const method = `/${serviceName}/${methodName}`;

		// Create stub for bidirectional streaming RPC
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const call = this.channel.makeBidiStreamRequest(
			method,
			(value: Record<string, unknown>) => {
				return Buffer.from(JSON.stringify(value));
			},
			(value: Buffer) => {
				return JSON.parse(value.toString("utf-8"));
			},
			{
				metadata: this.prepareMetadata(metadata),
				timeout: timeout ? timeout * 1000 : undefined,
			},
		);

		return new Promise((resolve, reject) => {
			const responses: Array<Record<string, unknown>> = [];

			// Write all requests
			const writeRequests = () => {
				for (const request of requests) {
					call.write(request);
				}
				call.end();
			};

			call.on("data", (response: Record<string, unknown>) => {
				responses.push(response);
			});

			call.on("end", () => {
				resolve(responses);
			});

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			call.on("error", (err: any) => {
				reject(err);
			});

			writeRequests();
		});
	}

	/**
	 * Initialize the gRPC channel (internal use for fixture tests).
	 *
	 * @param grpcChannel - The gRPC channel to use
	 */
	setChannel(grpcChannel: unknown): void {
		this.channel = grpcChannel;
	}
}
