/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreaming10MessagesSum } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming - 10 messages sum", async () => {
		// Tests client streaming RPC where client sends 10 integer values. Server sums all values and returns result.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.MathService",
			methodName: "SumNumbers",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreaming10MessagesSum(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(JSON.stringify({ sequence_id: "seq-001", count: 10, sum: 550, status: "COMPLETE" })),
		);
		expect(response.metadata).toBeDefined();
	});
});
