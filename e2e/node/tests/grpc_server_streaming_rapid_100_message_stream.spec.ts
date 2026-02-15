/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreamingRapid100MessageStream } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - rapid 100 message stream", async () => {
		// Tests server streaming RPC with 100 messages sent in rapid succession. Validates backpressure handling, buffering, and delivery of high-volume message streams without loss or corruption.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "StreamRapidMessages",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreamingRapid100MessageStream(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify("100 messages streamed successfully in sequence")));
		expect(response.metadata).toBeDefined();
	});
});
