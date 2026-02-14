/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcBidirectionalStreamingRpc } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Bidirectional streaming RPC", async () => {
		// Tests bidirectional streaming where both client and server send multiple messages. Covers duplex communication patterns.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
			authorization: "Bearer user-token",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ChatService",
			methodName: "Chat",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcBidirectionalStreamingRpc(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.metadata).toBeDefined();
	});
});
