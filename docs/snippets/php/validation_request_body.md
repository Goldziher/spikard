```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Validation\Validator;

final class CreateUserRequest
{
    public function __construct(
        public readonly string $email,
        public readonly int $age,
        public readonly string $username,
    ) {}

    public static function rules(): array
    {
        return [
            'email' => ['required', 'email'],
            'age' => ['required', 'integer', 'min:18'],
            'username' => ['required', 'string', 'regex:/^[a-zA-Z0-9_]+$/'],
        ];
    }
}

final class UsersController
{
    #[Post('/users')]
    public function create(Request $request): Response
    {
        $validator = new Validator($request->body, CreateUserRequest::rules());

        if ($validator->fails()) {
            return Response::json(['errors' => $validator->errors()], 400);
        }

        return Response::json([
            'id' => 'usr_123',
            'email' => $request->body['email'],
            'age' => $request->body['age'],
            'username' => $request->body['username'],
        ], 201);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new UsersController());
```
