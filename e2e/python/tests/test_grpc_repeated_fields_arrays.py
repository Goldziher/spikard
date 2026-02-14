@pytest.mark.asyncio
async def test_grpc_repeated_fields_arrays() -> None:
    """Tests arrays/repeated fields for primitive types and messages. Covers repeated field serialization and deserialization.."""

    from app.main import handle_grpc_repeated_fields_arrays

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.BlogService",
        method_name="CreatePost",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_repeated_fields_arrays(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"id":789,"title":"Getting Started with gRPC","content":"This is a comprehensive guide to gRPC...","tags":[{"id":1,"name":"gRPC"},{"id":2,"name":"Protocol Buffers"},{"id":3,"name":"RPC"}],"categories":["tutorial","programming","networking"]}'
    )
    assert response.metadata is not None
