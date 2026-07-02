```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class UsersController
{
    #[Get('/users')]
    public function list(): Response
    {
        return Response::json([
            'users' => [
                ['id' => 1, 'name' => 'Alice'],
                ['id' => 2, 'name' => 'Bob'],
            ]
        ]);
    }

    #[Get('/users/{id}')]
    public function show(Request $request): Response
    {
        $userId = (int) $request->pathParams['id'];
        return Response::json(['id' => $userId, 'name' => 'Alice']);
    }

    #[Post('/users')]
    public function create(Request $request): Response
    {
        $user = $request->body;
        return Response::json($user, 201);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new UsersController());

$app->run();
```
