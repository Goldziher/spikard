@pytest.mark.asyncio
async def test_grpc_grpc_compression_test_gzip() -> None:
    """Tests gRPC payload compression using gzip. Validates that compressed messages are properly decompressed and that header metadata indicates compression.."""

    from app.main import handle_grpc_grpc_compression_test_gzip

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "grpc-encoding": "gzip",
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.CompressionService",
        method_name="SendCompressed",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_compression_test_gzip(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"id":"compress-test-001","compressed":true}'
    assert response.metadata is not None
