/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreamingSingleMessage } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - single message", async () => {
		// Tests server streaming RPC that returns exactly one message. Verifies that single-message streams are properly handled and distinguished from unary responses.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "GetSingleMessage",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreamingSingleMessage(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify("Stream completed with one message")));
		expect(response.metadata).toBeDefined();
	});
});
