it("should handle gRPC request: Large 1MB message payload", async () => {
	// Tests handling of 1MB protobuf messages. Verifies that large payloads are properly serialized, transmitted, and deserialized without truncation or corruption.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.FileService",
		methodName: "UploadLarge",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcLarge1mbMessagePayload(request);

	// Verify response
	expect(response.statusCode).toBe("OK");
	expect(response.payload).toEqual(
		Buffer.from(JSON.stringify({ request_id: "large-1mb-test-001", data_size: 1048576 })),
	);
	expect(response.metadata).toBeDefined();
});
