@pytest.mark.asyncio
async def test_grpc_all_fields_set_to_zero_false_empty_values() -> None:
    """Tests proto3 default value behavior when all fields are explicitly set to zero, false, empty string. Validates that zero values are transmitted correctly.."""

    from app.main import handle_grpc_all_fields_set_to_zero_false_empty_values

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ZeroValueService",
        method_name="ProcessZeros",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_all_fields_set_to_zero_false_empty_values(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"success":true,"fields_received":5}'
    assert response.metadata is not None
