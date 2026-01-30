<?php
declare(strict_types=1);

/**
 * Phalcon HTTP server for workload benchmarking.
 *
 * This server implements all workload endpoints to measure Phalcon performance
 * against other frameworks. It includes both raw endpoints and validated endpoints.
 *
 * Raw endpoints (no validation) at their original paths.
 * Validated endpoints at /validated/... prefix paths.
 */

require_once __DIR__ . '/vendor/autoload.php';

use Phalcon\Di\FactoryDefault;
use Phalcon\Http\Request;
use Phalcon\Http\Response;
use Phalcon\Mvc\Micro;
use Phalcon\Validation;
use Phalcon\Validation\Validator\PresenceOf;
use Phalcon\Validation\Validator\Numericality;

$di = new FactoryDefault();
$app = new Micro($di);

function jsonResponse(Response $response, mixed $data, int $statusCode = 200): Response
{
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

// ============================================================================
// Validation functions for validated endpoints
// ============================================================================

function validateSmallPayload(array $data): array
{
    $validation = new Validation();
    $validation->add('name', new PresenceOf(['message' => 'name is required']));
    $validation->add('description', new PresenceOf(['message' => 'description is required']));
    $validation->add('price', new PresenceOf(['message' => 'price is required']));
    $validation->add('price', new Numericality(['message' => 'price must be numeric']));

    $messages = $validation->validate($data);
    if (count($messages)) {
        $errors = [];
        foreach ($messages as $message) {
            $errors[] = (string) $message;
        }
        return ['valid' => false, 'errors' => $errors];
    }

    // Validate optional tax field (optional float)
    if (isset($data['tax']) && $data['tax'] !== null && !is_numeric($data['tax'])) {
        return ['valid' => false, 'errors' => ['tax must be numeric']];
    }

    return ['valid' => true, 'data' => $data];
}

function validateMediumPayload(array $data): array
{
    $validation = new Validation();
    $validation->add('name', new PresenceOf(['message' => 'name is required']));
    $validation->add('price', new PresenceOf(['message' => 'price is required']));
    $validation->add('price', new Numericality(['message' => 'price must be numeric']));

    $messages = $validation->validate($data);
    if (count($messages)) {
        $errors = [];
        foreach ($messages as $message) {
            $errors[] = (string) $message;
        }
        return ['valid' => false, 'errors' => $errors];
    }

    // Validate nested image
    if (!isset($data['image']) || !is_array($data['image'])) {
        return ['valid' => false, 'errors' => ['image is required and must be an object']];
    }

    $imageValidation = new Validation();
    $imageValidation->add('url', new PresenceOf(['message' => 'image.url is required']));
    $imageValidation->add('name', new PresenceOf(['message' => 'image.name is required']));

    $imageMessages = $imageValidation->validate($data['image']);
    if (count($imageMessages)) {
        $errors = [];
        foreach ($imageMessages as $message) {
            $errors[] = (string) $message;
        }
        return ['valid' => false, 'errors' => $errors];
    }

    return ['valid' => true, 'data' => $data];
}

function validateLargePayload(array $data): array
{
    $validation = new Validation();
    $validation->add('name', new PresenceOf(['message' => 'name is required']));
    $validation->add('price', new PresenceOf(['message' => 'price is required']));
    $validation->add('price', new Numericality(['message' => 'price must be numeric']));

    $messages = $validation->validate($data);
    if (count($messages)) {
        $errors = [];
        foreach ($messages as $message) {
            $errors[] = (string) $message;
        }
        return ['valid' => false, 'errors' => $errors];
    }

    // Validate nested seller
    if (!isset($data['seller']) || !is_array($data['seller'])) {
        return ['valid' => false, 'errors' => ['seller is required and must be an object']];
    }

    $sellerValidation = new Validation();
    $sellerValidation->add('name', new PresenceOf(['message' => 'seller.name is required']));
    $sellerMessages = $sellerValidation->validate($data['seller']);
    if (count($sellerMessages)) {
        $errors = [];
        foreach ($sellerMessages as $message) {
            $errors[] = (string) $message;
        }
        return ['valid' => false, 'errors' => $errors];
    }

    // Validate nested address
    if (!isset($data['seller']['address']) || !is_array($data['seller']['address'])) {
        return ['valid' => false, 'errors' => ['seller.address is required and must be an object']];
    }

    $addressValidation = new Validation();
    $addressValidation->add('street', new PresenceOf(['message' => 'seller.address.street is required']));
    $addressValidation->add('city', new PresenceOf(['message' => 'seller.address.city is required']));
    $addressMessages = $addressValidation->validate($data['seller']['address']);
    if (count($addressMessages)) {
        $errors = [];
        foreach ($addressMessages as $message) {
            $errors[] = (string) $message;
        }
        return ['valid' => false, 'errors' => $errors];
    }

    // Validate nested country
    if (!isset($data['seller']['address']['country']) || !is_array($data['seller']['address']['country'])) {
        return ['valid' => false, 'errors' => ['seller.address.country is required and must be an object']];
    }

    $countryValidation = new Validation();
    $countryValidation->add('name', new PresenceOf(['message' => 'seller.address.country.name is required']));
    $countryValidation->add('code', new PresenceOf(['message' => 'seller.address.country.code is required']));
    $countryMessages = $countryValidation->validate($data['seller']['address']['country']);
    if (count($countryMessages)) {
        $errors = [];
        foreach ($countryMessages as $message) {
            $errors[] = (string) $message;
        }
        return ['valid' => false, 'errors' => $errors];
    }

    return ['valid' => true, 'data' => $data];
}

function validateVeryLargePayload(array $data): array
{
    $validation = new Validation();
    $validation->add('name', new PresenceOf(['message' => 'name is required']));

    $messages = $validation->validate($data);
    if (count($messages)) {
        $errors = [];
        foreach ($messages as $message) {
            $errors[] = (string) $message;
        }
        return ['valid' => false, 'errors' => $errors];
    }

    // Validate tags array
    if (!isset($data['tags']) || !is_array($data['tags'])) {
        return ['valid' => false, 'errors' => ['tags is required and must be an array']];
    }

    // Validate images array
    if (!isset($data['images']) || !is_array($data['images'])) {
        return ['valid' => false, 'errors' => ['images is required and must be an array']];
    }

    // Validate each image in the array
    foreach ($data['images'] as $index => $image) {
        if (!is_array($image)) {
            return ['valid' => false, 'errors' => ["images[$index] must be an object"]];
        }

        $imageValidation = new Validation();
        $imageValidation->add('url', new PresenceOf(['message' => "images[$index].url is required"]));
        $imageValidation->add('name', new PresenceOf(['message' => "images[$index].name is required"]));

        $imageMessages = $imageValidation->validate($image);
        if (count($imageMessages)) {
            $errors = [];
            foreach ($imageMessages as $message) {
                $errors[] = (string) $message;
            }
            return ['valid' => false, 'errors' => $errors];
        }
    }

    return ['valid' => true, 'data' => $data];
}

$app->after(function () use ($app): void {
    $app->response->setContentType('application/json', 'utf-8');
});

$app->notFound(function (): Response {
    $response = new Response();
    return jsonResponse($response, ['error' => 'Not Found'], 404);
});

// ============================================================================
// Health Check
// ============================================================================

$app->get('/health', function (): Response {
    $response = new Response();
    return jsonResponse($response, ['status' => 'ok']);
});

$app->get('/', function (): Response {
    $response = new Response();
    return jsonResponse($response, ['status' => 'ok']);
});

// ============================================================================
// Raw JSON body endpoints (no validation)
// ============================================================================

$app->post('/json/small', function () use ($app): Response {
    return jsonResponse($app->response, jsonBody($app->request));
});

$app->post('/json/medium', function () use ($app): Response {
    return jsonResponse($app->response, jsonBody($app->request));
});

$app->post('/json/large', function () use ($app): Response {
    return jsonResponse($app->response, jsonBody($app->request));
});

$app->post('/json/very-large', function () use ($app): Response {
    return jsonResponse($app->response, jsonBody($app->request));
});

// ============================================================================
// Validated JSON body endpoints (with validation prefix)
// ============================================================================

$app->post('/validated/json/small', function () use ($app): Response {
    $data = jsonBody($app->request);
    $result = validateSmallPayload($data);

    if (!$result['valid']) {
        return jsonResponse($app->response, ['errors' => $result['errors']], 400);
    }

    return jsonResponse($app->response, $result['data']);
});

$app->post('/validated/json/medium', function () use ($app): Response {
    $data = jsonBody($app->request);
    $result = validateMediumPayload($data);

    if (!$result['valid']) {
        return jsonResponse($app->response, ['errors' => $result['errors']], 400);
    }

    return jsonResponse($app->response, $result['data']);
});

$app->post('/validated/json/large', function () use ($app): Response {
    $data = jsonBody($app->request);
    $result = validateLargePayload($data);

    if (!$result['valid']) {
        return jsonResponse($app->response, ['errors' => $result['errors']], 400);
    }

    return jsonResponse($app->response, $result['data']);
});

$app->post('/validated/json/very-large', function () use ($app): Response {
    $data = jsonBody($app->request);
    $result = validateVeryLargePayload($data);

    if (!$result['valid']) {
        return jsonResponse($app->response, ['errors' => $result['errors']], 400);
    }

    return jsonResponse($app->response, $result['data']);
});

// ============================================================================
// Validated URL-encoded form endpoints
// ============================================================================

$app->post('/validated/urlencoded/simple', function () use ($app): Response {
    $data = $_POST ?? [];

    $validation = new Validation();
    $validation->add('name', new PresenceOf(['message' => 'name is required']));
    $validation->add('email', new PresenceOf(['message' => 'email is required']));
    $validation->add('age', new PresenceOf(['message' => 'age is required']));
    $validation->add('age', new Numericality(['message' => 'age must be numeric']));

    $messages = $validation->validate($data);
    if (count($messages)) {
        $errors = [];
        foreach ($messages as $message) {
            $errors[] = (string) $message;
        }
        return jsonResponse($app->response, ['errors' => $errors], 400);
    }

    // Validate boolean fields
    if (!isset($data['subscribe']) || !in_array($data['subscribe'], ['true', 'false', '1', '0', 1, 0, true, false], true)) {
        return jsonResponse($app->response, ['errors' => ['subscribe must be a boolean']], 400);
    }

    return jsonResponse($app->response, $data);
});

$app->post('/validated/urlencoded/complex', function () use ($app): Response {
    $data = $_POST ?? [];

    $validation = new Validation();
    $validation->add('username', new PresenceOf(['message' => 'username is required']));
    $validation->add('password', new PresenceOf(['message' => 'password is required']));
    $validation->add('email', new PresenceOf(['message' => 'email is required']));
    $validation->add('first_name', new PresenceOf(['message' => 'first_name is required']));
    $validation->add('last_name', new PresenceOf(['message' => 'last_name is required']));
    $validation->add('age', new PresenceOf(['message' => 'age is required']));
    $validation->add('age', new Numericality(['message' => 'age must be numeric']));
    $validation->add('country', new PresenceOf(['message' => 'country is required']));
    $validation->add('state', new PresenceOf(['message' => 'state is required']));
    $validation->add('city', new PresenceOf(['message' => 'city is required']));
    $validation->add('zip', new PresenceOf(['message' => 'zip is required']));
    $validation->add('phone', new PresenceOf(['message' => 'phone is required']));
    $validation->add('company', new PresenceOf(['message' => 'company is required']));
    $validation->add('job_title', new PresenceOf(['message' => 'job_title is required']));

    $messages = $validation->validate($data);
    if (count($messages)) {
        $errors = [];
        foreach ($messages as $message) {
            $errors[] = (string) $message;
        }
        return jsonResponse($app->response, ['errors' => $errors], 400);
    }

    // Validate boolean fields
    $boolFields = ['subscribe', 'newsletter', 'terms_accepted', 'privacy_accepted', 'marketing_consent', 'two_factor_enabled'];
    foreach ($boolFields as $field) {
        if (!isset($data[$field]) || !in_array($data[$field], ['true', 'false', '1', '0', 1, 0, true, false], true)) {
            return jsonResponse($app->response, ['errors' => [$field . ' must be a boolean']], 400);
        }
    }

    return jsonResponse($app->response, $data);
});

// ============================================================================
// Validated query parameter endpoints
// ============================================================================

$app->get('/validated/query/few', function () use ($app): Response {
    $params = $_GET ?? [];

    // Validate q is required string
    if (!isset($params['q']) || !is_string($params['q']) || $params['q'] === '') {
        return jsonResponse($app->response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['q' => 'q is required and must be a non-empty string'],
        ], 400);
    }

    // Validate page and limit are optional integers
    if (isset($params['page']) && filter_var($params['page'], FILTER_VALIDATE_INT) === false) {
        return jsonResponse($app->response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['page' => 'page must be a valid integer'],
        ], 400);
    }

    if (isset($params['limit']) && filter_var($params['limit'], FILTER_VALIDATE_INT) === false) {
        return jsonResponse($app->response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['limit' => 'limit must be a valid integer'],
        ], 400);
    }

    return jsonResponse($app->response, $params);
});

