/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreamingMidStreamError, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - mid-stream error", async () => {
  // Tests server streaming RPC that sends 5 messages successfully, then encounters an error before completing the stream. Validates partial stream delivery and error handling.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "StreamData",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingMidStreamError(request);

  // Verify response
  assertEquals(response.statusCode, "INTERNAL");
  assert(response.metadata !== undefined && response.metadata !== null);
});