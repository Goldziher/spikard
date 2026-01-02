/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcUnimplementedStatus12, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC UNIMPLEMENTED status 12", async () => {
  // Tests UNIMPLEMENTED gRPC status code. Returned when the server does not implement the requested RPC method.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.UnimplService",
    methodName: "NotYetImplemented",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcUnimplementedStatus12(request);

  // Verify response
  assertEquals(response.statusCode, "UNIMPLEMENTED");
  assert(response.metadata !== undefined && response.metadata !== null);
});