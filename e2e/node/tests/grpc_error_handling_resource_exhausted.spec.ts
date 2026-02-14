/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcErrorHandlingResourceExhausted } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Error handling - resource exhausted", async () => {
		// Tests bidirectional streaming RPC exceeding rate limits. Expects RESOURCE_EXHAUSTED status when client attempts to send 100 messages in rapid succession, exceeding the 100 requests/second rate limit threshold.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ErrorTestService",
			methodName: "RateLimitedChat",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcErrorHandlingResourceExhausted(request);

		// Verify response
		expect(response.statusCode).toBe("RESOURCE_EXHAUSTED");
		expect(response.metadata).toBeDefined();
	});
});
