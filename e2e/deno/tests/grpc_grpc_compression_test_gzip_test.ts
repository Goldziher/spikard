/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcCompressionTestGzip, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC compression test - gzip", async () => {
  // Tests gRPC payload compression using gzip. Validates that compressed messages are properly decompressed and that header metadata indicates compression.

  const metadata: Record<string, string> = {
    "grpc-encoding": "gzip",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.CompressionService",
    methodName: "SendCompressed",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcCompressionTestGzip(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ id: "compress-test-001", compressed: true })));
  assert(response.metadata !== undefined && response.metadata !== null);
});