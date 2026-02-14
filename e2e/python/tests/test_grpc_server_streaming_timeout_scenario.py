import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_server_streaming_timeout_scenario() -> None:
    """Tests server streaming RPC that exceeds the deadline/timeout. The server starts streaming but doesn't complete before the client-imposed timeout expires. Validates proper timeout handling and stream cancellation.."""

    from app.main import handle_grpc_server_streaming_timeout_scenario

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "grpc-timeout": "1000m",
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="StreamWithDelay",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_timeout_scenario(request)

    # Verify response
    assert response.status_code == "DEADLINE_EXCEEDED"
    assert response.metadata is not None
