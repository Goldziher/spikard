/**
 * E2E test for gRPC
 * @generated
 */

import { Buffer } from "node:buffer";
import { describe, expect, test } from "vitest";
import { handleGrpcClientStreamingRpc } from "../app/main.ts";

describe("grpc", () => {
	test("should handle gRPC request: Client streaming RPC", async () => {
		// Tests client streaming where client sends multiple messages. Covers streaming request aggregation patterns.

		const metadata: Record<string, string> = {
			"content-type": "application/grpc",
		};
		const request: GrpcRequest = {
			serviceName: "example.v1.FileService",
			methodName: "Upload",
			payload: Buffer.from(JSON.stringify({})),
			metadata,
		};

		const response = await handleGrpcClientStreamingRpc(request);

		// Verify response
		expect(response.statusCode).toBe("OK");
		expect(response.payload).toEqual(
			Buffer.from(
				JSON.stringify({
					file_id: "file-12345",
					total_bytes: 57,
					status: "COMPLETED",
					checksum: "d8e8fca2dc0f896fd7cb4cb0031ba249",
				}),
			),
		);
		expect(response.metadata).toBeDefined();
	});
});
