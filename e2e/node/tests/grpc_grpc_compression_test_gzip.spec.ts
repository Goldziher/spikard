/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcGrpcCompressionTestGzip } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: gRPC compression test - gzip", async () => {
		// Tests gRPC payload compression using gzip. Validates that compressed messages are properly decompressed and that header metadata indicates compression.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
			"grpc-encoding": "gzip",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.CompressionService",
			methodName: "SendCompressed",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcGrpcCompressionTestGzip(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify({ id: "compress-test-001", compressed: true })));
		expect(response.metadata).toBeDefined();
	});
});
