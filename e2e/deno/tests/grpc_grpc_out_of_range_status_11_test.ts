/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcOutOfRangeStatus11, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC OUT_OF_RANGE status 11", async () => {
  // Tests OUT_OF_RANGE gRPC status code. Returned when a value is outside the acceptable range.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.RangeService",
    methodName: "Check",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcOutOfRangeStatus11(request);

  // Verify response
  assertEquals(response.statusCode, "OUT_OF_RANGE");
  assert(response.metadata !== undefined && response.metadata !== null);
});