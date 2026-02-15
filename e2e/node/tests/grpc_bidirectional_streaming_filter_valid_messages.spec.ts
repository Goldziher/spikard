/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcBidirectionalStreamingFilterValidMessages } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Bidirectional streaming - filter valid messages", async () => {
		// Tests bidirectional streaming RPC where server filters out invalid messages during streaming.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.FilterService",
			methodName: "FilterValid",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcBidirectionalStreamingFilterValidMessages(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.metadata).toBeDefined();
	});
});
