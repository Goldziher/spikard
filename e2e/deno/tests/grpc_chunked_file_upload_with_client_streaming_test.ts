/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcChunkedFileUploadWithClientStreaming, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Chunked file upload with client streaming", async () => {
  // Tests client streaming RPC for chunked file uploads. Validates that multiple message chunks are properly accumulated and processed by the server.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StorageService",
    methodName: "ChunkedUpload",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcChunkedFileUploadWithClientStreaming(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ file_id: "chunked-upload-test", total_chunks: 5, total_size: 102400, upload_status: "completed" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});