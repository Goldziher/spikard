/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcMaximumFieldNumber536870911 } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Maximum field number 536870911", async () => {
		// Tests protobuf messages using the maximum allowed field number (536870911). Validates proper field number encoding in varint format.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.MaxFieldService",
			methodName: "TestMaxField",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcMaximumFieldNumber536870911(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(JSON.stringify({ id: 42, received_max: "Testing maximum field number" })),
		);
		expect(response.metadata).toBeDefined();
	});
});
