<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGrpcFailedPreconditionStatus9Test extends TestCase
{
    public function testGrpcGrpcFailedPreconditionStatus9(): void
    {
        // Tests FAILED_PRECONDITION gRPC status code. Returned when the RPC failed because the system is not in the required state.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["operation" => "finalize"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StateService',
            methodName: 'Proceed',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGrpcFailedPreconditionStatus9($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('FAILED_PRECONDITION', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
