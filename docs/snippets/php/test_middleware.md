```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Testing\TestClient;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\HookResult;

final class AuthMiddlewareTest extends TestCase
{
    public function testEnforcesAuthMiddleware(): void
    {
        $hooks = LifecycleHooks::builder()
            ->withOnRequest(function ($request): HookResult {
                $token = $request->headers['authorization'] ?? '';
                if (!str_starts_with($token, 'Bearer ')) {
                    return HookResult::respond(['error' => 'Unauthorized'], 401);
                }
                return HookResult::continue();
            })
            ->build();

        $app = new App();
        $app->withLifecycleHooks($hooks);
        $app->get('/protected', fn() => ['data' => 'secret']);

        $client = new TestClient($app);

        // Without auth
        $response = $client->get('/protected');
        $this->assertEquals(401, $response->getStatusCode());

        // With auth
        $response = $client->get('/protected', [
            'headers' => ['authorization' => 'Bearer token123'],
        ]);
        $this->assertEquals(200, $response->getStatusCode());
    }
}
```
