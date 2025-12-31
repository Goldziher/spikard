```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class OrdersController
{
    #[Get('/orders/{order_id}')]
    public function show(Request $request): Response
    {
        // Path parameters
        $orderId = (int) $request->pathParams['order_id'];

        // Query parameters (array for multi-value support)
        $includeDetails = ($request->queryParams['include_details'][0] ?? 'false') === 'true';
        $limit = (int) ($request->queryParams['limit'][0] ?? '10');

        return Response::json([
            'id' => $orderId,
            'details' => $includeDetails,
            'limit' => $limit
        ]);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new OrdersController());
```
