```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;

$app = new App(new ServerConfig(port: 8000));

$app = $app->onRequest(function (Request $request) {
    error_log("{$request->method()} {$request->path()}");
    return $request;
});
```
