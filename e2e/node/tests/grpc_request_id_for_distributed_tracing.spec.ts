/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcRequestIdForDistributedTracing } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Request ID for distributed tracing", async () => {
		// Tests request ID header propagation for distributed tracing. Validates X-Request-ID generation and propagation.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
			"x-request-id": "req-12345-67890",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.TracingService",
			methodName: "Trace",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcRequestIdForDistributedTracing(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify({ request_id: "req-12345-67890" })));
		expect(response.metadata).toBeDefined();
	});
});
