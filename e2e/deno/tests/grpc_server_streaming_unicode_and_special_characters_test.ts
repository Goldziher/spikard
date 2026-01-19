/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreamingUnicodeAndSpecialCharacters, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - unicode and special characters", async () => {
  // Tests server streaming RPC with messages containing unicode characters, emoji, special symbols, and multi-byte UTF-8 sequences. Validates proper encoding/decoding across the streaming pipeline.

  const metadata: Record<string, string> = {
    "encoding": "utf-8",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "StreamUnicodeMessages",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingUnicodeAndSpecialCharacters(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify("Unicode stream completed successfully")));
  assert(response.metadata !== undefined && response.metadata !== null);
});