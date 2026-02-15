import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_client_streaming_metadata_preserved_in_response() -> None:
    """Tests client streaming RPC where request metadata is forwarded to and preserved in the response. Validates metadata propagation through streaming pipeline.."""

    from app.main import handle_grpc_client_streaming_metadata_preserved_in_response

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "authorization": "Bearer token-xyz123",
        "x-trace-id": "trace-abc456",
        "content-type": "application/grpc",
        "x-user-id": "user-789",
        "custom-header": "custom-value",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.MetadataService",
        method_name="ProcessWithMetadata",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_metadata_preserved_in_response(request)

    # Verify response
    assert (
        response.payload
        == b'{"request_id":"req-meta-001","processed_by":"grpc-handler-1","received_user_id":"user-789","message_count":3,"trace_id":"trace-abc456","status":"COMPLETE_WITH_METADATA"}'
    )
    assert response.metadata is not None
