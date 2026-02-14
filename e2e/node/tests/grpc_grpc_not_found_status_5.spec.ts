/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcNotFoundStatus5 } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC NOT_FOUND status 5", async () => {
		// Tests NOT_FOUND gRPC status code. Returned when a requested resource (e.g., user, file) does not exist.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ResourceService",
			methodName: "Get",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcNotFoundStatus5(request);

		// Verify response
		expect(response.statusCode).toBe("NOT_FOUND");
		expect(response.metadata).toBeDefined();
	});
});
