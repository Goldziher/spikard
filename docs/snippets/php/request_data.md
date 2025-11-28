```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('POST', '/orders/{order_id}', function (Request $request) {
    $orderId = (int) $request->pathParams['order_id'];
    $order = $request->body;
    $verbose = $request->queryParams['verbose'][0] ?? 'false';

    return Response::json([
        'id' => $orderId,
        'item' => $order['item'] ?? '',
        'quantity' => $order['quantity'] ?? 0,
        'verbose' => $verbose
    ]);
});
```
