@pytest.mark.asyncio
async def test_grpc_full_authorization_context_with_role_based_access_control() -> None:
    """Tests complete authorization context including user roles, permissions, and resource-level access control.."""

    from app.main import handle_grpc_full_authorization_context_with_role_based_access_control

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "authorization": "Bearer token123",
        "content-type": "application/grpc",
        "x-user-id": "user-admin-001",
        "x-user-roles": "admin,editor",
        "x-user-permissions": "read,write,delete",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.AuthzService",
        method_name="CheckAccess",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_full_authorization_context_with_role_based_access_control(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'{"authorized":true,"message":"Access granted with admin privileges"}'
    assert response.metadata is not None
