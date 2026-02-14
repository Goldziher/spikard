@pytest.mark.asyncio
async def test_grpc_oauth2_bearer_token_authentication() -> None:
    """Tests OAuth2 Bearer token authentication. Validates token validation and scope checking.."""

    from app.main import handle_grpc_oauth2_bearer_token_authentication

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "authorization": "Bearer ya29.a0AfH6SMBx...",
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.OAuth2Service",
        method_name="CheckScope",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_oauth2_bearer_token_authentication(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"granted":true,"token_info":"oauth2_token"}'
    assert response.metadata is not None
