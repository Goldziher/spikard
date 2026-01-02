/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcCancelledStatus1, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC CANCELLED status 1", async () => {
  // Tests CANCELLED gRPC status code. Returned when the RPC was cancelled by the client or server.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.CancelService",
    methodName: "Operation",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcCancelledStatus1(request);

  // Verify response
  assertEquals(response.statusCode, "CANCELLED");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ id: "cancel-001" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});