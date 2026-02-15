import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_client_streaming_single_message_aggregation() -> None:
    """Tests client streaming RPC where client sends a single message. Server acknowledges and returns aggregated result.."""

    from app.main import handle_grpc_client_streaming_single_message_aggregation

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.AggregateService",
        method_name="AggregateData",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_single_message_aggregation(request)

    # Verify response
    assert response.payload == b'{"count":1,"total":42,"average":42.0,"status":"AGGREGATED"}'
    assert response.metadata is not None
