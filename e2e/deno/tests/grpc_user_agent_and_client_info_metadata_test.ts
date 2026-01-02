it("should handle gRPC request: User-Agent and client info metadata", async () => {
  // Tests User-Agent header handling and client identification. Validates proper user-agent parsing and logging.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
    "user-agent": "grpc-client/1.2.3 (linux; amd64)",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.ClientService",
    methodName: "Identify",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcUserAgentAndClientInfoMetadata(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ client_type: "grpc-client", client_version: "1.2.3" })));
  expect(response.metadata).toBeDefined();
});
