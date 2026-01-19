/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcServerStreamingTimeoutScenario, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Server streaming - timeout scenario", async () => {
  // Tests server streaming RPC that exceeds the deadline/timeout. The server starts streaming but doesn't complete before the client-imposed timeout expires. Validates proper timeout handling and stream cancellation.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
    "grpc-timeout": "1000m",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StreamService",
    methodName: "StreamWithDelay",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcServerStreamingTimeoutScenario(request);

  // Verify response
  assertEquals(response.statusCode, "DEADLINE_EXCEEDED");
  assert(response.metadata !== undefined && response.metadata !== null);
});