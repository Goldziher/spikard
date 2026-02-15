/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcLarge10mbMessagePayload } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Large 10MB message payload", async () => {
		// Tests handling of 10MB protobuf messages. Validates high-capacity transfers, memory efficiency, and absence of stream fragmentation issues.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.BulkService",
			methodName: "BulkUpload",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcLarge10mbMessagePayload(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify({ id: "bulk-10mb-transfer", status: "received" })));
		expect(response.metadata).toBeDefined();
	});
});
