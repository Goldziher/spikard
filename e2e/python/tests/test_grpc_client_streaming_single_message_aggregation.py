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
    assert response.payload == b'{"average":42.0,"count":1,"status":"AGGREGATED","total":42}'
    assert response.metadata is not None
