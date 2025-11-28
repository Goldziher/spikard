```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('GET', '/users/{id}', function (int $id) {
    return Response::json(['id' => $id, 'name' => 'Alice']);
});

$app->run();
```
