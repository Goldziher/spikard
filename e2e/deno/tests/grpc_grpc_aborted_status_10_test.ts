/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcAbortedStatus10, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC ABORTED status 10", async () => {
  // Tests ABORTED gRPC status code. Returned when an operation was aborted, typically due to a concurrency issue like conflict.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.TransactionService",
    methodName: "Commit",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcAbortedStatus10(request);

  // Verify response
  assertEquals(response.statusCode, "ABORTED");
  assert(response.metadata !== undefined && response.metadata !== null);
});