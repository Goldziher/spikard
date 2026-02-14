import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_client_streaming_rapid_high_frequency_messages() -> None:
    """Tests client streaming RPC with rapid-fire message delivery. Server handles 50 messages in quick succession and returns aggregated metrics.."""

    from app.main import handle_grpc_client_streaming_rapid_high_frequency_messages

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.MetricsService",
        method_name="ProcessEvents",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_rapid_high_frequency_messages(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"event_id":"rapid-batch-001","event_count":50,"min_value":0.1,"max_value":5.0,"avg_value":2.55,"throughput_mps":500.0,"status":"PROCESSED"}'
    )
    assert response.metadata is not None
