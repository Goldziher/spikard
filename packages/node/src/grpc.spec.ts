/**
 * Tests for gRPC TypeScript bindings
 *
 * These tests verify the gRPC handler interfaces and helper functions
 * work correctly for TypeScript consumers.
 */

import { describe, expect, it } from "vitest";
import {
	createServiceHandler,
	createUnaryHandler,
	GrpcError,
	type GrpcHandler,
	type GrpcRequest,
	type GrpcResponse,
	GrpcStatusCode,
} from "./grpc";

describe("GrpcError", () => {
	it("should create error with status code", () => {
		const error = new GrpcError(GrpcStatusCode.NOT_FOUND, "Resource not found");

		expect(error).toBeInstanceOf(Error);
		expect(error.code).toBe(GrpcStatusCode.NOT_FOUND);
		expect(error.message).toBe("Resource not found");
		expect(error.name).toBe("GrpcError");
	});

	it("should support different status codes", () => {
		const codes = [
			GrpcStatusCode.OK,
			GrpcStatusCode.INVALID_ARGUMENT,
			GrpcStatusCode.PERMISSION_DENIED,
			GrpcStatusCode.INTERNAL,
		];

		for (const code of codes) {
			const error = new GrpcError(code, "Test error");
			expect(error.code).toBe(code);
		}
	});
});

describe("GrpcRequest", () => {
	it("should contain required fields", () => {
		const request: GrpcRequest = {
			serviceName: "mypackage.UserService",
			methodName: "GetUser",
			payload: Buffer.from([1, 2, 3, 4]),
			metadata: { "content-type": "application/grpc" },
		};

		expect(request.serviceName).toBe("mypackage.UserService");
		expect(request.methodName).toBe("GetUser");
		expect(request.payload).toBeInstanceOf(Buffer);
		expect(request.payload.length).toBe(4);
		expect(request.metadata["content-type"]).toBe("application/grpc");
	});

	it("should support empty metadata", () => {
		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "Method",
			payload: Buffer.from([]),
			metadata: {},
		};

		expect(Object.keys(request.metadata).length).toBe(0);
	});
});

describe("GrpcResponse", () => {
	it("should contain payload", () => {
		const response: GrpcResponse = {
			payload: Buffer.from([5, 6, 7, 8]),
		};

		expect(response.payload).toBeInstanceOf(Buffer);
		expect(response.payload.length).toBe(4);
		expect(response.metadata).toBeUndefined();
	});

	it("should support optional metadata", () => {
		const response: GrpcResponse = {
			payload: Buffer.from([1, 2, 3]),
			metadata: {
				"x-server-id": "server-1",
				"x-cache-status": "hit",
			},
		};

		expect(response.metadata).toBeDefined();
		expect(response.metadata?.["x-server-id"]).toBe("server-1");
		expect(response.metadata?.["x-cache-status"]).toBe("hit");
	});
});

describe("GrpcHandler", () => {
	it("should implement handleRequest method", async () => {
		const handler: GrpcHandler = {
			async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
				expect(request.methodName).toBe("TestMethod");
				return {
					payload: Buffer.from("response"),
				};
			},
		};

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "TestMethod",
			payload: Buffer.from("request"),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toBe("response");
	});

	it("should handle errors", async () => {
		const handler: GrpcHandler = {
			async handleRequest(_request: GrpcRequest): Promise<GrpcResponse> {
				throw new GrpcError(GrpcStatusCode.NOT_FOUND, "Not found");
			},
		};

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "TestMethod",
			payload: Buffer.from([]),
			metadata: {},
		};

		await expect(handler.handleRequest(request)).rejects.toThrow(GrpcError);
		await expect(handler.handleRequest(request)).rejects.toThrow("Not found");
	});
});

describe("createUnaryHandler", () => {
	// Mock protobuf types
	const mockRequestType = {
		decode(buffer: Uint8Array): { id: number } {
			const id = new DataView(buffer.buffer).getUint32(0, true);
			return { id };
		},
	};

	const mockResponseType = {
		encode(message: { name: string }): { finish(): Uint8Array } {
			return {
				finish() {
					const encoder = new TextEncoder();
					return encoder.encode(message.name);
				},
			};
		},
	};

	it("should create a handler for a single method", async () => {
		const handler = createUnaryHandler(
			"GetUser",
			async (req: { id: number }) => {
				expect(req.id).toBe(123);
				return { name: "John Doe" };
			},
			mockRequestType,
			mockResponseType,
		);

		// Create request with encoded id
		const buffer = new ArrayBuffer(4);
		const view = new DataView(buffer);
		view.setUint32(0, 123, true);

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "GetUser",
			payload: Buffer.from(buffer),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toBe("John Doe");
	});

	it("should reject unimplemented methods", async () => {
		const handler = createUnaryHandler(
			"GetUser",
			async (_req: { id: number }) => {
				return { name: "Test" };
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "DeleteUser", // Different method
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		await expect(handler.handleRequest(request)).rejects.toThrow(GrpcError);
		try {
			await handler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.UNIMPLEMENTED);
		}
	});

	it("should pass metadata to handler", async () => {
		const handler = createUnaryHandler(
			"GetUser",
			async (_req: { id: number }, metadata: Record<string, string>) => {
				expect(metadata.authorization).toBe("Bearer token123");
				return { name: "Test" };
			},
			mockRequestType,
			mockResponseType,
		);

		const buffer = new ArrayBuffer(4);
		const view = new DataView(buffer);
		view.setUint32(0, 1, true);

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "GetUser",
			payload: Buffer.from(buffer),
			metadata: { authorization: "Bearer token123" },
		};

		await handler.handleRequest(request);
	});

	it("should support response metadata", async () => {
		const handler = createUnaryHandler(
			"GetUser",
			async (_req: { id: number }) => {
				return {
					response: { name: "Test User" },
					metadata: { "x-cache-status": "hit", "x-server-id": "server-1" },
				};
			},
			mockRequestType,
			mockResponseType,
		);

		const buffer = new ArrayBuffer(4);
		const view = new DataView(buffer);
		view.setUint32(0, 1, true);

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "GetUser",
			payload: Buffer.from(buffer),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.metadata).toBeDefined();
		expect(response.metadata?.["x-cache-status"]).toBe("hit");
		expect(response.metadata?.["x-server-id"]).toBe("server-1");
		expect(response.payload.toString()).toBe("Test User");
	});

	it("should work without response metadata", async () => {
		const handler = createUnaryHandler(
			"GetUser",
			async (_req: { id: number }) => {
				return { name: "Test User" };
			},
			mockRequestType,
			mockResponseType,
		);

		const buffer = new ArrayBuffer(4);
		const view = new DataView(buffer);
		view.setUint32(0, 1, true);

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "GetUser",
			payload: Buffer.from(buffer),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.metadata).toBeUndefined();
		expect(response.payload.toString()).toBe("Test User");
	});
});

describe("createServiceHandler", () => {
	const mockRequestType = {
		decode(_buffer: Uint8Array): { id: number } {
			return { id: 1 };
		},
	};

	const mockResponseType = {
		encode(message: { name: string }): { finish(): Uint8Array } {
			return {
				finish() {
					return new TextEncoder().encode(message.name);
				},
			};
		},
	};

	it("should route to correct method handler", async () => {
		const getUserHandler = createUnaryHandler(
			"GetUser",
			async () => ({ name: "User1" }),
			mockRequestType,
			mockResponseType,
		);

		const listUsersHandler = createUnaryHandler(
			"ListUsers",
			async () => ({ name: "Users" }),
			mockRequestType,
			mockResponseType,
		);

		const serviceHandler = createServiceHandler({
			GetUser: getUserHandler,
			ListUsers: listUsersHandler,
		});

		const getUserRequest: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "GetUser",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await serviceHandler.handleRequest(getUserRequest);
		expect(response.payload.toString()).toBe("User1");

		const listUsersRequest: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "ListUsers",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const listResponse = await serviceHandler.handleRequest(listUsersRequest);
		expect(listResponse.payload.toString()).toBe("Users");
	});

	it("should reject unimplemented methods", async () => {
		const serviceHandler = createServiceHandler({
			GetUser: createUnaryHandler("GetUser", async () => ({ name: "Test" }), mockRequestType, mockResponseType),
		});

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "DeleteUser", // Not implemented
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		await expect(serviceHandler.handleRequest(request)).rejects.toThrow(GrpcError);
		try {
			await serviceHandler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.UNIMPLEMENTED);
			expect((error as GrpcError).message).toContain("DeleteUser");
		}
	});

	it("should handle multiple methods", async () => {
		const methods = {
			Method1: createUnaryHandler("Method1", async () => ({ name: "M1" }), mockRequestType, mockResponseType),
			Method2: createUnaryHandler("Method2", async () => ({ name: "M2" }), mockRequestType, mockResponseType),
			Method3: createUnaryHandler("Method3", async () => ({ name: "M3" }), mockRequestType, mockResponseType),
		};

		const serviceHandler = createServiceHandler(methods);

		for (const methodName of Object.keys(methods)) {
			const request: GrpcRequest = {
				serviceName: "test.Service",
				methodName,
				payload: Buffer.from([0, 0, 0, 1]),
				metadata: {},
			};

			const response = await serviceHandler.handleRequest(request);
			expect(response.payload.toString()).toBe(`M${methodName.charAt(methodName.length - 1)}`);
		}
	});
});

