/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreamingWithLargeResponseData, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming with large response data", async () => {
  // Tests server streaming RPC that yields multiple large messages. Validates proper streaming protocol handling and backpressure management.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamingService",
    methodName: "StreamLargeData",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingWithLargeResponseData(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ stream_id: "stream-large-001", chunk_number: 1, is_final: false })));
  assert(response.metadata !== undefined && response.metadata !== null);
});