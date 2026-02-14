/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcErrorHandlingStreamErrorMidTransmission } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Error handling - stream error mid-transmission", async () => {
		// Tests server streaming RPC that errors after yielding 3 messages. The stream opens successfully and delivers 3 messages before encountering an INTERNAL error. Validates that partial stream data is delivered before the error is signaled.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ErrorTestService",
			methodName: "StreamWithError",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcErrorHandlingStreamErrorMidTransmission(request);

		// Verify response
		expect(response.statusCode).toBe("INTERNAL");
		expect(response.metadata).toBeDefined();
	});
});
