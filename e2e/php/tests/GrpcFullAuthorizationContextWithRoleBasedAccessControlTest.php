<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcFullAuthorizationContextWithRoleBasedAccessControlTest extends TestCase
{
    public function testGrpcFullAuthorizationContextWithRoleBasedAccessControl(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests complete authorization context including user roles, permissions, and resource-level access control.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "x-user-id" => "user-admin-001", "authorization" => "Bearer token123", "x-user-roles" => "admin,editor", "x-user-permissions" => "read,write,delete"];
        $requestPayload = json_encode(["resource_id" => "resource-456", "operation" => "write"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.AuthzService',
            methodName: 'CheckAccess',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcFullAuthorizationContextWithRoleBasedAccessControl($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["authorized" => true, "message" => "Access granted with admin privileges"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
