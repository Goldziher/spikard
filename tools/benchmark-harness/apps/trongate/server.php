<?php
declare(strict_types=1);

/**
 * Trongate HTTP server for workload benchmarking.
 *
 * This server implements all workload types to measure Trongate performance
 * against the pure Rust baseline.
 *
 * Generated for benchmark harness.
 */

// Set error handling
error_reporting(E_ALL);
ini_set('display_errors', '1');
ini_set('log_errors', '1');

// Get port from command line or use default
$port = (int)($argv[1] ?? 8000);

// Define base paths
define('BASEPATH', __DIR__ . '/');
define('APPPATH', BASEPATH . 'application/');
define('ENGPATH', BASEPATH . 'engine/');

// ============================================================================
// In-Memory User Storage
// ============================================================================

class UserStore {
    private static array $users = [];
    private static int $nextId = 1;

    public static function create(array $data): array {
        $user = array_merge($data, ['id' => self::$nextId++]);
        self::$users[$user['id']] = $user;
        return $user;
    }

    public static function get(int $id): ?array {
        return self::$users[$id] ?? null;
    }

    public static function update(int $id, array $data): ?array {
        if (!isset(self::$users[$id])) {
            return null;
        }
        self::$users[$id] = array_merge(self::$users[$id], $data);
        self::$users[$id]['id'] = $id;
        return self::$users[$id];
    }

    public static function delete(int $id): bool {
        if (!isset(self::$users[$id])) {
            return false;
        }
        unset(self::$users[$id]);
        return true;
    }

    public static function reset(): void {
        self::$users = [];
        self::$nextId = 1;
    }
}

// ============================================================================
// Router
// ============================================================================

class SimpleRouter {
    private array $routes = [];

    public function register(string $method, string $path, callable $handler): void {
        $pattern = $this->pathToPattern($path);
        $this->routes[] = [
            'method' => strtoupper($method),
            'pattern' => $pattern,
            'path' => $path,
            'handler' => $handler,
        ];
    }

    private function pathToPattern(string $path): string {
        // Replace {param} segments with a placeholder, escape, then restore as regex.
        $placeholder = '___PARAM_PLACEHOLDER___';
        $temp = preg_replace('/\{[^}]+\}/', $placeholder, $path);
        $escaped = preg_quote($temp, '#');
        $pattern = str_replace(preg_quote($placeholder, '#'), '[^/]+', $escaped);
        return '#^' . $pattern . '$#';
    }

    public function match(string $method, string $path): ?array {
        $method = strtoupper($method);
        foreach ($this->routes as $route) {
            if ($route['method'] === $method && preg_match($route['pattern'], $path)) {
                return $route;
            }
        }
        return null;
    }

    public function dispatch(string $method, string $path, array $query, array $body): array {
        $route = $this->match($method, $path);
        if (!$route) {
            return ['status' => 404, 'body' => ['error' => 'Not found']];
        }
        return call_user_func($route['handler'], $path, $query, $body);
    }
}

// ============================================================================
// HTTP Handler Functions
// ============================================================================

function jsonResponse(array $data, int $status = 200): array {
    return [
        'status' => $status,
        'headers' => ['Content-Type' => 'application/json'],
        'body' => $data,
    ];
}

function healthHandler(string $path, array $query, array $body): array {
    return jsonResponse(['status' => 'ok'], 200);
}

function createUserHandler(string $path, array $query, array $body): array {
    if (!is_array($body) || empty($body)) {
        return jsonResponse(['error' => 'Invalid request body'], 400);
    }

    $user = UserStore::create($body);
    return jsonResponse($user, 201);
}

function getUserHandler(string $path, array $query, array $body): array {
    if (!preg_match('#/users/(\d+)$#', $path, $matches)) {
        return jsonResponse(['error' => 'Invalid request'], 400);
    }

    $id = (int)$matches[1];
    $user = UserStore::get($id);

    if (!$user) {
        return jsonResponse(['error' => 'User not found'], 404);
    }

    return jsonResponse($user, 200);
}

