/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcErrorHandlingResourceNotFound } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Error handling - resource not found", async () => {
		// Tests NOT_FOUND gRPC status code. Returned when the requested resource does not exist. Validates unary RPC requesting non-existent resource.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ErrorTestService",
			methodName: "GetResource",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcErrorHandlingResourceNotFound(request);

		// Verify response
		expect(response.statusCode).toBe("NOT_FOUND");
		expect(response.metadata).toBeDefined();
	});
});
