```php
<?php

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

final class UsersController
{
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
        return Response::json($user);
    }
}

$app = $app->registerController(new UsersController());

$app->run();
```
