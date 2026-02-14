/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcAllFieldsSetToZeroFalseEmptyValues } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: All fields set to zero false empty values", async () => {
		// Tests proto3 default value behavior when all fields are explicitly set to zero, false, empty string. Validates that zero values are transmitted correctly.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.ZeroValueService",
			methodName: "ProcessZeros",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcAllFieldsSetToZeroFalseEmptyValues(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify({ success: true, fields_received: 5 })));
		expect(response.metadata).toBeDefined();
	});
});
