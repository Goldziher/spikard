import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_metadata_headers() -> None:
    """Tests gRPC metadata handling for request/response headers including authorization, tracing IDs, and custom headers.."""

    from app.main import handle_grpc_grpc_metadata_headers

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
        "authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
        "x-custom-header": "custom-value",
        "x-trace-id": "trace-abc123def456",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.MetadataService",
        method_name="CheckMetadata",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_metadata_headers(request)

    # Verify response
    assert (
        response.payload
        == b'{"request_id":"req-987654321","received_auth_header":"Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9","received_trace_id":"trace-abc123def456","received_custom_header":"custom-value","response_time_ms":45}'
    )
    assert response.metadata is not None
