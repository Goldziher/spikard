/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreamingSingleMessage, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - single message", async () => {
  // Tests server streaming RPC that returns exactly one message. Verifies that single-message streams are properly handled and distinguished from unary responses.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "GetSingleMessage",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingSingleMessage(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify("Stream completed with one message")));
  assert(response.metadata !== undefined && response.metadata !== null);
});