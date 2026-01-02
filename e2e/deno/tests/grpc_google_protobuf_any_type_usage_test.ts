it("should handle gRPC request: Google protobuf Any type usage", async () => {
  // Tests usage of google.protobuf.Any for storing arbitrary message types. Validates type URL encoding and message packing.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.AnyService",
    methodName: "ProcessAny",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGoogleProtobufAnyTypeUsage(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ request_id: "any-test-001", type_name: "example.v1.Container", success: true })));
  expect(response.metadata).toBeDefined();
});
