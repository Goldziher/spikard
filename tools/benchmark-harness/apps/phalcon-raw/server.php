<?php
declare(strict_types=1);

/**
 * Phalcon HTTP server for workload benchmarking.
 *
 * This server implements all workload endpoints to measure Phalcon performance
 * against other frameworks.
 */

require_once __DIR__ . '/vendor/autoload.php';

use Phalcon\Di\FactoryDefault;
use Phalcon\Http\Request;
use Phalcon\Http\Response;
use Phalcon\Mvc\Micro;

$di = new FactoryDefault();
$app = new Micro($di);

function jsonResponse(array $data, int $statusCode = 200): Response
{
    $response = new Response();
    $response->setJsonContent($data);
    $response->setStatusCode($statusCode);
    return $response;
}

function jsonBody(Request $request): array
{
    $input = $request->getRawBody();
    if ($input === '' || $input === null) {
        return [];
    }

    $decoded = json_decode($input, true);
    return is_array($decoded) ? $decoded : [];
}

function summarizeFiles(array $files): array
{
    $count = 0;
    $bytes = 0;

    foreach ($files as $file) {
        if (!is_array($file) || !isset($file['name'], $file['size'])) {
            continue;
        }

        if (is_array($file['name'])) {
            foreach ($file['name'] as $idx => $_name) {
                $size = is_array($file['size']) ? ($file['size'][$idx] ?? 0) : $file['size'];
                $count += 1;
                $bytes += (int) $size;
            }
        } else {
            $count += 1;
            $bytes += (int) $file['size'];
        }
    }

    return ['files_received' => $count, 'total_bytes' => $bytes];
}

$app->after(function () use ($app): void {
    $app->response->setContentType('application/json', 'utf-8');
});

$app->notFound(function (): Response {
    return jsonResponse(['error' => 'Not Found'], 404);
});

// ============================================================================
// Health Check
// ============================================================================

$app->get('/health', function (): Response {
    return jsonResponse(['status' => 'ok']);
});

$app->get('/', function (): Response {
    return jsonResponse(['status' => 'ok']);
});

// ============================================================================
// JSON body endpoints
// ============================================================================

$app->post('/json/small', function () use ($app): Response {
    return jsonResponse(jsonBody($app->request));
});

$app->post('/json/medium', function () use ($app): Response {
    return jsonResponse(jsonBody($app->request));
});

$app->post('/json/large', function () use ($app): Response {
    return jsonResponse(jsonBody($app->request));
});

$app->post('/json/very-large', function () use ($app): Response {
    return jsonResponse(jsonBody($app->request));
});

// ============================================================================
// Multipart form endpoints
// ============================================================================

$app->post('/multipart/small', function (): Response {
    return jsonResponse(summarizeFiles($_FILES));
});

$app->post('/multipart/medium', function (): Response {
    return jsonResponse(summarizeFiles($_FILES));
});

$app->post('/multipart/large', function (): Response {
    return jsonResponse(summarizeFiles($_FILES));
});

// ============================================================================
// URL-encoded form endpoints
// ============================================================================

$app->post('/urlencoded/simple', function (): Response {
    return jsonResponse($_POST ?? []);
});

$app->post('/urlencoded/complex', function (): Response {
    return jsonResponse($_POST ?? []);
});

// ============================================================================
// Path parameter endpoints
// ============================================================================

$app->get('/path/simple/{id}', function (string $id): Response {
    return jsonResponse(['id' => $id]);
});

$app->get('/path/multiple/{user_id}/{post_id}', function (string $user_id, string $post_id): Response {
    return jsonResponse(['user_id' => $user_id, 'post_id' => $post_id]);
});

$app->get('/path/deep/{org}/{team}/{project}/{resource}/{id}', function (
    string $org,
    string $team,
    string $project,
    string $resource,
    string $id
): Response {
    return jsonResponse([
        'org' => $org,
        'team' => $team,
        'project' => $project,
        'resource' => $resource,
        'id' => $id,
    ]);
});

$app->get('/path/int/{id}', function (string $id): Response {
    // Validate and convert integer
    if (filter_var($id, FILTER_VALIDATE_INT) === false) {
        return jsonResponse([
            'error' => 'Invalid integer',
            'code' => 'VALIDATION_ERROR',
            'details' => ['id' => 'must be a valid integer'],
        ], 400);
    }
    return jsonResponse(['id' => (int) $id]);
});

$app->get('/path/uuid/{uuid}', function (string $uuid): Response {
    // Validate UUID (RFC 4122)
    if (!preg_match('/^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$/i', $uuid)) {
        return jsonResponse([
            'error' => 'Invalid UUID',
            'code' => 'VALIDATION_ERROR',
            'details' => ['uuid' => 'must be a valid RFC 4122 UUID'],
        ], 400);
    }
    return jsonResponse(['uuid' => $uuid]);
});

$app->get('/path/date/{date}', function (string $date): Response {
    // Validate date (Y-m-d format)
    $parsed = DateTimeImmutable::createFromFormat('Y-m-d', $date);
    if ($parsed === false || $parsed->format('Y-m-d') !== $date) {
        return jsonResponse([
            'error' => 'Invalid date',
            'code' => 'VALIDATION_ERROR',
            'details' => ['date' => 'must be in Y-m-d format'],
        ], 400);
    }
    return jsonResponse(['date' => $date]);
});

// ============================================================================
// Query parameter endpoints
// ============================================================================

$app->get('/query/few', function (): Response {
    return jsonResponse($_GET ?? []);
});

$app->get('/query/medium', function (): Response {
    return jsonResponse($_GET ?? []);
});

$app->get('/query/many', function (): Response {
    return jsonResponse($_GET ?? []);
});

$app->handle($_SERVER['REQUEST_URI'] ?? '/');
