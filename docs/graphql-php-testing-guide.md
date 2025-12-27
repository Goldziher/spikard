# GraphQL Testing Guide for PHP

This guide demonstrates how to use the GraphQL testing methods added to the Spikard PHP binding.

## Quick Start

### 1. Create Test Routes with GraphQL Endpoint

```php
<?php
use Spikard\Native\TestClient;
use Spikard\Http\Response;

$graphqlHandler = function ($request) {
    $body = json_decode($request->rawBody, true) ?? [];

    // Handle the GraphQL request
    $query = $body['query'] ?? '';
    $variables = $body['variables'] ?? null;

    // Your GraphQL resolver logic here
    $result = resolveGraphQL($query, $variables);

    return Response::json($result);
};

$routes = [
    [
        'path' => '/graphql',
        'method' => 'POST',
        'handler' => $graphqlHandler,
    ],
];

$client = new TestClient($routes);
```

### 2. Test a Query

```php
$query = <<<'GQL'
query {
    user(id: 1) {
        id
        name
        email
    }
}
GQL;

$response = $client->graphql($query);

assert($response->getStatus() === 200);
$data = $response->graphqlData();
assert($data['user']['name'] === 'John Doe');
```

### 3. Test a Mutation

```php
$mutation = <<<'GQL'
mutation CreateUser($input: CreateUserInput!) {
    createUser(input: $input) {
        id
        name
    }
}
GQL;

$variables = [
    'input' => [
        'name' => 'Jane Smith',
        'email' => 'jane@example.com',
    ]
];

$response = $client->graphql($mutation, $variables);
$newUser = $response->graphqlData()['createUser'];
```

## Real-World Example: User Service Tests

```php
<?php declare(strict_types=1);

namespace MyApp\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Native\TestClient;
use Spikard\Http\Response;

class UserGraphQLTest extends TestCase
{
    private TestClient $client;
    private array $users = [
        ['id' => 1, 'name' => 'Alice', 'email' => 'alice@example.com'],
        ['id' => 2, 'name' => 'Bob', 'email' => 'bob@example.com'],
    ];

    protected function setUp(): void
    {
        $routes = [
            [
                'path' => '/graphql',
                'method' => 'POST',
                'handler' => function ($request) {
                    return $this->handleGraphQL($request);
                },
            ],
        ];

        $this->client = new TestClient($routes);
    }

    protected function tearDown(): void
    {
        $this->client->close();
    }

    private function handleGraphQL($request): Response
    {
        $body = json_decode($request->rawBody, true) ?? [];
        $query = $body['query'] ?? '';
        $variables = $body['variables'] ?? null;

        if (strpos($query, 'GetUser') !== false) {
            return $this->resolveGetUser($variables);
        }

        if (strpos($query, 'CreateUser') !== false) {
            return $this->resolveCreateUser($variables);
        }

        if (strpos($query, 'UpdateUser') !== false) {
            return $this->resolveUpdateUser($variables);
        }

        return Response::json(['errors' => [['message' => 'Unknown query']]]);
    }

    private function resolveGetUser(?array $variables): Response
    {
        $userId = $variables['id'] ?? 1;
        $user = array_find(
            $this->users,
            fn($u) => $u['id'] === $userId
        );

        if (!$user) {
            return Response::json([
                'data' => null,
                'errors' => [['message' => 'User not found']],
            ], 404);
        }

        return Response::json(['data' => ['user' => $user]]);
    }

    private function resolveCreateUser(?array $variables): Response
    {
        $input = $variables['input'] ?? [];
        $newUser = [
            'id' => count($this->users) + 1,
            'name' => $input['name'] ?? '',
            'email' => $input['email'] ?? '',
        ];

        $this->users[] = $newUser;

        return Response::json(['data' => ['createUser' => $newUser]]);
    }

    private function resolveUpdateUser(?array $variables): Response
    {
        $input = $variables['input'] ?? [];
        $userId = $input['id'] ?? null;

        $user = array_find(
            $this->users,
            fn($u) => $u['id'] === $userId
        );

        if (!$user) {
            return Response::json([
                'data' => null,
                'errors' => [['message' => 'User not found']],
            ], 404);
        }

        $user['name'] = $input['name'] ?? $user['name'];
        $user['email'] = $input['email'] ?? $user['email'];

        return Response::json(['data' => ['updateUser' => $user]]);
    }

    // Test Cases

    public function testGetUserQuery(): void
    {
        $query = <<<'GQL'
        query GetUser($id: ID!) {
            user(id: $id) {
                id
                name
                email
            }
        }
        GQL;

        $response = $this->client->graphql($query, ['id' => 1]);
        $this->assertEquals(200, $response->getStatus());

        $data = $response->graphqlData();
        $this->assertEquals(1, $data['user']['id']);
        $this->assertEquals('Alice', $data['user']['name']);
    }

    public function testGetUserNotFound(): void
    {
        $query = <<<'GQL'
        query GetUser($id: ID!) {
            user(id: $id) {
                id
                name
            }
        }
        GQL;

        $response = $this->client->graphql($query, ['id' => 999]);
        $this->assertEquals(404, $response->getStatus());

        $errors = $response->graphqlErrors();
        $this->assertNotEmpty($errors);
        $this->assertStringContainsString('User not found', $errors[0]['message']);
    }

    public function testCreateUserMutation(): void
    {
        $mutation = <<<'GQL'
        mutation CreateUser($input: CreateUserInput!) {
            createUser(input: $input) {
                id
                name
                email
            }
        }
        GQL;

        $response = $this->client->graphql($mutation, [
            'input' => [
                'name' => 'Charlie',
                'email' => 'charlie@example.com',
            ]
        ]);

        $this->assertEquals(200, $response->getStatus());
        $data = $response->graphqlData();
        $this->assertEquals('Charlie', $data['createUser']['name']);
        $this->assertEquals('charlie@example.com', $data['createUser']['email']);
    }

    public function testUpdateUserMutation(): void
    {
        $mutation = <<<'GQL'
        mutation UpdateUser($input: UpdateUserInput!) {
            updateUser(input: $input) {
                id
                name
                email
            }
        }
        GQL;

        $response = $this->client->graphql($mutation, [
            'input' => [
                'id' => 1,
                'name' => 'Alicia',
                'email' => 'alicia@example.com',
            ]
        ]);

        $this->assertEquals(200, $response->getStatus());
        $data = $response->graphqlData();
        $this->assertEquals('Alicia', $data['updateUser']['name']);
    }

    public function testMultipleOperations(): void
    {
        $doc = <<<'GQL'
        query GetUser($id: ID!) {
            user(id: $id) {
                id
                name
            }
        }

        query GetAllUsers {
            users {
                id
                name
            }
        }
        GQL;

        // Execute first operation
        $response1 = $this->client->graphql($doc, ['id' => 1], 'GetUser');
        $this->assertEquals(200, $response1->getStatus());
        $data1 = $response1->graphqlData();
        $this->assertIsArray($data1['user']);

        // Execute second operation
        $response2 = $this->client->graphql($doc, null, 'GetAllUsers');
        // Note: This would only work if implemented in your resolver
    }
}
```

