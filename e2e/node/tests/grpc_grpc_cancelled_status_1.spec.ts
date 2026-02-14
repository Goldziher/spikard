/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcCancelledStatus1 } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC CANCELLED status 1", async () => {
		// Tests CANCELLED gRPC status code. Returned when the RPC was cancelled by the client or server.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.CancelService",
			methodName: "Operation",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcCancelledStatus1(request);

		// Verify response
		expect(response.statusCode).toBe("CANCELLED");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify({ id: "cancel-001" })));
		expect(response.metadata).toBeDefined();
	});
});