$app->get('/validated/query/medium', function () use ($app): Response {
    $params = $_GET ?? [];

    // Validate required field 'search'
    if (!isset($params['search']) || !is_string($params['search']) || $params['search'] === '') {
        return jsonResponse($app->response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['search' => 'search is required and must be a non-empty string'],
        ], 400);
    }

    // Validate optional string fields
    $optionalStringFields = ['category', 'sort', 'order', 'filter'];
    foreach ($optionalStringFields as $field) {
        if (isset($params[$field]) && (!is_string($params[$field]) || $params[$field] === '')) {
            return jsonResponse($app->response, [
                'error' => 'Validation failed',
                'code' => 'VALIDATION_ERROR',
                'details' => [$field => $field . ' must be a non-empty string'],
            ], 400);
        }
    }

    // Validate optional integer fields
    if (isset($params['page']) && filter_var($params['page'], FILTER_VALIDATE_INT) === false) {
        return jsonResponse($app->response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['page' => 'page must be a valid integer'],
        ], 400);
    }

    if (isset($params['limit']) && filter_var($params['limit'], FILTER_VALIDATE_INT) === false) {
        return jsonResponse($app->response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['limit' => 'limit must be a valid integer'],
        ], 400);
    }

    return jsonResponse($app->response, $params);
});

