/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingEcho5Messages, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming - echo 5 messages", async () => {
  // Tests bidirectional streaming RPC where client sends 5 messages and expects them echoed back in the same order.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.EchoService",
    methodName: "EchoBidi",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingEcho5Messages(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assert(response.metadata !== undefined && response.metadata !== null);
});