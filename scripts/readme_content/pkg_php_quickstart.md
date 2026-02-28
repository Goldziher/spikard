```php
<?php

declare(strict_types=1);

require_once 'vendor/autoload.php';

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class UserController
{
    #[Get('/users/{id}')]
    public function get(Request $request): Response
    {
        $userId = (int) $request->pathParams['id'];
        return Response::json([
            'id' => $userId,
            'name' => 'Alice',
            'email' => 'alice@example.com',
        ]);
    }

    #[Post('/users')]
    public function create(Request $request): Response
    {
        $data = $request->jsonBody();

        // Automatic validation
        return Response::json([
            'id' => 1,
            'name' => $data['name'],
            'email' => $data['email'],
        ], 201);
    }
}

$config = new ServerConfig(port: 8000);
$app = (new App($config))->registerController(new UserController());
$app->run();
```