$app->get('/validated/query/many', function () use ($app): Response {
    $params = $_GET ?? [];

    // Validate required field 'q'
    if (!isset($params['q']) || !is_string($params['q']) || $params['q'] === '') {
        return jsonResponse($app->response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['q' => 'q is required and must be a non-empty string'],
        ], 400);
    }

    // Validate optional string fields
    $optionalStringFields = ['category', 'subcategory', 'brand', 'color', 'size', 'material', 'sort', 'order'];
    foreach ($optionalStringFields as $field) {
        if (isset($params[$field]) && (!is_string($params[$field]) || $params[$field] === '')) {
            return jsonResponse($app->response, [
                'error' => 'Validation failed',
                'code' => 'VALIDATION_ERROR',
                'details' => [$field => $field . ' must be a non-empty string'],
            ], 400);
        }
    }

    // Validate optional numeric fields (number type)
    $optionalNumericFields = ['min_price', 'max_price'];
    foreach ($optionalNumericFields as $field) {
        if (isset($params[$field]) && !is_numeric($params[$field])) {
            return jsonResponse($app->response, [
                'error' => 'Validation failed',
                'code' => 'VALIDATION_ERROR',
                'details' => [$field => $field . ' must be numeric'],
            ], 400);
        }
    }

    // Validate rating as integer (not just numeric)
    if (isset($params['rating']) && filter_var($params['rating'], FILTER_VALIDATE_INT) === false) {
        return jsonResponse($app->response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['rating' => 'rating must be a valid integer'],
        ], 400);
    }

    // Validate optional integer fields
    $optionalIntegerFields = ['page', 'limit'];
    foreach ($optionalIntegerFields as $field) {
        if (isset($params[$field]) && filter_var($params[$field], FILTER_VALIDATE_INT) === false) {
            return jsonResponse($app->response, [
                'error' => 'Validation failed',
                'code' => 'VALIDATION_ERROR',
                'details' => [$field => $field . ' must be a valid integer'],
            ], 400);
        }
    }

    // Validate optional boolean fields
    $optionalBooleanFields = ['in_stock', 'on_sale'];
    foreach ($optionalBooleanFields as $field) {
        if (isset($params[$field]) && !in_array($params[$field], ['true', 'false', '1', '0', 1, 0, true, false], true)) {
            return jsonResponse($app->response, [
                'error' => 'Validation failed',
                'code' => 'VALIDATION_ERROR',
                'details' => [$field => $field . ' must be a boolean'],
            ], 400);
        }
    }

    return jsonResponse($app->response, $params);
});

