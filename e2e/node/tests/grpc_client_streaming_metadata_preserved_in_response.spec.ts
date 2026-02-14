/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingMetadataPreservedInResponse } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming - metadata preserved in response", async () => {
		// Tests client streaming RPC where request metadata is forwarded to and preserved in the response. Validates metadata propagation through streaming pipeline.

		const metadata: Record<string, string> = {
			"x-trace-id": "trace-abc456",
			"x-user-id": "user-789",
			"content-type": "application/grpc",
			authorization: "Bearer token-xyz123",
			"custom-header": "custom-value",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.MetadataService",
			methodName: "ProcessWithMetadata",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingMetadataPreservedInResponse(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					request_id: "req-meta-001",
					processed_by: "grpc-handler-1",
					received_user_id: "user-789",
					message_count: 3,
					trace_id: "trace-abc456",
					status: "COMPLETE_WITH_METADATA",
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
