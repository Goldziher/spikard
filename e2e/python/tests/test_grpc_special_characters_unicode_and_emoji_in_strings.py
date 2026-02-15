import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_special_characters_unicode_and_emoji_in_strings() -> None:
    """Tests handling of unicode characters, emojis, and special characters in protobuf string fields. Validates proper UTF-8 encoding/decoding.."""

    from app.main import handle_grpc_special_characters_unicode_and_emoji_in_strings

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.EchoService",
        method_name="EchoSpecial",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_special_characters_unicode_and_emoji_in_strings(request)

    # Verify response
    assert (
        response.payload
        == b'{"echo":"Hello \xe4\xb8\x96\xe7\x95\x8c \xd0\x9f\xd1\x80\xd0\xb8\xd0\xb2\xd0\xb5\xd1\x82 \xd7\xa9\xd7\x9c\xd7\x95\xd7\x9d \xd9\x85\xd8\xb1\xd8\xad\xd8\xa8\xd8\xa7"}'
    )
    assert response.metadata is not None
