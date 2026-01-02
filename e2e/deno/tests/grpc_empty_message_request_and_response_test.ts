it("should handle gRPC request: Empty message request and response", async () => {
  // Tests handling of empty protobuf messages with no fields. Validates that the protocol correctly handles minimal payloads.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.PingService",
    methodName: "Ping",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcEmptyMessageRequestAndResponse(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({  })));
  expect(response.metadata).toBeDefined();
});
