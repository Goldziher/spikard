/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreamingNestedObjectMessages, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - nested object messages", async () => {
  // Tests server streaming RPC with complex nested message structures. Validates proper serialization and deserialization of deeply nested protobuf objects in streaming context.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "StreamPeople",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingNestedObjectMessages(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify("3 people with nested objects streamed successfully")));
  assert(response.metadata !== undefined && response.metadata !== null);
});