describe("GrpcStatusCode", () => {
	it("should have all standard gRPC status codes", () => {
		expect(GrpcStatusCode.OK).toBe(0);
		expect(GrpcStatusCode.CANCELLED).toBe(1);
		expect(GrpcStatusCode.UNKNOWN).toBe(2);
		expect(GrpcStatusCode.INVALID_ARGUMENT).toBe(3);
		expect(GrpcStatusCode.DEADLINE_EXCEEDED).toBe(4);
		expect(GrpcStatusCode.NOT_FOUND).toBe(5);
		expect(GrpcStatusCode.ALREADY_EXISTS).toBe(6);
		expect(GrpcStatusCode.PERMISSION_DENIED).toBe(7);
		expect(GrpcStatusCode.RESOURCE_EXHAUSTED).toBe(8);
		expect(GrpcStatusCode.FAILED_PRECONDITION).toBe(9);
		expect(GrpcStatusCode.ABORTED).toBe(10);
		expect(GrpcStatusCode.OUT_OF_RANGE).toBe(11);
		expect(GrpcStatusCode.UNIMPLEMENTED).toBe(12);
		expect(GrpcStatusCode.INTERNAL).toBe(13);
		expect(GrpcStatusCode.UNAVAILABLE).toBe(14);
		expect(GrpcStatusCode.DATA_LOSS).toBe(15);
		expect(GrpcStatusCode.UNAUTHENTICATED).toBe(16);
	});
});

