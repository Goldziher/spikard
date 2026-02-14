/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreamingEmptyStream } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - empty stream", async () => {
		// Tests server streaming RPC that returns an empty stream. The server opens the stream but sends no messages before completing successfully.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "GetEmptyStream",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreamingEmptyStream(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify("Stream completed with no messages")));
		expect(response.metadata).toBeDefined();
	});
});