// ============================================================================
// Validated multipart form endpoints
// ============================================================================

$app->post('/validated/multipart/small', function (): Response {
    $response = new Response();
    $result = summarizeFiles($_FILES);

    if ($result['files_received'] === 0) {
        return jsonResponse($response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['files' => 'At least one file is required'],
        ], 400);
    }

    return jsonResponse($response, $result);
});

$app->post('/validated/multipart/medium', function (): Response {
    $response = new Response();
    $result = summarizeFiles($_FILES);

    if ($result['files_received'] === 0) {
        return jsonResponse($response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['files' => 'At least one file is required'],
        ], 400);
    }

    return jsonResponse($response, $result);
});

$app->post('/validated/multipart/large', function (): Response {
    $response = new Response();
    $result = summarizeFiles($_FILES);

    if ($result['files_received'] === 0) {
        return jsonResponse($response, [
            'error' => 'Validation failed',
            'code' => 'VALIDATION_ERROR',
            'details' => ['files' => 'At least one file is required'],
        ], 400);
    }

    return jsonResponse($response, $result);
});

// ============================================================================
// Validated path parameter endpoints
// ============================================================================

$app->get('/validated/path/simple/{id}', function (string $id): Response {
    $response = new Response();
    // Validate: non-empty, alphanumeric with _ or -, max 255 chars
    if (!preg_match('/^[a-zA-Z0-9_-]+$/', $id) || empty($id) || strlen($id) > 255) {
        return jsonResponse($response, [
            'error' => 'Invalid path parameter',
            'code' => 'VALIDATION_ERROR',
            'details' => ['id' => 'must be alphanumeric with _ or -, non-empty, max 255 chars'],
        ], 400);
    }
    return jsonResponse($response, ['id' => $id]);
});