function updateUserHandler(string $path, array $query, array $body): array {
    if (!preg_match('#/users/(\d+)$#', $path, $matches)) {
        return jsonResponse(['error' => 'Invalid request'], 400);
    }

    $id = (int)$matches[1];
    if (!is_array($body) || empty($body)) {
        return jsonResponse(['error' => 'Invalid request body'], 400);
    }

    $user = UserStore::update($id, $body);

    if (!$user) {
        return jsonResponse(['error' => 'User not found'], 404);
    }

    return jsonResponse($user, 200);
}

function deleteUserHandler(string $path, array $query, array $body): array {
    if (!preg_match('#/users/(\d+)$#', $path, $matches)) {
        return jsonResponse(['error' => 'Invalid request'], 400);
    }

    $id = (int)$matches[1];
    $deleted = UserStore::delete($id);

    if (!$deleted) {
        return jsonResponse(['error' => 'User not found'], 404);
    }

    return jsonResponse(['deleted' => true], 204);
}

function echoHandler(string $path, array $query, array $body): array {
    return jsonResponse($body ?? [], 200);
}

// ============================================================================
// Validation Functions
// ============================================================================

/**
 * Validate fields against a schema definition.
 *
 * @param array $data Data to validate
 * @param array $schema Schema definition [field => [type, required, ...]]
 * @return array ['valid' => bool, 'errors' => [...]]
 */
function validateFields(array $data, array $schema): array {
    $errors = [];

    foreach ($schema as $field => $rules) {
        $type = $rules['type'] ?? 'string';
        $required = $rules['required'] ?? true;
        $nullable = $rules['nullable'] ?? false;

        // Check if field exists
        if (!isset($data[$field])) {
            if ($required) {
                $errors[$field] = "Field '$field' is required";
            }
            continue;
        }

        $value = $data[$field];

        // Check null
        if ($value === null) {
            if (!$nullable) {
                $errors[$field] = "Field '$field' cannot be null";
            }
            continue;
        }

        // Type validation
        switch ($type) {
            case 'string':
                if (!is_string($value)) {
                    $errors[$field] = "Field '$field' must be a string";
                } elseif (empty($value) && $required) {
                    $errors[$field] = "Field '$field' cannot be empty";
                }
                break;

            case 'numeric':
                if (!is_numeric($value)) {
                    $errors[$field] = "Field '$field' must be numeric";
                }
                break;

            case 'integer':
                if (filter_var($value, FILTER_VALIDATE_INT) === false) {
                    $errors[$field] = "Field '$field' must be an integer";
                }
                break;

            case 'boolean':
                // Accept boolean-like values
                if (!is_bool($value) && !in_array($value, ['true', 'false', '1', '0', 1, 0, 'on', 'off'], true)) {
                    $errors[$field] = "Field '$field' must be a boolean";
                }
                break;

            case 'array':
                if (!is_array($value)) {
                    $errors[$field] = "Field '$field' must be an array";
                } elseif (isset($rules['items']) && !empty($value)) {
                    // Validate array items
                    $itemSchema = $rules['items'];
                    foreach ($value as $idx => $item) {
                        if (isset($itemSchema['type']) && $itemSchema['type'] === 'object' && isset($itemSchema['properties'])) {
                            $itemResult = validateFields($item, $itemSchema['properties']);
                            if (!$itemResult['valid']) {
                                foreach ($itemResult['errors'] as $itemField => $itemError) {
                                    $errors["$field[$idx].$itemField"] = $itemError;
                                }
                            }
                        } elseif (isset($itemSchema['type']) && $itemSchema['type'] === 'string' && !is_string($item)) {
                            $errors["$field[$idx]"] = "Array item must be a string";
                        }
                    }
                }
                break;

            case 'object':
                if (!is_array($value)) {
                    $errors[$field] = "Field '$field' must be an object";
                } elseif (isset($rules['properties'])) {
                    // Validate nested object
                    $nestedResult = validateFields($value, $rules['properties']);
                    if (!$nestedResult['valid']) {
                        foreach ($nestedResult['errors'] as $nestedField => $nestedError) {
                            $errors["$field.$nestedField"] = $nestedError;
                        }
                    }
                }
                break;
        }
    }

    return [
        'valid' => empty($errors),
        'errors' => $errors,
    ];
}

