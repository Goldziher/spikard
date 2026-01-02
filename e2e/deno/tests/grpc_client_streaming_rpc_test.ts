/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcClientStreamingRpc, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Client streaming RPC", async () => {
  // Tests client streaming where client sends multiple messages. Covers streaming request aggregation patterns.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.FileService",
    methodName: "Upload",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcClientStreamingRpc(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ file_id: "file-12345", total_bytes: 57, status: "COMPLETED", checksum: "d8e8fca2dc0f896fd7cb4cb0031ba249" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});