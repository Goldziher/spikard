it("should handle gRPC request: gRPC RESOURCE_EXHAUSTED status 8", async () => {
  // Tests RESOURCE_EXHAUSTED gRPC status code. Returned when the server has run out of resources (disk space, memory, connections, etc.).

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ResourceService",
    methodName: "AllocateMemory",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcResourceExhaustedStatus8(request);

  // Verify response
  expect(response.statusCode).toBe("RESOURCE_EXHAUSTED");
  expect(response.metadata).toBeDefined();
});