function validatedJsonSmallHandler(string $path, array $query, array $body): array {
    $schema = [
        'name' => ['type' => 'string', 'required' => true],
        'description' => ['type' => 'string', 'required' => true],
        'price' => ['type' => 'numeric', 'required' => true],
        'tax' => ['type' => 'numeric', 'required' => true, 'nullable' => true],
    ];

    $result = validateFields($body, $schema);
    if (!$result['valid']) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => $result['errors'],
        ], 400);
    }

    return jsonResponse($body, 200);
}

function validatedJsonMediumHandler(string $path, array $query, array $body): array {
    $schema = [
        'name' => ['type' => 'string', 'required' => true],
        'price' => ['type' => 'numeric', 'required' => true],
        'image' => [
            'type' => 'object',
            'required' => true,
            'properties' => [
                'url' => ['type' => 'string', 'required' => true],
                'name' => ['type' => 'string', 'required' => true],
            ],
        ],
    ];

    $result = validateFields($body, $schema);
    if (!$result['valid']) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => $result['errors'],
        ], 400);
    }

    return jsonResponse($body, 200);
}

function validatedJsonLargeHandler(string $path, array $query, array $body): array {
    $schema = [
        'name' => ['type' => 'string', 'required' => true],
        'price' => ['type' => 'numeric', 'required' => true],
        'seller' => [
            'type' => 'object',
            'required' => true,
            'properties' => [
                'name' => ['type' => 'string', 'required' => true],
                'address' => [
                    'type' => 'object',
                    'required' => true,
                    'properties' => [
                        'street' => ['type' => 'string', 'required' => true],
                        'city' => ['type' => 'string', 'required' => true],
                        'country' => [
                            'type' => 'object',
                            'required' => true,
                            'properties' => [
                                'name' => ['type' => 'string', 'required' => true],
                                'code' => ['type' => 'string', 'required' => true],
                            ],
                        ],
                    ],
                ],
            ],
        ],
    ];

    $result = validateFields($body, $schema);
    if (!$result['valid']) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => $result['errors'],
        ], 400);
    }

    return jsonResponse($body, 200);
}

function validatedJsonVeryLargeHandler(string $path, array $query, array $body): array {
    $schema = [
        'name' => ['type' => 'string', 'required' => true],
        'tags' => [
            'type' => 'array',
            'required' => true,
            'items' => ['type' => 'string'],
        ],
        'images' => [
            'type' => 'array',
            'required' => true,
            'items' => [
                'type' => 'object',
                'properties' => [
                    'url' => ['type' => 'string', 'required' => true],
                    'name' => ['type' => 'string', 'required' => true],
                ],
            ],
        ],
    ];

    $result = validateFields($body, $schema);
    if (!$result['valid']) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => $result['errors'],
        ], 400);
    }

    return jsonResponse($body, 200);
}

function validatedUrlencodedSimpleHandler(string $path, array $query, array $body): array {
    $schema = [
        'name' => ['type' => 'string', 'required' => true],
        'email' => ['type' => 'string', 'required' => true],
        'age' => ['type' => 'integer', 'required' => true],
        'subscribe' => ['type' => 'boolean', 'required' => true],
    ];

    $result = validateFields($body, $schema);
    if (!$result['valid']) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => $result['errors'],
        ], 400);
    }

    return jsonResponse($body, 200);
}

