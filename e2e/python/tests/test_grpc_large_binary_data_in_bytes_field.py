@pytest.mark.asyncio
async def test_grpc_large_binary_data_in_bytes_field() -> None:
    """Tests handling of large binary data in protobuf bytes fields. Validates proper base64 encoding/decoding and preservation of binary integrity.."""

    from app.main import handle_grpc_large_binary_data_in_bytes_field

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.BinaryService",
        method_name="UploadBinary",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_large_binary_data_in_bytes_field(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"file_id":"binary-large-001","bytes_received":512000}'
    assert response.metadata is not None
