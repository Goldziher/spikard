it("should handle gRPC request: Oneof field handling", async () => {
  // Tests oneof fields where only one field in the group can be set at a time. Validates proper mutual exclusivity and serialization.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.OneofService",
    methodName: "ProcessOneof",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcOneofFieldHandling(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ received_type: "text_data", data_present: true })));
  expect(response.metadata).toBeDefined();
});
