/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcErrorHandlingPermissionDeniedClientStreaming } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Error handling - permission denied client streaming", async () => {
		// Tests client streaming RPC accessing unauthorized resource. Expects PERMISSION_DENIED status when client sends restricted access level requests. Demonstrates permission validation on streaming upload operations.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ErrorTestService",
			methodName: "UploadRestricted",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcErrorHandlingPermissionDeniedClientStreaming(request);

		// Verify response
		expect(response.statusCode).toBe("PERMISSION_DENIED");
		expect(response.metadata).toBeDefined();
	});
});
