/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreaming1mbMessages, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - 1MB messages", async () => {
  // Tests server streaming RPC with large message payloads (approximately 1MB each). Validates that the streaming framework can handle large individual messages without truncation or memory issues.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "StreamLargeMessages",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreaming1mbMessages(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify("3 large messages streamed successfully")));
  assert(response.metadata !== undefined && response.metadata !== null);
});