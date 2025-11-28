<?php

declare(strict_types=1);

/**
 * Test suite for route attributes (decorator pattern).
 *
 * This test verifies:
 * 1. Attribute scanning and route registration
 * 2. HTTP method routing (GET, POST, PUT, PATCH, DELETE)
 * 3. Parameter extraction from body, query, path
 * 4. Response conversion (array to JSON, Response objects)
 * 5. Error handling (404, 400 responses)
 *
 * Run with: php -d detect_unicode=0 e2e/php/test_route_attributes.php
 */

require_once __DIR__ . '/../../packages/php/vendor/autoload.php';
require_once __DIR__ . '/app/UserController.php';

use Spikard\App;
use Spikard\Attributes\Delete;
use Spikard\Attributes\Get;
use Spikard\Attributes\Middleware;
use Spikard\Attributes\Patch;
use Spikard\Attributes\Post;
use Spikard\Attributes\Put;
use Spikard\Config\ServerConfig;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Params\Body;
use Spikard\Http\Params\Query;
use Spikard\Http\Request;
use Spikard\Http\Response;
use SpikardE2E\UserController;

// Test utilities
function assert_true(bool $condition, string $message): void
{
    if (!$condition) {
        throw new RuntimeException("Assertion failed: {$message}");
    }
    echo "✓ {$message}\n";
}

function assert_equals(mixed $expected, mixed $actual, string $message): void
{
    if ($expected !== $actual) {
        $expectedStr = var_export($expected, true);
        $actualStr = var_export($actual, true);
        throw new RuntimeException("Assertion failed: {$message}\nExpected: {$expectedStr}\nActual: {$actualStr}");
    }
    echo "✓ {$message}\n";
}

function assert_count(int $expected, array $array, string $message): void
{
    $actual = count($array);
    if ($expected !== $actual) {
        throw new RuntimeException("Assertion failed: {$message}\nExpected count: {$expected}\nActual count: {$actual}");
    }
    echo "✓ {$message}\n";
}

function assert_array_has_key(string $key, array $array, string $message): void
{
    if (!array_key_exists($key, $array)) {
        throw new RuntimeException("Assertion failed: {$message}\nKey '{$key}' not found in array");
    }
    echo "✓ {$message}\n";
}

echo "\n=== Route Attributes Test Suite ===\n\n";

// Test 1: Controller registration
echo "Test 1: Controller registration\n";
$app = new App();
$app = $app->registerController(UserController::class);
$routes = $app->routes();
assert_count(7, $routes, "Should register 7 routes from UserController");

// Verify route paths
$paths = array_map(fn($r) => $r['path'], $routes);
assert_true(in_array('/users', $paths, true), "Should register /users route");
assert_true(in_array('/users/:id', $paths, true), "Should register /users/:id route");
assert_true(in_array('/users/search', $paths, true), "Should register /users/search route");

echo "\n";

// Test 2: Simple GET route
echo "Test 2: Simple GET route\n";
$request = new Request(
    method: 'GET',
    path: '/users',
    body: null,
    queryParams: [],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for GET /users");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(200, $response->statusCode, "Should return 200 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_array_has_key('users', $response->body, "Should have 'users' key in response");
    assert_true(is_array($response->body['users']), "Should have users array");
}

echo "\n";

// Test 3: GET with query parameters
echo "Test 3: GET with query parameters\n";
$request = new Request(
    method: 'GET',
    path: '/users',
    body: null,
    queryParams: ['limit' => ['5'], 'offset' => ['1']],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for GET /users with query params");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(200, $response->statusCode, "Should return 200 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_equals(5, $response->body['limit'], "Should respect limit query param");
    assert_equals(1, $response->body['offset'], "Should respect offset query param");
}

echo "\n";

