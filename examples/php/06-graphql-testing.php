<?php
/**
 * GraphQL Testing Example
 *
 * This example demonstrates how to use Spikard's native test client for
 * testing GraphQL queries, mutations, and error handling.
 *
 * Run with: php examples/php/06-graphql-testing.php
 */

declare(strict_types=1);

require_once __DIR__ . '/../../vendor/autoload.php';

use Spikard\Native\TestClient;
use Spikard\Http\Response;

// Define a GraphQL endpoint handler
$graphqlHandler = function ($request) {
    $body = json_decode($request->rawBody, true) ?? [];

    // Extract GraphQL operation details
    $query = $body['query'] ?? '';
    $variables = $body['variables'] ?? null;
    $operationName = $body['operationName'] ?? null;

    // Determine operation type (simplified logic)
    if (strpos($query, 'mutation') !== false) {
        $responseData = handleMutation($query, $variables);
    } elseif (strpos($query, 'subscription') !== false) {
        return Response::json(
            ['errors' => [['message' => 'Subscriptions not supported']]],
            400
        );
    } else {
        $responseData = handleQuery($query, $variables);
    }

    return Response::json($responseData);
};

// Simple query handler
function handleQuery(string $query, ?array $variables): array
{
    // Check if it's a GetUser query
    if (strpos($query, 'GetUser') !== false || strpos($query, 'user') !== false) {
        $userId = $variables['id'] ?? 1;
        return [
            'data' => [
                'user' => [
                    'id' => $userId,
                    'name' => 'John Doe',
                    'email' => 'john@example.com',
                    'role' => 'admin',
                ],
            ],
        ];
    }

    // Default hello query
    return [
        'data' => [
            'hello' => 'Hello, GraphQL World!',
            'version' => '0.6.2',
        ],
    ];
}

// Simple mutation handler
function handleMutation(string $query, ?array $variables): array
{
    if (strpos($query, 'CreateUser') !== false) {
        $input = $variables['input'] ?? [];
        return [
            'data' => [
                'createUser' => [
                    'id' => 999,
                    'name' => $input['name'] ?? 'New User',
                    'email' => $input['email'] ?? 'user@example.com',
                    'success' => true,
                ],
            ],
        ];
    }

    if (strpos($query, 'DeleteUser') !== false) {
        return [
            'data' => [
                'deleteUser' => [
                    'id' => $variables['id'] ?? 1,
                    'success' => true,
                ],
            ],
        ];
    }

    return [
        'data' => null,
        'errors' => [
            ['message' => 'Unknown mutation'],
        ],
    ];
}

// Create test client with GraphQL route
$routes = [
    [
        'path' => '/graphql',
        'method' => 'POST',
        'handler' => $graphqlHandler,
    ],
];

$client = new TestClient($routes);

echo "========================================\n";
echo "GraphQL Testing Examples\n";
echo "========================================\n\n";

// Test 1: Simple query
echo "1. Simple Query Test\n";
echo "-------------------\n";

$query1 = <<<'GQL'
query {
    hello
    version
}
GQL;

$response1 = $client->graphql($query1);
$data1 = $response1->graphqlData();

echo "Status: {$response1->getStatus()}\n";
echo "Data: " . json_encode($data1, JSON_PRETTY_PRINT) . "\n\n";

// Test 2: Query with variables
echo "2. Query with Variables\n";
echo "-----------------------\n";

$query2 = <<<'GQL'
query GetUser($id: ID!) {
    user(id: $id) {
        id
        name
        email
        role
    }
}
GQL;

$variables2 = ['id' => 42];
$response2 = $client->graphql($query2, $variables2);
$data2 = $response2->graphqlData();

echo "Status: {$response2->getStatus()}\n";
echo "Query Variables: " . json_encode($variables2) . "\n";
echo "Data: " . json_encode($data2, JSON_PRETTY_PRINT) . "\n\n";

// Test 3: Mutation
echo "3. Mutation Test\n";
echo "----------------\n";

$mutation3 = <<<'GQL'
mutation CreateUser($input: CreateUserInput!) {
    createUser(input: $input) {
        id
        name
        email
        success
    }
}
GQL;

$variables3 = [
    'input' => [
        'name' => 'Alice Smith',
        'email' => 'alice@example.com',
    ],
];

$response3 = $client->graphql($mutation3, $variables3);
$data3 = $response3->graphqlData();

echo "Status: {$response3->getStatus()}\n";
echo "Data: " . json_encode($data3, JSON_PRETTY_PRINT) . "\n\n";

// Test 4: Query with operation name
echo "4. Query with Operation Name\n";
echo "----------------------------\n";

$multiOpQuery = <<<'GQL'
query HelloQuery {
    hello
    version
}

query UserQuery {
    user(id: 1) {
        id
        name
    }
}
GQL;

$response4 = $client->graphql($multiOpQuery, null, 'HelloQuery');
$data4 = $response4->graphqlData();

echo "Status: {$response4->getStatus()}\n";
echo "Operation: HelloQuery\n";
echo "Data: " . json_encode($data4, JSON_PRETTY_PRINT) . "\n\n";

// Test 5: Using graphqlWithStatus for status separation
echo "5. GraphQL with Status Separation\n";
echo "---------------------------------\n";

$query5 = <<<'GQL'
query {
    hello
}
GQL;

$statusAndResponse = $client->graphqlWithStatus($query5);
$status = $statusAndResponse[0];

echo "Status Code: {$status}\n";
echo "Response: " . json_encode($statusAndResponse[1]) . "\n\n";

// Test 6: Error handling
echo "6. Error Handling\n";
echo "-----------------\n";

$errorQuery = <<<'GQL'
mutation {
    unknownMutation {
        result
    }
}
GQL;

$response6 = $client->graphql($errorQuery);

if ($response6->getStatus() === 200) {
    try {
        $errors = $response6->graphqlErrors();
        if (!empty($errors)) {
            echo "GraphQL Errors Detected:\n";
            foreach ($errors as $error) {
                echo "  - " . json_encode($error) . "\n";
            }
            echo "\n";
        }
    } catch (Exception $e) {
        echo "Error extraction failed: " . $e->getMessage() . "\n\n";
    }
}

// Clean up
$client->close();

echo "========================================\n";
echo "All tests completed successfully!\n";
echo "========================================\n";
