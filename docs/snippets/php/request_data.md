```php
<?php

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

final class OrderController
{
    #[Post('/orders/{order_id}')]
    public function create(Request $request): Response
    {
        $orderId = (int) $request->pathParams['order_id'];
        $order = $request->body;
        $verbose = $request->queryParams['verbose'][0] ?? 'false';

        return Response::json([
            'id' => $orderId,
            'item' => $order['item'] ?? '',
            'quantity' => $order['quantity'] ?? 0,
            'verbose' => $verbose
        ]);
    }
}

$app = $app->registerController(new OrderController());
```
