```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('GET', '/orders/{order_id}', function (Request $request) {
    $orderId = (int) $request->pathParams['order_id'];
    $includeDetails = $request->queryParams['include_details'][0] ?? 'false';

    return Response::json([
        'id' => $orderId,
        'details' => $includeDetails === 'true'
    ]);
});
```
