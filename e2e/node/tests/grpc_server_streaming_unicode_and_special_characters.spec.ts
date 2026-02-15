/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreamingUnicodeAndSpecialCharacters } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - unicode and special characters", async () => {
		// Tests server streaming RPC with messages containing unicode characters, emoji, special symbols, and multi-byte UTF-8 sequences. Validates proper encoding/decoding across the streaming pipeline.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
			encoding: "utf-8",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "StreamUnicodeMessages",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreamingUnicodeAndSpecialCharacters(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify("Unicode stream completed successfully")));
		expect(response.metadata).toBeDefined();
	});
});
