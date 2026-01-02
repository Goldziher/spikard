it("should handle gRPC request: gRPC ALREADY_EXISTS status 6", async () => {
  // Tests ALREADY_EXISTS gRPC status code. Returned when trying to create a resource that already exists.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.CreateService",
    methodName: "Create",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcAlreadyExistsStatus6(request);

  // Verify response
  expect(response.statusCode).toBe("ALREADY_EXISTS");
  expect(response.metadata).toBeDefined();
});
