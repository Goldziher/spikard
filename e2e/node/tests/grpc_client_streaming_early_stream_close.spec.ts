/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingEarlyStreamClose } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming - early stream close", async () => {
		// Tests client streaming RPC where client closes stream after sending 3 messages instead of the expected 5. Server should gracefully handle partial stream.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "SendChunks",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingEarlyStreamClose(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({ session_id: "sess-early-001", received_chunks: 3, expected_chunks: 5, status: "INCOMPLETE" }),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
