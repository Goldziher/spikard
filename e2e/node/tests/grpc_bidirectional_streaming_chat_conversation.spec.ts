/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcBidirectionalStreamingChatConversation } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Bidirectional streaming - chat conversation", async () => {
		// Tests bidirectional streaming RPC simulating a chat-like service with alternating messages.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ChatService",
			methodName: "Chat",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcBidirectionalStreamingChatConversation(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.metadata).toBeDefined();
	});
});
