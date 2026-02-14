/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreamingTimeoutScenario } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - timeout scenario", async () => {
		// Tests server streaming RPC that exceeds the deadline/timeout. The server starts streaming but doesn't complete before the client-imposed timeout expires. Validates proper timeout handling and stream cancellation.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
			"grpc-timeout": "1000m",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "StreamWithDelay",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreamingTimeoutScenario(request);

		// Verify response
		expect(response.statusCode).toBe("DEADLINE_EXCEEDED");
		expect(response.metadata).toBeDefined();
	});
});
