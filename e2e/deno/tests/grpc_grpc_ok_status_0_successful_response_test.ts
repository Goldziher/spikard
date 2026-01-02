it("should handle gRPC request: gRPC OK status 0 - successful response", async () => {
  // Tests successful gRPC response with OK status code. Validates basic request-response completion.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.StatusService",
    methodName: "CheckStatus",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcGrpcOkStatus0SuccessfulResponse(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ request_id: "status-ok-001", status: "success" })));
  expect(response.metadata).toBeDefined();
});
