/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreamingEmptyStream, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - empty stream", async () => {
  // Tests server streaming RPC that returns an empty stream. The server opens the stream but sends no messages before completing successfully.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "GetEmptyStream",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingEmptyStream(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify("Stream completed with no messages")));
  assert(response.metadata !== undefined && response.metadata !== null);
});