$app->get('/validated/path/multiple/{user_id}/{post_id}', function (string $user_id, string $post_id): Response {
    $response = new Response();
    // Validate user_id
    if (!preg_match('/^[a-zA-Z0-9_-]+$/', $user_id) || empty($user_id) || strlen($user_id) > 255) {
        return jsonResponse($response, [
            'error' => 'Invalid path parameter',
            'code' => 'VALIDATION_ERROR',
            'details' => ['user_id' => 'must be alphanumeric with _ or -, non-empty, max 255 chars'],
        ], 400);
    }
    // Validate post_id
    if (!preg_match('/^[a-zA-Z0-9_-]+$/', $post_id) || empty($post_id) || strlen($post_id) > 255) {
        return jsonResponse($response, [
            'error' => 'Invalid path parameter',
            'code' => 'VALIDATION_ERROR',
            'details' => ['post_id' => 'must be alphanumeric with _ or -, non-empty, max 255 chars'],
        ], 400);
    }
    return jsonResponse($response, ['user_id' => $user_id, 'post_id' => $post_id]);
});

$app->get('/validated/path/deep/{org}/{team}/{project}/{resource}/{id}', function (
    string $org,
    string $team,
    string $project,
    string $resource,
    string $id
): Response {
    $response = new Response();
    // Validate all parameters
    $params = ['org' => $org, 'team' => $team, 'project' => $project, 'resource' => $resource, 'id' => $id];
    foreach ($params as $name => $value) {
        if (!preg_match('/^[a-zA-Z0-9_-]+$/', $value) || empty($value) || strlen($value) > 255) {
            return jsonResponse($response, [
                'error' => 'Invalid path parameter',
                'code' => 'VALIDATION_ERROR',
                'details' => [$name => 'must be alphanumeric with _ or -, non-empty, max 255 chars'],
            ], 400);
        }
    }
    return jsonResponse($response, [
        'org' => $org,
        'team' => $team,
        'project' => $project,
        'resource' => $resource,
        'id' => $id,
    ]);
});

