```php
<?php

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App(new ServerConfig(port: 8000));

final class PaymentsController
{
    #[Post('/payments')]
    public function create(Request $request): Response
    {
        $payment = $request->body;

        // Validation
        if (!isset($payment['id'], $payment['amount'])) {
            return Response::json(['error' => 'Invalid payment data'], 400);
        }

        return Response::json($payment);
    }
}

$app = $app->registerController(new PaymentsController());
```
