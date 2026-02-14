@pytest.mark.asyncio
async def test_grpc_client_streaming_unicode_string_aggregation() -> None:
    """Tests client streaming RPC with Unicode strings that are concatenated. Validates proper UTF-8 handling across multiple messages.."""

    from app.main import handle_grpc_client_streaming_unicode_string_aggregation

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.TextService",
        method_name="ConcatenateStrings",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_unicode_string_aggregation(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b"{\"fragment_id\":\"unicode-001\",\"result\":\"Hello, 世界! Привет 🌍\",\"fragment_count\":4,\"total_length\":26,\"status\":\"CONCATENATED\"}"
    assert response.metadata is not None
