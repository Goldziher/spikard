import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_server_streaming_rpc() -> None:
    """Tests server streaming where the server sends multiple responses. Covers streaming response patterns.."""

    from app.main import handle_grpc_server_streaming_rpc

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ItemService",
        method_name="ListItems",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_rpc(request)

    # Verify response
    assert (
        response.payload
        == b'[{"id":101,"name":"Item 1","description":"First item in category","price":9.99},{"id":102,"name":"Item 2","description":"Second item in category","price":14.99},{"id":103,"name":"Item 3","description":"Third item in category","price":19.99}]'
    )
    assert response.metadata is not None
