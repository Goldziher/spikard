/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcCorsRelatedMetadataHeaders, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: CORS-related metadata headers", async () => {
  // Tests CORS-related metadata in gRPC calls. Validates origin validation and cross-origin request handling.

  const metadata: Record<string, string> = {
    "access-control-request-method": "POST",
    "origin": "https://example.com",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.CorsService",
    methodName: "CheckCors",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcCorsRelatedMetadataHeaders(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ allowed: true })));
  assert(response.metadata !== undefined && response.metadata !== null);
});