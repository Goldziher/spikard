import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_echo_5_messages() -> None:
    """Tests bidirectional streaming RPC where client sends 5 messages and expects them echoed back in the same order.."""

    from app.main import handle_grpc_bidirectional_streaming_echo_5_messages

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.EchoService",
        method_name="EchoBidi",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_echo_5_messages(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.metadata is not None