## API Reference Quick Reference

| Method | Description | Returns |
|--------|-------------|---------|
| `graphql(string, ?array, ?string)` | Send GraphQL query/mutation | `PhpTestResponse` |
| `graphqlWithStatus(string, ?array, ?string)` | Send GraphQL, get status separately | `array[int, PhpTestResponse]` |
| `graphqlData()` | Extract `data` field from response | `array` |
| `graphqlErrors()` | Extract `errors` array from response | `array` |

## Best Practices

### 1. Always Check Status and Errors

```php
$response = $client->graphql($query);

// Check HTTP status
if ($response->getStatus() !== 200) {
    $this->fail("GraphQL request failed with status {$response->getStatus()}");
}

// Check for GraphQL errors
$errors = $response->graphqlErrors();
if (!empty($errors)) {
    $this->fail("GraphQL errors: " . json_encode($errors));
}

// Extract data
$data = $response->graphqlData();
```

### 2. Use Descriptive Query Names

```php
// Good - clear what's being tested
$query = <<<'GQL'
query GetUserWithPosts($id: ID!) {
    user(id: $id) {
        id
        name
        posts { id title }
    }
}
GQL;

// Less clear
$query = <<<'GQL'
query($id: ID!) {
    user(id: $id) { id name posts { id title } }
}
GQL;
```

### 3. Separate Test Data from Logic

```php
private function getUserQuery(): string
{
    return <<<'GQL'
    query GetUser($id: ID!) {
        user(id: $id) { id name email }
    }
    GQL;
}

private function createUserMutation(): string
{
    return <<<'GQL'
    mutation CreateUser($input: CreateUserInput!) {
        createUser(input: $input) { id name email }
    }
    GQL;
}

public function testGetUser(): void
{
    $response = $this->client->graphql(
        $this->getUserQuery(),
        ['id' => 1]
    );
    // assertions...
}
```

### 4. Handle Both Success and Error Cases

```php
public function testCreateUserValidation(): void
{
    $response = $this->client->graphql(
        $this->createUserMutation(),
        ['input' => ['name' => '']] // Invalid input
    );

    // Could be HTTP error or GraphQL error
    if ($response->getStatus() !== 200) {
        $this->assertEquals(400, $response->getStatus());
    } else {
        $errors = $response->graphqlErrors();
        $this->assertNotEmpty($errors);
    }
}
```

## Debugging Tips

### 1. Print Response Details

```php
$response = $client->graphql($query);
echo "Status: " . $response->getStatus() . "\n";
echo "Body: " . $response->getBody() . "\n";
echo "Headers: " . json_encode($response->getHeaders()) . "\n";
```

### 2. Check Raw JSON

```php
$body = json_decode($response->getBody(), true);
var_dump($body); // See full GraphQL response
```

### 3. Use Variables for Dynamic Values

```php
// Good - easy to test with different values
$response = $client->graphql($query, ['userId' => 42]);

// Less ideal - values hardcoded in query
$response = $client->graphql('query { user(id: 42) { name } }');
```

## See Also

- [GraphQL Testing API Documentation](/docs/graphql-php-testing.md)
- [PHP API Reference](/docs/reference/api-php.md)
- [Example: GraphQL Testing](/examples/php/06-graphql-testing.php)
