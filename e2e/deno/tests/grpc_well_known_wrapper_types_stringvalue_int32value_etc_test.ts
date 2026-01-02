it("should handle gRPC request: Well-known wrapper types StringValue Int32Value etc", async () => {
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
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ id: "wrapper-test-001", name_present: true, name_value: "Test Name", count_present: true, count_value: 42 })));
  expect(response.metadata).toBeDefined();
});
