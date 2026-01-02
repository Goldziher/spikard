/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcDeadlineExceededStatus4, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC DEADLINE_EXCEEDED status 4", async () => {
  // Tests DEADLINE_EXCEEDED gRPC status code. Returned when the RPC does not complete within the specified time limit.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.TimeoutService",
    methodName: "SlowOp",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcDeadlineExceededStatus4(request);

  // Verify response
  assertEquals(response.statusCode, "DEADLINE_EXCEEDED");
  assert(response.metadata !== undefined && response.metadata !== null);
});