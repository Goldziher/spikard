it("should handle gRPC request: gRPC UNIMPLEMENTED status 12", async () => {
  // Tests UNIMPLEMENTED gRPC status code. Returned when the server does not implement the requested RPC method.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.UnimplService",
    methodName: "NotYetImplemented",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcUnimplementedStatus12(request);

  // Verify response
  expect(response.statusCode).toBe("UNIMPLEMENTED");
  expect(response.metadata).toBeDefined();
});
