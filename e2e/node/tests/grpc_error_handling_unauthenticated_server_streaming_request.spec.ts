/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcErrorHandlingUnauthenticatedServerStreamingRequest } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Error handling - unauthenticated server streaming request", async () => {
		// Tests server streaming RPC without required auth metadata. Expects UNAUTHENTICATED status when authorization header is missing.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ErrorTestService",
			methodName: "SecureStream",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcErrorHandlingUnauthenticatedServerStreamingRequest(request);

		// Verify response
		expect(response.statusCode).toBe("UNAUTHENTICATED");
		expect(response.metadata).toBeDefined();
	});
});
