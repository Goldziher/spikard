import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_server_streaming_nested_object_messages() -> None:
    """Tests server streaming RPC with complex nested message structures. Validates proper serialization and deserialization of deeply nested protobuf objects in streaming context.."""

    from app.main import handle_grpc_server_streaming_nested_object_messages

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="StreamPeople",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_nested_object_messages(request)

    # Verify response
    assert response.payload == b'"3 people with nested objects streamed successfully"'
    assert response.metadata is not None
