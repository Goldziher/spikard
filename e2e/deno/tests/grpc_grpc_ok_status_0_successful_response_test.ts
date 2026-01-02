/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcOkStatus0SuccessfulResponse, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC OK status 0 - successful response", async () => {
  // Tests successful gRPC response with OK status code. Validates basic request-response completion.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StatusService",
    methodName: "CheckStatus",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcOkStatus0SuccessfulResponse(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ request_id: "status-ok-001", status: "success" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});