import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_client_streaming_unicode_string_aggregation() -> None:
    """Tests client streaming RPC with Unicode strings that are concatenated. Validates proper UTF-8 handling across multiple messages.."""

    from app.main import handle_grpc_client_streaming_unicode_string_aggregation

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.TextService",
        method_name="ConcatenateStrings",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_unicode_string_aggregation(request)

    # Verify response
    assert (
        response.payload
        == b'{"fragment_id":"unicode-001","result":"Hello, \xe4\xb8\x96\xe7\x95\x8c! \xd0\x9f\xd1\x80\xd0\xb8\xd0\xb2\xd0\xb5\xd1\x82 \xf0\x9f\x8c\x8d","fragment_count":4,"total_length":26,"status":"CONCATENATED"}'
    )
    assert response.metadata is not None
