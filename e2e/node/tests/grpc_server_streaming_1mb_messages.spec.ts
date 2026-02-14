/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreaming1mbMessages } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - 1MB messages", async () => {
		// Tests server streaming RPC with large message payloads (approximately 1MB each). Validates that the streaming framework can handle large individual messages without truncation or memory issues.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "StreamLargeMessages",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreaming1mbMessages(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify("3 large messages streamed successfully")));
		expect(response.metadata).toBeDefined();
	});
});
