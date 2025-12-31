/**
 * Tests for gRPC TypeScript bindings
 *
 * These tests verify the gRPC handler interfaces and helper functions
 * work correctly for TypeScript consumers.
 */

import { describe, expect, it } from "vitest";
import {
	GrpcError,
	GrpcStatusCode,
	createServiceHandler,
	createUnaryHandler,
	type GrpcHandler,
	type GrpcRequest,
	type GrpcResponse,
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
				expect(metadata["authorization"]).toBe("Bearer token123");
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
});

describe("createServiceHandler", () => {
	const mockRequestType = {
		decode(buffer: Uint8Array): { id: number } {
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
			GetUser: createUnaryHandler(
				"GetUser",
				async () => ({ name: "Test" }),
				mockRequestType,
				mockResponseType,
			),
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
			Method1: createUnaryHandler(
				"Method1",
				async () => ({ name: "M1" }),
				mockRequestType,
				mockResponseType,
			),
			Method2: createUnaryHandler(
				"Method2",
				async () => ({ name: "M2" }),
				mockRequestType,
				mockResponseType,
			),
			Method3: createUnaryHandler(
				"Method3",
				async () => ({ name: "M3" }),
				mockRequestType,
				mockResponseType,
			),
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
