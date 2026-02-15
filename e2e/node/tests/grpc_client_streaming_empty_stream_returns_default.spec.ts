/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingEmptyStreamReturnsDefault } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming - empty stream returns default", async () => {
		// Tests client streaming RPC where client sends no messages (empty stream). Server gracefully handles empty input and returns default response.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.OptionalService",
			methodName: "ProcessOptional",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingEmptyStreamReturnsDefault(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					request_id: "empty-stream-req",
					message_count: 0,
					result: "DEFAULT_RESULT",
					is_default: true,
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
