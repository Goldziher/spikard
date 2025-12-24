```php
<?php

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

final class UserController
{
    #[Get('/users/{id}')]
    public function show(Request $request): Response
    {
        $userId = (int) $request->pathParams['id'];
        return Response::json(['id' => $userId, 'name' => 'Alice']);
    }
}

$app = $app->registerController(new UserController());

$app->run();
```
