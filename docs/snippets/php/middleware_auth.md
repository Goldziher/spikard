```php
<?php

declare(strict_types=1);

use Firebase\JWT\JWT;
use Firebase\JWT\Key;
use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\HookResult;
use Spikard\Http\Request;
use Spikard\Http\Response;

$hooks = LifecycleHooks::builder()
    ->withPreHandler(function (Request $request): HookResult {
        // Extract token from Authorization header
        $authHeader = $request->headers['authorization'] ?? '';

        if (!str_starts_with($authHeader, 'Bearer ')) {
            return HookResult::shortCircuit(
                Response::json(['error' => 'Missing or invalid authorization header'], 401)
            );
        }

        $token = substr($authHeader, 7); // Strip "Bearer "

        try {
            // Verify and decode JWT
            $payload = JWT::decode($token, new Key('your-secret-key', 'HS256'));

            // Enrich context with authenticated user
            $request->context['user_id'] = $payload->sub;
            $request->context['roles'] = $payload->roles ?? [];

            return HookResult::continue();
        } catch (\Exception $e) {
            return HookResult::shortCircuit(
                Response::json(['error' => 'Invalid token'], 401)
            );
        }
    })
    ->build();

$app = (new App(new ServerConfig(port: 8000)))
    ->withLifecycleHooks($hooks);
```
