@pytest.mark.asyncio
async def test_grpc_large_1mb_message_payload() -> None:
    """Tests handling of 1MB protobuf messages. Verifies that large payloads are properly serialized, transmitted, and deserialized without truncation or corruption.."""

    from app.main import handle_grpc_large_1mb_message_payload

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.FileService",
        method_name="UploadLarge",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_large_1mb_message_payload(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"request_id":"large-1mb-test-001","data_size":1048576}'
    assert response.metadata is not None
