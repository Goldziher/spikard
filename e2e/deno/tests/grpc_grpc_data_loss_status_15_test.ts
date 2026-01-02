/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcDataLossStatus15, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC DATA_LOSS status 15", async () => {
  // Tests DATA_LOSS gRPC status code. Returned when unrecoverable data loss or corruption occurred.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.DataService",
    methodName: "Process",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcDataLossStatus15(request);

  // Verify response
  assertEquals(response.statusCode, "DATA_LOSS");
  assert(response.metadata !== undefined && response.metadata !== null);
});