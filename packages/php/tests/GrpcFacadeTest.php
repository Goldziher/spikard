<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Grpc;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;
use Spikard\Grpc\Service;

final class GrpcFacadeTest extends TestCase
{
    public function testCreateService(): void
    {
        $service = Grpc::createService();

        self::assertInstanceOf(Service::class, $service);
    }

    public function testCreateRequest(): void
    {
        $request = Grpc::createRequest(
            'test.Service',
            'TestMethod',
            'payload'
        );

        self::assertInstanceOf(Request::class, $request);
        self::assertSame('test.Service', $request->serviceName);
        self::assertSame('TestMethod', $request->methodName);
        self::assertSame('payload', $request->payload);
    }

    public function testCreateRequestWithMetadata(): void
    {
        $metadata = ['auth' => 'token'];

        $request = Grpc::createRequest(
            'test.Service',
            'TestMethod',
            'payload',
            $metadata
        );

        self::assertSame($metadata, $request->metadata);
    }

    public function testCreateResponse(): void
    {
        $response = Grpc::createResponse('response_payload');

        self::assertInstanceOf(Response::class, $response);
        self::assertSame('response_payload', $response->payload);
    }

    public function testCreateResponseWithMetadata(): void
    {
        $metadata = ['content-type' => 'application/grpc'];

        $response = Grpc::createResponse('payload', $metadata);

        self::assertSame($metadata, $response->metadata);
    }

    public function testCreateErrorResponse(): void
    {
        $response = Grpc::createErrorResponse('Something went wrong');

        self::assertInstanceOf(Response::class, $response);
        self::assertSame('', $response->payload);
        self::assertSame('INTERNAL', $response->getMetadata('grpc-status'));
        self::assertSame('Something went wrong', $response->getMetadata('grpc-message'));
    }

    public function testCreateErrorResponseWithMetadata(): void
    {
        $customMetadata = ['request-id' => '123'];

        $response = Grpc::createErrorResponse('Error message', $customMetadata);

        self::assertSame('INTERNAL', $response->getMetadata('grpc-status'));
        self::assertSame('Error message', $response->getMetadata('grpc-message'));
        self::assertSame('123', $response->getMetadata('request-id'));
    }

    public function testFacadeCreatesNewInstances(): void
    {
        $service1 = Grpc::createService();
        $service2 = Grpc::createService();

        self::assertNotSame($service1, $service2);
    }

    public function testFacadeRequestResponse(): void
    {
        $request = Grpc::createRequest('my.Service', 'MyMethod', 'req_data');
        $response = Grpc::createResponse('resp_data');

        self::assertSame('req_data', $request->payload);
        self::assertSame('resp_data', $response->payload);
    }

    public function testFacadeIntegration(): void
    {
        $service = Grpc::createService();
        $handler = new class implements \Spikard\Grpc\HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                return Grpc::createResponse('response');
            }
        };

        $service->registerHandler('test.Service', $handler);
        self::assertTrue($service->hasHandler('test.Service'));

        $request = Grpc::createRequest('test.Service', 'Method', 'payload');
        $response = $service->handleRequest($request);

        self::assertSame('response', $response->payload);
    }
}
