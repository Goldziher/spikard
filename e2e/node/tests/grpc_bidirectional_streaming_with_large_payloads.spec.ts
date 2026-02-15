/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcBidirectionalStreamingWithLargePayloads } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Bidirectional streaming with large payloads", async () => {
		// Tests bidirectional streaming RPC with large messages in both directions. Validates concurrent read/write handling and proper message ordering.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.BiDirectionalService",
			methodName: "Exchange",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcBidirectionalStreamingWithLargePayloads(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(JSON.stringify({ message_id: "bi-large-001", sequence: 1, direction: "server-to-client" })),
		);
		expect(response.metadata).toBeDefined();
	});
});
