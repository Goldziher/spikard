# JSON-RPC 2.0 PHP Example

This example demonstrates how to build a JSON-RPC 2.0 server using Spikard's PHP bindings.

## Features

- Math operations: `math.add`, `math.subtract`, `math.multiply`
- User management: `user.create`, `user.getById`
- Automatic method documentation via JSON-RPC metadata
- Full parameter and result schema validation

## Running the Server

```bash
# Install PHP dependencies
composer install

# Run the server
php app.php
```

The server will start on `http://localhost:8000`.

## Making Requests

### Add Two Numbers

```bash
curl -X POST http://localhost:8000/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "math.add",
    "params": {"a": 5, "b": 3},
    "id": 1
  }'
```

Response:
```json
{
  "jsonrpc": "2.0",
  "result": 8,
  "id": 1
}
```

### Create a User

```bash
curl -X POST http://localhost:8000/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "user.create",
    "params": {
      "email": "john@example.com",
      "name": "John Doe"
    },
    "id": 2
  }'
```

Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "id": 5231,
    "email": "john@example.com",
    "name": "John Doe",
    "created_at": "2024-12-10T15:30:45+00:00"
  },
  "id": 2
}
```

### Get User by ID

```bash
curl -X POST http://localhost:8000/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "user.getById",
    "params": {"id": 123},
    "id": 3
  }'
```

Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "id": 123,
    "email": "user123@example.com",
    "name": "User 123"
  },
  "id": 3
}
```

### Health Check (Non-JSON-RPC)

```bash
curl http://localhost:8000/health
```

Response:
```json
{
  "status": "healthy"
}
```

## Key Concepts

### JsonRpcMethodInfo

The `JsonRpcMethodInfo` class encapsulates metadata for a JSON-RPC method:

```php
$methodInfo = new JsonRpcMethodInfo(
    methodName: 'user.create',
    description: 'Create a new user',
    paramsSchema: [...],     // JSON Schema for parameters
    resultSchema: [...],     // JSON Schema for result
    tags: ['users', 'admin'], // For documentation organization
    deprecated: false
);
```

### Route Registration

Register a JSON-RPC method with the `jsonrpc_method` parameter:

```php
$app->post('/rpc', jsonrpc_method: $methodInfo, function ($email, $name) {
    return ['id' => 1, 'email' => $email, 'name' => $name];
});
```

### Method Naming Rules

Method names must follow these rules:
- Non-empty string
- Alphanumeric characters, dots (.), underscores (_), and hyphens (-)
- Cannot start with a dot or hyphen
- Cannot end with a dot
- Cannot contain consecutive dots
- The reserved prefix `rpc.` is not allowed

Valid examples: `math.add`, `user.create`, `get_user_by_id`, `myMethod`

## Validation

The framework automatically validates:
- Method names (alphanumeric, dots, underscores only)
- Parameter schemas (if provided)
- Result schemas (if provided)

Invalid requests will receive a JSON-RPC 2.0 error response:

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32602,
    "message": "Invalid params",
    "data": "Missing required parameter 'a'"
  },
  "id": 1
}
```

## Documentation

Generated OpenRPC documentation can be accessed at `/openrpc.json` when OpenAPI is enabled.

## See Also

- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
- [Spikard Documentation](https://github.com/Goldziher/spikard)
- [Python Example](../jsonrpc-python/)
- [Node.js Example](../jsonrpc-node/)
- [Ruby Example](../jsonrpc-ruby/)
