```php
<?php

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

$app = $app->addRoute('POST', '/payments', function (Request $request) {
    $payment = $request->jsonBody();

    // Validation
    if (!isset($payment['id'], $payment['amount'])) {
        return Response::json(['error' => 'Invalid payment data'], 400);
    }

    return Response::json($payment);
});
```
