/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcErrorHandlingDeadlineExceeded, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Error handling - deadline exceeded", async () => {
  // Tests server streaming RPC that exceeds deadline. Expects DEADLINE_EXCEEDED status when RPC time exceeds configured timeout.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ErrorTestService",
    methodName: "SlowStream",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingDeadlineExceeded(request);

  // Verify response
  assertEquals(response.statusCode, "DEADLINE_EXCEEDED");
  assert(response.metadata !== undefined && response.metadata !== null);
});