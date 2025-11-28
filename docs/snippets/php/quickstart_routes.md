```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('GET', '/users/{id}', function (Request $request) {
    $userId = (int) $request->pathParams['id'];
    return Response::json(['id' => $userId, 'name' => 'Alice']);
});

$app = $app->addRoute('POST', '/users', function (Request $request) {
    $user = $request->body;
    return Response::json($user);
});

$app->run();
```
