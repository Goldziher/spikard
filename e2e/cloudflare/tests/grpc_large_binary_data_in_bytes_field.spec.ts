it("should handle gRPC request: Large binary data in bytes field", async () => {
	// Tests handling of large binary data in protobuf bytes fields. Validates proper base64 encoding/decoding and preservation of binary integrity.

	const metadata: Record<string, string> = {
		"content-type": "application/grpc",
	};
	const request: GrpcRequest = {
		serviceName: "example.v1.BinaryService",
		methodName: "UploadBinary",
		payload: Buffer.from(JSON.stringify({})),
		metadata,
	};

	const response = await handleGrpcLargeBinaryDataInBytesField(request);

	// Verify response
	expect(response.statusCode).toBe("OK");
	expect(response.payload).toEqual(
		Buffer.from(JSON.stringify({ file_id: "binary-large-001", bytes_received: 512000 })),
	);
	expect(response.metadata).toBeDefined();
});
