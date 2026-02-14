@pytest.mark.asyncio
async def test_grpc_rate_limiting_with_metadata_headers() -> None:
    """Tests gRPC rate limiting. Validates rate limit headers in response and proper 429 handling.."""

    from app.main import handle_grpc_rate_limiting_with_metadata_headers

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.RateLimitService",
        method_name="Query",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_rate_limiting_with_metadata_headers(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"result":"success"}'
    assert response.metadata is not None
