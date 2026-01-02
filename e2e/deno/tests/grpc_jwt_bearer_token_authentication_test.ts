it("should handle gRPC request: JWT Bearer token authentication", async () => {
  // Tests JWT authentication via gRPC metadata. Validates that JWT tokens are properly extracted and validated from authorization header.

  const metadata: Record<string, string> = {
    "content-type": "application/grpc",
    "authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLTEyMyIsImlhdCI6MTUxNjIzOTAyMn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
  };
  const request: GrpcRequest = {
    serviceName: "example.v1.AuthService",
    methodName: "SecureAction",
    payload: Buffer.from(JSON.stringify({})),
    metadata,
  };

  const response = await handleGrpcJwtBearerTokenAuthentication(request);

  // Verify response
  expect(response.statusCode).toBe("OK");
  expect(response.payload).toEqual(Buffer.from(JSON.stringify({ user_id: "user-123", action: "read" })));
  expect(response.metadata).toBeDefined();
});
