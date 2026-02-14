@pytest.mark.asyncio
async def test_grpc_special_characters_unicode_and_emoji_in_strings() -> None:
    """Tests handling of unicode characters, emojis, and special characters in protobuf string fields. Validates proper UTF-8 encoding/decoding.."""

    from app.main import handle_grpc_special_characters_unicode_and_emoji_in_strings

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.EchoService",
        method_name="EchoSpecial",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_special_characters_unicode_and_emoji_in_strings(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b"{\"echo\":\"Hello 世界 Привет שלום مرحبا\"}"
    assert response.metadata is not None
