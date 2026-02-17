<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Http\Response;
use Spikard\Native\TestClient;

/**
 * GraphQL test client tests for Spikard PHP binding.
 *
 * These tests verify GraphQL query/mutation execution via the native test client.
 */
class GraphQLTestClientTest extends TestCase
{
    private TestClient $client;

    protected function setUp(): void
    {
        if (!\function_exists('spikard_version')) {
            $this->markTestSkipped('Spikard PHP extension is not loaded.');
        }

        // Create a minimal GraphQL route for testing
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function (\Spikard\Http\Request $request): \Spikard\Http\Response {
                    // Parse GraphQL query from request body
                    $body = $this->decodeBody($request->body);
                    /** @var string $query */
                    $query = $body['query'] ?? '';
                    /** @var array<string, mixed>|null $variables */
                    $variables = $body['variables'] ?? null;
                    /** @var string|null $operationName */
                    $operationName = $body['operationName'] ?? null;

                    // Simple GraphQL response for testing
                    $response = [
                        'data' => [
                            'hello' => 'Hello, World!',
                            'version' => '0.6.2',
                        ],
                    ];

                    return Response::json($response, 200);
                },
            ],
        ];

        $this->client = $this->createClient($routes);
    }

    protected function tearDown(): void
    {
        if (isset($this->client)) {
            $this->client->close();
        }
    }

    /**
     * @param array<int, array{method: string, path: string, handler: object, websocket?: bool, sse?: bool, handler_name?: string}> $routes
     */
    private function createClient(array $routes): TestClient
    {
        /** @var array<int, array{method: string, path: string, handler_name: string, handler: object, websocket?: bool, sse?: bool}> $normalized */
        $normalized = [];
        foreach ($routes as $route) {
            /** @var object $handler */
            $handler = $route['handler'];
            /** @var object $finalHandler */
            $finalHandler = $handler;
            if (!\is_callable($handler) && \method_exists($handler, 'handle')) {
                $wrappedHandler = $handler;
                $finalHandler = new class ($wrappedHandler) implements \Spikard\Handlers\HandlerInterface {
                    /**  */
                    private object $handler;

                    public function __construct(object $handler)
                    {
                        $this->handler = $handler;
                    }

                    public function matches(\Spikard\Http\Request $request): bool
                    {
                        return true;
                    }

                    public function handle(\Spikard\Http\Request $request): \Spikard\Http\Response
                    {
                        if (\is_callable($this->handler)) {
                            /**  */
                            $result = ($this->handler)($request);
                        } elseif (\method_exists($this->handler, 'handle')) {
                            $result = $this->handler->handle($request);
                        } else {
                            return \Spikard\Http\Response::json(['error' => 'Handler is not callable'], 500);
                        }
                        if ($result instanceof \Spikard\Http\Response) {
                            return $result;
                        }
                        return \Spikard\Http\Response::json($result);
                    }

                    public function __invoke(\Spikard\Http\Request $request): \Spikard\Http\Response
                    {
                        return $this->handle($request);
                    }
                };
            }

            $handlerName = $route['handler_name'] ?? \spl_object_hash($finalHandler);
            $normalizedRoute = [
                'method' => $route['method'],
                'path' => $route['path'],
                'handler_name' => $handlerName,
                'handler' => $finalHandler,
            ];
            if (isset($route['websocket'])) {
                $normalizedRoute['websocket'] = (bool) $route['websocket'];
            }
            if (isset($route['sse'])) {
                $normalizedRoute['sse'] = (bool) $route['sse'];
            }

            $normalized[] = $normalizedRoute;
        }

        return new TestClient($normalized);
    }

    /**
     * @return array<string, mixed>
     */
    private function decodeBody(mixed $body): array
    {
        if (\is_array($body)) {
            /** @var array<string, mixed> $result */
            $result = $body;
            return $result;
        }
        if (\is_string($body)) {
            $decoded = \json_decode($body, true);
            /** @var array<string, mixed> $result */
            $result = \is_array($decoded) ? $decoded : [];
            return $result;
        }

        return [];
    }

    /**
     * Test sending a basic GraphQL query.
     */
    public function testGraphQLQuery(): void
    {
        $query = <<<'GQL'
        query {
            hello
            version
        }
        GQL;

        $response = $this->client->graphql($query);

        $this->assertEquals(200, $response->getStatus());
        /** @var array<string, mixed> $data */
        $data = $response->graphqlData();
        $this->assertEquals('Hello, World!', $data['hello']);
        $this->assertEquals('0.6.2', $data['version']);
    }

    /**
     * Test GraphQL query with variables.
     */
    public function testGraphQLQueryWithVariables(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function (\Spikard\Http\Request $request) {
                    $body = $this->decodeBody($request->body);
                    /** @var array<string, mixed>|null $variables */
                    $variables = $body['variables'] ?? null;
                    $idValue = \is_array($variables) && isset($variables['id']) ? $variables['id'] : 1;
                    \assert(\is_int($idValue) || \is_scalar($idValue), 'idValue must be int or scalar');
                    $userId = (int)$idValue;

                    $response = [
                        'data' => [
                            'user' => [
                                'id' => $userId,
                                'name' => 'John Doe',
                            ],
                        ],
                    ];

                    return Response::json($response, 200);
                },
            ],
        ];

        $client = $this->createClient($routes);

        $query = <<<'GQL'
        query GetUser($id: ID!) {
            user(id: $id) {
                id
                name
            }
        }
        GQL;

        /** @var array<string, int> $variables */
        $variables = ['id' => 42];
        $response = $client->graphql($query, $variables);

        $this->assertEquals(200, $response->getStatus());
        /** @var array<string, mixed> $data */
        $data = $response->graphqlData();
        /** @var array<string, mixed> $user */
        $user = $data['user'];
        $this->assertEquals(42, $user['id']);
        $this->assertEquals('John Doe', $user['name']);

        $client->close();
    }

    /**
     * Test GraphQL query with operation name.
     */
    public function testGraphQLQueryWithOperationName(): void
    {
        $query = <<<'GQL'
        query HelloQuery {
            hello
        }

        query VersionQuery {
            version
        }
        GQL;

        $variables = null;
        $response = $this->client->graphql($query, $variables, 'HelloQuery');

        $this->assertEquals(200, $response->getStatus());
        $data = $response->graphqlData();
        $this->assertEquals('Hello, World!', $data['hello']);
    }

    /**
     * Test GraphQL error extraction.
     */
    public function testGraphQLErrors(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function ($request) {
                    $response = [
                        'data' => null,
                        'errors' => [
                            [
                                'message' => 'Field "unknown" doesn\'t exist on type "Query"',
                                'locations' => [['line' => 1, 'column' => 3]],
                            ],
                        ],
                    ];

                    return Response::json($response, 200);
                },
            ],
        ];

        $client = $this->createClient($routes);

        $query = <<<'GQL'
        query {
            unknown
        }
        GQL;

        $response = $client->graphql($query);

        $this->assertEquals(200, $response->getStatus());
        /** @var array<int, array<string, mixed>> $errors */
        $errors = $response->graphqlErrors();
        $this->assertCount(1, $errors);
        /** @var string $errorMessage */
        $errorMessage = $errors[0]['message'];
        $this->assertStringContainsString('unknown', $errorMessage);

        $client->close();
    }

    /**
     * Test GraphQL with status code separation.
     */
    public function testGraphQLWithStatus(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function ($request) {
                    $response = [
                        'data' => [
                            'hello' => 'Success!',
                        ],
                    ];

                    return Response::json($response, 200);
                },
            ],
        ];

        $client = $this->createClient($routes);

        $query = 'query { hello }';
        /** @var array<int, mixed> $statusAndResponse */
        $statusAndResponse = $client->graphqlWithStatus($query);

        $this->assertCount(2, $statusAndResponse);
        $this->assertEquals(200, $statusAndResponse[0]);

        $client->close();
    }

    /**
     * Test GraphQL subscription method is exposed and reports protocol errors.
     */
    public function testGraphQLSubscriptionWithoutWebSocketRouteFails(): void
    {
        $this->expectException(\Exception::class);
        $this->client->graphqlSubscription('subscription { ticker }');
    }

    /**
     * Test GraphQL mutation.
     */
    public function testGraphQLMutation(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function (\Spikard\Http\Request $request) {
                    $body = $this->decodeBody($request->body);
                    /** @var array<string, mixed>|null $variables */
                    $variables = $body['variables'] ?? null;
                    $nameValue = \is_array($variables) && isset($variables['name']) ? $variables['name'] : 'New User';
                    $emailValue = \is_array($variables) && isset($variables['email']) ? $variables['email'] : 'user@example.com';
                    \assert(\is_string($nameValue) || \is_scalar($nameValue), 'nameValue must be string or scalar');
                    \assert(\is_string($emailValue) || \is_scalar($emailValue), 'emailValue must be string or scalar');
                    $userName = (string)$nameValue;
                    $userEmail = (string)$emailValue;

                    $response = [
                        'data' => [
                            'createUser' => [
                                'id' => 123,
                                'name' => $userName,
                                'email' => $userEmail,
                            ],
                        ],
                    ];

                    return Response::json($response, 200);
                },
            ],
        ];

        $client = $this->createClient($routes);

        $mutation = <<<'GQL'
        mutation CreateUser($name: String!, $email: String!) {
            createUser(name: $name, email: $email) {
                id
                name
                email
            }
        }
        GQL;

        /** @var array<string, string> $variables */
        $variables = [
            'name' => 'Alice Smith',
            'email' => 'alice@example.com',
        ];

        $response = $client->graphql($mutation, $variables);

        $this->assertEquals(200, $response->getStatus());
        /** @var array<string, mixed> $data */
        $data = $response->graphqlData();
        /** @var array<string, mixed> $createUser */
        $createUser = $data['createUser'];
        $this->assertEquals(123, $createUser['id']);
        $this->assertEquals('Alice Smith', $createUser['name']);
        $this->assertEquals('alice@example.com', $createUser['email']);

        $client->close();
    }

    /**
     * Test that graphqlData throws on missing data field.
     */
    public function testGraphQLDataMissingField(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function ($request) {
                    $response = ['errors' => [['message' => 'Some error']]];

                    return Response::json($response, 400);
                },
            ],
        ];

        $client = $this->createClient($routes);

        $response = $client->graphql('query { hello }');

        $this->expectException(\Exception::class);
        $response->graphqlData();

        $client->close();
    }

    /**
     * Test GraphQL response with null data but errors present.
     */
    public function testGraphQLNullDataWithErrors(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function ($request) {
                    $response = [
                        'data' => null,
                        'errors' => [
                            [
                                'message' => 'Authentication required',
                                'extensions' => ['code' => 'UNAUTHENTICATED'],
                            ],
                        ],
                    ];

                    return Response::json($response, 401);
                },
            ],
        ];

        $client = $this->createClient($routes);
        $response = $client->graphql('query { secret }');

        $this->assertEquals(401, $response->getStatus());
        $errors = $response->graphqlErrors();
        $this->assertCount(1, $errors);
        $this->assertEquals('Authentication required', $errors[0]['message']);

        $client->close();
    }

    /**
     * Test GraphQL response with multiple errors.
     */
    public function testGraphQLMultipleErrors(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function ($request) {
                    $response = [
                        'data' => null,
                        'errors' => [
                            ['message' => 'Field "a" not found'],
                            ['message' => 'Field "b" not found'],
                            ['message' => 'Field "c" not found'],
                        ],
                    ];

                    return Response::json($response, 200);
                },
            ],
        ];

        $client = $this->createClient($routes);
        $response = $client->graphql('query { a b c }');

        $errors = $response->graphqlErrors();
        $this->assertCount(3, $errors);
        $this->assertEquals('Field "a" not found', $errors[0]['message']);
        $this->assertEquals('Field "b" not found', $errors[1]['message']);
        $this->assertEquals('Field "c" not found', $errors[2]['message']);

        $client->close();
    }

    /**
     * Test GraphQL response with empty errors array.
     */
    public function testGraphQLEmptyErrorsArray(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function ($request) {
                    $response = [
                        'data' => ['result' => 'success'],
                        'errors' => [],
                    ];

                    return Response::json($response, 200);
                },
            ],
        ];

        $client = $this->createClient($routes);
        $response = $client->graphql('query { result }');

        /** @var array<int, array<string, mixed>> $errors */
        $errors = $response->graphqlErrors();
        $this->assertCount(0, $errors);

        $client->close();
    }

    /**
     * Test GraphQL response without errors field.
     */
    public function testGraphQLNoErrorsField(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function ($request) {
                    $response = [
                        'data' => [
                            'status' => 'ok',
                            'timestamp' => 1234567890,
                        ],
                    ];

                    return Response::json($response, 200);
                },
            ],
        ];

        $client = $this->createClient($routes);
        $response = $client->graphql('query { status timestamp }');

        /** @var array<int, array<string, mixed>> $errors */
        $errors = $response->graphqlErrors();
        $this->assertCount(0, $errors);

        $client->close();
    }
}
