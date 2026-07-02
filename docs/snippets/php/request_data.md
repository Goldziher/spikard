```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class OrderController
{
    #[Post('/orders/{order_id}')]
    public function create(Request $request): Response
    {
        // Path parameters
        $orderId = (int) $request->pathParams['order_id'];

        // Request body (parsed JSON)
        $order = $request->body;
        $item = $order['item'] ?? '';
        $quantity = (int) ($order['quantity'] ?? 0);

        // Query parameters
        $verbose = ($request->queryParams['verbose'][0] ?? 'false') === 'true';

        // Headers
        $contentType = $request->headers['content-type'] ?? 'unknown';

        return Response::json([
            'id' => $orderId,
            'item' => $item,
            'quantity' => $quantity,
            'verbose' => $verbose,
            'content_type' => $contentType
        ]);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new OrderController());
```
