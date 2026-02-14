/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcUnknownStatus2 } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC UNKNOWN status 2", async () => {
		// Tests UNKNOWN gRPC status code. Used for errors that do not fit any other status code.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.UnknownService",
			methodName: "Fail",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcUnknownStatus2(request);

		// Verify response
		expect(response.statusCode).toBe("UNKNOWN");
		expect(response.metadata).toBeDefined();
	});
});
