/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcErrorHandlingDeadlineExceeded } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Error handling - deadline exceeded", async () => {
		// Tests server streaming RPC that exceeds deadline. Expects DEADLINE_EXCEEDED status when RPC time exceeds configured timeout.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ErrorTestService",
			methodName: "SlowStream",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcErrorHandlingDeadlineExceeded(request);

		// Verify response
		expect(response.statusCode).toBe("DEADLINE_EXCEEDED");
		expect(response.metadata).toBeDefined();
	});
});
