/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcFailedPreconditionStatus9, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC FAILED_PRECONDITION status 9", async () => {
  // Tests FAILED_PRECONDITION gRPC status code. Returned when the RPC failed because the system is not in the required state.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StateService",
    methodName: "Proceed",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcFailedPreconditionStatus9(request);

  // Verify response
  assertEquals(response.statusCode, "FAILED_PRECONDITION");
  assert(response.metadata !== undefined && response.metadata !== null);
});