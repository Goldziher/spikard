/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcOauth2BearerTokenAuthentication, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: OAuth2 Bearer token authentication", async () => {
  // Tests OAuth2 Bearer token authentication. Validates token validation and scope checking.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
    "authorization": "Bearer ya29.a0AfH6SMBx...",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.OAuth2Service",
    methodName: "CheckScope",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcOauth2BearerTokenAuthentication(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ granted: true, token_info: "oauth2_token" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});