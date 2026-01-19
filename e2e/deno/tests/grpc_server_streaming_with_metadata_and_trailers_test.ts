/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreamingWithMetadataAndTrailers, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - with metadata and trailers", async () => {
  // Tests server streaming RPC with gRPC metadata headers and trailers. Validates that metadata is accessible before streaming begins and trailers are delivered after stream completion.

  const metadata: Record<string, string> = {
    "x-client-version": "1.0.0",
    "content-type": "application/grpc",
    "x-request-id": "metadata-stream-001",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "StreamWithMetadata",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingWithMetadataAndTrailers(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify("Stream completed with metadata and trailers")));
  assert(response.metadata !== undefined && response.metadata !== null);
});