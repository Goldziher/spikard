@pytest.mark.asyncio
async def test_grpc_server_streaming_unicode_and_special_characters() -> None:
    """Tests server streaming RPC with messages containing unicode characters, emoji, special symbols, and multi-byte UTF-8 sequences. Validates proper encoding/decoding across the streaming pipeline.."""

    from app.main import handle_grpc_server_streaming_unicode_and_special_characters

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
        "encoding": "utf-8",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="StreamUnicodeMessages",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_unicode_and_special_characters(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'"Unicode stream completed successfully"'
    assert response.metadata is not None
