# Route Attributes (Decorator Pattern) for PHP Bindings

## Overview

Route attributes provide a decorator-based pattern for defining HTTP routes in PHP, matching the ergonomics of Python's `@app.get()` decorators and TypeScript's decorator support. This implementation uses PHP 8.0+ attributes for a clean, declarative routing experience.

## Features

- **Declarative routing** using PHP 8.0+ attributes
- **HTTP method-specific attributes**: `#[Get]`, `#[Post]`, `#[Put]`, `#[Patch]`, `#[Delete]`
- **Automatic parameter extraction** from request body, query, and path parameters
- **Response type conversion** (arrays, strings, Response objects)
- **Route-level middleware** support via `#[Middleware]` attribute
- **PHPStan level max compatible** with full type safety

## Basic Usage

### Simple Controller

```php
<?php

use Spikard\Attributes\{Get, Post, Put, Delete};
use Spikard\Http\Params\Body;
use Spikard\Http\Response;

class UserController
{
    #[Get('/users')]
    public function list(): array
    {
        return ['users' => []];
    }

    #[Get('/users/:id')]
    public function get(string $id): array
    {
        return ['user' => ['id' => $id]];
    }

    #[Post('/users')]
    public function create(array $data = new Body()): array
    {
        return ['user' => $data];
    }

    #[Put('/users/:id')]
    public function update(string $id, array $data = new Body()): Response
    {
        return new Response(['user' => $data], 200);
    }

    #[Delete('/users/:id')]
    public function delete(string $id): array
    {
        return ['deleted' => true];
    }
}
```

### Registering Controllers

```php
use Spikard\App;

$app = new App();
$app = $app->registerController(UserController::class);

// Or with an instance for dependency injection:
$controller = new UserController($dependency);
$app = $app->registerController($controller);
```

## Parameter Extraction

### Path Parameters

Parameters matching path segments (e.g., `:id`) are automatically extracted by name:

```php
#[Get('/users/:id')]
public function get(string $id): array
{
    return ['id' => $id];
}
```

### Query Parameters

Parameters matching query string names are automatically extracted:

```php
#[Get('/users')]
public function list(?int $limit = 10, ?int $offset = 0): array
{
    // ?limit=20&offset=5 -> $limit=20, $offset=5
    return ['limit' => $limit, 'offset' => $offset];
}
```

### Request Body

Use the `Body` param wrapper for request body parameters:

```php
use Spikard\Http\Params\Body;

#[Post('/users')]
public function create(array $data = new Body()): array
{
    return ['user' => $data];
}
```

## Response Types

Controller methods can return various types, which are automatically converted:

### Array Response (JSON)

```php
#[Get('/users')]
public function list(): array
{
    return ['users' => []]; // -> 200 OK, Content-Type: application/json
}
```

### Response Object

```php
use Spikard\Http\Response;

#[Get('/users/:id')]
public function get(string $id): Response
{
    return new Response(['user' => []], 200, ['X-Custom' => 'value']);
}
```

### String Response

```php
#[Get('/health')]
public function health(): string
{
    return 'OK'; // -> 200 OK, Content-Type: text/plain
}
```

### Null Response (No Content)

```php
#[Delete('/users/:id')]
public function delete(string $id): ?array
{
    // ... deletion logic
    return null; // -> 204 No Content
}
```

## Middleware Support

Apply middleware to specific routes using the `#[Middleware]` attribute:

```php
use Spikard\Attributes\{Get, Middleware};

class AdminController
{
    #[Get('/admin/users')]
    #[Middleware(AuthMiddleware::class)]
    #[Middleware(AdminMiddleware::class)]
    public function adminList(): array
    {
        return ['users' => []];
    }
}
```

Multiple middleware attributes can be stacked, and they execute in order from top to bottom.

## Route Schemas

Routes can include JSON schemas for request/response validation:

```php
#[Post(
    '/users',
    requestSchema: [
        'type' => 'object',
        'required' => ['name', 'email'],
        'properties' => [
            'name' => ['type' => 'string'],
            'email' => ['type' => 'string', 'format' => 'email'],
        ],
    ],
    responseSchema: [
        'type' => 'object',
        'required' => ['user'],
        'properties' => [
            'user' => ['type' => 'object'],
        ],
    ]
)]
public function create(array $data = new Body()): array
{
    return ['user' => $data];
}
```

## Error Handling

Controllers can return error responses directly:

