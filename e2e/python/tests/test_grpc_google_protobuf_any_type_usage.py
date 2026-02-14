import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_google_protobuf_any_type_usage() -> None:
    """Tests usage of google.protobuf.Any for storing arbitrary message types. Validates type URL encoding and message packing.."""

    from app.main import handle_grpc_google_protobuf_any_type_usage

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.AnyService",
        method_name="ProcessAny",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_google_protobuf_any_type_usage(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"request_id":"any-test-001","type_name":"example.v1.Container","success":true}'
    assert response.metadata is not None
