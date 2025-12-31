<?php

declare(strict_types=1);

namespace Spikard\Tests;

use InvalidArgumentException;
use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\Grpc\HandlerInterface;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;
use Spikard\Grpc\Service;

final class GrpcServiceTest extends TestCase
{
    private Service $service;

    protected function setUp(): void
    {
        $this->service = new Service();
    }

    public function testCreateService(): void
    {
        self::assertInstanceOf(Service::class, $this->service);
    }

    public function testRegisterHandler(): void
    {
        $handler = $this->createMockHandler();

        $result = $this->service->registerHandler('test.Service', $handler);

        self::assertSame($this->service, $result);
        self::assertTrue($this->service->hasHandler('test.Service'));
    }

    public function testRegisterHandlerMethodChaining(): void
    {
        $handler1 = $this->createMockHandler();
        $handler2 = $this->createMockHandler();

        $result = $this->service
            ->registerHandler('service1.Service', $handler1)
            ->registerHandler('service2.Service', $handler2);

        self::assertSame($this->service, $result);
        self::assertTrue($this->service->hasHandler('service1.Service'));
        self::assertTrue($this->service->hasHandler('service2.Service'));
    }

    public function testGetHandler(): void
    {
        $handler = $this->createMockHandler();
        $this->service->registerHandler('test.Service', $handler);

        $retrieved = $this->service->getHandler('test.Service');

        self::assertSame($handler, $retrieved);
    }

    public function testGetHandlerNotFound(): void
    {
        $handler = $this->service->getHandler('nonexistent.Service');

        self::assertNull($handler);
    }

    public function testHasHandler(): void
    {
        $handler = $this->createMockHandler();
        $this->service->registerHandler('test.Service', $handler);

        self::assertTrue($this->service->hasHandler('test.Service'));
        self::assertFalse($this->service->hasHandler('other.Service'));
    }

    public function testRegisterEmptyServiceName(): void
    {
        $handler = $this->createMockHandler();

        $this->expectException(InvalidArgumentException::class);
        $this->service->registerHandler('', $handler);
    }

    public function testRegisterUnqualifiedServiceName(): void
    {
        $handler = $this->createMockHandler();

        $this->expectException(InvalidArgumentException::class);
        $this->service->registerHandler('SimpleService', $handler);
    }

    public function testGetServiceNames(): void
    {
        $handler1 = $this->createMockHandler();
        $handler2 = $this->createMockHandler();

        $this->service->registerHandler('service1.Service', $handler1);
        $this->service->registerHandler('service2.Service', $handler2);

        $names = $this->service->getServiceNames();

        self::assertCount(2, $names);
        self::assertContains('service1.Service', $names);
        self::assertContains('service2.Service', $names);
    }

    public function testGetServiceNamesEmpty(): void
    {
        $names = $this->service->getServiceNames();

        self::assertSame([], $names);
    }

    public function testGetHandlerCount(): void
    {
        self::assertSame(0, $this->service->getHandlerCount());

        $handler1 = $this->createMockHandler();
        $handler2 = $this->createMockHandler();

        $this->service->registerHandler('service1.Service', $handler1);
        self::assertSame(1, $this->service->getHandlerCount());

        $this->service->registerHandler('service2.Service', $handler2);
        self::assertSame(2, $this->service->getHandlerCount());
    }

    public function testClear(): void
    {
        $handler = $this->createMockHandler();
        $this->service->registerHandler('test.Service', $handler);

        self::assertTrue($this->service->hasHandler('test.Service'));

        $result = $this->service->clear();

        self::assertSame($this->service, $result);
        self::assertFalse($this->service->hasHandler('test.Service'));
        self::assertSame(0, $this->service->getHandlerCount());
    }

    public function testGetAllHandlers(): void
    {
        $handler1 = $this->createMockHandler();
        $handler2 = $this->createMockHandler();

        $this->service->registerHandler('service1.Service', $handler1);
        $this->service->registerHandler('service2.Service', $handler2);

        $handlers = $this->service->getAllHandlers();

        self::assertCount(2, $handlers);
        self::assertSame($handler1, $handlers['service1.Service']);
        self::assertSame($handler2, $handlers['service2.Service']);
    }

    public function testHandleRequestSuccess(): void
    {
        $expectedResponse = new Response('response_payload');

        // Pass the response via constructor
        $handler = new class($expectedResponse) implements HandlerInterface {
            public function __construct(private Response $response) {}

            public function handleRequest(Request $request): Response
            {
                return $this->response;
            }
        };

        $this->service->registerHandler('test.Service', $handler);

        $request = new Request('test.Service', 'TestMethod', 'payload');
        $response = $this->service->handleRequest($request);

        self::assertSame('response_payload', $response->payload);
    }

    public function testHandleRequestNotFound(): void
    {
        $request = new Request('missing.Service', 'Method', 'payload');

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No gRPC handler registered for service "missing.Service"');

        $this->service->handleRequest($request);
    }

    public function testRegisterMultipleHandlers(): void
    {
        $handler1 = $this->createMockHandler();
        $handler2 = $this->createMockHandler();
        $handler3 = $this->createMockHandler();

        $this->service
            ->registerHandler('user.Service', $handler1)
            ->registerHandler('post.Service', $handler2)
            ->registerHandler('comment.Service', $handler3);

        self::assertSame(3, $this->service->getHandlerCount());
        self::assertSame($handler1, $this->service->getHandler('user.Service'));
        self::assertSame($handler2, $this->service->getHandler('post.Service'));
        self::assertSame($handler3, $this->service->getHandler('comment.Service'));
    }

    public function testReplaceHandler(): void
    {
        $handler1 = $this->createMockHandler();
        $handler2 = $this->createMockHandler();

        $this->service->registerHandler('test.Service', $handler1);
        self::assertSame($handler1, $this->service->getHandler('test.Service'));

        $this->service->registerHandler('test.Service', $handler2);
        self::assertSame($handler2, $this->service->getHandler('test.Service'));
        self::assertSame(1, $this->service->getHandlerCount());
    }

    public function testServiceNameValidation(): void
    {
        $handler = $this->createMockHandler();

        // Valid fully qualified names
        $this->service->registerHandler('package.Service', $handler);
        $this->service->registerHandler('my.package.Service', $handler);
        $this->service->registerHandler('my.package.service.Service', $handler);

        // All should be registered
        self::assertSame(3, $this->service->getHandlerCount());
    }

    /**
     * Create a mock handler that returns a default response.
     */
    private function createMockHandler(): HandlerInterface
    {
        return new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                return new Response('default_response');
            }
        };
    }
}
