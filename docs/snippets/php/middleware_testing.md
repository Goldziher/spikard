```php
<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Config\HookResult;

final class AuthGuardTest extends TestCase
{
    public function testAllowsValidToken(): void
    {
        $request = new Request(
            method: 'GET',
            path: '/api/users',
            headers: ['authorization' => 'Bearer valid-jwt-token']
        );

        $result = $this->authGuard($request);

        $this->assertInstanceOf(HookResult::class, $result);
        $this->assertArrayHasKey('user_id', $request->context);
    }

    public function testRejectsMissingToken(): void
    {
        $request = new Request(
            method: 'GET',
            path: '/api/users',
            headers: []
        );

        $result = $this->authGuard($request);

        $this->assertTrue($result->isShortCircuit());
        $this->assertEquals(401, $result->getResponse()->status);
    }

    private function authGuard(Request $request): HookResult
    {
        // Your auth guard implementation
        $authHeader = $request->headers['authorization'] ?? '';

        if (!str_starts_with($authHeader, 'Bearer ')) {
            return HookResult::shortCircuit(
                Response::json(['error' => 'Unauthorized'], 401)
            );
        }

        $request->context['user_id'] = 'extracted-user-id';
        return HookResult::continue();
    }
}
```
