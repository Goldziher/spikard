/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcAbortedStatus10 } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC ABORTED status 10", async () => {
		// Tests ABORTED gRPC status code. Returned when an operation was aborted, typically due to a concurrency issue like conflict.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.TransactionService",
			methodName: "Commit",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcAbortedStatus10(request);

		// Verify response
		expect(response.statusCode).toBe("ABORTED");
		expect(response.metadata).toBeDefined();
	});
});
