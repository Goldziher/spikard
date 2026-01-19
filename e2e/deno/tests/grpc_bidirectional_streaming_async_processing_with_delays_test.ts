/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingAsyncProcessingWithDelays, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming - async processing with delays", async () => {
  // Tests bidirectional streaming RPC with asynchronous message processing.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.AsyncService",
    methodName: "ProcessAsync",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingAsyncProcessingWithDelays(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assert(response.metadata !== undefined && response.metadata !== null);
});