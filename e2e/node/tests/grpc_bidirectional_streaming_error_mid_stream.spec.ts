/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcBidirectionalStreamingErrorMidStream } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Bidirectional streaming - error mid-stream", async () => {
		// Tests bidirectional streaming RPC where server returns error after processing some messages.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ErrorService",
			methodName: "ProcessWithError",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcBidirectionalStreamingErrorMidStream(request);

		// Verify response
		expect(response.statusCode).toBe("INTERNAL");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify("Error after processing 2 messages")));
		expect(response.metadata).toBeDefined();
	});
});
