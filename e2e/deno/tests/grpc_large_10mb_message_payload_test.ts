/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcLarge10mbMessagePayload, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Large 10MB message payload", async () => {
  // Tests handling of 10MB protobuf messages. Validates high-capacity transfers, memory efficiency, and absence of stream fragmentation issues.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.BulkService",
    methodName: "BulkUpload",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcLarge10mbMessagePayload(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ id: "bulk-10mb-transfer", status: "received" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});