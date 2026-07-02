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
    #[Get('/users')]
    public function list(Request $request): Response
    {
        $rules = [
            'page' => ['nullable', 'integer', 'min:1'],
            'limit' => ['nullable', 'integer', 'min:1', 'max:100'],
            'sort_by' => ['nullable', 'string', 'in:name,email,created_at'],
            'min_age' => ['nullable', 'integer', 'min:0', 'max:120'],
        ];

        $validator = new Validator($request->query, $rules);

        if ($validator->fails()) {
            return Response::json(['errors' => $validator->errors()], 400);
        }

        $page = (int) ($request->query['page'] ?? 1);
        $limit = (int) ($request->query['limit'] ?? 10);

        return Response::json([
            'page' => $page,
            'limit' => $limit,
            'users' => [],
        ]);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new UsersController());
```
