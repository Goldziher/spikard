/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingWithLargeBatchRequests } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming with large batch requests", async () => {
		// Tests client streaming RPC with large batch requests. Validates server accumulation of multiple large client messages.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.BatchService",
			methodName: "ProcessBatch",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingWithLargeBatchRequests(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(JSON.stringify({ batch_id: "batch-large-001", items_processed: 100, total_bytes: 5242880 })),
		);
		expect(response.metadata).toBeDefined();
	});
});
