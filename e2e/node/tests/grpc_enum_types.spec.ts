/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcEnumTypes } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Enum types", async () => {
		// Tests enum definitions and serialization. Covers enum fields with named constants.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.OrderService",
			methodName: "CreateOrder",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcEnumTypes(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({ id: 1001, product_name: "Laptop", quantity: 2, status: "PENDING", priority: "HIGH" }),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
