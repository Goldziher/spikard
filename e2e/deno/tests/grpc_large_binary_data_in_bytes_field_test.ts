/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcLargeBinaryDataInBytesField, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Large binary data in bytes field", async () => {
  // Tests handling of large binary data in protobuf bytes fields. Validates proper base64 encoding/decoding and preservation of binary integrity.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.BinaryService",
    methodName: "UploadBinary",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcLargeBinaryDataInBytesField(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ file_id: "binary-large-001", bytes_received: 512000 })));
  assert(response.metadata !== undefined && response.metadata !== null);
});