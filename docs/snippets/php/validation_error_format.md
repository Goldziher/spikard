```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Validation\Validator;
use Spikard\Validation\ValidationError;

function formatValidationErrors(Validator $validator): array
{
    $details = [];

    foreach ($validator->errors() as $field => $messages) {
        foreach ($messages as $message) {
            $details[] = [
                'field' => $field,
                'message' => $message,
                'type' => 'validation_error',
            ];
        }
    }

    return [
        'error' => 'validation_failed',
        'message' => 'Request validation failed',
        'details' => $details,
    ];
}

final class UsersController
{
    #[Post('/users')]
    public function create(Request $request): Response
    {
        $rules = [
            'email' => ['required', 'email'],
            'age' => ['required', 'integer', 'min:18'],
            'username' => ['required', 'string', 'regex:/^[a-zA-Z0-9_]+$/'],
        ];

        $validator = new Validator($request->body, $rules);

        if ($validator->fails()) {
            return Response::json(formatValidationErrors($validator), 422);
        }

        // Process valid request
        return Response::json([
            'id' => 'usr_123',
            'email' => $request->body['email'],
        ], 201);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new UsersController());
```
