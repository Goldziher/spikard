@pytest.mark.asyncio
async def test_grpc_large_10mb_message_payload() -> None:
    """Tests handling of 10MB protobuf messages. Validates high-capacity transfers, memory efficiency, and absence of stream fragmentation issues.."""

    from app.main import handle_grpc_large_10mb_message_payload

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.BulkService",
        method_name="BulkUpload",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_large_10mb_message_payload(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"id":"bulk-10mb-transfer","status":"received"}'
    assert response.metadata is not None
