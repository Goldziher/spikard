```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('GET', '/health', function () {
    return Response::json(['status' => 'ok']);
});

$app = $app->addRoute('POST', '/users', function (Request $request) {
    $user = $request->jsonBody();
    return Response::json($user);
});
```
