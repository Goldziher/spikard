/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingMetadataPreservedInResponse, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming - metadata preserved in response", async () => {
  // Tests client streaming RPC where request metadata is forwarded to and preserved in the response. Validates metadata propagation through streaming pipeline.

  const metadata: Record<string, string> = {
    "custom-header": "custom-value",
    "x-user-id": "user-789",
    "content-type": "application/grpc",
    "x-trace-id": "trace-abc456",
    "authorization": "Bearer token-xyz123",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.MetadataService",
    methodName: "ProcessWithMetadata",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingMetadataPreservedInResponse(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ request_id: "req-meta-001", processed_by: "grpc-handler-1", received_user_id: "user-789", message_count: 3, trace_id: "trace-abc456", status: "COMPLETE_WITH_METADATA" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});