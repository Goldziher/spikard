<?php

declare(strict_types=1);

use Phalcon\Di\FactoryDefault;
use Phalcon\Http\Request;
use Phalcon\Http\Response;
use Phalcon\Mvc\Micro;
use Phalcon\Validation;
use Phalcon\Validation\Validator\PresenceOf;
use Phalcon\Validation\Validator\Numericality;

$di = new FactoryDefault();
$app = new Micro($di);

/**
 * Parse JSON request body
 */
function jsonBody(Request $request): array
{
    $input = $request->getRawBody();
    if ($input === '' || $input === null) {
        return [];
    }
    $decoded = json_decode($input, true);
    return is_array($decoded) ? $decoded : [];
}

/**
 * Send JSON response
 */
function jsonResponse(Response $response, mixed $data, int $status = 200): Response
{
    $response->setStatusCode($status);
    $response->setContentType('application/json', 'UTF-8');
    $response->setJsonContent($data);
    return $response;
}

/**
 * Validate small payload
 */
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

    return ['valid' => true, 'data' => $data];
}

/**
 * Validate medium payload (with nested image object)
 */
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

/**
 * Validate large payload (with nested seller -> address -> country)
 */
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

/**
 * Validate very large payload (with arrays)
 */
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

// JSON body routes with validation
$app->post('/json/small', function () use ($app) {
    $data = jsonBody($app->request);
    $result = validateSmallPayload($data);

    if (!$result['valid']) {
        return jsonResponse($app->response, ['errors' => $result['errors']], 400);
    }

    return jsonResponse($app->response, $result['data']);
});

$app->post('/json/medium', function () use ($app) {
    $data = jsonBody($app->request);
    $result = validateMediumPayload($data);

    if (!$result['valid']) {
        return jsonResponse($app->response, ['errors' => $result['errors']], 400);
    }

    return jsonResponse($app->response, $result['data']);
});

$app->post('/json/large', function () use ($app) {
    $data = jsonBody($app->request);
    $result = validateLargePayload($data);

    if (!$result['valid']) {
        return jsonResponse($app->response, ['errors' => $result['errors']], 400);
    }

    return jsonResponse($app->response, $result['data']);
});

$app->post('/json/very-large', function () use ($app) {
    $data = jsonBody($app->request);
    $result = validateVeryLargePayload($data);

    if (!$result['valid']) {
        return jsonResponse($app->response, ['errors' => $result['errors']], 400);
    }

    return jsonResponse($app->response, $result['data']);
});

// Multipart form routes (mock responses - validation not applicable)
$app->post('/multipart/small', function () use ($app) {
    return jsonResponse($app->response, ['files_received' => 1, 'total_bytes' => 1024]);
});

$app->post('/multipart/medium', function () use ($app) {
    return jsonResponse($app->response, ['files_received' => 2, 'total_bytes' => 10240]);
});

$app->post('/multipart/large', function () use ($app) {
    return jsonResponse($app->response, ['files_received' => 5, 'total_bytes' => 102400]);
});

// URL-encoded routes (echo back parsed data)
$app->post('/urlencoded/simple', function () use ($app) {
    $data = $app->request->getPost();
    return jsonResponse($app->response, $data);
});

$app->post('/urlencoded/complex', function () use ($app) {
    $data = $app->request->getPost();
    return jsonResponse($app->response, $data);
});

// Path parameter routes (no validation for path params in this benchmark)
$app->get('/path/simple/{id}', function (string $id) use ($app) {
    return jsonResponse($app->response, ['id' => $id]);
});

$app->get('/path/multiple/{user_id}/{post_id}', function (string $user_id, string $post_id) use ($app) {
    return jsonResponse($app->response, ['user_id' => $user_id, 'post_id' => $post_id]);
});

$app->get('/path/deep/{org}/{team}/{project}/{resource}/{id}', function (
    string $org,
    string $team,
    string $project,
    string $resource,
    string $id
) use ($app) {
    return jsonResponse($app->response, [
        'org' => $org,
        'team' => $team,
        'project' => $project,
        'resource' => $resource,
        'id' => $id,
    ]);
});

$app->get('/path/int/{id:[0-9]+}', function (string $id) use ($app) {
    return jsonResponse($app->response, ['id' => (int) $id]);
});

$app->get('/path/uuid/{id}', function (string $id) use ($app) {
    return jsonResponse($app->response, ['id' => $id]);
});

$app->get('/path/date/{date}', function (string $date) use ($app) {
    return jsonResponse($app->response, ['date' => $date]);
});

// Query parameter routes (echo back query params)
$app->get('/query/few', function () use ($app) {
    $query = $app->request->getQuery();
    return jsonResponse($app->response, $query);
});

$app->get('/query/medium', function () use ($app) {
    $query = $app->request->getQuery();
    return jsonResponse($app->response, $query);
});

$app->get('/query/many', function () use ($app) {
    $query = $app->request->getQuery();
    return jsonResponse($app->response, $query);
});

// Health check routes
$app->get('/health', function () use ($app) {
    return jsonResponse($app->response, ['status' => 'ok']);
});

$app->get('/', function () use ($app) {
    return jsonResponse($app->response, ['status' => 'ok']);
});

$app->handle($_SERVER['REQUEST_URI']);