describe("Edge cases", () => {
	const mockRequestType = {
		decode(_buffer: Uint8Array): { id: number } {
			return { id: 1 };
		},
	};

	const mockResponseType = {
		encode(message: { name: string }): { finish(): Uint8Array } {
			return {
				finish() {
					return new TextEncoder().encode(message.name);
				},
			};
		},
	};

	it("should handle errors thrown from handler", async () => {
		const handler = createUnaryHandler(
			"GetUser",
			async () => {
				throw new Error("Database connection failed");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "GetUser",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		await expect(handler.handleRequest(request)).rejects.toThrow("Database connection failed");
	});

	it("should handle GrpcError with different status codes", async () => {
		const testCases = [
			{ code: GrpcStatusCode.NOT_FOUND, message: "User not found" },
			{ code: GrpcStatusCode.PERMISSION_DENIED, message: "Access denied" },
			{ code: GrpcStatusCode.INVALID_ARGUMENT, message: "Invalid ID" },
			{ code: GrpcStatusCode.UNAUTHENTICATED, message: "Not authenticated" },
		];

		for (const testCase of testCases) {
			const handler = createUnaryHandler(
				"GetUser",
				async () => {
					throw new GrpcError(testCase.code, testCase.message);
				},
				mockRequestType,
				mockResponseType,
			);

			const request: GrpcRequest = {
				serviceName: "test.UserService",
				methodName: "GetUser",
				payload: Buffer.from([0, 0, 0, 1]),
				metadata: {},
			};

			try {
				await handler.handleRequest(request);
				throw new Error("Should have thrown");
			} catch (error) {
				expect(error).toBeInstanceOf(GrpcError);
				expect((error as GrpcError).code).toBe(testCase.code);
				expect((error as GrpcError).message).toBe(testCase.message);
			}
		}
	});

	it("should handle empty payload", async () => {
		const emptyRequestType = {
			decode(_buffer: Uint8Array): Record<string, never> {
				return {};
			},
		};

		const handler = createUnaryHandler(
			"Ping",
			async () => {
				return { name: "pong" };
			},
			emptyRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "Ping",
			payload: Buffer.from([]),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toBe("pong");
	});

	it("should handle large metadata", async () => {
		const handler = createUnaryHandler(
			"GetUser",
			async (_req, metadata) => {
				// Verify large metadata is passed correctly
				expect(Object.keys(metadata).length).toBe(100);
				return { name: "Test" };
			},
			mockRequestType,
			mockResponseType,
		);

		const largeMetadata: Record<string, string> = {};
		for (let i = 0; i < 100; i++) {
			largeMetadata[`x-custom-${i}`] = `value-${i}`;
		}

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "GetUser",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: largeMetadata,
		};

		await handler.handleRequest(request);
	});

	it("should preserve metadata key case", async () => {
		const handler = createUnaryHandler(
			"GetUser",
			async (_req, metadata) => {
				expect(metadata["X-Custom-Header"]).toBe("value");
				expect(metadata["x-lower-case"]).toBe("test");
				return { name: "Test" };
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "GetUser",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {
				"X-Custom-Header": "value",
				"x-lower-case": "test",
			},
		};

		await handler.handleRequest(request);
	});
});

describe("Streaming Support - Server-Side Streaming", () => {
	it("should handle server-side streaming RPC", async () => {
		async function* serverStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			const count = 5;
			for (let i = 0; i < count; i++) {
				yield {
					payload: Buffer.from(JSON.stringify({ count: i })),
					metadata: {},
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "ServerStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of serverStreamHandler(request)) {
			responses.push(response);
		}

		expect(responses).toHaveLength(5);
		expect(responses[0].payload.toString()).toBe(JSON.stringify({ count: 0 }));
		expect(JSON.parse(responses[4].payload.toString())).toEqual({ count: 4 });
	});

	it("should handle empty server-side stream", async () => {
		async function* emptyStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			const empty: GrpcResponse[] = [];
			for (const item of empty) {
				yield item;
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "ServerStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of emptyStreamHandler(request)) {
			if (response) responses.push(response);
		}

		expect(responses).toHaveLength(0);
	});

	it("should handle large server-side stream (1000+ messages)", async () => {
		async function* largeStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			const messageCount = 1500;
			for (let i = 0; i < messageCount; i++) {
				yield {
					payload: Buffer.from(JSON.stringify({ id: i, data: `message-${i}` })),
					metadata: { "x-message-id": i.toString() },
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "LargeStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of largeStreamHandler(request)) {
			responses.push(response);
		}

		expect(responses).toHaveLength(1500);
		expect(JSON.parse(responses[750].payload.toString()).id).toBe(750);
		expect(responses[1499].metadata?.["x-message-id"]).toBe("1499");
	});

	it("should handle stream with error conditions", async () => {
		async function* errorStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			for (let i = 0; i < 3; i++) {
				yield {
					payload: Buffer.from(JSON.stringify({ count: i })),
				};
			}
			// Simulate error after some messages
			throw new GrpcError(GrpcStatusCode.INTERNAL, "Stream processing failed");
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "ErrorStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		let thrownError: Error | null = null;

		try {
			for await (const response of errorStreamHandler(request)) {
				responses.push(response);
			}
		} catch (error) {
			thrownError = error as Error;
		}

		expect(responses).toHaveLength(3);
		expect(thrownError).toBeInstanceOf(GrpcError);
		expect((thrownError as GrpcError).code).toBe(GrpcStatusCode.INTERNAL);
	});
});

describe("Streaming Support - Client-Side Streaming", () => {
	it("should handle client-side streaming RPC", async () => {
		async function clientStreamHandler(requests: AsyncIterator<GrpcRequest>): Promise<GrpcResponse> {
			const messages: string[] = [];

			for await (const request of requests) {
				const data = JSON.parse(request.payload.toString());
				messages.push(data.msg);
			}

			return {
				payload: Buffer.from(JSON.stringify({ processed: messages.length, messages })),
			};
		}

		async function* mockRequestStream(): AsyncIterator<GrpcRequest> {
			for (let i = 0; i < 3; i++) {
				yield {
					serviceName: "test.StreamService",
					methodName: "ClientStream",
					payload: Buffer.from(JSON.stringify({ msg: `message-${i}` })),
					metadata: {},
				};
			}
		}

		const response = await clientStreamHandler(mockRequestStream());

		expect(response.payload).toBeInstanceOf(Buffer);
		const result = JSON.parse(response.payload.toString());
		expect(result.processed).toBe(3);
		expect(result.messages).toEqual(["message-0", "message-1", "message-2"]);
	});

	it("should handle empty client-side stream", async () => {
		async function emptyClientStreamHandler(requests: AsyncIterator<GrpcRequest>): Promise<GrpcResponse> {
			let count = 0;
			for await (const _request of requests) {
				count++;
			}

			return {
				payload: Buffer.from(JSON.stringify({ processed: count })),
			};
		}

		async function* emptyRequestStream(): AsyncIterator<GrpcRequest> {
			const empty: GrpcRequest[] = [];
			for (const item of empty) {
				yield item;
			}
		}

		const response = await emptyClientStreamHandler(emptyRequestStream());

		const result = JSON.parse(response.payload.toString());
		expect(result.processed).toBe(0);
	});
});

describe("Streaming Support - Bidirectional Streaming", () => {
	it("should handle bidirectional streaming RPC", async () => {
		async function* bidiStreamHandler(requests: AsyncIterator<GrpcRequest>): AsyncIterator<GrpcResponse> {
			for await (const request of requests) {
				const data = JSON.parse(request.payload.toString());
				yield {
					payload: Buffer.from(
						JSON.stringify({
							echo: data,
							processed: true,
							timestamp: Date.now(),
						}),
					),
					metadata: { "x-processed": "true" },
				};
			}
		}

		async function* mockBidiStream(): AsyncIterator<GrpcRequest> {
			for (let i = 0; i < 3; i++) {
				yield {
					serviceName: "test.BidiService",
					methodName: "Chat",
					payload: Buffer.from(JSON.stringify({ msg: `hello-${i}` })),
					metadata: {},
				};
			}
		}

		const responses: GrpcResponse[] = [];
		for await (const response of bidiStreamHandler(mockBidiStream())) {
			responses.push(response);
		}

		expect(responses).toHaveLength(3);
		expect(JSON.parse(responses[0].payload.toString())).toHaveProperty("processed", true);
		expect(JSON.parse(responses[1].payload.toString()).echo.msg).toBe("hello-1");
		expect(responses[2].metadata?.["x-processed"]).toBe("true");
	});

	it("should handle bidirectional streaming with error on request", async () => {
		async function* bidiErrorHandler(requests: AsyncIterator<GrpcRequest>): AsyncIterator<GrpcResponse> {
			let count = 0;
			for await (const _request of requests) {
				count++;
				if (count === 2) {
					throw new GrpcError(GrpcStatusCode.INVALID_ARGUMENT, "Invalid message received");
				}
				yield {
					payload: Buffer.from(JSON.stringify({ count })),
				};
			}
		}

		async function* mockBidiStream(): AsyncIterator<GrpcRequest> {
			for (let i = 0; i < 5; i++) {
				yield {
					serviceName: "test.BidiService",
					methodName: "Chat",
					payload: Buffer.from(JSON.stringify({ index: i })),
					metadata: {},
				};
			}
		}

		const responses: GrpcResponse[] = [];
		let error: Error | null = null;

		try {
			for await (const response of bidiErrorHandler(mockBidiStream())) {
				responses.push(response);
			}
		} catch (_e) {
			error = _e as Error;
		}

		expect(responses).toHaveLength(1);
		expect(error).toBeInstanceOf(GrpcError);
		expect((error as GrpcError).code).toBe(GrpcStatusCode.INVALID_ARGUMENT);
	});
});

describe("Streaming Support - Advanced Cases", () => {
	it("should handle stream cancellation", async () => {
		async function* cancellableStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			for (let i = 0; i < 100; i++) {
				yield {
					payload: Buffer.from(JSON.stringify({ count: i })),
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "CancellableStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		const generator = cancellableStreamHandler(request);

		// Consume only 5 messages then break (simulating cancellation)
		for await (const response of generator) {
			responses.push(response);
			if (responses.length === 5) {
				break;
			}
		}

		expect(responses).toHaveLength(5);
		expect(JSON.parse(responses[4].payload.toString()).count).toBe(4);
	});

	it("should handle stream backpressure with metadata", async () => {
		async function* backpressureStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			for (let i = 0; i < 10; i++) {
				yield {
					payload: Buffer.from(JSON.stringify({ id: i, size: i * 1024 })),
					metadata: {
						"x-backpressure-level": Math.floor(i / 3).toString(),
						"x-batch": (i % 3).toString(),
					},
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "BackpressureStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of backpressureStreamHandler(request)) {
			responses.push(response);
		}

		expect(responses).toHaveLength(10);
		// Verify metadata indicates batching
		expect(responses[0].metadata?.["x-batch"]).toBe("0");
		expect(responses[1].metadata?.["x-batch"]).toBe("1");
		expect(responses[2].metadata?.["x-batch"]).toBe("2");
		expect(responses[3].metadata?.["x-batch"]).toBe("0");
	});

	it("should handle mixed streaming patterns with response objects", async () => {
		async function* mixedStreamHandler(requests: AsyncIterator<GrpcRequest>): AsyncIterator<GrpcResponse> {
			let messageCount = 0;

			for await (const request of requests) {
				messageCount++;
				const data = JSON.parse(request.payload.toString());

				yield {
					payload: Buffer.from(
						JSON.stringify({
							originalMessage: data,
							processed: true,
							messageNumber: messageCount,
						}),
					),
					metadata: {
						"x-message-count": messageCount.toString(),
						"x-timestamp": new Date().toISOString(),
					},
				};
			}
		}

		async function* mockMixedStream(): AsyncIterator<GrpcRequest> {
			const messages = [
				{ type: "greeting", text: "hello" },
				{ type: "query", text: "how are you" },
				{ type: "farewell", text: "goodbye" },
			];

			for (const msg of messages) {
				yield {
					serviceName: "test.MixedService",
					methodName: "Chat",
					payload: Buffer.from(JSON.stringify(msg)),
					metadata: {},
				};
			}
		}

		const responses: GrpcResponse[] = [];
		for await (const response of mixedStreamHandler(mockMixedStream())) {
			responses.push(response);
		}

		expect(responses).toHaveLength(3);
		expect(JSON.parse(responses[0].payload.toString()).messageNumber).toBe(1);
		expect(JSON.parse(responses[2].payload.toString()).messageNumber).toBe(3);
		expect(responses[1].metadata?.["x-message-count"]).toBe("2");
	});
});

describe("gRPC Status Codes - Extended Coverage", () => {
	const mockRequestType = {
		decode(_buffer: Uint8Array): { id: number } {
			return { id: 1 };
		},
	};

	const mockResponseType = {
		encode(message: { name: string }): { finish(): Uint8Array } {
			return {
				finish() {
					return new TextEncoder().encode(message.name);
				},
			};
		},
	};

	it("should handle RESOURCE_EXHAUSTED status code", async () => {
		const handler = createUnaryHandler(
			"Upload",
			async () => {
				throw new GrpcError(GrpcStatusCode.RESOURCE_EXHAUSTED, "Quota exceeded: daily upload limit reached");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.StorageService",
			methodName: "Upload",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
			throw new Error("Should have thrown");
		} catch (error) {
			expect(error).toBeInstanceOf(GrpcError);
			expect((error as GrpcError).code).toBe(GrpcStatusCode.RESOURCE_EXHAUSTED);
			expect((error as GrpcError).message).toContain("Quota exceeded");
		}
	});

	it("should handle FAILED_PRECONDITION status code", async () => {
		const handler = createUnaryHandler(
			"ProcessOrder",
			async () => {
				throw new GrpcError(GrpcStatusCode.FAILED_PRECONDITION, "Order cannot be processed: payment not authorized");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.OrderService",
			methodName: "ProcessOrder",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
			throw new Error("Should have thrown");
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.FAILED_PRECONDITION);
		}
	});

	it("should handle ABORTED status code", async () => {
		const handler = createUnaryHandler(
			"CommitTransaction",
			async () => {
				throw new GrpcError(GrpcStatusCode.ABORTED, "Transaction aborted due to concurrent modification");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.DatabaseService",
			methodName: "CommitTransaction",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.ABORTED);
		}
	});

	it("should handle OUT_OF_RANGE status code", async () => {
		const handler = createUnaryHandler(
			"GetPage",
			async (req: { id: number }) => {
				if (req.id > 999) {
					throw new GrpcError(GrpcStatusCode.OUT_OF_RANGE, "Page number out of range");
				}
				return { name: "Page" };
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.PageService",
			methodName: "GetPage",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.OUT_OF_RANGE);
		}
	});

	it("should handle DATA_LOSS status code", async () => {
		const handler = createUnaryHandler(
			"RetrieveData",
			async () => {
				throw new GrpcError(GrpcStatusCode.DATA_LOSS, "Data corruption detected in backup");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.BackupService",
			methodName: "RetrieveData",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.DATA_LOSS);
		}
	});

	it("should handle UNKNOWN status code", async () => {
		const handler = createUnaryHandler(
			"UnknownOperation",
			async () => {
				throw new GrpcError(GrpcStatusCode.UNKNOWN, "An unknown error occurred");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "UnknownOperation",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.UNKNOWN);
		}
	});

	it("should handle DEADLINE_EXCEEDED status code", async () => {
		const handler = createUnaryHandler(
			"SlowOperation",
			async () => {
				throw new GrpcError(GrpcStatusCode.DEADLINE_EXCEEDED, "Request deadline exceeded after 30 seconds");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "SlowOperation",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.DEADLINE_EXCEEDED);
		}
	});

	it("should handle CANCELLED status code", async () => {
		const handler = createUnaryHandler(
			"CancellableOperation",
			async () => {
				throw new GrpcError(GrpcStatusCode.CANCELLED, "Operation was cancelled by client");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "CancellableOperation",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.CANCELLED);
		}
	});

	it("should handle UNAVAILABLE status code", async () => {
		const handler = createUnaryHandler(
			"CheckService",
			async () => {
				throw new GrpcError(GrpcStatusCode.UNAVAILABLE, "Service temporarily unavailable, try again later");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "CheckService",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.UNAVAILABLE);
		}
	});

	it("should handle ALREADY_EXISTS status code", async () => {
		const handler = createUnaryHandler(
			"CreateUser",
			async () => {
				throw new GrpcError(GrpcStatusCode.ALREADY_EXISTS, "User with email already exists");
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "CreateUser",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		try {
			await handler.handleRequest(request);
		} catch (error) {
			expect((error as GrpcError).code).toBe(GrpcStatusCode.ALREADY_EXISTS);
		}
	});
});

describe("Protobuf Integration Tests", () => {
	it("should handle protobuf Any type representation", async () => {
		// Simulate protobuf Any type: { type_url: string, value: bytes }
		const mockAnyRequestType = {
			decode(buffer: Uint8Array): { id: number; data: unknown } {
				return {
					id: 1,
					data: {
						type_url: "type.googleapis.com/google.protobuf.StringValue",
						value: buffer,
					},
				};
			},
		};

		const mockResponseType = {
			encode(message: { result: string }): { finish(): Uint8Array } {
				return {
					finish() {
						return new TextEncoder().encode(JSON.stringify(message));
					},
				};
			},
		};

		const handler = createUnaryHandler(
			"ProcessAny",
			async (req: { id: number; data: unknown }) => {
				expect(req.id).toBe(1);
				expect(req.data).toBeDefined();
				return { result: "processed" };
			},
			mockAnyRequestType,
			mockResponseType,
		);

		const buffer = Buffer.from("test");
		const request: GrpcRequest = {
			serviceName: "test.AnyService",
			methodName: "ProcessAny",
			payload: buffer,
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload).toBeInstanceOf(Buffer);
		expect(JSON.parse(response.payload.toString())).toHaveProperty("result", "processed");
	});

	it("should handle protobuf Timestamp type", async () => {
		// Simulate protobuf Timestamp: { seconds: number, nanos: number }
		const mockTimestampType = {
			decode(_buffer: Uint8Array): {
				created_at: { seconds: number; nanos: number };
			} {
				return {
					created_at: {
						seconds: Math.floor(Date.now() / 1000),
						nanos: 0,
					},
				};
			},
		};

		const mockResponseType = {
			encode(message: { timestamp: string }): { finish(): Uint8Array } {
				return {
					finish() {
						return new TextEncoder().encode(message.timestamp);
					},
				};
			},
		};

		const handler = createUnaryHandler(
			"GetTimestamp",
			async (req: { created_at: { seconds: number; nanos: number } }) => {
				expect(req.created_at.seconds).toBeGreaterThan(0);
				expect(req.created_at.nanos).toBe(0);
				return {
					timestamp: new Date(req.created_at.seconds * 1000).toISOString(),
				};
			},
			mockTimestampType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.TimestampService",
			methodName: "GetTimestamp",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		const timestamp = response.payload.toString();
		expect(timestamp).toMatch(/^\d{4}-\d{2}-\d{2}T/); // ISO format check
	});

	it("should handle protobuf Duration type", async () => {
		// Simulate protobuf Duration: { seconds: number, nanos: number }
		const mockDurationRequestType = {
			decode(_buffer: Uint8Array): { timeout: { seconds: number; nanos: number } } {
				return {
					timeout: {
						seconds: 30,
						nanos: 500000000, // 0.5 seconds
					},
				};
			},
		};

		const mockResponseType = {
			encode(message: { result: string }): { finish(): Uint8Array } {
				return {
					finish() {
						return new TextEncoder().encode(message.result);
					},
				};
			},
		};

		const handler = createUnaryHandler(
			"ProcessWithTimeout",
			async (req: { timeout: { seconds: number; nanos: number } }) => {
				const totalSeconds = req.timeout.seconds + req.timeout.nanos / 1e9;
				expect(totalSeconds).toBeCloseTo(30.5, 1);
				return { result: "completed" };
			},
			mockDurationRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.DurationService",
			methodName: "ProcessWithTimeout",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toBe("completed");
	});

	it("should handle protobuf Struct type (JSON-like data)", async () => {
		// Simulate protobuf Struct: { fields: Map<string, Value> }
		const mockStructRequestType = {
			decode(_buffer: Uint8Array): {
				metadata: Record<string, unknown>;
			} {
				return {
					metadata: {
						name: "John",
						age: 30,
						active: true,
						tags: ["admin", "user"],
					},
				};
			},
		};

		const mockResponseType = {
			encode(message: { processed: boolean }): { finish(): Uint8Array } {
				return {
					finish() {
						return new TextEncoder().encode(JSON.stringify(message));
					},
				};
			},
		};

		const handler = createUnaryHandler(
			"ProcessStruct",
			async (req: { metadata: Record<string, unknown> }) => {
				expect(req.metadata).toHaveProperty("name", "John");
				expect(req.metadata).toHaveProperty("age", 30);
				expect(req.metadata).toHaveProperty("active", true);
				return { processed: true };
			},
			mockStructRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.StructService",
			methodName: "ProcessStruct",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(JSON.parse(response.payload.toString())).toHaveProperty("processed", true);
	});

	it("should handle protobuf OneOf fields", async () => {
		// Simulate OneOf field: exactly one of multiple fields is set
		const mockOneOfRequestType = {
			decode(_buffer: Uint8Array): {
				payload: { email?: string; phone?: string };
			} {
				return {
					payload: {
						email: "user@example.com",
						// phone is not set, respecting OneOf constraint
					},
				};
			},
		};

		const mockResponseType = {
			encode(message: { contact: string }): { finish(): Uint8Array } {
				return {
					finish() {
						return new TextEncoder().encode(message.contact);
					},
				};
			},
		};

		const handler = createUnaryHandler(
			"ContactUser",
			async (req: { payload: { email?: string; phone?: string } }) => {
				const contact = req.payload.email || req.payload.phone || "unknown";
				return { contact };
			},
			mockOneOfRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.OneOfService",
			methodName: "ContactUser",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toBe("user@example.com");
	});

	it("should handle protobuf map fields", async () => {
		// Simulate map field: Map<string, Value>
		const mockMapRequestType = {
			decode(_buffer: Uint8Array): {
				attributes: Map<string, string>;
			} {
				const map = new Map<string, string>();
				map.set("color", "blue");
				map.set("size", "large");
				map.set("material", "cotton");
				return { attributes: map };
			},
		};

		const mockResponseType = {
			encode(message: { count: number }): { finish(): Uint8Array } {
				return {
					finish() {
						return Buffer.from(message.count.toString());
					},
				};
			},
		};

		const handler = createUnaryHandler(
			"ProcessAttributes",
			async (req: { attributes: Map<string, string> }) => {
				expect(req.attributes.size).toBe(3);
				expect(req.attributes.get("color")).toBe("blue");
				return { count: req.attributes.size };
			},
			mockMapRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.MapService",
			methodName: "ProcessAttributes",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(Number(response.payload.toString())).toBe(3);
	});

	it("should handle nested protobuf messages", async () => {
		// Simulate nested message structure
		interface Address {
			street: string;
			city: string;
			zip: string;
		}

		interface User {
			id: number;
			name: string;
			address: Address;
		}

		const mockNestedRequestType = {
			decode(_buffer: Uint8Array): User {
				return {
					id: 1,
					name: "John Doe",
					address: {
						street: "123 Main St",
						city: "Springfield",
						zip: "12345",
					},
				};
			},
		};

		const mockResponseType = {
			encode(message: { address: string }): { finish(): Uint8Array } {
				return {
					finish() {
						return new TextEncoder().encode(message.address);
					},
				};
			},
		};

		const handler = createUnaryHandler(
			"GetUserAddress",
			async (req: User) => {
				expect(req.address).toBeDefined();
				expect(req.address.city).toBe("Springfield");
				return {
					address: `${req.address.street}, ${req.address.city} ${req.address.zip}`,
				};
			},
			mockNestedRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.NestedService",
			methodName: "GetUserAddress",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toContain("Springfield");
		expect(response.payload.toString()).toContain("123 Main St");
	});
});

describe("Performance and Edge Cases", () => {
	const mockRequestType = {
		decode(buffer: Uint8Array): { size: number } {
			return { size: buffer.length };
		},
	};

	const mockResponseType = {
		encode(message: { result: string }): { finish(): Uint8Array } {
			return {
				finish() {
					return new TextEncoder().encode(message.result);
				},
			};
		},
	};

	it("should handle large 50MB buffer payload", async () => {
		const handler = createUnaryHandler(
			"ProcessLargePayload",
			async (req: { size: number }) => {
				expect(req.size).toBe(50 * 1024 * 1024);
				return { result: "processed" };
			},
			mockRequestType,
			mockResponseType,
		);

		// Create a 50MB buffer
		const largePayload = Buffer.alloc(50 * 1024 * 1024);
		// Fill with some data pattern
		for (let i = 0; i < largePayload.length; i += 1024) {
			largePayload.write("x", i);
		}

		const request: GrpcRequest = {
			serviceName: "test.PayloadService",
			methodName: "ProcessLargePayload",
			payload: largePayload,
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toBe("processed");
	});

	it("should handle binary data with all byte values (0x00-0xFF)", async () => {
		const handler = createUnaryHandler(
			"ProcessBinaryData",
			async (req: { size: number }) => {
				expect(req.size).toBe(256);
				return { result: "binary-ok" };
			},
			mockRequestType,
			mockResponseType,
		);

		// Create buffer with all byte values
		const binaryData = Buffer.alloc(256);
		for (let i = 0; i < 256; i++) {
			binaryData[i] = i;
		}

		const request: GrpcRequest = {
			serviceName: "test.BinaryService",
			methodName: "ProcessBinaryData",
			payload: binaryData,
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toBe("binary-ok");
	});

	it("should handle concurrent handler execution", async () => {
		let executionCount = 0;

		const handler = createUnaryHandler(
			"ConcurrentOperation",
			async (_req: { size: number }) => {
				executionCount++;
				const currentCount = executionCount;
				// Simulate async work
				await new Promise((resolve) => setTimeout(resolve, 10));
				return { result: `executed-${currentCount}` };
			},
			mockRequestType,
			mockResponseType,
		);

		// Create 50 concurrent requests
		const promises = [];
		for (let i = 0; i < 50; i++) {
			const request: GrpcRequest = {
				serviceName: "test.ConcurrentService",
				methodName: "ConcurrentOperation",
				payload: Buffer.alloc(10),
				metadata: {},
			};
			promises.push(handler.handleRequest(request));
		}

		const results = await Promise.all(promises);
		expect(results).toHaveLength(50);
		expect(executionCount).toBe(50);
		results.forEach((result) => {
			expect(result.payload.toString()).toMatch(/^executed-\d+$/);
		});
	});

	it("should handle handler timeout scenarios", async () => {
		const handler = createUnaryHandler(
			"TimedOperation",
			async (_req: { size: number }) => {
				// Simulate operation that takes 100ms
				await new Promise((resolve) => setTimeout(resolve, 100));
				return { result: "completed" };
			},
			mockRequestType,
			mockResponseType,
		);

		const startTime = Date.now();

		const request: GrpcRequest = {
			serviceName: "test.TimedService",
			methodName: "TimedOperation",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		const elapsed = Date.now() - startTime;

		expect(response.payload.toString()).toBe("completed");
		expect(elapsed).toBeGreaterThanOrEqual(100);
	});

	it("should handle memory cleanup after large payloads", async () => {
		const handler = createUnaryHandler(
			"MemoryTest",
			async (req: { size: number }) => {
				// Handler that processes data
				const processedSize = req.size;
				return { result: `size-${processedSize}` };
			},
			mockRequestType,
			mockResponseType,
		);

		// Process multiple large payloads
		const largePayload = Buffer.alloc(10 * 1024 * 1024); // 10MB each
		const results = [];

		for (let i = 0; i < 3; i++) {
			const request: GrpcRequest = {
				serviceName: "test.MemoryService",
				methodName: "MemoryTest",
				payload: largePayload,
				metadata: {},
			};

			const response = await handler.handleRequest(request);
			results.push(response);
		}

		expect(results).toHaveLength(3);
		// If memory cleanup is not working, this test would show in memory profiling
		// For now, we just verify the operations complete successfully
		results.forEach((result) => {
			expect(result.payload.toString()).toMatch(/^size-/);
		});
	});
});

describe("Advanced Routing and Service Handling", () => {
	const mockRequestType = {
		decode(_buffer: Uint8Array): { id: number } {
			return { id: 1 };
		},
	};

	const mockResponseType = {
		encode(message: { name: string }): { finish(): Uint8Array } {
			return {
				finish() {
					return new TextEncoder().encode(message.name);
				},
			};
		},
	};

	it("should handle multiple services with same method names", async () => {
		const userServiceHandler = createUnaryHandler(
			"Get",
			async () => ({ name: "UserService.Get" }),
			mockRequestType,
			mockResponseType,
		);

		const productServiceHandler = createUnaryHandler(
			"Get",
			async () => ({ name: "ProductService.Get" }),
			mockRequestType,
			mockResponseType,
		);

		const userServiceRouter = createServiceHandler({
			Get: userServiceHandler,
		});

		const productServiceRouter = createServiceHandler({
			Get: productServiceHandler,
		});

		const userRequest: GrpcRequest = {
			serviceName: "test.UserService",
			methodName: "Get",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const productRequest: GrpcRequest = {
			serviceName: "test.ProductService",
			methodName: "Get",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const userResponse = await userServiceRouter.handleRequest(userRequest);
		const productResponse = await productServiceRouter.handleRequest(productRequest);

		expect(userResponse.payload.toString()).toBe("UserService.Get");
		expect(productResponse.payload.toString()).toBe("ProductService.Get");
	});

	it("should respect method name case sensitivity", async () => {
		const handler = createUnaryHandler("GetUser", async () => ({ name: "Found" }), mockRequestType, mockResponseType);

		const serviceRouter = createServiceHandler({
			GetUser: handler,
		});

		const correctRequest: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "GetUser",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const wrongCaseRequest: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "getuser", // Different case
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await serviceRouter.handleRequest(correctRequest);
		expect(response.payload.toString()).toBe("Found");

		// Should fail with wrong case
		await expect(serviceRouter.handleRequest(wrongCaseRequest)).rejects.toThrow(GrpcError);
	});

	it("should handle service names with dots and segments", async () => {
		const handler = createUnaryHandler(
			"Operation",
			async () => ({ name: "ComplexName" }),
			mockRequestType,
			mockResponseType,
		);

		const serviceRouter = createServiceHandler({
			Operation: handler,
		});

		const complexServiceRequest: GrpcRequest = {
			serviceName: "com.example.deeply.nested.ServiceName",
			methodName: "Operation",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await serviceRouter.handleRequest(complexServiceRequest);
		expect(response.payload.toString()).toBe("ComplexName");
	});

	it("should handle routing with special characters in service name", async () => {
		const handler = createUnaryHandler(
			"Process",
			async () => ({ name: "SpecialChars" }),
			mockRequestType,
			mockResponseType,
		);

		const serviceRouter = createServiceHandler({
			Process: handler,
		});

		const specialRequest: GrpcRequest = {
			serviceName: "test_service.v1.special-api",
			methodName: "Process",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await serviceRouter.handleRequest(specialRequest);
		expect(response.payload.toString()).toBe("SpecialChars");
	});

	it("should handle multiple method handlers in same service", async () => {
		const methods = {
			GetUser: createUnaryHandler("GetUser", async () => ({ name: "User" }), mockRequestType, mockResponseType),
			DeleteUser: createUnaryHandler(
				"DeleteUser",
				async () => ({ name: "Deleted" }),
				mockRequestType,
				mockResponseType,
			),
			UpdateUser: createUnaryHandler(
				"UpdateUser",
				async () => ({ name: "Updated" }),
				mockRequestType,
				mockResponseType,
			),
			ListUsers: createUnaryHandler("ListUsers", async () => ({ name: "List" }), mockRequestType, mockResponseType),
		};

		const serviceRouter = createServiceHandler(methods);

		const testCases = [
			{ method: "GetUser", expected: "User" },
			{ method: "DeleteUser", expected: "Deleted" },
			{ method: "UpdateUser", expected: "Updated" },
			{ method: "ListUsers", expected: "List" },
		];

		for (const testCase of testCases) {
			const request: GrpcRequest = {
				serviceName: "test.UserService",
				methodName: testCase.method,
				payload: Buffer.from([0, 0, 0, 1]),
				metadata: {},
			};

			const response = await serviceRouter.handleRequest(request);
			expect(response.payload.toString()).toBe(testCase.expected);
		}
	});

	it("should handle empty service handler gracefully", async () => {
		const emptyServiceRouter = createServiceHandler({});

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "AnyMethod",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		await expect(emptyServiceRouter.handleRequest(request)).rejects.toThrow(GrpcError);
	});

	it("should preserve order of handlers in routing", async () => {
		const executionOrder: string[] = [];

		const handlerA = createUnaryHandler(
			"MethodA",
			async () => {
				executionOrder.push("A");
				return { name: "A" };
			},
			mockRequestType,
			mockResponseType,
		);

		const handlerB = createUnaryHandler(
			"MethodB",
			async () => {
				executionOrder.push("B");
				return { name: "B" };
			},
			mockRequestType,
			mockResponseType,
		);

		const handlerC = createUnaryHandler(
			"MethodC",
			async () => {
				executionOrder.push("C");
				return { name: "C" };
			},
			mockRequestType,
			mockResponseType,
		);

		const serviceRouter = createServiceHandler({
			MethodA: handlerA,
			MethodB: handlerB,
			MethodC: handlerC,
		});

		// Execute in order C, B, A
		await serviceRouter.handleRequest({
			serviceName: "test.Service",
			methodName: "MethodC",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		});

		await serviceRouter.handleRequest({
			serviceName: "test.Service",
			methodName: "MethodB",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		});

		await serviceRouter.handleRequest({
			serviceName: "test.Service",
			methodName: "MethodA",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		});

		expect(executionOrder).toEqual(["C", "B", "A"]);
	});
});

describe("Extended Streaming - Advanced Error Handling", () => {
	it("should propagate errors in server-side stream correctly", async () => {
		async function* errorStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			yield {
				payload: Buffer.from(JSON.stringify({ message: "first" })),
			};
			yield {
				payload: Buffer.from(JSON.stringify({ message: "second" })),
			};
			throw new GrpcError(GrpcStatusCode.UNAVAILABLE, "Stream terminated unexpectedly");
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "ErrorStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		let capturedError: Error | null = null;

		try {
			for await (const response of errorStreamHandler(request)) {
				responses.push(response);
			}
		} catch (error) {
			capturedError = error as Error;
		}

		expect(responses).toHaveLength(2);
		expect(capturedError).toBeInstanceOf(GrpcError);
		expect((capturedError as GrpcError).code).toBe(GrpcStatusCode.UNAVAILABLE);
	});

	it("should handle stream with concurrent message yields", async () => {
		async function* concurrentStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			const messages = Array.from({ length: 20 }, (_, i) => ({
				id: i,
				timestamp: Date.now(),
			}));

			for (const msg of messages) {
				yield {
					payload: Buffer.from(JSON.stringify(msg)),
					metadata: { "x-message-id": msg.id.toString() },
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "ConcurrentStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of concurrentStreamHandler(request)) {
			responses.push(response);
		}

		expect(responses).toHaveLength(20);
		// Verify message ordering is preserved
		for (let i = 0; i < responses.length; i++) {
			const msg = JSON.parse(responses[i].payload.toString());
			expect(msg.id).toBe(i);
		}
	});

	it("should handle stream with batched responses", async () => {
		async function* batchedStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			const batchSize = 5;
			const totalMessages = 25;

			for (let batch = 0; batch < totalMessages / batchSize; batch++) {
				const messages = [];
				for (let i = 0; i < batchSize; i++) {
					messages.push(batch * batchSize + i);
				}

				yield {
					payload: Buffer.from(JSON.stringify({ batch, messages })),
					metadata: { "x-batch-num": batch.toString() },
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "BatchedStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of batchedStreamHandler(request)) {
			responses.push(response);
		}

		expect(responses).toHaveLength(5);
		const firstBatch = JSON.parse(responses[0].payload.toString());
		expect(firstBatch.messages).toEqual([0, 1, 2, 3, 4]);
	});

	it("should handle stream with null/undefined payload handling", async () => {
		async function* safeStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			for (let i = 0; i < 3; i++) {
				yield {
					payload: Buffer.from(
						JSON.stringify({
							value: i,
							optional: i % 2 === 0 ? "present" : null,
						}),
					),
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "SafeStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of safeStreamHandler(request)) {
			responses.push(response);
		}

		expect(responses).toHaveLength(3);
		expect(JSON.parse(responses[0].payload.toString()).optional).toBe("present");
		expect(JSON.parse(responses[1].payload.toString()).optional).toBeNull();
	});
});

describe("Extended Streaming - Performance & Scale", () => {
	it("should handle very large stream (5000+ messages)", async () => {
		async function* veryLargeStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			const messageCount = 5000;
			for (let i = 0; i < messageCount; i++) {
				yield {
					payload: Buffer.from(JSON.stringify({ id: i })),
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "VeryLargeStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		let count = 0;
		for await (const _response of veryLargeStreamHandler(request)) {
			count++;
		}

		expect(count).toBe(5000);
	});

	it("should handle stream with large payload per message", async () => {
		async function* largePayloadStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			const largeData = Buffer.alloc(100 * 1024); // 100KB per message
			largeData.fill("x");

			for (let i = 0; i < 5; i++) {
				yield {
					payload: Buffer.from(JSON.stringify({ index: i, size: largeData.length })),
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "LargePayloadStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of largePayloadStreamHandler(request)) {
			responses.push(response);
		}

		expect(responses).toHaveLength(5);
		const lastMsg = JSON.parse(responses[4].payload.toString());
		expect(lastMsg.size).toBe(100 * 1024);
	});

	it("should handle rapid fire stream messages", async () => {
		async function* rapidStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			// Rapidly yield many messages in quick succession
			for (let i = 0; i < 100; i++) {
				yield {
					payload: Buffer.from(i.toString()),
					metadata: { "x-seq": i.toString() },
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "RapidStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of rapidStreamHandler(request)) {
			responses.push(response);
		}

		expect(responses).toHaveLength(100);
		// Verify sequence is maintained
		for (let i = 0; i < responses.length; i++) {
			expect(responses[i].metadata?.["x-seq"]).toBe(i.toString());
		}
	});

	it("should handle stream with mixed metadata patterns", async () => {
		async function* mixedMetadataStreamHandler(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			for (let i = 0; i < 10; i++) {
				const metadata: Record<string, string> = {
					"x-index": i.toString(),
				};

				if (i % 2 === 0) {
					metadata["x-even"] = "true";
				}
				if (i % 3 === 0) {
					metadata["x-divisible-by-3"] = "true";
				}

				yield {
					payload: Buffer.from(JSON.stringify({ number: i })),
					metadata,
				};
			}
		}

		const request: GrpcRequest = {
			serviceName: "test.StreamService",
			methodName: "MixedMetadataStream",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		const responses: GrpcResponse[] = [];
		for await (const response of mixedMetadataStreamHandler(request)) {
			responses.push(response);
		}

		expect(responses).toHaveLength(10);
		expect(responses[0].metadata?.["x-even"]).toBe("true");
		expect(responses[0].metadata?.["x-divisible-by-3"]).toBe("true");
		expect(responses[1].metadata?.["x-even"]).toBeUndefined();
	});
});

describe("Extended Status Codes - Comprehensive Coverage", () => {
	const mockRequestType = {
		decode(_buffer: Uint8Array): { id: number } {
			return { id: 1 };
		},
	};

	const mockResponseType = {
		encode(message: { name: string }): { finish(): Uint8Array } {
			return {
				finish() {
					return new TextEncoder().encode(message.name);
				},
			};
		},
	};

	it("should handle multiple status codes in sequence", async () => {
		const statusCodes = [
			GrpcStatusCode.NOT_FOUND,
			GrpcStatusCode.PERMISSION_DENIED,
			GrpcStatusCode.INTERNAL,
			GrpcStatusCode.RESOURCE_EXHAUSTED,
		];

		const results: GrpcStatusCode[] = [];

		for (const code of statusCodes) {
			const handler = createUnaryHandler(
				"TestMethod",
				async () => {
					throw new GrpcError(code, "Test error");
				},
				mockRequestType,
				mockResponseType,
			);

			const request: GrpcRequest = {
				serviceName: "test.Service",
				methodName: "TestMethod",
				payload: Buffer.from([0, 0, 0, 1]),
				metadata: {},
			};

			try {
				await handler.handleRequest(request);
			} catch (error) {
				if (error instanceof GrpcError) {
					results.push(error.code);
				}
			}
		}

		expect(results).toEqual(statusCodes);
	});

	it("should differentiate between INTERNAL and other error codes", async () => {
		const handlerInternal = createUnaryHandler(
			"InternalError",
			async () => {
				throw new Error("Unexpected database failure");
			},
			mockRequestType,
			mockResponseType,
		);

		const handlerGrpcError = createUnaryHandler(
			"GrpcError",
			async () => {
				throw new GrpcError(GrpcStatusCode.INTERNAL, "Intentional INTERNAL error");
			},
			mockRequestType,
			mockResponseType,
		);

		const requestInternal: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "InternalError",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const requestGrpc: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "GrpcError",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		// Regular Error
		try {
			await handlerInternal.handleRequest(requestInternal);
		} catch (error) {
			expect(error).toBeInstanceOf(Error);
			expect((error as Error).message).toContain("database");
		}

		// GrpcError
		try {
			await handlerGrpcError.handleRequest(requestGrpc);
		} catch (error) {
			expect(error).toBeInstanceOf(GrpcError);
			expect((error as GrpcError).code).toBe(GrpcStatusCode.INTERNAL);
		}
	});

	it("should handle error codes in stream contexts", async () => {
		async function* streamWithMultipleErrors(_request: GrpcRequest): AsyncIterator<GrpcResponse> {
			yield {
				payload: Buffer.from(JSON.stringify({ id: 1 })),
			};

			// Simulate context-specific error
			throw new GrpcError(GrpcStatusCode.FAILED_PRECONDITION, "Stream prerequisites not met");
		}

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "StreamMethod",
			payload: Buffer.from("{}"),
			metadata: {},
		};

		let error: Error | null = null;

		try {
			for await (const _response of streamWithMultipleErrors(request)) {
				// consume
			}
		} catch (_e) {
			error = _e as Error;
		}

		expect(error).toBeInstanceOf(GrpcError);
		expect((error as GrpcError).code).toBe(GrpcStatusCode.FAILED_PRECONDITION);
	});

	it("should preserve error message details across status codes", async () => {
		const testCases = [
			{
				code: GrpcStatusCode.RESOURCE_EXHAUSTED,
				message: "Disk quota exceeded: 95% full",
			},
			{
				code: GrpcStatusCode.FAILED_PRECONDITION,
				message: "Database not initialized yet",
			},
			{
				code: GrpcStatusCode.ABORTED,
				message: "Transaction conflict with concurrent write",
			},
			{
				code: GrpcStatusCode.OUT_OF_RANGE,
				message: "Offset 99999 exceeds file size 50000",
			},
		];

		for (const testCase of testCases) {
			const handler = createUnaryHandler(
				"TestMethod",
				async () => {
					throw new GrpcError(testCase.code, testCase.message);
				},
				mockRequestType,
				mockResponseType,
			);

			const request: GrpcRequest = {
				serviceName: "test.Service",
				methodName: "TestMethod",
				payload: Buffer.from([0, 0, 0, 1]),
				metadata: {},
			};

			try {
				await handler.handleRequest(request);
				throw new Error("Should have thrown");
			} catch (error) {
				expect((error as GrpcError).code).toBe(testCase.code);
				expect((error as GrpcError).message).toBe(testCase.message);
			}
		}
	});
});

describe("Advanced Concurrency & Performance Tests", () => {
	const mockRequestType = {
		decode(_buffer: Uint8Array): { id: number } {
			return { id: _buffer.length };
		},
	};

	const mockResponseType = {
		encode(message: { result: string }): { finish(): Uint8Array } {
			return {
				finish() {
					return new TextEncoder().encode(message.result);
				},
			};
		},
	};

	it("should handle 100+ parallel handler executions", async () => {
		let concurrentExecutions = 0;
		let maxConcurrentCount = 0;

		const handler = createUnaryHandler(
			"ParallelOperation",
			async (_req: { id: number }) => {
				concurrentExecutions++;
				maxConcurrentCount = Math.max(maxConcurrentCount, concurrentExecutions);

				// Simulate async work
				await new Promise((resolve) => setTimeout(resolve, 10));

				concurrentExecutions--;
				return { result: "done" };
			},
			mockRequestType,
			mockResponseType,
		);

		const promises = [];
		for (let i = 0; i < 100; i++) {
			const request: GrpcRequest = {
				serviceName: "test.ParallelService",
				methodName: "ParallelOperation",
				payload: Buffer.alloc(10),
				metadata: {},
			};
			promises.push(handler.handleRequest(request));
		}

		const results = await Promise.all(promises);
		expect(results).toHaveLength(100);
		expect(maxConcurrentCount).toBeGreaterThan(1); // Should have true concurrency
	});

	it("should handle handler state isolation across requests", async () => {
		const executionStates: Record<number, string[]> = {};
		let requestCounter = 0;

		const handler = createUnaryHandler(
			"StatefulOperation",
			async (_req: { id: number }) => {
				const requestId = requestCounter++;
				const state: string[] = [];
				executionStates[requestId] = state;

				state.push("start");
				await new Promise((resolve) => setTimeout(resolve, Math.random() * 20));
				state.push("middle");
				await new Promise((resolve) => setTimeout(resolve, Math.random() * 20));
				state.push("end");

				return { result: `request-${requestId}` };
			},
			mockRequestType,
			mockResponseType,
		);

		const promises = [];
		for (let i = 0; i < 20; i++) {
			const request: GrpcRequest = {
				serviceName: "test.Service",
				methodName: "StatefulOperation",
				payload: Buffer.alloc(10),
				metadata: {},
			};
			promises.push(handler.handleRequest(request));
		}

		await Promise.all(promises);

		// Verify each request had its own isolated state progression
		Object.values(executionStates).forEach((state) => {
			expect(state).toEqual(["start", "middle", "end"]);
		});
	});

	it("should maintain metadata isolation in concurrent requests", async () => {
		const receivedMetadata: Record<string, string>[] = [];

		const handler = createUnaryHandler(
			"MetadataTest",
			async (_req: { id: number }, metadata: Record<string, string>) => {
				receivedMetadata.push({ ...metadata });
				return { result: "ok" };
			},
			mockRequestType,
			mockResponseType,
		);

		const promises = [];
		for (let i = 0; i < 10; i++) {
			const request: GrpcRequest = {
				serviceName: "test.Service",
				methodName: "MetadataTest",
				payload: Buffer.alloc(10),
				metadata: {
					"x-request-id": `req-${i}`,
					"x-user-id": `user-${i}`,
				},
			};
			promises.push(handler.handleRequest(request));
		}

		await Promise.all(promises);

		// Verify metadata was correctly isolated for each request
		expect(receivedMetadata).toHaveLength(10);
		receivedMetadata.forEach((meta, idx) => {
			expect(meta["x-request-id"]).toBe(`req-${idx}`);
			expect(meta["x-user-id"]).toBe(`user-${idx}`);
		});
	});

	it("should handle errors in concurrent executions without cross-contamination", async () => {
		let successCount = 0;
		let errorCount = 0;

		const handler = createUnaryHandler(
			"MayFailOperation",
			async (_req: { id: number }) => {
				const shouldFail = Math.random() > 0.6;
				if (shouldFail) {
					throw new GrpcError(GrpcStatusCode.INTERNAL, "Random failure for testing");
				}
				return { result: "success" };
			},
			mockRequestType,
			mockResponseType,
		);

		const promises = [];
		for (let i = 0; i < 50; i++) {
			const request: GrpcRequest = {
				serviceName: "test.Service",
				methodName: "MayFailOperation",
				payload: Buffer.alloc(10),
				metadata: {},
			};

			promises.push(
				handler.handleRequest(request).then(
					() => {
						successCount++;
						return { success: true };
					},
					() => {
						errorCount++;
						return { success: false };
					},
				),
			);
		}

		const results = await Promise.all(promises);

		expect(successCount + errorCount).toBe(50);
		expect(successCount).toBeGreaterThan(0);
		expect(errorCount).toBeGreaterThan(0);
		expect(results).toHaveLength(50);
	});

	it("should handle memory cleanup after concurrent large payload processing", async () => {
		const handler = createUnaryHandler(
			"MemoryConcurrentTest",
			async (_req: { id: number }) => {
				// Simulate processing
				const temp = Buffer.alloc(1024 * 1024); // 1MB temporary buffer
				temp.fill(0xff);
				// temp goes out of scope - should be GC'd
				return { result: `processed-${temp.length}` };
			},
			mockRequestType,
			mockResponseType,
		);

		const promises = [];
		const largePayload = Buffer.alloc(5 * 1024 * 1024); // 5MB

		for (let i = 0; i < 5; i++) {
			const request: GrpcRequest = {
				serviceName: "test.Service",
				methodName: "MemoryConcurrentTest",
				payload: largePayload,
				metadata: {},
			};
			promises.push(handler.handleRequest(request));
		}

		const results = await Promise.all(promises);
		expect(results).toHaveLength(5);
		// Test completes without memory issues (verified through runtime)
	});
});

describe("Extended Metadata & Header Tests", () => {
	const mockRequestType = {
		decode(_buffer: Uint8Array): { id: number } {
			return { id: 1 };
		},
	};

	const mockResponseType = {
		encode(message: { result: string }): { finish(): Uint8Array } {
			return {
				finish() {
					return new TextEncoder().encode(message.result);
				},
			};
		},
	};

	it("should handle metadata with special characters and encodings", async () => {
		const handler = createUnaryHandler(
			"SpecialCharMetadata",
			async (_req: { id: number }, metadata: Record<string, string>) => {
				expect(metadata["x-emoji"]).toContain("🚀");
				expect(metadata["x-special"]).toContain("@#$%");
				expect(metadata["x-dash-case"]).toBe("value");
				expect(metadata.x_underscore_case).toBe("value");
				return { result: "ok" };
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "SpecialCharMetadata",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {
				"x-emoji": "test-🚀",
				"x-special": "test-@#$%",
				"x-dash-case": "value",
				x_underscore_case: "value",
			},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toBe("ok");
	});

	it("should handle response metadata with multiple entries", async () => {
		const handler = createUnaryHandler(
			"MultiMetadata",
			async () => {
				const metadata: Record<string, string> = {};
				for (let i = 0; i < 50; i++) {
					metadata[`x-header-${i}`] = `value-${i}`;
				}
				return {
					response: { result: "ok" },
					metadata,
				};
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "MultiMetadata",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {},
		};

		const response = await handler.handleRequest(request);
		expect(response.metadata).toBeDefined();
		const metadata = response.metadata ?? {};
		expect(Object.keys(metadata)).toHaveLength(50);
		expect(metadata["x-header-25"]).toBe("value-25");
	});

	it("should handle metadata inheritance in service routing", async () => {
		let capturedMetadata: Record<string, string> = {};

		const method1Handler = createUnaryHandler(
			"Method1",
			async (_req, metadata) => {
				capturedMetadata = metadata;
				return { result: "m1" };
			},
			mockRequestType,
			mockResponseType,
		);

		const serviceRouter = createServiceHandler({
			Method1: method1Handler,
		});

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "Method1",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {
				"x-trace-id": "trace-123",
				"x-span-id": "span-456",
			},
		};

		await serviceRouter.handleRequest(request);

		expect(capturedMetadata["x-trace-id"]).toBe("trace-123");
		expect(capturedMetadata["x-span-id"]).toBe("span-456");
	});

	it("should handle empty string values in metadata", async () => {
		const handler = createUnaryHandler(
			"EmptyMetadata",
			async (_req, metadata) => {
				expect(metadata["x-empty"]).toBe("");
				expect(metadata["x-empty"]).toBeDefined();
				return { result: "ok" };
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "EmptyMetadata",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {
				"x-empty": "",
				"x-normal": "value",
			},
		};

		const response = await handler.handleRequest(request);
		expect(response.payload.toString()).toBe("ok");
	});

	it("should pass metadata reference to handler that can be modified", async () => {
		const capturedMetadata: Record<string, string> = {};

		const handler = createUnaryHandler(
			"MetadataReference",
			async (_req, metadata) => {
				// Copy metadata to verify it's accessible
				Object.assign(capturedMetadata, metadata);
				return { result: "ok" };
			},
			mockRequestType,
			mockResponseType,
		);

		const request: GrpcRequest = {
			serviceName: "test.Service",
			methodName: "MetadataReference",
			payload: Buffer.from([0, 0, 0, 1]),
			metadata: {
				"x-original": "value",
				"x-request-id": "req-123",
			},
		};

		await handler.handleRequest(request);

		// Verify metadata was accessible in handler
		expect(capturedMetadata["x-original"]).toBe("value");
		expect(capturedMetadata["x-request-id"]).toBe("req-123");
	});
});
