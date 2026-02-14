import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_server_streaming_mid_stream_error() -> None:
    """Tests server streaming RPC that sends 5 messages successfully, then encounters an error before completing the stream. Validates partial stream delivery and error handling.."""

    from app.main import handle_grpc_server_streaming_mid_stream_error

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="StreamData",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_mid_stream_error(request)

    # Verify response
    assert (
        response.payload
        == b'[{"sequence":1,"payload":"Message 1"},{"sequence":2,"payload":"Message 2"},{"sequence":3,"payload":"Message 3"},{"sequence":4,"payload":"Message 4"},{"sequence":5,"payload":"Message 5"}]'
    )
    assert response.metadata is not None