```php
use Spikard\Http\Response;

#[Get('/users/:id')]
public function get(string $id): array|Response
{
    $user = $this->findUser($id);

    if ($user === null) {
        return new Response(
            body: ['error' => 'User not found'],
            statusCode: 404,
            headers: ['Content-Type' => 'application/json']
        );
    }

    return ['user' => $user];
}
```

## Available Attributes

### Route Attributes

- `#[Get(path, middleware?, name?, requestSchema?, responseSchema?, parameterSchema?)]`
- `#[Post(path, middleware?, name?, requestSchema?, responseSchema?, parameterSchema?)]`
- `#[Put(path, middleware?, name?, requestSchema?, responseSchema?, parameterSchema?)]`
- `#[Patch(path, middleware?, name?, requestSchema?, responseSchema?, parameterSchema?)]`
- `#[Delete(path, middleware?, name?, requestSchema?, responseSchema?, parameterSchema?)]`
- `#[Route(method, path, middleware?, name?, requestSchema?, responseSchema?, parameterSchema?)]` - Base attribute

### Middleware Attribute

- `#[Middleware(class-string, options?)]` - Can be repeated on the same method

### Parameter Attributes (via default values)

- `new Body(default?, defaultFactory?, schema?)` - Request body extraction
- `new Query(default?, defaultFactory?, schema?)` - Query parameter extraction
- `new Path(default?, defaultFactory?, schema?)` - Path parameter extraction
- `new Header(default?, defaultFactory?, schema?)` - Header extraction
- `new Cookie(default?, defaultFactory?, schema?)` - Cookie extraction

## Implementation Details

### Files Created

1. **Attribute Classes**:
   - `/packages/php/src/Attributes/Route.php` - Base route attribute
   - `/packages/php/src/Attributes/Get.php` - GET route attribute
   - `/packages/php/src/Attributes/Post.php` - POST route attribute
   - `/packages/php/src/Attributes/Put.php` - PUT route attribute
   - `/packages/php/src/Attributes/Patch.php` - PATCH route attribute
   - `/packages/php/src/Attributes/Delete.php` - DELETE route attribute
   - `/packages/php/src/Attributes/Middleware.php` - Middleware attribute

2. **Handler Implementation**:
   - `/packages/php/src/Handlers/ControllerMethodHandler.php` - Adapts controller methods to HandlerInterface

3. **App Integration**:
   - Updated `/packages/php/src/App.php` with `registerController()` method

4. **Examples and Tests**:
   - `/e2e/php/app/UserController.php` - Complete example controller
   - `/e2e/php/test_route_attributes.php` - Comprehensive test suite

### Design Decisions

1. **Parameter Extraction Strategy**:
   - Path parameters: Matched by name to `:param` segments
   - Query parameters: Matched by name to query string keys
   - Body parameters: Use `new Body()` as default value
   - Automatic inference based on parameter names and request context

2. **Response Conversion**:
   - Arrays → JSON responses with 200 status
   - Strings → Text responses with 200 status
   - Null → 204 No Content
   - Response objects → Used as-is
   - Other types → JSON-wrapped with `{"result": value}`

3. **PHPStan Compatibility**:
   - Removed generic template annotations (not supported with callable bounds)
   - Used proper type narrowing for mixed types
   - All code passes PHPStan level max

## Limitations

1. **No True Parameter Attributes**: PHP's Body/Query/etc. classes are used as default values, not as parameter attributes. This is because they serve dual purposes in the existing codebase.

2. **Middleware Attribute**: Currently accepts class strings only. Callable middleware must be wrapped in a class.

3. **Route Priority**: Routes are registered in method order. No explicit priority system.

4. **DI Container**: Complex type parameters (non-builtin) are not yet resolved from DI container automatically.

## Testing

Run the comprehensive test suite:

```bash
php -d detect_unicode=0 e2e/php/test_route_attributes.php
```

Test coverage includes:
- ✓ Route attribute scanning and registration
- ✓ HTTP method routing (GET, POST, PUT, PATCH, DELETE)
- ✓ Parameter extraction (body, query, path)
- ✓ Response type conversion
- ✓ Error handling (404, 400)
- ✓ Middleware attribute support
- ✓ Private/non-attributed method filtering

## Comparison with Other Bindings

### Python
```python
@app.get("/users")
def list_users():
    return {"users": []}
```

### PHP (Now)
```php
#[Get('/users')]
public function list(): array {
    return ['users' => []];
}
```

The PHP implementation achieves similar ergonomics to Python and TypeScript bindings while maintaining type safety and PHPStan compatibility.
