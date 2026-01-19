/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreamingRapid100MessageStream, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - rapid 100 message stream", async () => {
  // Tests server streaming RPC with 100 messages sent in rapid succession. Validates backpressure handling, buffering, and delivery of high-volume message streams without loss or corruption.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "StreamRapidMessages",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingRapid100MessageStream(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify("100 messages streamed successfully in sequence")));
  assert(response.metadata !== undefined && response.metadata !== null);
});