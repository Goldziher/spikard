/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingMessageSizeLimitExceeded, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming - message size limit exceeded", async () => {
  // Tests client streaming RPC where one message exceeds the max_message_size limit. Server rejects the oversized message and terminates the stream.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
    "grpc-max-message-size": "4096",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.PayloadService",
    methodName: "ProcessPayloads",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingMessageSizeLimitExceeded(request);

  // Verify response
  assertEquals(response.statusCode, "RESOURCE_EXHAUSTED");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ message_id: "payload-002", processed_count: 1, status: "FAILED", error_detail: "Message payload size 10240 exceeds maximum allowed size 4096" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});