/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcGrpcMetadataHeaders, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: gRPC metadata headers", async () => {
  // Tests gRPC metadata handling for request/response headers including authorization, tracing IDs, and custom headers.

  const metadata: Record<string, string> = {
    "x-custom-header": "custom-value",
    "content-type": "application/grpc",
    "authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
    "x-trace-id": "trace-abc123def456",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.MetadataService",
    methodName: "CheckMetadata",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcMetadataHeaders(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ request_id: "req-987654321", received_auth_header: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9", received_trace_id: "trace-abc123def456", received_custom_header: "custom-value", response_time_ms: 45 })));
  assert(response.metadata !== undefined && response.metadata !== null);
});