/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcCustomAuthenticationSchemeHeader } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Custom authentication scheme header", async () => {
		// Tests custom authentication header scheme. Validates that custom auth headers are properly extracted and validated.

		const metadata: Record<string, string> = {
			"x-custom-auth": "CustomScheme token_value_123",
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.CustomAuthService",
			methodName: "Execute",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcCustomAuthenticationSchemeHeader(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify({ success: true })));
		expect(response.metadata).toBeDefined();
	});
});
