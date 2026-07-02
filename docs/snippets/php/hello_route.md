```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\Response;

final class HelloController
{
    #[Get('/')]
    public function index(): Response
    {
        return Response::text('Hello, World!');
    }

    #[Get('/hello/{name}')]
    public function greet(\Spikard\Http\Request $request): Response
    {
        $name = $request->pathParams['name'];
        return Response::json(['message' => "Hello, {$name}!"]);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new HelloController());

$app->run();
```
