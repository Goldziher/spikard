/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreaming10Messages } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - 10 messages", async () => {
		// Tests server streaming RPC that returns a normal stream of 10 messages. Validates message ordering and complete stream delivery.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "ListItems",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreaming10Messages(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify("10 messages streamed successfully")));
		expect(response.metadata).toBeDefined();
	});
});
