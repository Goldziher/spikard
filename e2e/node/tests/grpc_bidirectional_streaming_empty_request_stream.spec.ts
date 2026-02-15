/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcBidirectionalStreamingEmptyRequestStream } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Bidirectional streaming - empty request stream", async () => {
		// Tests bidirectional streaming RPC with empty request stream but server sends response.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.EmptyService",
			methodName: "HandleEmpty",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcBidirectionalStreamingEmptyRequestStream(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.metadata).toBeDefined();
	});
});
