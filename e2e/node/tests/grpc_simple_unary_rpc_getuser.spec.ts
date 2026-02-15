/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcSimpleUnaryRpcGetuser } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Simple unary RPC - GetUser", async () => {
		// Tests basic unary gRPC call with scalar types (int32, string). Covers fundamental request-response pattern.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
			authorization: "Bearer test-token",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.UserService",
			methodName: "GetUser",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcSimpleUnaryRpcGetuser(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(JSON.stringify({ id: 123, name: "Alice Johnson", email: "alice@example.com" })),
		);
		expect(response.metadata).toBeDefined();
	});
});
