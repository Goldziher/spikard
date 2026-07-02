```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Validation\Validator;

final class UsersController
{
    private function userResponseRules(): array
    {
        return [
            'id' => ['required', 'string'],
            'email' => ['required', 'email'],
            'age' => ['required', 'integer'],
        ];
    }

    private function userListResponseRules(): array
    {
        return [
            'users' => ['required', 'array'],
            'users.*.id' => ['required', 'string'],
            'users.*.email' => ['required', 'email'],
            'users.*.age' => ['required', 'integer'],
            'total' => ['required', 'integer'],
            'page' => ['required', 'integer'],
        ];
    }

    #[Get('/users')]
    public function list(Request $request): Response
    {
        $users = [
            ['id' => 'usr_1', 'email' => 'alice@example.com', 'age' => 30],
            ['id' => 'usr_2', 'email' => 'bob@example.com', 'age' => 25],
        ];

        $response = [
            'users' => $users,
            'total' => count($users),
            'page' => 1,
        ];

        // Validate response before returning
        $validator = new Validator($response, $this->userListResponseRules());

        if ($validator->fails()) {
            return Response::json([
                'error' => 'Response validation failed',
                'details' => $validator->errors(),
            ], 500);
        }

        return Response::json($response);
    }

    #[Get('/invalid')]
    public function invalid(Request $request): Response
    {
        $response = ['id' => 'usr_1', 'email' => 'test@example.com'];
        // Missing 'age' field

        $validator = new Validator($response, $this->userResponseRules());

        if ($validator->fails()) {
            return Response::json([
                'error' => 'Response validation failed',
                'details' => $validator->errors(),
            ], 500);
        }

        return Response::json($response);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new UsersController());
```
