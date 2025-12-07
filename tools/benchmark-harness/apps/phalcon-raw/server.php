<?php
declare(strict_types=1);

/**
 * Phalcon Framework Benchmark Server
 *
 * This server implements REST API endpoints for benchmarking Phalcon's performance
 * against other frameworks. It follows the benchmark schema with JSON responses
 * and in-memory user storage.
 *
 * Features:
 * - Health check endpoint
 * - User CRUD operations (Create, Read, Update, Delete)
 * - In-memory user storage
 * - JSON request/response handling
 * - Strict types and proper error handling
 */

require_once __DIR__ . '/vendor/autoload.php';

use Phalcon\Mvc\Micro;
use Phalcon\Di\FactoryDefault;
use Phalcon\Http\Response;
use Phalcon\Http\Request;

/**
 * In-memory user storage
 */
class UserStore {
    /** @var array<int, array<string, mixed>> */
    private static array $users = [];
    /** @var int */
    private static int $nextId = 1;

    /**
     * Create a new user
     *
     * @param array<string, mixed> $data User data
     * @return array<string, mixed> Created user with ID
     */
    public static function create(array $data): array {
        $id = self::$nextId++;
        $user = array_merge(['id' => $id], $data);
        self::$users[$id] = $user;
        return $user;
    }

    /**
     * Get user by ID
     *
     * @param int $id User ID
     * @return array<string, mixed>|null User data or null if not found
     */
    public static function get(int $id): ?array {
        return self::$users[$id] ?? null;
    }

    /**
     * Update user by ID
     *
     * @param int $id User ID
     * @param array<string, mixed> $data Updated user data
     * @return array<string, mixed>|null Updated user or null if not found
     */
    public static function update(int $id, array $data): ?array {
        if (!isset(self::$users[$id])) {
            return null;
        }
        $user = array_merge(self::$users[$id], $data);
        self::$users[$id] = $user;
        return $user;
    }

    /**
     * Delete user by ID
     *
     * @param int $id User ID
     * @return bool True if deleted, false if not found
     */
    public static function delete(int $id): bool {
        if (!isset(self::$users[$id])) {
            return false;
        }
        unset(self::$users[$id]);
        return true;
    }

    /**
     * Get all users
     *
     * @return array<int, array<string, mixed>> All users
     */
    public static function all(): array {
        return array_values(self::$users);
    }

    /**
     * Clear all users (for testing)
     *
     * @return void
     */
    public static function clear(): void {
        self::$users = [];
        self::$nextId = 1;
    }
}

// Initialize DI container
$di = new FactoryDefault();

// Create Phalcon micro application
$app = new Micro($di);

// Configure JSON response content type
$app->after(function () use ($app): void {
    $app->response->setContentType('application/json', 'utf-8');
});

/**
 * Helper: Send JSON response
 *
 * @param array<string, mixed> $data Response data
 * @param int $statusCode HTTP status code
 * @return Response
 */
function sendJson(array $data, int $statusCode = 200): Response {
    $response = new Response();
    $response->setJsonContent($data);
    $response->setStatusCode($statusCode);
    return $response;
}

/**
 * Helper: Get JSON body from request
 *
 * @param Request $request Phalcon request
 * @return array<string, mixed>
 */
function getJsonBody(Request $request): array {
    $input = $request->getRawBody();
    if (empty($input)) {
        return [];
    }
    $decoded = json_decode($input, true);
    return is_array($decoded) ? $decoded : [];
}

// ============================================================================
// Health Check
// ============================================================================

$app->get('/health', function (): Response {
    return sendJson(['status' => 'ok']);
});

// ============================================================================
// User CRUD Operations
// ============================================================================

/**
 * POST /users - Create a new user
 * Request body: { name: string, email: string, ...other fields }
 * Response: { id: int, name: string, email: string, ...other fields }
 */
$app->post('/users', function () use ($app): Response {
    $request = $app->request;
    $data = getJsonBody($request);

    if (empty($data)) {
        return sendJson(['error' => 'Request body is required'], 400);
    }

    $user = UserStore::create($data);
    return sendJson($user, 201);
});

/**
 * GET /users/:id - Get user by ID
 * Response: { id: int, name: string, email: string, ...other fields }
 */
$app->get('/users/{id}', function (string $id) use ($app): Response {
    $userId = (int) $id;
    $user = UserStore::get($userId);

    if ($user === null) {
        return sendJson(['error' => 'User not found'], 404);
    }

    return sendJson($user);
});

/**
 * PUT /users/:id - Update user
 * Request body: { name?: string, email?: string, ...fields to update }
 * Response: { id: int, name: string, email: string, ...updated fields }
 */
$app->put('/users/{id}', function (string $id) use ($app): Response {
    $userId = (int) $id;
    $data = getJsonBody($app->request);

    if (empty($data)) {
        return sendJson(['error' => 'Request body is required'], 400);
    }

    $user = UserStore::update($userId, $data);

    if ($user === null) {
        return sendJson(['error' => 'User not found'], 404);
    }

    return sendJson($user);
});

/**
 * DELETE /users/:id - Delete user
 * Response: { message: "User deleted" } or error
 */
$app->delete('/users/{id}', function (string $id): Response {
    $userId = (int) $id;
    $deleted = UserStore::delete($userId);

    if (!$deleted) {
        return sendJson(['error' => 'User not found'], 404);
    }

    return sendJson(['message' => 'User deleted']);
});

// ============================================================================
// Server Startup
// ============================================================================

if (PHP_SAPI === 'cli' && !empty($argv[1])) {
    // Extract port from command line arguments
    $port = (int) $argv[1];

    // Handle server startup
    try {
        $address = '0.0.0.0:' . $port;
        error_log("[phalcon] Starting server on port {$port}");

        // Create a simple HTTP server using PHP's built-in server if available
        // For production, use actual HTTP server like Apache or Nginx
        if (function_exists('proc_open')) {
            // Use PHP built-in server
            $command = "php -S {$address} -t " . __DIR__;
            proc_open($command, [], $pipes);
        }
    } catch (Throwable $e) {
        error_log("[phalcon] Error: " . $e->getMessage());
        exit(1);
    }
}

// Handle requests
try {
    $app->handle($app->request->getURI());
} catch (Throwable $e) {
    error_log('[phalcon] Error: ' . $e->getMessage());
    $response = new Response();
    $response->setJsonContent(['error' => $e->getMessage()]);
    $response->setStatusCode(500);
    echo $response->getContent();
}
