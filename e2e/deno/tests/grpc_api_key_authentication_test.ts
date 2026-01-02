/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcApiKeyAuthentication, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: API key authentication", async () => {
  // Tests API key authentication via gRPC metadata. Validates that API keys are properly validated and associated with clients.

  const metadata: Record<string, string> = {
    "x-api-key": "sk_live_abc123def456",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ApiService",
    methodName: "FetchResource",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcApiKeyAuthentication(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ data: "resource_data", client_id: "client-api-001" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});