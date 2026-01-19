/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingBothStreamsEmpty, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming - both streams empty", async () => {
  // Tests bidirectional streaming RPC where both request and response streams are empty.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.EmptyBothService",
    methodName: "Empty",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingBothStreamsEmpty(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assert(response.metadata !== undefined && response.metadata !== null);
});