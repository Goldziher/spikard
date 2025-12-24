```php
<?php

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

final class OrdersController
{
    #[Get('/orders/{order_id}')]
    public function show(Request $request): Response
    {
        $orderId = (int) $request->pathParams['order_id'];
        $includeDetails = $request->queryParams['include_details'][0] ?? 'false';

        return Response::json([
            'id' => $orderId,
            'details' => $includeDetails === 'true'
        ]);
    }
}

$app = $app->registerController(new OrdersController());
```
