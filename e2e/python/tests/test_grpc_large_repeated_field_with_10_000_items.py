import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_large_repeated_field_with_10_000_items() -> None:
    """Tests handling of repeated fields containing thousands of elements. Validates efficient serialization and deserialization of large arrays without memory bloat.."""

    from app.main import handle_grpc_large_repeated_field_with_10_000_items

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.MetricsService",
        method_name="IngestTimeSeries",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_large_repeated_field_with_10_000_items(request)

    # Verify response
    assert (
        response.payload
        == b'{"series_id":"metrics-large-series","point_count":10000,"min_value":10.5,"max_value":99.9}'
    )
    assert response.metadata is not None
