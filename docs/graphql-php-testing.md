# GraphQL Testing in PHP Binding

This document describes the GraphQL testing methods added to the Spikard PHP binding's native test client.

## Overview

The PHP binding now includes comprehensive GraphQL testing support through the `Spikard\Native\TestClient` class. These methods allow you to:

- Send GraphQL queries and mutations with variables
- Extract query results from responses
- Handle GraphQL errors properly
- Test operation names and multiple operations
- Separate HTTP status codes from GraphQL responses

## API Methods

### TestClient::graphql()

Send a GraphQL query or mutation to the `/graphql` endpoint.

**Signature:**
```php
public function graphql(
    string $query,
    ?array $variables = null,
    ?string $operationName = null
): PhpTestResponse
```

**Parameters:**

- `$query` (string): The GraphQL query or mutation string
- `$variables` (array|null): Optional variables for the GraphQL operation
- `$operationName` (string|null): Optional operation name for multi-operation documents

**Returns:** `PhpTestResponse` - The HTTP response from the GraphQL endpoint

**Example:**

```php
use Spikard\Native\TestClient;

$client = new TestClient($routes);

// Simple query
$response = $client->graphql('query { hello }');

// Query with variables
$response = $client->graphql(
    'query GetUser($id: ID!) { user(id: $id) { name } }',
    ['id' => 42]
);

// Multiple operations with operation name
$multiOp = <<<'GQL'
query HelloQuery { hello }
query UserQuery { user { name } }
GQL;
$response = $client->graphql($multiOp, null, 'HelloQuery');
```

### TestClient::graphqlWithStatus()

Send a GraphQL operation and receive the HTTP status code separately.

**Signature:**
```php
public function graphqlWithStatus(
    string $query,
    ?array $variables = null,
    ?string $operationName = null
): array
```

**Returns:** Array with two elements: `[int $statusCode, PhpTestResponse $response]`

**Example:**

```php
[$status, $response] = $client->graphqlWithStatus('query { hello }');

if ($status === 200) {
    $data = $response->graphqlData();
} else {
    // Handle error
}
```

### PhpTestResponse::graphqlData()

Extract the `data` field from a GraphQL response.

**Signature:**
```php
public function graphqlData(): array
```

**Returns:** The parsed `data` object from the GraphQL response

**Throws:** `PhpException` if the response doesn't contain a `data` field

**Example:**

```php
$response = $client->graphql('query { user { id name } }');
$data = $response->graphqlData();

// $data = [
//     'user' => [
//         'id' => '1',
//         'name' => 'John Doe'
//     ]
// ]
```

### PhpTestResponse::graphqlErrors()

Extract the `errors` array from a GraphQL response.

**Signature:**
```php
public function graphqlErrors(): array
```

**Returns:** Array of error objects from the GraphQL response (empty array if no errors)

**Example:**

```php
$response = $client->graphql('query { invalidField }');
$errors = $response->graphqlErrors();

foreach ($errors as $error) {
    echo "Error: " . $error['message'];
    if (isset($error['locations'])) {
        echo " at line {$error['locations'][0]['line']}";
    }
}
```

## Usage Patterns

### Basic Query

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
echo $data['user']['name']; // Output: John Doe
```

### Query with Variables

```php
$query = <<<'GQL'
query GetUser($id: ID!) {
    user(id: $id) {
        id
        name
        email
    }
}
GQL;

$response = $client->graphql($query, ['id' => '42']);
$user = $response->graphqlData()['user'];
```

### Mutation

```php
$mutation = <<<'GQL'
mutation CreateUser($input: CreateUserInput!) {
    createUser(input: $input) {
        id
        name
        email
    }
}
GQL;

$response = $client->graphql($mutation, [
    'input' => [
        'name' => 'Alice Smith',
        'email' => 'alice@example.com',
    ]
]);

$newUser = $response->graphqlData()['createUser'];
echo "Created user: {$newUser['id']}";
```

### Error Handling

```php
$response = $client->graphql('query { invalidField }');

// Check for GraphQL errors
$errors = $response->graphqlErrors();
if (!empty($errors)) {
    foreach ($errors as $error) {
        error_log("GraphQL error: " . $error['message']);
    }
}

// Or check HTTP status
if ($response->getStatus() >= 400) {
    // Handle HTTP error
}
```

### Multiple Operations

```php
$doc = <<<'GQL'
query GetUser($id: ID!) {
    user(id: $id) { name }
}

query GetPosts($userId: ID!) {
    posts(userId: $userId) { title }
}
GQL;

// Execute first operation
$response1 = $client->graphql($doc, ['id' => '1'], 'GetUser');
$user = $response1->graphqlData()['user'];

// Execute second operation
$response2 = $client->graphql($doc, ['userId' => '1'], 'GetPosts');
$posts = $response2->graphqlData()['posts'];
```

## Implementation Details

### Zval Conversion

The GraphQL methods convert PHP arrays (Zval types) to `serde_json::Value` using the `zval_to_json()` helper function. This ensures proper type conversion across the ext-php-rs FFI boundary.

```rust
// In the graphql method:
let variables_json = match variables {
    Some(v) => {
        let json_val = zval_to_json(v).map_err(|e| PhpException::default(e))?;
        Some(json_val)
    }
    None => None,
};
```

### Response Handling

The response from the direct HTTP dispatch is converted to a `PhpTestResponse` object, which provides both raw access to the response and GraphQL-specific extraction methods.

### Async Runtime

The methods use the global Tokio runtime (retrieved via `super::get_runtime()`) to execute the async HTTP dispatch operations synchronously from PHP code.

## Code Location

- **Implementation:** `/crates/spikard-php/src/php/testing.rs`
  - `PhpNativeTestClient::graphql()` - line 313
  - `PhpNativeTestClient::graphql_with_status()` - line 357
  - `PhpTestResponse::graphql_data()` - line 108
  - `PhpTestResponse::graphql_errors()` - line 121

- **Tests:** `/packages/php/tests/GraphQLTestClientTest.php`

- **Examples:** `/examples/php/06-graphql-testing.php`

## Testing

The implementation includes comprehensive PHPUnit tests in `GraphQLTestClientTest.php`:

- `testGraphQLQuery()` - Basic query execution
- `testGraphQLQueryWithVariables()` - Query with variable substitution
- `testGraphQLQueryWithOperationName()` - Multiple operations with operation selection
- `testGraphQLErrors()` - Error extraction from responses
- `testGraphQLWithStatus()` - Status code separation
- `testGraphQLMutation()` - Mutation execution
- `testGraphQLDataMissingField()` - Error handling for invalid responses

Run tests with:
```bash
cd packages/php
composer test
```

## Notes

1. **Endpoint:** GraphQL queries are always sent to `/graphql` as POST requests
2. **Content-Type:** Automatically set to `application/json`
3. **Null Variables:** Passing `null` as variables is equivalent to omitting them
4. **Error Responses:** Both GraphQL errors and HTTP errors should be checked
5. **Data Field:** GraphQL responses should contain a `data` field even with errors (per GraphQL spec)

## Related Documentation

- [PHP API Reference](/docs/reference/api-php.md)
- [Architecture Decision Records](/docs/adr/)
- [Testing Guide](/docs/testing.md)
