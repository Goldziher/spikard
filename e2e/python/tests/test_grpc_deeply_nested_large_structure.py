@pytest.mark.asyncio
async def test_grpc_deeply_nested_large_structure() -> None:
    """Tests deeply nested protobuf messages with complex hierarchies. Validates that nested message serialization handles proper field numbering and recursive structures.."""

    from app.main import handle_grpc_deeply_nested_large_structure

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.RegistryService",
        method_name="RegisterPerson",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_deeply_nested_large_structure(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"success":true,"person":{"name":"John Doe","address":{"street":"123 Main St","city":"Springfield"}}}'
    )
    assert response.metadata is not None
