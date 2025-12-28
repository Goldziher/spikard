<?php declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
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
        // Create a minimal GraphQL route for testing
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function (\Spikard\Http\Request $request) {
                    // Parse GraphQL query from request body
                    $bodyStr = $request->body;
                    if (!is_string($bodyStr)) {
                        $bodyStr = '';
                    }
                    /** @var array<string, mixed> $body */
                    $body = json_decode($bodyStr, true) ?? [];
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

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        200,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $this->client = new TestClient($routes);
    }

    protected function tearDown(): void
    {
        $this->client->close();
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
        $data = $response->graphqlData();
        $this->assertIsArray($data);
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
                    $bodyStr = $request->body;
                    if (!is_string($bodyStr)) {
                        $bodyStr = '';
                    }
                    /** @var array<string, mixed> $body */
                    $body = json_decode($bodyStr, true) ?? [];
                    /** @var array<string, mixed>|null $variables */
                    $variables = $body['variables'] ?? null;
                    /** @var mixed $idValue */
                    $idValue = is_array($variables) && isset($variables['id']) ? $variables['id'] : 1;
                    /** @var int $userId */
                    $userId = (int)$idValue;

                    $response = [
                        'data' => [
                            'user' => [
                                'id' => $userId,
                                'name' => 'John Doe',
                            ],
                        ],
                    ];

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        200,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $client = new TestClient($routes);

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
        $data = $response->graphqlData();
        $this->assertIsArray($data);
        $this->assertIsArray($data['user']);
        $this->assertEquals(42, $data['user']['id']);
        $this->assertEquals('John Doe', $data['user']['name']);

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

        $response = $this->client->graphql($query, null, 'HelloQuery');

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

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        200,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $client = new TestClient($routes);

        $query = <<<'GQL'
        query {
            unknown
        }
        GQL;

        $response = $client->graphql($query);

        $this->assertEquals(200, $response->getStatus());
        $errors = $response->graphqlErrors();
        $this->assertIsArray($errors);
        $this->assertCount(1, $errors);
        $this->assertIsArray($errors[0]);
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

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        200,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $client = new TestClient($routes);

        $query = 'query { hello }';
        /** @var array<int, mixed> $statusAndResponse */
        $statusAndResponse = $client->graphqlWithStatus($query);

        $this->assertCount(2, $statusAndResponse);
        $this->assertEquals(200, $statusAndResponse[0]);

        $client->close();
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
                    $bodyStr = $request->body;
                    if (!is_string($bodyStr)) {
                        $bodyStr = '';
                    }
                    /** @var array<string, mixed> $body */
                    $body = json_decode($bodyStr, true) ?? [];
                    /** @var array<string, mixed>|null $variables */
                    $variables = $body['variables'] ?? null;
                    /** @var mixed $nameValue */
                    $nameValue = is_array($variables) && isset($variables['name']) ? $variables['name'] : 'New User';
                    /** @var mixed $emailValue */
                    $emailValue = is_array($variables) && isset($variables['email']) ? $variables['email'] : 'user@example.com';
                    /** @var string $userName */
                    $userName = (string)$nameValue;
                    /** @var string $userEmail */
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

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        200,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $client = new TestClient($routes);

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
        $data = $response->graphqlData();
        $this->assertIsArray($data);
        $this->assertIsArray($data['createUser']);
        $this->assertEquals(123, $data['createUser']['id']);
        $this->assertEquals('Alice Smith', $data['createUser']['name']);
        $this->assertEquals('alice@example.com', $data['createUser']['email']);

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

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        400,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $client = new TestClient($routes);

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

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        401,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $client = new TestClient($routes);
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

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        200,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $client = new TestClient($routes);
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

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        200,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $client = new TestClient($routes);
        $response = $client->graphql('query { result }');

        $errors = $response->graphqlErrors();
        $this->assertIsArray($errors);
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

                    return new \Spikard\Response(
                        json_encode($response) ?: '{}',
                        200,
                        ['content-type' => 'application/json']
                    );
                },
            ],
        ];

        $client = new TestClient($routes);
        $response = $client->graphql('query { status timestamp }');

        $errors = $response->graphqlErrors();
        $this->assertIsArray($errors);
        $this->assertCount(0, $errors);

        $client->close();
    }
}
