/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingMessageSizeLimitExceeded } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming - message size limit exceeded", async () => {
		// Tests client streaming RPC where one message exceeds the max_message_size limit. Server rejects the oversized message and terminates the stream.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
			"grpc-max-message-size": "4096",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.PayloadService",
			methodName: "ProcessPayloads",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingMessageSizeLimitExceeded(request);

		// Verify response
		expect(response.statusCode).toBe("RESOURCE_EXHAUSTED");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					message_id: "payload-002",
					processed_count: 1,
					status: "FAILED",
					error_detail: "Message payload size 10240 exceeds maximum allowed size 4096",
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
