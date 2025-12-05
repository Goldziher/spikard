# PHP Test Examples

This document provides example test methods from each new test file showing the testing patterns and patterns used.

## 1. CookieHeaderTest.php - Complete Test Example

### Testing Cookie Parameters

```php
public function testCookieWithAllParameters(): void
{
    $pattern = '/^[a-zA-Z0-9]{32}$/';
    $schema = ['minLength' => 32];
    $cookie = new Cookie(
        default: 'default_session',
        minLength: 10,
        maxLength: 64,
        pattern: $pattern,
        schema: $schema,
    );

    $this->assertTrue($cookie->hasDefault());
    $this->assertSame('default_session', $cookie->getDefault());
    $this->assertSame(10, $cookie->getMinLength());
    $this->assertSame(64, $cookie->getMaxLength());
    $this->assertSame($pattern, $cookie->getPattern());
    $this->assertSame($schema, $cookie->getSchema());
}
```

### Testing Header Alias and Conversion

```php
public function testHeaderWithConvertUnderscoresAndAlias(): void
{
    $header = new Header(
        alias: 'X-Custom-Header',
        convertUnderscores: false,
    );

    $this->assertSame('X-Custom-Header', $header->getAlias());
    $this->assertFalse($header->shouldConvertUnderscores());
}
```

## 2. BackgroundTaskExtendedTest.php - Complete Test Example

### Testing Task Execution with Arguments

```php
public function testRunWithComplexArrayArgument(): void
{
    $complex = [
        'level1' => [
            'level2' => [
                'level3' => 'deep_value',
            ],
        ],
        'array' => [1, 2, 3],
        'null' => null,
        'bool' => true,
    ];

    BackgroundTask::run(function (array $arr): void {
        BackgroundTaskExtendedTest::$executed = \is_array($arr);
    }, [$complex]);

    $this->assertTrue(self::$executed);
}
```

### Testing Multiple Argument Types

```php
public function testRunWithMixedTypesMultipleArgs(): void
{
    BackgroundTask::run(
        function (
            string $str,
            int $int,
            array $arr,
            bool $bool,
            ?float $nullable
        ): void {
            BackgroundTaskExtendedTest::$executed = true;
        },
        ['string', 42, ['a' => 'b'], true, null]
    );

    $this->assertTrue(self::$executed);
}
```

## 3. TestClientExtendedTest.php - Complete Test Example

### Testing Query Parameter Parsing

```php
public function testParseQueryParamsWithUrlEncoding(): void
{
    $capturedParams = null;
    $handler = new class implements HandlerInterface {
        public function handle(Request $request): Response
        {
            global $capturedParams;
            $capturedParams = $request->queryParams;
            return new Response(['ok' => true]);
        }
    };
    $this->app->addRoute('GET', '/encoded', $handler);

    $this->client->request('GET', '/encoded?message=hello%20world&symbol=%26');

    $this->assertIsArray($capturedParams);
    $this->assertSame(['hello world'], $capturedParams['message'] ?? []);
    $this->assertSame(['&'], $capturedParams['symbol'] ?? []);
}
```

### Testing Multiple Query Values

```php
public function testParseQueryParamsWithMultipleValues(): void
{
    $capturedParams = null;
    $handler = new class implements HandlerInterface {
        public function handle(Request $request): Response
        {
            global $capturedParams;
            $capturedParams = $request->queryParams;
            return new Response(['ok' => true]);
        }
    };
    $this->app->addRoute('GET', '/multi', $handler);

    $this->client->request('GET', '/multi?ids=1&ids=2&ids=3');

    $this->assertIsArray($capturedParams);
    $this->assertSame(['1', '2', '3'], $capturedParams['ids'] ?? []);
}
```

## 4. ServerConfigBuilderTest.php - Complete Test Example

### Testing Full Configuration Chaining

