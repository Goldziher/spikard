import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_request_id_for_distributed_tracing() -> None:
    """Tests request ID header propagation for distributed tracing. Validates X-Request-ID generation and propagation.."""

    from app.main import handle_grpc_request_id_for_distributed_tracing

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
        "x-request-id": "req-12345-67890",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.TracingService",
        method_name="Trace",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_request_id_for_distributed_tracing(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"request_id":"req-12345-67890"}'
    assert response.metadata is not None
