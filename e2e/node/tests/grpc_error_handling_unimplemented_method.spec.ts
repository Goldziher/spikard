/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcErrorHandlingUnimplementedMethod } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Error handling - unimplemented method", async () => {
		// Tests unary RPC calling an unimplemented method. Validates that UNIMPLEMENTED status is returned when the server does not support the requested RPC method. This fixture ensures proper error handling for feature requests that are not yet available in the current server implementation.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ErrorTestService",
			methodName: "FutureFeature",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcErrorHandlingUnimplementedMethod(request);

		// Verify response
		expect(response.statusCode).toBe("UNIMPLEMENTED");
		expect(response.metadata).toBeDefined();
	});
});
