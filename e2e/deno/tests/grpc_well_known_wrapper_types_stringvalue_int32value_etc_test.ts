/**
 * E2E test for gRPC
 * @generated
 */

import { handleGrpcWellKnownWrapperTypesStringvalueInt32valueEtc, type GrpcRequest, type GrpcResponse } from "../app/main.ts";
import { assertEquals, assert } from "jsr:@std/assert@1";
import { Buffer } from "node:buffer";

Deno.test("grpc: should handle gRPC request: Well-known wrapper types StringValue Int32Value etc", async () => {
  // Tests usage of google.protobuf wrapper types (StringValue, Int32Value, BoolValue) for nullable scalar types. Validates proper null/present distinction.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.WrapperService",
    methodName: "ProcessWrapper",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcWellKnownWrapperTypesStringvalueInt32valueEtc(request);

  // Verify response
  assertEquals(response.statusCode, "OK");
  assertEquals(response.payload, Buffer.from(JSON.stringify({ id: "wrapper-test-001", name_present: true, name_value: "Test Name", count_present: true, count_value: 42 })));
  assert(response.metadata !== undefined && response.metadata !== null);
});