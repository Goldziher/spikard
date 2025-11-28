```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

$app = $app->preHandler(function (Request $request) {
    $token = $request->header('authorization');
    if ($token !== 'Bearer dev-token') {
        return Response::json(['error' => 'unauthorized'], 401);
    }
    return $request;
});
```
