```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Attributes\Put;
use Spikard\Attributes\Delete;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class ResourceController
{
    #[Get('/items')]
    public function list(): Response
    {
        return Response::json(['items' => []]);
    }

    #[Post('/items')]
    public function create(Request $request): Response
    {
        return Response::json($request->body, 201);
    }

    #[Put('/items/{id}')]
    public function update(Request $request): Response
    {
        $id = (int) $request->pathParams['id'];
        return Response::json(['id' => $id, 'updated' => true]);
    }

    #[Delete('/items/{id}')]
    public function delete(Request $request): Response
    {
        return Response::json(null, 204);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new ResourceController());
```
