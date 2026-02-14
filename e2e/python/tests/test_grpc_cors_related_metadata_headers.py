import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_cors_related_metadata_headers() -> None:
    """Tests CORS-related metadata in gRPC calls. Validates origin validation and cross-origin request handling.."""

    from app.main import handle_grpc_cors_related_metadata_headers

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
        "origin": "https://example.com",
        "access-control-request-method": "POST",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.CorsService",
        method_name="CheckCors",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_cors_related_metadata_headers(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"allowed":true}'
    assert response.metadata is not None
