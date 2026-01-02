/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcUnknownStatus2, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC UNKNOWN status 2", async () => {
  // Tests UNKNOWN gRPC status code. Used for errors that do not fit any other status code.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.UnknownService",
    methodName: "Fail",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcUnknownStatus2(request);

  // Verify response
  assertEquals(response.statusCode, "UNKNOWN");
  assert(response.metadata !== undefined && response.metadata !== null);
});