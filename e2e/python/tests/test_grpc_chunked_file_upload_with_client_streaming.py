@pytest.mark.asyncio
async def test_grpc_chunked_file_upload_with_client_streaming() -> None:
    """Tests client streaming RPC for chunked file uploads. Validates that multiple message chunks are properly accumulated and processed by the server.."""

    from app.main import handle_grpc_chunked_file_upload_with_client_streaming

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StorageService",
        method_name="ChunkedUpload",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_chunked_file_upload_with_client_streaming(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"file_id":"chunked-upload-test","total_chunks":5,"total_size":102400,"upload_status":"completed"}'
    )
    assert response.metadata is not None
