```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('GET', '/orders/{order_id}', function (Request $request, int $order_id) {
    $includeDetails = $request->query('include_details', false);

    return Response::json([
        'id' => $order_id,
        'details' => $includeDetails
    ]);
});
```
