Use PHPUnit with the built-in `TestClient`:

```php
use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;

final class ApiTest extends TestCase
{
    public function testGetUser(): void
    {
        $app = (new App())->registerController(new class () {
            #[Get('/users/{id}')]
            public function user(Request $request): Response
            {
                return Response::json(['id' => $request->pathParams['id']]);
            }
        });

        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/123');

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => '123'], $response->body);
    }
}
```

TestClient supports HTTP requests, WebSocket connections, and SSE streams.
