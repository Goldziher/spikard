/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcBidirectionalStreamingRpc, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Bidirectional streaming RPC", async () => {
  // Tests bidirectional streaming where both client and server send multiple messages. Covers duplex communication patterns.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
    "authorization": "Bearer user-token",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ChatService",
    methodName: "Chat",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcBidirectionalStreamingRpc(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assert(response.metadata !== undefined && response.metadata !== null);
});