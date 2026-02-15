/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingUnicodeStringAggregation } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming - Unicode string aggregation", async () => {
		// Tests client streaming RPC with Unicode strings that are concatenated. Validates proper UTF-8 handling across multiple messages.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.TextService",
			methodName: "ConcatenateStrings",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingUnicodeStringAggregation(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					fragment_id: "unicode-001",
					result: "Hello, 世界! Привет 🌍",
					fragment_count: 4,
					total_length: 26,
					status: "CONCATENATED",
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
