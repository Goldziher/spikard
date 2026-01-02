/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcEnumTypes, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Enum types", async () => {
  // Tests enum definitions and serialization. Covers enum fields with named constants.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.OrderService",
    methodName: "CreateOrder",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcEnumTypes(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ id: 1001, product_name: "Laptop", quantity: 2, status: "PENDING", priority: "HIGH" })));
  assert(response.metadata !== undefined && response.metadata !== null);
});