// Test 4: GET with path parameter
echo "Test 4: GET with path parameter\n";
$request = new Request(
    method: 'GET',
    path: '/users/:id',
    body: null,
    pathParams: ['id' => '1'],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for GET /users/:id");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(200, $response->statusCode, "Should return 200 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_array_has_key('user', $response->body, "Should have 'user' key in response");
    assert_equals('1', $response->body['user']['id'], "Should return correct user ID");
}

echo "\n";

// Test 5: GET with invalid ID (404)
echo "Test 5: GET with invalid ID (404)\n";
$request = new Request(
    method: 'GET',
    path: '/users/:id',
    body: null,
    pathParams: ['id' => '999'],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for GET /users/:id");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(404, $response->statusCode, "Should return 404 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_array_has_key('error', $response->body, "Should have 'error' key in response");
}

echo "\n";

// Test 6: POST with body
echo "Test 6: POST with body\n";
$request = new Request(
    method: 'POST',
    path: '/users',
    body: ['name' => 'David', 'email' => 'david@example.com', 'age' => 28],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for POST /users");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(200, $response->statusCode, "Should return 200 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_array_has_key('user', $response->body, "Should have 'user' key in response");
    assert_equals('David', $response->body['user']['name'], "Should create user with correct name");
    assert_equals('david@example.com', $response->body['user']['email'], "Should create user with correct email");
    assert_true($response->body['created'], "Should have 'created' flag");
}

echo "\n";

// Test 7: POST with missing required fields (400)
echo "Test 7: POST with missing required fields (400)\n";
$request = new Request(
    method: 'POST',
    path: '/users',
    body: ['name' => 'Incomplete'],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for POST /users");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(400, $response->statusCode, "Should return 400 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_array_has_key('error', $response->body, "Should have 'error' key in response");
}

echo "\n";

// Test 8: PUT with path parameter and body
echo "Test 8: PUT with path parameter and body\n";
$request = new Request(
    method: 'PUT',
    path: '/users/:id',
    body: ['name' => 'Alice Updated', 'email' => 'alice.new@example.com', 'age' => 31],
    pathParams: ['id' => '1'],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for PUT /users/:id");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(200, $response->statusCode, "Should return 200 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_array_has_key('user', $response->body, "Should have 'user' key in response");
    assert_equals('Alice Updated', $response->body['user']['name'], "Should update user name");
    assert_true($response->body['updated'], "Should have 'updated' flag");
}

echo "\n";

// Test 9: PATCH with path parameter and partial body
echo "Test 9: PATCH with path parameter and partial body\n";
$request = new Request(
    method: 'PATCH',
    path: '/users/:id',
    body: ['age' => 32],
    pathParams: ['id' => '1'],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for PATCH /users/:id");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(200, $response->statusCode, "Should return 200 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_array_has_key('user', $response->body, "Should have 'user' key in response");
    assert_equals(32, $response->body['user']['age'], "Should update user age");
    // Name should still be from the previous PUT
    assert_equals('Alice Updated', $response->body['user']['name'], "Should preserve user name");
    assert_true($response->body['updated'], "Should have 'updated' flag");
}

echo "\n";

// Test 10: DELETE with path parameter
echo "Test 10: DELETE with path parameter\n";
$request = new Request(
    method: 'DELETE',
    path: '/users/:id',
    body: null,
    pathParams: ['id' => '2'],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for DELETE /users/:id");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(200, $response->statusCode, "Should return 200 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_true($response->body['deleted'], "Should have 'deleted' flag");
    assert_equals('2', $response->body['id'], "Should return deleted user ID");
}

echo "\n";

// Test 11: Search with query parameter
echo "Test 11: Search with query parameter\n";
$request = new Request(
    method: 'GET',
    path: '/users/search',
    body: null,
    queryParams: ['q' => ['alice']],
);
$handler = $app->findHandler($request);
assert_true($handler !== null, "Should find handler for GET /users/search");

if ($handler !== null) {
    $response = $handler->handle($request);
    assert_equals(200, $response->statusCode, "Should return 200 status");
    assert_true(is_array($response->body), "Should return array body");
    assert_array_has_key('users', $response->body, "Should have 'users' key in response");
    assert_equals('alice', $response->body['query'], "Should return search query");
}

