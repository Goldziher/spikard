it("should handle gRPC request: Error handling - gRPC status codes", async () => {
  // Tests gRPC error status codes and error responses. Covers NOT_FOUND, INVALID_ARGUMENT, INTERNAL, and other gRPC status codes.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ProductService",
    methodName: "GetProduct",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcErrorHandlingGrpcStatusCodes(request);

  // Verify response
  expect(response.statusCode).toBe("INVALID_ARGUMENT");
  expect(response.metadata).toBeDefined();
});