function validatedUrlencodedComplexHandler(string $path, array $query, array $body): array {
    $schema = [
        'username' => ['type' => 'string', 'required' => true],
        'password' => ['type' => 'string', 'required' => true],
        'email' => ['type' => 'string', 'required' => true],
        'first_name' => ['type' => 'string', 'required' => true],
        'last_name' => ['type' => 'string', 'required' => true],
        'age' => ['type' => 'integer', 'required' => true],
        'country' => ['type' => 'string', 'required' => true],
        'state' => ['type' => 'string', 'required' => true],
        'city' => ['type' => 'string', 'required' => true],
        'zip' => ['type' => 'string', 'required' => true],
        'phone' => ['type' => 'string', 'required' => true],
        'company' => ['type' => 'string', 'required' => true],
        'job_title' => ['type' => 'string', 'required' => true],
        'subscribe' => ['type' => 'boolean', 'required' => true],
        'newsletter' => ['type' => 'boolean', 'required' => true],
        'terms_accepted' => ['type' => 'boolean', 'required' => true],
        'privacy_accepted' => ['type' => 'boolean', 'required' => true],
        'marketing_consent' => ['type' => 'boolean', 'required' => true],
        'two_factor_enabled' => ['type' => 'boolean', 'required' => true],
    ];

    $result = validateFields($body, $schema);
    if (!$result['valid']) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => $result['errors'],
        ], 400);
    }

    return jsonResponse($body, 200);
}

function validatedQueryFewHandler(string $path, array $query, array $body): array {
    $schema = [
        'q' => ['type' => 'string', 'required' => true],
        'page' => ['type' => 'integer', 'required' => false],
        'limit' => ['type' => 'integer', 'required' => false],
    ];

    $result = validateFields($query, $schema);
    if (!$result['valid']) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => $result['errors'],
        ], 400);
    }

    return jsonResponse($query, 200);
}

function validatedQueryMediumHandler(string $path, array $query, array $body): array {
    $schema = [
        'search' => ['type' => 'string', 'required' => true],
        'category' => ['type' => 'string', 'required' => false],
        'sort' => ['type' => 'string', 'required' => false],
        'order' => ['type' => 'string', 'required' => false],
        'page' => ['type' => 'integer', 'required' => false],
        'limit' => ['type' => 'integer', 'required' => false],
        'filter' => ['type' => 'string', 'required' => false],
    ];

    $result = validateFields($query, $schema);
    if (!$result['valid']) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => $result['errors'],
        ], 400);
    }

    return jsonResponse($query, 200);
}

function validatedQueryManyHandler(string $path, array $query, array $body): array {
    $schema = [
        'q' => ['type' => 'string', 'required' => true],
        'category' => ['type' => 'string', 'required' => false],
        'subcategory' => ['type' => 'string', 'required' => false],
        'brand' => ['type' => 'string', 'required' => false],
        'min_price' => ['type' => 'numeric', 'required' => false],
        'max_price' => ['type' => 'numeric', 'required' => false],
        'color' => ['type' => 'string', 'required' => false],
        'size' => ['type' => 'string', 'required' => false],
        'material' => ['type' => 'string', 'required' => false],
        'rating' => ['type' => 'integer', 'required' => false],
        'sort' => ['type' => 'string', 'required' => false],
        'order' => ['type' => 'string', 'required' => false],
        'page' => ['type' => 'integer', 'required' => false],
        'limit' => ['type' => 'integer', 'required' => false],
        'in_stock' => ['type' => 'boolean', 'required' => false],
        'on_sale' => ['type' => 'boolean', 'required' => false],
    ];

    $result = validateFields($query, $schema);
    if (!$result['valid']) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => $result['errors'],
        ], 400);
    }

    return jsonResponse($query, 200);
}

function fileUploadHandler(string $path, array $query, array $body): array {
    $filesReceived = count($_FILES);
    $totalBytes = 0;

    foreach ($_FILES as $fileInfo) {
        if (is_array($fileInfo['size'] ?? null)) {
            $totalBytes += array_sum($fileInfo['size']);
        } else {
            $totalBytes += $fileInfo['size'] ?? 0;
        }
    }

    return jsonResponse([
        'files_received' => $filesReceived,
        'total_bytes' => $totalBytes,
    ], 200);
}

