/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcUnavailableStatus14ServiceUnavailable, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC UNAVAILABLE status 14 - service unavailable", async () => {
  // Tests UNAVAILABLE gRPC status code. Returned when the service is temporarily unavailable.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.UnavailService",
    methodName: "Request",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcUnavailableStatus14ServiceUnavailable(request);

  // Verify response
  assertEquals(response.statusCode, "UNAVAILABLE");
  assert(response.metadata !== undefined && response.metadata !== null);
});