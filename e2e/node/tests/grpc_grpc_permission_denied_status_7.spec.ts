/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcPermissionDeniedStatus7 } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC PERMISSION_DENIED status 7", async () => {
		// Tests PERMISSION_DENIED gRPC status code. Returned when the caller does not have sufficient permissions.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.SecureService",
			methodName: "AdminAction",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcPermissionDeniedStatus7(request);

		// Verify response
		expect(response.statusCode).toBe("PERMISSION_DENIED");
		expect(response.metadata).toBeDefined();
	});
});
