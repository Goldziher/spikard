/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcFailedPreconditionStatus9 } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC FAILED_PRECONDITION status 9", async () => {
		// Tests FAILED_PRECONDITION gRPC status code. Returned when the RPC failed because the system is not in the required state.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StateService",
			methodName: "Proceed",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcFailedPreconditionStatus9(request);

		// Verify response
		expect(response.statusCode).toBe("FAILED_PRECONDITION");
		expect(response.metadata).toBeDefined();
	});
});
