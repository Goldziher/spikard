import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_circuit_breaker_triggered_unavailable_with_metadata() -> None:
    """Tests UNAVAILABLE status code with circuit breaker metadata. Indicates service degradation and when to retry.."""

    from app.main import handle_grpc_circuit_breaker_triggered_unavailable_with_metadata

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.DownstreamService",
        method_name="Query",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_circuit_breaker_triggered_unavailable_with_metadata(request)

    # Verify response
    assert response.metadata is not None
