```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('POST', '/orders/{order_id}', function (Request $request, int $order_id) {
    $order = $request->jsonBody();
    $verbose = $request->query('verbose', false);

    return Response::json([
        'id' => $order_id,
        'item' => $order['item'],
        'quantity' => $order['quantity'],
        'verbose' => $verbose
    ]);
});
```
