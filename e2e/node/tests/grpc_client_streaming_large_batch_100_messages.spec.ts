/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingLargeBatch100Messages } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming - large batch 100 messages", async () => {
		// Tests client streaming RPC with 100 messages in the stream. Validates performance with large batch aggregation.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.BatchService",
			methodName: "ProcessBatch",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingLargeBatch100Messages(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					batch_id: "batch-large-001",
					total_items: 100,
					total_value: 5050,
					average_value: 50.5,
					status: "PROCESSED",
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
