@pytest.mark.asyncio
async def test_grpc_client_streaming_rpc() -> None:
    """Tests client streaming where client sends multiple messages. Covers streaming request aggregation patterns.."""

    from app.main import handle_grpc_client_streaming_rpc

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.FileService",
        method_name="Upload",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_rpc(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"file_id":"file-12345","total_bytes":57,"status":"COMPLETED","checksum":"d8e8fca2dc0f896fd7cb4cb0031ba249"}'
    )
    assert response.metadata is not None
