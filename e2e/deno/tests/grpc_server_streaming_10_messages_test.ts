/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreaming10Messages, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - 10 messages", async () => {
  // Tests server streaming RPC that returns a normal stream of 10 messages. Validates message ordering and complete stream delivery.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "ListItems",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreaming10Messages(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify("10 messages streamed successfully")));
  assert(response.metadata !== undefined && response.metadata !== null);
});