$app->get('/validated/path/int/{id}', function (string $id): Response {
    $response = new Response();
    // Validate and convert integer
    if (filter_var($id, FILTER_VALIDATE_INT) === false) {
        return jsonResponse($response, [
            'error' => 'Invalid integer',
            'code' => 'VALIDATION_ERROR',
            'details' => ['id' => 'must be a valid integer'],
        ], 400);
    }
    return jsonResponse($response, ['id' => (int) $id]);
});

$app->get('/validated/path/uuid/{uuid}', function (string $uuid): Response {
    $response = new Response();
    // Validate UUID (RFC 4122)
    if (!preg_match('/^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$/i', $uuid)) {
        return jsonResponse($response, [
            'error' => 'Invalid UUID',
            'code' => 'VALIDATION_ERROR',
            'details' => ['uuid' => 'must be a valid RFC 4122 UUID'],
        ], 400);
    }
    return jsonResponse($response, ['uuid' => $uuid]);
});

$app->get('/validated/path/date/{date}', function (string $date): Response {
    $response = new Response();
    // Validate date (Y-m-d format)
    $parsed = DateTimeImmutable::createFromFormat('Y-m-d', $date);
    if ($parsed === false || $parsed->format('Y-m-d') !== $date) {
        return jsonResponse($response, [
            'error' => 'Invalid date',
            'code' => 'VALIDATION_ERROR',
            'details' => ['date' => 'must be in Y-m-d format'],
        ], 400);
    }
    return jsonResponse($response, ['date' => $date]);
});

// ============================================================================
// Multipart form endpoints
// ============================================================================

$app->post('/multipart/small', function (): Response {
    $response = new Response();
    return jsonResponse($response, summarizeFiles($_FILES));
});

$app->post('/multipart/medium', function (): Response {
    $response = new Response();
    return jsonResponse($response, summarizeFiles($_FILES));
});

$app->post('/multipart/large', function (): Response {
    $response = new Response();
    return jsonResponse($response, summarizeFiles($_FILES));
});

// ============================================================================
// URL-encoded form endpoints
// ============================================================================

$app->post('/urlencoded/simple', function (): Response {
    $response = new Response();
    return jsonResponse($response, $_POST ?? []);
});

$app->post('/urlencoded/complex', function (): Response {
    $response = new Response();
    return jsonResponse($response, $_POST ?? []);
});

// ============================================================================
// Path parameter endpoints
// ============================================================================

$app->get('/path/simple/{id}', function (string $id): Response {
    $response = new Response();
    return jsonResponse($response, ['id' => $id]);
});

$app->get('/path/multiple/{user_id}/{post_id}', function (string $user_id, string $post_id): Response {
    $response = new Response();
    return jsonResponse($response, ['user_id' => $user_id, 'post_id' => $post_id]);
});

