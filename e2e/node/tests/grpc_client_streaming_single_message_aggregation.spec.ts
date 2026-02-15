/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingSingleMessageAggregation } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming - single message aggregation", async () => {
		// Tests client streaming RPC where client sends a single message. Server acknowledges and returns aggregated result.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.AggregateService",
			methodName: "AggregateData",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingSingleMessageAggregation(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(JSON.stringify({ count: 1, total: 42, average: 42.0, status: "AGGREGATED" })),
		);
		expect(response.metadata).toBeDefined();
	});
});
