/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreamingNestedObjectMessages } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - nested object messages", async () => {
		// Tests server streaming RPC with complex nested message structures. Validates proper serialization and deserialization of deeply nested protobuf objects in streaming context.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "StreamPeople",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreamingNestedObjectMessages(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify("3 people with nested objects streamed successfully")));
		expect(response.metadata).toBeDefined();
	});
});