$app->get('/path/deep/{org}/{team}/{project}/{resource}/{id}', function (
    string $org,
    string $team,
    string $project,
    string $resource,
    string $id
): Response {
    $response = new Response();
    return jsonResponse($response, [
        'org' => $org,
        'team' => $team,
        'project' => $project,
        'resource' => $resource,
        'id' => $id,
    ]);
});

$app->get('/path/int/{id}', function (string $id): Response {
    $response = new Response();
    // Raw endpoint - no validation
    return jsonResponse($response, ['id' => $id]);
});

$app->get('/path/uuid/{uuid}', function (string $uuid): Response {
    $response = new Response();
    // Raw endpoint - no validation
    return jsonResponse($response, ['uuid' => $uuid]);
});

$app->get('/path/date/{date}', function (string $date): Response {
    $response = new Response();
    // Raw endpoint - no validation
    return jsonResponse($response, ['date' => $date]);
});

// ============================================================================
// Query parameter endpoints
// ============================================================================

$app->get('/query/few', function (): Response {
    $response = new Response();
    return jsonResponse($response, $_GET ?? []);
});

$app->get('/query/medium', function (): Response {
    $response = new Response();
    return jsonResponse($response, $_GET ?? []);
});

$app->get('/query/many', function (): Response {
    $response = new Response();
    return jsonResponse($response, $_GET ?? []);
});

// ============================================================================
// Server Startup - Swoole HTTP Server (async, multi-coroutine)
// ============================================================================

$port = (int)($argv[1] ?? 8000);

if (class_exists(\OpenSwoole\Http\Server::class)) {
    $server = new \OpenSwoole\Http\Server('0.0.0.0', $port);
    $server->set([
        'worker_num' => openswoole_cpu_num(),
        'enable_coroutine' => true,
        'log_level' => OPENSWOOLE_LOG_WARNING,
    ]);
    $server->on('request', function (\OpenSwoole\Http\Request $swReq, \OpenSwoole\Http\Response $swResp) use ($app, $di): void {
        // Populate PHP superglobals for Phalcon compatibility
        $_SERVER['REQUEST_METHOD'] = $swReq->server['request_method'] ?? 'GET';
        $_SERVER['REQUEST_URI'] = $swReq->server['request_uri'] ?? '/';
        $_SERVER['CONTENT_TYPE'] = $swReq->header['content-type'] ?? '';
        $_GET = $swReq->get ?? [];
        $_POST = $swReq->post ?? [];
        $_FILES = $swReq->files ?? [];

        ob_start();
        try {
            $app->handle($_SERVER['REQUEST_URI']);
            $body = ob_get_clean();
            $response = $app->response;
            $swResp->status($response->getStatusCode() ?: 200);
            $swResp->header('Content-Type', 'application/json');
            $swResp->end($response->getContent() ?: $body);
        } catch (\Throwable $e) {
            ob_end_clean();
            $swResp->status(500);
            $swResp->header('Content-Type', 'application/json');
            $swResp->end(json_encode(['error' => $e->getMessage()]));
        }
    });
    fwrite(STDERR, "[phalcon] Starting Swoole server on 0.0.0.0:$port\n");
    $server->start();
} else {
    // Fallback to built-in PHP server (single-threaded, for local dev only)
    fwrite(STDERR, "[phalcon] Swoole not available, falling back to php -S (single-threaded)\n");

    // When run directly from CLI, spawn php -S pointing back to this script as a router
    if (php_sapi_name() === 'cli') {
        $cmd = sprintf('php -S 0.0.0.0:%d %s', $port, escapeshellarg(__FILE__));
        fwrite(STDERR, "[phalcon] Launching: $cmd\n");
        $exitCode = null;
        passthru($cmd, $exitCode);
        exit($exitCode ?? 1);
    }

    // When invoked as built-in server router, handle the request
    $app->handle($_SERVER['REQUEST_URI'] ?? '/');
}
