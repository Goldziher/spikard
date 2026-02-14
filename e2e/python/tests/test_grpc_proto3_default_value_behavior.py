@pytest.mark.asyncio
async def test_grpc_proto3_default_value_behavior() -> None:
    """Tests how proto3 handles implicit default values. When fields are omitted from the request, response should reflect appropriate defaults.."""

    from app.main import handle_grpc_proto3_default_value_behavior

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.DefaultService",
        method_name="CheckDefaults",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_proto3_default_value_behavior(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"id":1,"name":"","active":false,"has_id":true}'
    assert response.metadata is not None
