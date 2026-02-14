import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_well_known_wrapper_types_stringvalue_int32value_etc() -> None:
    """Tests usage of google.protobuf wrapper types (StringValue, Int32Value, BoolValue) for nullable scalar types. Validates proper null/present distinction.."""

    from app.main import handle_grpc_well_known_wrapper_types_stringvalue_int32value_etc

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.WrapperService",
        method_name="ProcessWrapper",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_well_known_wrapper_types_stringvalue_int32value_etc(request)

    # Verify response
    assert response.status_code == "OK"
    assert (
        response.payload
        == b'{"id":"wrapper-test-001","name_present":true,"name_value":"Test Name","count_present":true,"count_value":42}'
    )
    assert response.metadata is not None
