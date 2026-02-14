/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcDataLossStatus15 } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC DATA_LOSS status 15", async () => {
		// Tests DATA_LOSS gRPC status code. Returned when unrecoverable data loss or corruption occurred.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.DataService",
			methodName: "Process",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcDataLossStatus15(request);

		// Verify response
		expect(response.statusCode).toBe("DATA_LOSS");
		expect(response.metadata).toBeDefined();
	});
});
