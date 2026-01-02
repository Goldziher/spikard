/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingWithLargeBatchRequests, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming with large batch requests", async () => {
  // Tests client streaming RPC with large batch requests. Validates server accumulation of multiple large client messages.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.BatchService",
    methodName: "ProcessBatch",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingWithLargeBatchRequests(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ batch_id: "batch-large-001", items_processed: 100, total_bytes: 5242880 })));
  assert(response.metadata !== undefined && response.metadata !== null);
});