/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingPingPongPairs, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming - ping pong pairs", async () => {
  // Tests bidirectional streaming RPC with request-response pairs (ping-pong pattern).

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.PingService",
    methodName: "PingPong",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingPingPongPairs(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assert(response.metadata !== undefined && response.metadata !== null);
});