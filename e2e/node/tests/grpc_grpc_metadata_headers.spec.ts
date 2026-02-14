/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcMetadataHeaders } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC metadata headers", async () => {
		// Tests gRPC metadata handling for request/response headers including authorization, tracing IDs, and custom headers.

		const metadata: Record<string, string> = {
			authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
			"x-trace-id": "trace-abc123def456",
			"content-type": "application/grpc",
			"x-custom-header": "custom-value",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.MetadataService",
			methodName: "CheckMetadata",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcMetadataHeaders(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					request_id: "req-987654321",
					received_auth_header: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
					received_trace_id: "trace-abc123def456",
					received_custom_header: "custom-value",
					response_time_ms: 45,
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
