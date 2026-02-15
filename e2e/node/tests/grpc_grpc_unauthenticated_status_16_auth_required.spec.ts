/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcUnauthenticatedStatus16AuthRequired } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC UNAUTHENTICATED status 16 - auth required", async () => {
		// Tests UNAUTHENTICATED gRPC status code. Returned when the request lacks valid authentication credentials.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.AuthService",
			methodName: "SecureOp",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcUnauthenticatedStatus16AuthRequired(request);

		// Verify response
		expect(response.statusCode).toBe("UNAUTHENTICATED");
		expect(response.metadata).toBeDefined();
	});
});
