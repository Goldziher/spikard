<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Grpc;
use Spikard\Grpc\HandlerInterface;
use Spikard\Grpc\Request;
use Spikard\Grpc\Response;

/**
 * Integration test for gRPC handler registration and request handling.
 *
 * This test demonstrates a realistic scenario where:
 * 1. A service registry is created
 * 2. Multiple handlers are registered
 * 3. Requests are routed to the appropriate handlers
 * 4. Responses are generated and returned
 */
final class GrpcIntegrationTest extends TestCase
{
    /**
     * Test a simple gRPC handler that processes user requests.
     */
    public function testUserServiceHandler(): void
    {
        // Create a mock user service handler
        $userServiceHandler = new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                // In a real scenario, this would deserialize the protobuf payload
                // For this test, we'll simulate the behavior

                if ($request->methodName === 'GetUser') {
                    // Simulate getting a user from the database
                    // Real code would deserialize the request protobuf

                    // Simulate a response (in real code, serialize a protobuf message)
                    $userData = json_encode([
                        'id' => 123,
                        'name' => 'John Doe',
                        'email' => 'john@example.com',
                    ]);

                    return new Response($userData);
                }

                return Response::error('Unknown method: ' . $request->methodName);
            }
        };

        // Register the handler
        $service = Grpc::createService();
        $service->registerHandler('example.UserService', $userServiceHandler);

        // Create and handle a request
        $request = Grpc::createRequest(
            'example.UserService',
            'GetUser',
            json_encode(['id' => 123])
        );

        $response = $service->handleRequest($request);

        // Verify the response
        self::assertNotEmpty($response->payload);
        $userData = json_decode($response->payload, true);
        self::assertSame('John Doe', $userData['name']);
    }

    /**
     * Test multiple handlers for different services.
     */
    public function testMultipleServiceHandlers(): void
    {
        // Create a service registry
        $service = Grpc::createService();

        // Register UserService handler
        $service->registerHandler('example.UserService', new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                return new Response('user_response');
            }
        });

        // Register PostService handler
        $service->registerHandler('example.PostService', new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                return new Response('post_response');
            }
        });

        // Register CommentService handler
        $service->registerHandler('example.CommentService', new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                return new Response('comment_response');
            }
        });

        // Verify all services are registered
        self::assertSame(3, $service->getHandlerCount());

        // Test routing to each service
        $userRequest = Grpc::createRequest('example.UserService', 'GetUser', '');
        self::assertSame('user_response', $service->handleRequest($userRequest)->payload);

        $postRequest = Grpc::createRequest('example.PostService', 'GetPost', '');
        self::assertSame('post_response', $service->handleRequest($postRequest)->payload);

        $commentRequest = Grpc::createRequest('example.CommentService', 'GetComment', '');
        self::assertSame('comment_response', $service->handleRequest($commentRequest)->payload);
    }

    /**
     * Test error handling in gRPC handlers.
     */
    public function testErrorHandling(): void
    {
        $service = Grpc::createService();

        $errorHandler = new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                if ($request->methodName === 'InvalidMethod') {
                    return Response::error('Method not implemented');
                }

                return new Response('success');
            }
        };

        $service->registerHandler('example.Service', $errorHandler);

        // Test successful request
        $successRequest = Grpc::createRequest('example.Service', 'ValidMethod', '');
        $successResponse = $service->handleRequest($successRequest);
        self::assertSame('success', $successResponse->payload);

        // Test error request
        $errorRequest = Grpc::createRequest('example.Service', 'InvalidMethod', '');
        $errorResponse = $service->handleRequest($errorRequest);
        self::assertSame('', $errorResponse->payload);
        self::assertSame('INTERNAL', $errorResponse->getMetadata('grpc-status'));
        self::assertSame('Method not implemented', $errorResponse->getMetadata('grpc-message'));
    }

    /**
     * Test request metadata handling.
     */
    public function testRequestMetadata(): void
    {
        $service = Grpc::createService();

        $metadataAwareHandler = new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                // Extract authorization from metadata
                $auth = $request->getMetadata('authorization') ?? 'none';

                // Return metadata in response
                return new Response(
                    'authorized',
                    ['x-user-auth' => $auth]
                );
            }
        };

        $service->registerHandler('example.Service', $metadataAwareHandler);

        // Create request with metadata
        $request = Grpc::createRequest(
            'example.Service',
            'ProtectedMethod',
            '',
            ['authorization' => 'Bearer token123']
        );

        $response = $service->handleRequest($request);

        self::assertSame('authorized', $response->payload);
        self::assertSame('Bearer token123', $response->getMetadata('x-user-auth'));
    }

    /**
     * Test binary payload handling (simulating protobuf messages).
     */
    public function testBinaryPayloadHandling(): void
    {
        $service = Grpc::createService();

        $binaryHandler = new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                // In a real scenario, this would be protobuf serialization
                // For testing, we'll just echo the payload back

                $payloadSize = $request->getPayloadSize();

                // Create a response with size information encoded
                $responseData = pack('I', $payloadSize);

                return new Response($responseData);
            }
        };

        $service->registerHandler('example.Service', $binaryHandler);

        // Create request with binary payload
        $binaryPayload = "\x00\x01\x02\x03\x04\x05";
        $request = Grpc::createRequest(
            'example.Service',
            'ProcessBinary',
            $binaryPayload
        );

        $response = $service->handleRequest($request);

        // Verify response contains the payload size
        $responseSize = unpack('I', $response->payload)[1];
        self::assertSame(6, $responseSize);
    }

    /**
     * Test handler replacement.
     */
    public function testHandlerReplacement(): void
    {
        $service = Grpc::createService();

        $handler1 = new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                return new Response('handler1');
            }
        };

        $handler2 = new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                return new Response('handler2');
            }
        };

        // Register first handler
        $service->registerHandler('example.Service', $handler1);
        $request = Grpc::createRequest('example.Service', 'Method', '');
        self::assertSame('handler1', $service->handleRequest($request)->payload);

        // Replace with second handler
        $service->registerHandler('example.Service', $handler2);
        self::assertSame('handler2', $service->handleRequest($request)->payload);

        // Service count should still be 1
        self::assertSame(1, $service->getHandlerCount());
    }

    /**
     * Test request routing based on service name.
     */
    public function testServiceRouting(): void
    {
        $service = Grpc::createService();

        // Register multiple services
        $services = [
            'user.UserService',
            'post.PostService',
            'comment.CommentService',
            'auth.AuthService',
        ];

        foreach ($services as $serviceName) {
            $service->registerHandler($serviceName, new class($serviceName) implements HandlerInterface {
                public function __construct(private string $serviceName) {}

                public function handleRequest(Request $request): Response
                {
                    return new Response($this->serviceName);
                }
            });
        }

        // Test routing
        foreach ($services as $serviceName) {
            $request = Grpc::createRequest($serviceName, 'TestMethod', '');
            $response = $service->handleRequest($request);
            self::assertSame($serviceName, $response->payload);
        }
    }

    /**
     * Test handler lifecycle (register, use, clear).
     */
    public function testHandlerLifecycle(): void
    {
        $service = Grpc::createService();

        $handler = new class implements HandlerInterface {
            public function handleRequest(Request $request): Response
            {
                return new Response('response');
            }
        };

        // Register
        $service->registerHandler('example.Service', $handler);
        self::assertTrue($service->hasHandler('example.Service'));

        // Use
        $request = Grpc::createRequest('example.Service', 'Method', '');
        $response = $service->handleRequest($request);
        self::assertSame('response', $response->payload);

        // Clear
        $service->clear();
        self::assertFalse($service->hasHandler('example.Service'));
        self::assertSame(0, $service->getHandlerCount());
    }
}
