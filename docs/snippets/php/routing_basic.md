```php
<?php

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

final class BasicController
{
    #[Get('/health')]
    public function health(): Response
    {
        return Response::json(['status' => 'ok']);
    }

    #[Post('/users')]
    public function create(Request $request): Response
    {
        $user = $request->jsonBody();
        return Response::json($user);
    }
}

$app = $app->registerController(new BasicController());
```
