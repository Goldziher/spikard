import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_map_field_handling_map_string_message() -> None:
    """Tests protobuf map fields with string keys and message values. Validates proper key-value pair serialization and access patterns.."""

    from app.main import handle_grpc_map_field_handling_map_string_message

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.MapService",
        method_name="ProcessMap",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_map_field_handling_map_string_message(request)

    # Verify response
    assert response.payload == b'{"id":"map-test-001","map_count":3,"keys":["key1","key2","key3"]}'
    assert response.metadata is not None
