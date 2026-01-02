it("should handle gRPC request: OAuth2 Bearer token authentication", async () => {
	// Tests OAuth2 Bearer token authentication. Validates token validation and scope checking.

	const metadata: Record<string, string> = {
		authorization: "Bearer ya29.a0AfH6SMBx...",
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.OAuth2Service",
		methodName: "CheckScope",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcOauth2BearerTokenAuthentication(request);

	// Verify response
	expect(response.statusCode).toBe("OK");
	expect(response.payload).toEqual(Buffer.from(JSON.stringify({ granted: true, token_info: "oauth2_token" })));
	expect(response.metadata).toBeDefined();
});