function validatedFileUploadHandler(string $path, array $query, array $body): array {
    $filesReceived = count($_FILES);
    $totalBytes = 0;

    foreach ($_FILES as $fileInfo) {
        if (is_array($fileInfo['size'] ?? null)) {
            $totalBytes += array_sum($fileInfo['size']);
        } else {
            $totalBytes += $fileInfo['size'] ?? 0;
        }
    }

    if ($filesReceived === 0) {
        return jsonResponse([
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['files' => 'At least one file is required'],
        ], 400);
    }

    return jsonResponse([
        'files_received' => $filesReceived,
        'total_bytes' => $totalBytes,
    ], 200);
}

// ============================================================================
// Request Parsing
// ============================================================================

function parseJsonBody(): array {
    $input = file_get_contents('php://input');
    if (empty($input)) {
        return [];
    }

    $decoded = json_decode($input, true);
    return is_array($decoded) ? $decoded : [];
}

function parseUrlEncodedBody(): array {
    parse_str(file_get_contents('php://input'), $data);
    return $data ?? [];
}

function parseRequest(): array {
    $method = $_SERVER['REQUEST_METHOD'] ?? 'GET';
    $path = parse_url($_SERVER['REQUEST_URI'] ?? '/', PHP_URL_PATH);
    $query = $_GET ?? [];

    $contentType = $_SERVER['CONTENT_TYPE'] ?? '';

    $body = [];
    if (in_array($method, ['POST', 'PUT', 'PATCH', 'DELETE'])) {
        if (strpos($contentType, 'application/json') !== false) {
            $body = parseJsonBody();
        } elseif (strpos($contentType, 'application/x-www-form-urlencoded') !== false) {
            $body = parseUrlEncodedBody();
        }
    }

    return [$method, $path, $query, $body];
}

// ============================================================================
// Server Setup
// ============================================================================

