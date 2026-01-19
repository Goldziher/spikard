/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcCustomAuthenticationSchemeHeader, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Custom authentication scheme header", async () => {
  // Tests custom authentication header scheme. Validates that custom auth headers are properly extracted and validated.

  const metadata: Record<string, string> = {
    "x-custom-auth": "CustomScheme token_value_123",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.CustomAuthService",
    methodName: "Execute",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcCustomAuthenticationSchemeHeader(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ success: true })));
  assert(response.metadata !== undefined && response.metadata !== null);
});