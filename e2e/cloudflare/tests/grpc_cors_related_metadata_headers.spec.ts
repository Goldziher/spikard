it("should handle gRPC request: CORS-related metadata headers", async () => {
	// Tests CORS-related metadata in gRPC calls. Validates origin validation and cross-origin request handling.

	const metadata: Record<string, string> = {
		"access-control-request-method": "POST",
		"content-type": "application/grpc",
		origin: "https://example.com",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.CorsService",
		methodName: "CheckCors",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcCorsRelatedMetadataHeaders(request);

	// Verify response
	expect(response.statusCode).toBe("OK");
	expect(response.payload).toEqual(Buffer.from(JSON.stringify({ allowed: true })));
	expect(response.metadata).toBeDefined();
});
