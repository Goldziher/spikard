<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Closure;
use InvalidArgumentException;
use PHPUnit\Framework\TestCase;
use Spikard\Http\Params\Body;
use Spikard\Http\Params\Cookie;
use Spikard\Http\Params\Header;
use Spikard\Http\Params\Path;
use Spikard\Http\Params\Query;

final class ParamsTest extends TestCase
{
    // Query parameter tests
    public function testQueryWithStaticDefault(): void
    {
        $query = new Query(default: 10);

        $this->assertTrue($query->hasDefault());
        $this->assertSame(10, $query->getDefault());
        $this->assertSame(10, $query()); // Test __invoke
        $this->assertNull($query->getSchema());
    }

    public function testQueryWithDefaultFactory(): void
    {
        $query = new Query(defaultFactory: fn () => [1, 2, 3]);

        $this->assertTrue($query->hasDefault());
        $this->assertSame([1, 2, 3], $query->getDefault());
        $this->assertSame([1, 2, 3], $query());
    }

    public function testQueryWithSchema(): void
    {
        $schema = ['minimum' => 1, 'maximum' => 100];
        $query = new Query(default: 10, schema: $schema);

        $this->assertSame($schema, $query->getSchema());
        $this->assertSame(10, $query->getDefault());
    }

    public function testQueryWithoutDefaults(): void
    {
        $query = new Query();

        $this->assertFalse($query->hasDefault());
        $this->assertSame(null, $query->getDefault());
        $this->assertSame(null, $query());
    }

    public function testQueryCannotHaveBothDefaultAndFactory(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('Cannot specify both "default" and "defaultFactory"');

        new Query(default: 10, defaultFactory: fn () => 20);
    }

    // Path parameter tests
    public function testPathWithNoDefaults(): void
    {
        $path = new Path();

        $this->assertFalse($path->hasDefault());
        $this->assertSame(null, $path->getDefault());
    }

    public function testPathWithSchema(): void
    {
        $schema = ['type' => 'integer', 'minimum' => 1];
        $path = new Path(schema: $schema);

        $this->assertSame($schema, $path->getSchema());
    }

    public function testPathWithDefault(): void
    {
        // Rarely used but should work
        $path = new Path(default: 1);

        $this->assertTrue($path->hasDefault());
        $this->assertSame(1, $path->getDefault());
    }

    // Header parameter tests
    public function testHeaderWithDefault(): void
    {
        $header = new Header(default: 'application/json');

        $this->assertTrue($header->hasDefault());
        $this->assertSame('application/json', $header->getDefault());
    }

    public function testHeaderWithFactory(): void
    {
        $header = new Header(defaultFactory: fn () => 'Bearer token-' . time());

        $this->assertTrue($header->hasDefault());
        $this->assertIsString($header->getDefault());
        $this->assertStringStartsWith('Bearer token-', $header->getDefault());
    }

    public function testHeaderWithSchema(): void
    {
        $schema = ['pattern' => '^Bearer .+$'];
        $header = new Header(schema: $schema);

        $this->assertSame($schema, $header->getSchema());
    }

    // Cookie parameter tests
    public function testCookieWithDefault(): void
    {
        $cookie = new Cookie(default: 'session_id_123');

        $this->assertTrue($cookie->hasDefault());
        $this->assertSame('session_id_123', $cookie->getDefault());
    }

    public function testCookieWithFactory(): void
    {
        $cookie = new Cookie(defaultFactory: fn () => bin2hex(random_bytes(16)));

        $this->assertTrue($cookie->hasDefault());
        // Verify the factory can be invoked and produces output
        $factory = fn () => bin2hex(random_bytes(16));
        $result = $factory();
        $this->assertSame(32, strlen($result)); // 16 bytes = 32 hex chars
    }

    // Body parameter tests
    public function testBodyWithDefault(): void
    {
        $defaultBody = ['key' => 'value'];
        $body = new Body(default: $defaultBody);

        $this->assertTrue($body->hasDefault());
        $this->assertSame($defaultBody, $body->getDefault());
    }

    public function testBodyWithFactory(): void
    {
        $body = new Body(defaultFactory: fn () => ['timestamp' => time()]);

        $this->assertTrue($body->hasDefault());
        // Verify the factory can be invoked and produces expected output
        $factory = fn () => ['timestamp' => time()];
        $result = $factory();
        $this->assertArrayHasKey('timestamp', $result);
    }

    public function testBodyWithSchema(): void
    {
        $schema = [
            'type' => 'object',
            'properties' => ['name' => ['type' => 'string']],
            'required' => ['name'],
        ];
        $body = new Body(schema: $schema);

        $this->assertSame($schema, $body->getSchema());
    }

    // Cross-cutting tests for all param types
    public function testAllParamTypesShareBaseInterface(): void
    {
        $query = new Query(default: 1);
        $path = new Path(default: 2);
        $header = new Header(default: 'h');
        $cookie = new Cookie(default: 'c');
        $body = new Body(default: ['b']);

        $this->assertTrue($query->hasDefault());
        $this->assertTrue($path->hasDefault());
        $this->assertTrue($header->hasDefault());
        $this->assertTrue($cookie->hasDefault());
        $this->assertTrue($body->hasDefault());

        $this->assertSame(1, $query());
        $this->assertSame(2, $path());
        $this->assertSame('h', $header());
        $this->assertSame('c', $cookie());
        $this->assertSame(['b'], $body());
    }

    public function testDefaultFactoryIsCalledEachTime(): void
    {
        $counter = 0;
        $query = new Query(defaultFactory: function () use (&$counter): int {
            return ++$counter;
        });

        $this->assertSame(1, $query->getDefault());
        $this->assertSame(2, $query->getDefault());
        $this->assertSame(3, $query());
    }

    public function testComplexDefaultFactoryWithClosure(): void
    {
        $config = ['multiplier' => 10];
        $query = new Query(defaultFactory: fn () => $config['multiplier'] * 2);

        $this->assertSame(20, $query->getDefault());
    }

    public function testNullDefaultValue(): void
    {
        $query = new Query(default: null);

        // null is a valid default, but hasDefault checks for non-null
        $this->assertFalse($query->hasDefault());
        $this->assertSame(null, $query->getDefault());
    }

    public function testComplexArrayDefault(): void
    {
        $defaultValue = [
            'nested' => ['array' => [1, 2, 3]],
            'bool' => true,
            'string' => 'value',
        ];
        $query = new Query(default: $defaultValue);

        $this->assertSame($defaultValue, $query->getDefault());
    }

    public function testSchemaCanBeComplex(): void
    {
        $schema = [
            'type' => 'object',
            'properties' => [
                'id' => ['type' => 'integer', 'minimum' => 1],
                'name' => ['type' => 'string', 'minLength' => 1],
                'tags' => ['type' => 'array', 'items' => ['type' => 'string']],
            ],
            'required' => ['id', 'name'],
            'additionalProperties' => false,
        ];
        $body = new Body(schema: $schema);

        $this->assertSame($schema, $body->getSchema());
    }
}
