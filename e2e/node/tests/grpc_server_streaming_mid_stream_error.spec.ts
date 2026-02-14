/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreamingMidStreamError } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - mid-stream error", async () => {
		// Tests server streaming RPC that sends 5 messages successfully, then encounters an error before completing the stream. Validates partial stream delivery and error handling.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "StreamData",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreamingMidStreamError(request);

		// Verify response
		expect(response.statusCode).toBe("INTERNAL");
		expect(response.metadata).toBeDefined();
	});
});
