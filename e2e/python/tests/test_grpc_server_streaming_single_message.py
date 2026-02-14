import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_server_streaming_single_message() -> None:
    """Tests server streaming RPC that returns exactly one message. Verifies that single-message streams are properly handled and distinguished from unary responses.."""

    from app.main import handle_grpc_server_streaming_single_message

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="GetSingleMessage",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_single_message(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'"Stream completed with one message"'
    assert response.metadata is not None
