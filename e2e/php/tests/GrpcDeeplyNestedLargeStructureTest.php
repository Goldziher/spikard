<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcDeeplyNestedLargeStructureTest extends TestCase
{
    public function testGrpcDeeplyNestedLargeStructure(): void
    {
        // Tests deeply nested protobuf messages with complex hierarchies. Validates that nested message serialization handles proper field numbering and recursive structures.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["person" => ["name" => "John Doe", "address" => ["street" => "123 Main St", "city" => "Springfield", "zip" => "12345"], "company" => ["name" => "Tech Corp", "address" => ["street" => "456 Tech Ave", "city" => "Silicon Valley", "zip" => "94025"], "employee_count" => 500]]]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.RegistryService',
            methodName: 'RegisterPerson',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcDeeplyNestedLargeStructure($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["success" => true, "person" => ["name" => "John Doe", "address" => ["street" => "123 Main St", "city" => "Springfield"]]]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
