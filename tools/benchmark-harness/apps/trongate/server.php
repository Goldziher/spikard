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
        // Replace {id} with placeholder, escape, then restore as regex
        $placeholder = '___ID_PLACEHOLDER___';
        $temp = str_replace('{id}', $placeholder, $path);
        $escaped = preg_quote($temp, '#');
        $pattern = str_replace(preg_quote($placeholder, '#'), '\\d+', $escaped);
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
    error_log("[trongate] Starting server on 0.0.0.0:$port");

    $router = setupRoutes();

    // Create and bind socket
    $sock = @socket_create(AF_INET, SOCK_STREAM, SOL_TCP);
    if (!$sock) {
        die("Failed to create socket\n");
    }

    socket_set_option($sock, SOL_SOCKET, SO_REUSEADDR, 1);

    if (!@socket_bind($sock, '0.0.0.0', $port)) {
        die("Failed to bind to port $port\n");
    }

    if (!@socket_listen($sock, 128)) {
        die("Failed to listen\n");
    }

    error_log("[trongate] Server listening on 0.0.0.0:$port");

    // Main server loop
    while (true) {
        $client = @socket_accept($sock);
        if (!$client) {
            usleep(1000);
            continue;
        }

        // Read HTTP request
        $buffer = '';
        $headers = '';
        while (strlen($buffer) < 65536) {
            $chunk = @socket_read($client, 4096);
            if ($chunk === false || $chunk === '') {
                break;
            }
            $buffer .= $chunk;
            if (strpos($buffer, "\r\n\r\n") !== false) {
                break;
            }
        }

        if (empty($buffer)) {
            @socket_close($client);
            continue;
        }

        // Parse request line
        [$requestLine, $rest] = explode("\r\n", $buffer, 2);
        $parts = explode(' ', $requestLine);
        $method = $parts[0] ?? 'GET';
        $requestUri = $parts[1] ?? '/';

        // Parse headers and body
        [$headers, $body] = explode("\r\n\r\n", $buffer, 2);
        $headerLines = explode("\r\n", $headers);

        // Parse path and query
        $parsedUrl = parse_url($requestUri);
        $path = $parsedUrl['path'] ?? '/';
        $queryStr = $parsedUrl['query'] ?? '';
        $query = [];
        if (!empty($queryStr)) {
            parse_str($queryStr, $query);
        }

        // Parse request body based on content type
        $contentType = '';
        $contentLength = 0;
        foreach ($headerLines as $header) {
            if (stripos($header, 'Content-Type:') === 0) {
                $contentType = trim(substr($header, 13));
            }
            if (stripos($header, 'Content-Length:') === 0) {
                $contentLength = (int)trim(substr($header, 15));
            }
        }

        $requestBody = [];
        if (in_array($method, ['POST', 'PUT', 'PATCH', 'DELETE']) && !empty($body)) {
            if (strpos($contentType, 'application/json') !== false) {
                $requestBody = json_decode($body, true) ?? [];
            } elseif (strpos($contentType, 'application/x-www-form-urlencoded') !== false) {
                parse_str($body, $requestBody);
            } elseif (strpos($contentType, 'multipart/form-data') !== false) {
                // Simple multipart parsing
                $parts = explode('--', $body);
                $fileCount = 0;
                $totalBytes = 0;
                foreach ($parts as $part) {
                    if (strpos($part, 'filename=') !== false) {
                        $fileCount++;
                        if (preg_match('/filename="[^"]*"\r\n\r\n(.+?)\r\n/s', $part, $m)) {
                            $totalBytes += strlen($m[1]);
                        }
                    }
                }
                $requestBody = ['files' => $fileCount, 'bytes' => $totalBytes];
            }
        }

        // Dispatch request
        $response = $router->dispatch($method, $path, $query, $requestBody);

        $status = $response['status'] ?? 200;
        $headers = $response['headers'] ?? ['Content-Type' => 'application/json'];
        $body = $response['body'] ?? [];

        // Build HTTP response
        $statusTexts = [
            200 => 'OK',
            201 => 'Created',
            204 => 'No Content',
            400 => 'Bad Request',
            404 => 'Not Found',
            500 => 'Internal Server Error',
        ];
        $statusText = $statusTexts[$status] ?? 'Unknown';

        $responseBody = json_encode($body);
        $responseHeaders = "HTTP/1.1 $status $statusText\r\n";
        foreach ($headers as $name => $value) {
            $responseHeaders .= "$name: $value\r\n";
        }
        $responseHeaders .= "Content-Length: " . strlen($responseBody) . "\r\n";
        $responseHeaders .= "Connection: close\r\n\r\n";

        @socket_write($client, $responseHeaders . $responseBody);
        @socket_close($client);
    }

    socket_close($sock);
}
