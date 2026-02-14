@pytest.mark.asyncio
async def test_grpc_empty_message_request_and_response() -> None:
    """Tests handling of empty protobuf messages with no fields. Validates that the protocol correctly handles minimal payloads.."""

    from app.main import handle_grpc_empty_message_request_and_response

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.PingService",
        method_name="Ping",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_empty_message_request_and_response(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b"{}"
    assert response.metadata is not None
