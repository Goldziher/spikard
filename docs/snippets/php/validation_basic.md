```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class PaymentsController
{
    #[Post('/payments')]
    public function create(Request $request): Response
    {
        $payment = $request->body;

        // Manual validation
        $errors = [];
        if (!isset($payment['id']) || !is_string($payment['id'])) {
            $errors[] = 'id is required and must be a string';
        }
        if (!isset($payment['amount']) || !is_numeric($payment['amount'])) {
            $errors[] = 'amount is required and must be numeric';
        }
        if (isset($payment['amount']) && $payment['amount'] <= 0) {
            $errors[] = 'amount must be positive';
        }

        if (!empty($errors)) {
            return Response::json(['errors' => $errors], 400);
        }

        return Response::json([
            'id' => $payment['id'],
            'amount' => (float) $payment['amount'],
            'status' => 'pending'
        ], 201);
    }
}

$app = (new App(new ServerConfig(port: 8000)))
    ->registerController(new PaymentsController());
```
