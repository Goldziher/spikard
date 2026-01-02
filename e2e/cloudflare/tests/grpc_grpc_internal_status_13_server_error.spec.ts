it("should handle gRPC request: gRPC INTERNAL status 13 - server error", async () => {
	// Tests INTERNAL gRPC status code. Returned when an internal server error occurs.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.InternalService",
		methodName: "Fail",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcGrpcInternalStatus13ServerError(request);

	// Verify response
	expect(response.statusCode).toBe("INTERNAL");
	expect(response.metadata).toBeDefined();
});
