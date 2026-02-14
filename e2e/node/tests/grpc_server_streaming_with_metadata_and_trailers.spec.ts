/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcServerStreamingWithMetadataAndTrailers } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Server streaming - with metadata and trailers", async () => {
		// Tests server streaming RPC with gRPC metadata headers and trailers. Validates that metadata is accessible before streaming begins and trailers are delivered after stream completion.

		const metadata: Record<string, string> = {
			"x-client-version": "1.0.0",
			"x-request-id": "metadata-stream-001",
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.StreamService",
			methodName: "StreamWithMetadata",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcServerStreamingWithMetadataAndTrailers(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(Buffer.from(JSON.stringify("Stream completed with metadata and trailers")));
		expect(response.metadata).toBeDefined();
	});
});