function setupRoutes(): SimpleRouter {
    $router = new SimpleRouter();

    // Health check
    $router->register('GET', '/health', fn($p, $q, $b) => healthHandler($p, $q, $b));

    // Root endpoint
    $router->register('GET', '/', fn($p, $q, $b) => healthHandler($p, $q, $b));

    // Benchmark JSON endpoints
    $router->register('POST', '/json/small', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/json/medium', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/json/large', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/json/very-large', fn($p, $q, $b) => echoHandler($p, $q, $b));

    // Benchmark multipart endpoints
    $router->register('POST', '/multipart/small', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/multipart/medium', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/multipart/large', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));

    // Benchmark URL-encoded endpoints
    $router->register('POST', '/urlencoded/simple', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/urlencoded/complex', fn($p, $q, $b) => echoHandler($p, $q, $b));

    // Benchmark path endpoints
    $router->register('GET', '/path/simple/{id}', function (string $p, array $q, array $b): array {
        // Extract id from path
        if (preg_match('#/path/simple/([^/]+)$#', $p, $matches)) {
            return jsonResponse(['id' => $matches[1]], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });

    $router->register('GET', '/path/multiple/{user_id}/{post_id}', function (string $p, array $q, array $b): array {
        // Extract user_id and post_id from path
        if (preg_match('#/path/multiple/([^/]+)/([^/]+)$#', $p, $matches)) {
            return jsonResponse([
                'user_id' => $matches[1],
                'post_id' => $matches[2],
            ], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });

    $router->register('GET', '/path/deep/{org}/{team}/{project}/{resource}/{id}', function (string $p, array $q, array $b): array {
        // Extract all path segments
        if (preg_match('#/path/deep/([^/]+)/([^/]+)/([^/]+)/([^/]+)/([^/]+)$#', $p, $matches)) {
            return jsonResponse([
                'org' => $matches[1],
                'team' => $matches[2],
                'project' => $matches[3],
                'resource' => $matches[4],
                'id' => $matches[5],
            ], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });

    $router->register('GET', '/path/int/{id}', function (string $p, array $q, array $b): array {
        // Extract raw integer from path (no validation)
        if (preg_match('#/path/int/([^/]+)$#', $p, $matches)) {
            $id = $matches[1];
            return jsonResponse(['id' => $id], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });

    $router->register('GET', '/path/uuid/{uuid}', function (string $p, array $q, array $b): array {
        // Extract raw UUID from path (no validation)
        if (preg_match('#/path/uuid/([^/]+)$#', $p, $matches)) {
            $uuid = $matches[1];
            return jsonResponse(['uuid' => $uuid], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });

    $router->register('GET', '/path/date/{date}', function (string $p, array $q, array $b): array {
        // Extract raw date from path (no validation)
        if (preg_match('#/path/date/([^/]+)$#', $p, $matches)) {
            $date = $matches[1];
            return jsonResponse(['date' => $date], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });

    // Benchmark query endpoints
    $router->register('GET', '/query/few', fn($p, $q, $b) => jsonResponse($q, 200));
    $router->register('GET', '/query/medium', fn($p, $q, $b) => jsonResponse($q, 200));
    $router->register('GET', '/query/many', fn($p, $q, $b) => jsonResponse($q, 200));

    // Validated benchmark endpoints (mirror of raw endpoints under /validated/ prefix)
    $router->register('POST', '/validated/json/small', fn($p, $q, $b) => validatedJsonSmallHandler($p, $q, $b));
    $router->register('POST', '/validated/json/medium', fn($p, $q, $b) => validatedJsonMediumHandler($p, $q, $b));
    $router->register('POST', '/validated/json/large', fn($p, $q, $b) => validatedJsonLargeHandler($p, $q, $b));
    $router->register('POST', '/validated/json/very-large', fn($p, $q, $b) => validatedJsonVeryLargeHandler($p, $q, $b));
    $router->register('POST', '/validated/multipart/small', fn($p, $q, $b) => validatedFileUploadHandler($p, $q, $b));
    $router->register('POST', '/validated/multipart/medium', fn($p, $q, $b) => validatedFileUploadHandler($p, $q, $b));
    $router->register('POST', '/validated/multipart/large', fn($p, $q, $b) => validatedFileUploadHandler($p, $q, $b));
    $router->register('POST', '/validated/urlencoded/simple', fn($p, $q, $b) => validatedUrlencodedSimpleHandler($p, $q, $b));
    $router->register('POST', '/validated/urlencoded/complex', fn($p, $q, $b) => validatedUrlencodedComplexHandler($p, $q, $b));
    $router->register('GET', '/validated/path/simple/{id}', function (string $p, array $q, array $b): array {
        if (preg_match('#/validated/path/simple/([^/]+)$#', $p, $matches)) {
            $id = $matches[1];
            if (!preg_match('/^[a-zA-Z0-9_-]+$/', $id) || empty($id) || strlen($id) > 255) {
                return jsonResponse([
                    'error' => 'Invalid path parameter',
                    'code' => 'VALIDATION_ERROR',
                    'details' => ['id' => 'must be alphanumeric with _ or -, non-empty, max 255 chars'],
                ], 400);
            }
            return jsonResponse(['id' => $id], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });
    $router->register('GET', '/validated/path/multiple/{user_id}/{post_id}', function (string $p, array $q, array $b): array {
        if (preg_match('#/validated/path/multiple/([^/]+)/([^/]+)$#', $p, $matches)) {
            $user_id = $matches[1];
            $post_id = $matches[2];
            if (!preg_match('/^[a-zA-Z0-9_-]+$/', $user_id) || empty($user_id) || strlen($user_id) > 255) {
                return jsonResponse([
                    'error' => 'Invalid path parameter',
                    'code' => 'VALIDATION_ERROR',
                    'details' => ['user_id' => 'must be alphanumeric with _ or -, non-empty, max 255 chars'],
                ], 400);
            }
            if (!preg_match('/^[a-zA-Z0-9_-]+$/', $post_id) || empty($post_id) || strlen($post_id) > 255) {
                return jsonResponse([
                    'error' => 'Invalid path parameter',
                    'code' => 'VALIDATION_ERROR',
                    'details' => ['post_id' => 'must be alphanumeric with _ or -, non-empty, max 255 chars'],
                ], 400);
            }
            return jsonResponse(['user_id' => $user_id, 'post_id' => $post_id], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });
    $router->register('GET', '/validated/path/deep/{org}/{team}/{project}/{resource}/{id}', function (string $p, array $q, array $b): array {
        if (preg_match('#/validated/path/deep/([^/]+)/([^/]+)/([^/]+)/([^/]+)/([^/]+)$#', $p, $matches)) {
            $org = $matches[1];
            $team = $matches[2];
            $project = $matches[3];
            $resource = $matches[4];
            $id = $matches[5];
            $params = ['org' => $org, 'team' => $team, 'project' => $project, 'resource' => $resource, 'id' => $id];
            foreach ($params as $name => $value) {
                if (!preg_match('/^[a-zA-Z0-9_-]+$/', $value) || empty($value) || strlen($value) > 255) {
                    return jsonResponse([
                        'error' => 'Invalid path parameter',
                        'code' => 'VALIDATION_ERROR',
                        'details' => [$name => 'must be alphanumeric with _ or -, non-empty, max 255 chars'],
                    ], 400);
                }
            }
            return jsonResponse($params, 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });
    $router->register('GET', '/validated/path/int/{id}', function (string $p, array $q, array $b): array {
        if (preg_match('#/validated/path/int/([^/]+)$#', $p, $matches)) {
            $id = $matches[1];
            if (filter_var($id, FILTER_VALIDATE_INT) === false) {
                return jsonResponse(['error' => 'Invalid integer', 'code' => 'VALIDATION_ERROR', 'details' => ['id' => 'must be a valid integer']], 400);
            }
            return jsonResponse(['id' => (int) $id], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });
    $router->register('GET', '/validated/path/uuid/{uuid}', function (string $p, array $q, array $b): array {
        if (preg_match('#/validated/path/uuid/([^/]+)$#', $p, $matches)) {
            $uuid = $matches[1];
            if (!preg_match('/^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$/i', $uuid)) {
                return jsonResponse(['error' => 'Invalid UUID', 'code' => 'VALIDATION_ERROR', 'details' => ['uuid' => 'must be a valid RFC 4122 UUID']], 400);
            }
            return jsonResponse(['uuid' => $uuid], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });
    $router->register('GET', '/validated/path/date/{date}', function (string $p, array $q, array $b): array {
        if (preg_match('#/validated/path/date/([^/]+)$#', $p, $matches)) {
            $date = $matches[1];
            $parsed = DateTimeImmutable::createFromFormat('Y-m-d', $date);
            if ($parsed === false || $parsed->format('Y-m-d') !== $date) {
                return jsonResponse(['error' => 'Invalid date', 'code' => 'VALIDATION_ERROR', 'details' => ['date' => 'must be in Y-m-d format']], 400);
            }
            return jsonResponse(['date' => $date], 200);
        }
        return jsonResponse(['error' => 'Invalid path'], 400);
    });
    $router->register('GET', '/validated/query/few', fn($p, $q, $b) => validatedQueryFewHandler($p, $q, $b));
    $router->register('GET', '/validated/query/medium', fn($p, $q, $b) => validatedQueryMediumHandler($p, $q, $b));
    $router->register('GET', '/validated/query/many', fn($p, $q, $b) => validatedQueryManyHandler($p, $q, $b));

    // User CRUD
    $router->register('POST', '/users', fn($p, $q, $b) => createUserHandler($p, $q, $b));
    $router->register('GET', '/users/{id}', fn($p, $q, $b) => getUserHandler($p, $q, $b));
    $router->register('PUT', '/users/{id}', fn($p, $q, $b) => updateUserHandler($p, $q, $b));
    $router->register('DELETE', '/users/{id}', fn($p, $q, $b) => deleteUserHandler($p, $q, $b));

    // Echo endpoints
    $router->register('POST', '/items/', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/items', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/products', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/contact', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/', fn($p, $q, $b) => echoHandler($p, $q, $b));

    // Nested JSON endpoints
    $router->register('POST', '/items/nested', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/payment', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/billing', fn($p, $q, $b) => echoHandler($p, $q, $b));

    // List endpoints
    $router->register('POST', '/items/list', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/items/validated', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/items/optional-all', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/items/list-validated', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/events/', fn($p, $q, $b) => echoHandler($p, $q, $b));

    // Large payload endpoints
    $router->register('POST', '/api/v1/data', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/config', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/data', fn($p, $q, $b) => echoHandler($p, $q, $b));

    // Path parameter
    $router->register('PATCH', '/items/{id}', function (string $path, array $query, array $body): array {
        return jsonResponse(array_merge(['id' => $path], $body), 200);
    });

    // File uploads
    $router->register('POST', '/files/optional', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/files/list', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/files/upload', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/files/image', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/files/document', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/files/validated', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/files/images-only', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/files/required', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));
    $router->register('POST', '/upload', fn($p, $q, $b) => fileUploadHandler($p, $q, $b));

    // Form endpoints
    $router->register('POST', '/login/', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/register/', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/form/', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/form/validated', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/form/tags', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/token', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/register', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/profile', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/accounts', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/tags', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/subscribe', fn($p, $q, $b) => echoHandler($p, $q, $b));
    $router->register('POST', '/settings', fn($p, $q, $b) => echoHandler($p, $q, $b));

    return $router;
}

// ============================================================================
// Main Server Loop
// ============================================================================

if (php_sapi_name() === 'cli' && isset($argv[0]) && basename($argv[0]) === 'server.php') {
    $router = setupRoutes();

    if (!class_exists(\OpenSwoole\Http\Server::class)) {
        fwrite(STDERR, "[trongate] ERROR: OpenSwoole extension is required but not installed.\n");
        fwrite(STDERR, "[trongate] Install with: pecl install openswoole\n");
        exit(1);
    }

    $server = new \OpenSwoole\Http\Server('0.0.0.0', $port);
    $server->set([
        'worker_num' => \OpenSwoole\Util::getCPUNum(),
        'enable_coroutine' => true,
        'log_level' => \OpenSwoole\Constant::LOG_WARNING,
    ]);
    $server->on('request', function (\OpenSwoole\Http\Request $swReq, \OpenSwoole\Http\Response $swResp) use ($router): void {
        $method = strtoupper($swReq->server['request_method'] ?? 'GET');
        $uri = $swReq->server['request_uri'] ?? '/';
        $query = $swReq->get ?? [];

        $contentType = $swReq->header['content-type'] ?? '';
        $rawBody = $swReq->rawContent();
        $body = [];

        $_FILES = [];
        if ($swReq->files) {
            $_FILES = $swReq->files;
        }

        if (in_array($method, ['POST', 'PUT', 'PATCH', 'DELETE']) && !empty($rawBody)) {
            if (str_contains($contentType, 'application/json')) {
                $body = json_decode($rawBody, true) ?? [];
            } elseif (str_contains($contentType, 'application/x-www-form-urlencoded')) {
                parse_str($rawBody, $body);
            }
        }

        $result = $router->dispatch($method, $uri, $query, $body);

        $swResp->status($result['status'] ?? 200);
        $swResp->header('Content-Type', 'application/json');
        $swResp->end(json_encode($result['body'] ?? []));
    });

    error_log("[trongate] Starting OpenSwoole server on 0.0.0.0:$port");
    $server->start();
}
