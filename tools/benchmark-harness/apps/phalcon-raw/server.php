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

$app->get('/path/multiple/{user_id}/{post_id}', function (string $userId, string $postId): Response {
    return jsonResponse(['user_id' => $userId, 'post_id' => $postId]);
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
    return jsonResponse(['id' => (int) $id]);
});

$app->get('/path/uuid/{uuid}', function (string $uuid): Response {
    return jsonResponse(['uuid' => $uuid]);
});

$app->get('/path/date/{date}', function (string $date): Response {
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
