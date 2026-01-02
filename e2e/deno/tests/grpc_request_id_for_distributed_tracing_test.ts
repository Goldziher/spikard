/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcRequestIdForDistributedTracing, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Request ID for distributed tracing", async () => {
  // Tests request ID header propagation for distributed tracing. Validates X-Request-ID generation and propagation.

  const metadata: Record<string, string> = {
    "x-request-id": "req-12345-67890",
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.TracingService",
    methodName: "Trace",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcRequestIdForDistributedTracing(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ request_id: "req-12345-67890" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});