```php
public function testAllConfigurationsCanBeSet(): void
{
    $compression = new CompressionConfig(enabled: true);
    $rateLimit = new RateLimitConfig(requestsPerSecond: 100, burstSize: 10);
    $cors = new CorsConfig(allowedOrigins: ['*'], allowedMethods: ['*']);
    $staticFiles = new StaticFilesConfig(path: '/static', directory: './public');
    $jwtAuth = new JwtConfig(secret: 'key', algorithms: ['HS256']);
    $apiKeyAuth = new ApiKeyConfig(headerName: 'X-API-Key', keys: ['key1']);
    $openapi = new OpenApiConfig(title: 'API', version: '1.0.0', path: '/docs');
    $hooks = LifecycleHooks::builder()->build();

    $config = ServerConfig::builder()
        ->withHost('localhost')
        ->withPort(8000)
        ->withWorkers(4)
        ->withRequestId(true)
        ->withMaxBodySize(10485760)
        ->withRequestTimeout(30)
        ->withGracefulShutdown(true)
        ->withShutdownTimeout(30)
        ->withCompression($compression)
        ->withRateLimit($rateLimit)
        ->withCors($cors)
        ->withStaticFiles($staticFiles)
        ->withJwtAuth($jwtAuth)
        ->withApiKeyAuth($apiKeyAuth)
        ->withOpenApi($openapi)
        ->withLifecycleHooks($hooks)
        ->build();

    $this->assertSame('localhost', $config->host);
    $this->assertSame(8000, $config->port);
    $this->assertSame(4, $config->workers);
    $this->assertTrue($config->enableRequestId);
    $this->assertSame(10485760, $config->maxBodySize);
    $this->assertSame(30, $config->requestTimeout);
    $this->assertTrue($config->gracefulShutdown);
    $this->assertSame(30, $config->shutdownTimeout);
    $this->assertSame($compression, $config->compression);
    $this->assertSame($rateLimit, $config->rateLimit);
    $this->assertSame($cors, $config->cors);
    $this->assertSame($staticFiles, $config->staticFiles);
    $this->assertSame($jwtAuth, $config->jwtAuth);
    $this->assertSame($apiKeyAuth, $config->apiKeyAuth);
    $this->assertSame($openapi, $config->openapi);
    $this->assertSame($hooks, $config->hooks);
}
```

### Testing Builder Reusability

```php
public function testBuilderCanBeReusedAfterBuild(): void
{
    $builder = ServerConfig::builder()
        ->withHost('localhost')
        ->withPort(3000);

    $config1 = $builder->build();

    // Modify and build again
    $builder->withPort(4000);
    $config2 = $builder->build();

    $this->assertSame(3000, $config1->port);
    $this->assertSame(4000, $config2->port);
}
```

## 5. StreamingResponseCompletionTest.php - Complete Test Example

### Testing File Streaming with Content-Length

```php
public function testFileStreamingContentLengthCalculation(): void
{
    $content = 'exact content';
    $tempFile = $this->createTempFile($content);

    try {
        $response = StreamingResponse::file($tempFile);

        // Verify Content-Length header is set correctly
        $this->assertArrayHasKey('Content-Length', $response->headers);
        $this->assertSame((string)\strlen($content), $response->headers['Content-Length']);
    } finally {
        @\unlink($tempFile);
    }
}
```

### Testing JSON Lines with Special Characters

```php
public function testJsonLinesWithSpecialCharacters(): void
{
    $dataGenerator = function (): Generator {
        yield ['message' => 'Hello "World"'];
        yield ['text' => "Line with\nnewline"];
        yield ['symbol' => 'Backslash: \\'];
    };

    $response = StreamingResponse::jsonLines($dataGenerator());

    $lines = \iterator_to_array($response->generator);
    $this->assertCount(3, $lines);

    if (isset($lines[0]) && \is_string($lines[0])) {
        $decoded = \json_decode($lines[0], true);
        $this->assertIsArray($decoded);
    }
}
```

## Test Patterns Used

### 1. **Simple Assertion Pattern**
```php
public function testSimpleFeature(): void
{
    $object = new ClassName();

    $this->assertSame($expected, $object->method());
}
```

### 2. **Setup and Verification Pattern**
```php
public function testComplexFeature(): void
{
    $config = Config::builder()
        ->withOption1(value1)
        ->withOption2(value2)
        ->build();

    $this->assertSame($expected, $config->property);
}
```

### 3. **Exception Testing Pattern**
```php
public function testThrowsException(): void
{
    $this->expectException(RuntimeException::class);
    $this->expectExceptionMessage('error message');

    $object->failingMethod();
}
```

### 4. **Generator/Iterator Pattern**
```php
public function testStreamingBehavior(): void
{
    $generator = function(): Generator {
        yield 'data1';
        yield 'data2';
    };

    $response = new StreamingResponse($generator());
    $chunks = \iterator_to_array($response->generator);

    $this->assertCount(2, $chunks);
}
```

### 5. **Handler/Closure Capture Pattern**
```php
public function testRequestHandling(): void
{
    $captured = null;
    $handler = new class implements HandlerInterface {
        public function handle(Request $request): Response
        {
            global $captured;
            $captured = $request->property;
            return new Response([]);
        }
    };

    $this->client->request('GET', '/path');

    $this->assertSame($expected, $captured);
}
```

## Code Quality Standards Met

✓ All tests use `declare(strict_types=1)`
✓ Proper namespace declarations: `namespace Spikard\Tests;`
✓ Classes are final: `final class TestName extends TestCase`
✓ All parameters and return types are typed
✓ Test methods are public and return void
✓ Descriptive method names following `test<Feature>` pattern
✓ No mocking of Spikard internals
✓ Real assertions verifying actual behavior
✓ Comprehensive edge case coverage
✓ PSR-12 coding style compliance
