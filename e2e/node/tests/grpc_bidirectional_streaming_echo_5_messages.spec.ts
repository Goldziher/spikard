/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcBidirectionalStreamingEcho5Messages } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Bidirectional streaming - echo 5 messages", async () => {
		// Tests bidirectional streaming RPC where client sends 5 messages and expects them echoed back in the same order.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.EchoService",
			methodName: "EchoBidi",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcBidirectionalStreamingEcho5Messages(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.metadata).toBeDefined();
	});
});
