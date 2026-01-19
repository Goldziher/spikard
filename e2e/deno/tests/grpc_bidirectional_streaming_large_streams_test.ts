/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingLargeStreams, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming - large streams", async () => {
  // Tests bidirectional streaming RPC with 50+ messages in both directions.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.LargeStreamService",
    methodName: "ProcessLarge",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingLargeStreams(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assert(response.metadata !== undefined && response.metadata !== null);
});