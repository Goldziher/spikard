/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcBidirectionalStreamingTransformToUppercase } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Bidirectional streaming - transform to uppercase", async () => {
		// Tests bidirectional streaming RPC where server transforms incoming messages to uppercase.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.TransformService",
			methodName: "Transform",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcBidirectionalStreamingTransformToUppercase(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.metadata).toBeDefined();
	});
});