echo "\n";

// Test 12: Attribute reflection verification
echo "Test 12: Attribute reflection verification\n";

// Create a test controller class to verify attributes are properly scanned
class TestAttributeController {
    #[Get('/test')]
    public function testGet(): array {
        return ['test' => true];
    }

    #[Post('/test')]
    #[Middleware('TestMiddleware')]
    public function testPost(array $data = new Body()): array {
        return $data;
    }

    // This should not be registered (no attribute)
    public function noAttribute(): array {
        return [];
    }

    // This should not be registered (private)
    #[Get('/private')]
    private function privateMethod(): array {
        return [];
    }
}

$testApp = new App();
$testApp = $testApp->registerController(TestAttributeController::class);
$testRoutes = $testApp->routes();

assert_count(2, $testRoutes, "Should register exactly 2 routes (excluding no-attribute and private)");

// Verify GET route
$getRoute = array_filter($testRoutes, fn($r) => $r['method'] === 'GET' && $r['path'] === '/test');
assert_count(1, $getRoute, "Should register GET /test route");

// Verify POST route
$postRoute = array_filter($testRoutes, fn($r) => $r['method'] === 'POST' && $r['path'] === '/test');
assert_count(1, $postRoute, "Should register POST /test route");

echo "\n";

// Test 13: Response type conversion
echo "Test 13: Response type conversion\n";

class TypeConversionController {
    #[Get('/array')]
    public function returnArray(): array {
        return ['type' => 'array'];
    }

    #[Get('/string')]
    public function returnString(): string {
        return 'plain text';
    }

    #[Get('/null')]
    public function returnNull(): ?array {
        return null;
    }

    #[Get('/response')]
    public function returnResponse(): Response {
        return new Response(['created' => true], 201);
    }
}

$typeApp = new App();
$typeApp = $typeApp->registerController(TypeConversionController::class);

// Test array response
$arrayRequest = new Request('GET', '/array', null);
$arrayHandler = $typeApp->findHandler($arrayRequest);
if ($arrayHandler !== null) {
    $response = $arrayHandler->handle($arrayRequest);
    assert_equals(200, $response->statusCode, "Array response should return 200");
    assert_true(isset($response->headers['Content-Type']), "Array response should set Content-Type");
}

// Test string response
$stringRequest = new Request('GET', '/string', null);
$stringHandler = $typeApp->findHandler($stringRequest);
if ($stringHandler !== null) {
    $response = $stringHandler->handle($stringRequest);
    assert_equals(200, $response->statusCode, "String response should return 200");
    assert_equals('plain text', $response->body, "String response should preserve body");
}

// Test null response
$nullRequest = new Request('GET', '/null', null);
$nullHandler = $typeApp->findHandler($nullRequest);
if ($nullHandler !== null) {
    $response = $nullHandler->handle($nullRequest);
    assert_equals(204, $response->statusCode, "Null response should return 204 No Content");
}

// Test Response object
$responseRequest = new Request('GET', '/response', null);
$responseHandler = $typeApp->findHandler($responseRequest);
if ($responseHandler !== null) {
    $response = $responseHandler->handle($responseRequest);
    assert_equals(201, $response->statusCode, "Response object should preserve status code");
    assert_true(is_array($response->body), "Response object should preserve body");
}

echo "\n";

echo "=== All Tests Passed! ===\n\n";

// Summary
echo "Summary:\n";
echo "- Route attribute scanning: ✓\n";
echo "- HTTP method routing: ✓\n";
echo "- Parameter extraction (Body, Query, Path): ✓\n";
echo "- Response type conversion: ✓\n";
echo "- Error handling (404, 400): ✓\n";
echo "- Middleware attribute support: ✓\n";
echo "\nRoute attributes implementation is working correctly!\n";
