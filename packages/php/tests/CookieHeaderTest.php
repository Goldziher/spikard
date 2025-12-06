<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Closure;
use PHPUnit\Framework\TestCase;
use Spikard\Http\Params\Cookie;
use Spikard\Http\Params\Header;

/**
 * Comprehensive tests for Cookie and Header param classes.
 *
 * Tests all public methods and branches for:
 * - Spikard\Http\Params\Cookie (increase from 25% to 80%+)
 * - Spikard\Http\Params\Header (increase from 33.33% to 80%+)
 */
final class CookieHeaderTest extends TestCase
{
    // ======================== Cookie Tests ========================

    public function testCookieBasicConstruction(): void
    {
        $cookie = new Cookie();

        $this->assertFalse($cookie->hasDefault());
        $this->assertNull($cookie->getMinLength());
        $this->assertNull($cookie->getMaxLength());
        $this->assertNull($cookie->getPattern());
        $this->assertNull($cookie->getSchema());
    }

    public function testCookieWithDefault(): void
    {
        $cookie = new Cookie(default: 'session_id_123');

        $this->assertTrue($cookie->hasDefault());
        $this->assertSame('session_id_123', $cookie->getDefault());
        $this->assertSame('session_id_123', $cookie());
    }

    public function testCookieWithDefaultFactory(): void
    {
        $cookie = new Cookie(defaultFactory: fn (): string => \bin2hex(\random_bytes(16)));

        $this->assertTrue($cookie->hasDefault());
        $generated = (string) $cookie();
        $this->assertSame(32, \strlen($generated)); // 16 bytes = 32 hex chars
    }

    public function testCookieWithMinLength(): void
    {
        $cookie = new Cookie(minLength: 10);

        $this->assertSame(10, $cookie->getMinLength());
        $this->assertNull($cookie->getMaxLength());
        $this->assertNull($cookie->getPattern());
    }

    public function testCookieWithMaxLength(): void
    {
        $cookie = new Cookie(maxLength: 256);

        $this->assertNull($cookie->getMinLength());
        $this->assertSame(256, $cookie->getMaxLength());
        $this->assertNull($cookie->getPattern());
    }

    public function testCookieWithMinAndMaxLength(): void
    {
        $cookie = new Cookie(minLength: 10, maxLength: 256);

        $this->assertSame(10, $cookie->getMinLength());
        $this->assertSame(256, $cookie->getMaxLength());
    }

    public function testCookieWithPattern(): void
    {
        $pattern = '/^[a-zA-Z0-9]{32}$/';
        $cookie = new Cookie(pattern: $pattern);

        $this->assertSame($pattern, $cookie->getPattern());
    }

    public function testCookieWithPatternAndLengths(): void
    {
        $pattern = '/^[a-zA-Z0-9]+$/';
        $cookie = new Cookie(
            minLength: 8,
            maxLength: 64,
            pattern: $pattern
        );

        $this->assertSame(8, $cookie->getMinLength());
        $this->assertSame(64, $cookie->getMaxLength());
        $this->assertSame($pattern, $cookie->getPattern());
    }

    public function testCookieWithSchema(): void
    {
        $schema = [
            'type' => 'string',
            'minLength' => 10,
            'maxLength' => 256,
            'pattern' => '^[a-zA-Z0-9]+$',
        ];
        $cookie = new Cookie(schema: $schema);

        $this->assertSame($schema, $cookie->getSchema());
    }

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

    public function testCookieWithFactoryAndLengths(): void
    {
        $counter = 0;
        $cookie = new Cookie(
            defaultFactory: function () use (&$counter): string {
                return 'session_' . (++$counter);
            },
            minLength: 5,
            maxLength: 20,
        );

        $this->assertSame('session_1', $cookie->getDefault());
        $this->assertSame('session_2', $cookie->getDefault());
        $this->assertSame(5, $cookie->getMinLength());
        $this->assertSame(20, $cookie->getMaxLength());
    }

    public function testCookieWithNullDefault(): void
    {
        $cookie = new Cookie(default: null, minLength: 5, maxLength: 50);

        $this->assertFalse($cookie->hasDefault());
        $this->assertSame(5, $cookie->getMinLength());
    }

    public function testCookieInvokeCallsDefault(): void
    {
        $cookie = new Cookie(default: 'test_value');

        $result = $cookie();
        $this->assertSame('test_value', $result);
    }

    public function testCookieInvokeWithFactory(): void
    {
        $counter = 0;
        $cookie = new Cookie(
            defaultFactory: function () use (&$counter): int {
                return ++$counter;
            }
        );

        $this->assertSame(1, $cookie());
        $this->assertSame(2, $cookie());
        $this->assertSame(3, $cookie());
    }

    // ======================== Header Tests ========================

    public function testHeaderBasicConstruction(): void
    {
        $header = new Header();

        $this->assertFalse($header->hasDefault());
        $this->assertNull($header->getDefault());
        $this->assertNull($header->getAlias());
        $this->assertTrue($header->shouldConvertUnderscores());
        $this->assertNull($header->getSchema());
    }

    public function testHeaderWithDefault(): void
    {
        $header = new Header(default: 'application/json');

        $this->assertTrue($header->hasDefault());
        $this->assertSame('application/json', $header->getDefault());
        $this->assertSame('application/json', $header());
    }

    public function testHeaderWithDefaultFactory(): void
    {
        $header = new Header(defaultFactory: fn () => 'Bearer token-' . \time());

        $this->assertTrue($header->hasDefault());
        $default = $header->getDefault();
        $this->assertIsString($default);
        $this->assertStringStartsWith('Bearer token-', $default);
    }

    public function testHeaderWithAlias(): void
    {
        $header = new Header(alias: 'X-API-Key');

        $this->assertSame('X-API-Key', $header->getAlias());
        $this->assertTrue($header->shouldConvertUnderscores());
    }

    public function testHeaderWithAliasAndDefault(): void
    {
        $header = new Header(default: 'secret_key_123', alias: 'X-API-Key');

        $this->assertSame('X-API-Key', $header->getAlias());
        $this->assertTrue($header->hasDefault());
        $this->assertSame('secret_key_123', $header->getDefault());
    }

    public function testHeaderWithConvertUnderscoresTrue(): void
    {
        $header = new Header(convertUnderscores: true);

        $this->assertTrue($header->shouldConvertUnderscores());
    }

    public function testHeaderWithConvertUnderscoresFalse(): void
    {
        $header = new Header(convertUnderscores: false);

        $this->assertFalse($header->shouldConvertUnderscores());
    }

    public function testHeaderWithConvertUnderscoresAndAlias(): void
    {
        $header = new Header(
            alias: 'X-Custom-Header',
            convertUnderscores: false,
        );

        $this->assertSame('X-Custom-Header', $header->getAlias());
        $this->assertFalse($header->shouldConvertUnderscores());
    }

    public function testHeaderWithSchema(): void
    {
        $schema = ['type' => 'string', 'minLength' => 32];
        $header = new Header(schema: $schema);

        $this->assertSame($schema, $header->getSchema());
    }

    public function testHeaderWithAllParameters(): void
    {
        $schema = ['minLength' => 32];
        $header = new Header(
            default: 'Bearer token',
            alias: 'X-API-Key',
            convertUnderscores: false,
            schema: $schema,
        );

        $this->assertTrue($header->hasDefault());
        $this->assertSame('Bearer token', $header->getDefault());
        $this->assertSame('X-API-Key', $header->getAlias());
        $this->assertFalse($header->shouldConvertUnderscores());
        $this->assertSame($schema, $header->getSchema());
    }

    public function testHeaderWithFactoryAndAlias(): void
    {
        $counter = 0;
        $header = new Header(
            defaultFactory: function () use (&$counter): string {
                return 'Bearer token-' . (++$counter);
            },
            alias: 'Authorization',
            convertUnderscores: true,
        );

        $this->assertSame('Bearer token-1', $header->getDefault());
        $this->assertSame('Bearer token-2', $header->getDefault());
        $this->assertSame('Authorization', $header->getAlias());
        $this->assertTrue($header->shouldConvertUnderscores());
    }

    public function testHeaderWithNullDefault(): void
    {
        $header = new Header(default: null, alias: 'X-Optional');

        $this->assertFalse($header->hasDefault());
        $this->assertNull($header->getDefault());
        $this->assertSame('X-Optional', $header->getAlias());
    }

    public function testHeaderInvokeWithDefault(): void
    {
        $header = new Header(default: 'text/html');

        $result = $header();
        $this->assertSame('text/html', $result);
    }

    public function testHeaderInvokeWithFactory(): void
    {
        $counter = 0;
        $header = new Header(
            defaultFactory: function () use (&$counter): int {
                return ++$counter * 100;
            }
        );

        $this->assertSame(100, $header());
        $this->assertSame(200, $header());
        $this->assertSame(300, $header());
    }

    public function testHeaderDefaultConvertUnderscoresIsTrue(): void
    {
        $header = new Header();

        // Verify default value
        $this->assertTrue($header->shouldConvertUnderscores());
    }

    public function testHeaderComplexSchema(): void
    {
        $schema = [
            'type' => 'string',
            'pattern' => '^Bearer [A-Za-z0-9\\-._~+/]+=*$',
            'minLength' => 10,
            'maxLength' => 1000,
        ];
        $header = new Header(schema: $schema);

        $this->assertSame($schema, $header->getSchema());
        $this->assertArrayHasKey('pattern', $header->getSchema());
        $this->assertArrayHasKey('minLength', $header->getSchema());
    }

    // ======================== Cross-Parameter Tests ========================

    public function testCookieAndHeaderShareBaseInterface(): void
    {
        $cookie = new Cookie(default: 'session_123');
        $header = new Header(default: 'application/json');

        // Both should have hasDefault method
        $this->assertTrue($cookie->hasDefault());
        $this->assertTrue($header->hasDefault());

        // Both should have getDefault method
        $this->assertSame('session_123', $cookie->getDefault());
        $this->assertSame('application/json', $header->getDefault());

        // Both should support __invoke
        $this->assertSame('session_123', $cookie());
        $this->assertSame('application/json', $header());
    }

    public function testCookiePatternRegex(): void
    {
        $validPattern = '/^[a-zA-Z0-9]{32}$/';
        $cookie = new Cookie(pattern: $validPattern);

        $retrieved = $cookie->getPattern();
        $this->assertSame($validPattern, $retrieved);

        // Verify the pattern is a valid regex
        $result = @\preg_match($validPattern, 'test');
        $this->assertFalse($result === false);
    }

    public function testHeaderAliasVariations(): void
    {
        $aliases = [
            'X-API-Key',
            'Content-Type',
            'Authorization',
            'X-Custom-Header',
            'Custom_Header',
        ];

        foreach ($aliases as $alias) {
            $header = new Header(alias: $alias);
            $this->assertSame($alias, $header->getAlias());
        }
    }

    public function testCookieLengthConstraints(): void
    {
        $lengths = [
            ['min' => 1, 'max' => 10],
            ['min' => 10, 'max' => 100],
            ['min' => 100, 'max' => 1000],
        ];

        foreach ($lengths as $constraint) {
            $cookie = new Cookie(
                minLength: $constraint['min'],
                maxLength: $constraint['max']
            );

            $this->assertSame($constraint['min'], $cookie->getMinLength());
            $this->assertSame($constraint['max'], $cookie->getMaxLength());
        }
    }

    public function testHeaderFactoryReturnsDifferentValues(): void
    {
        $values = [];
        $header = new Header(
            defaultFactory: function () use (&$values): string {
                $value = 'token_' . \count($values);
                $values[] = $value;
                return $value;
            }
        );

        $val1 = $header->getDefault();
        $val2 = $header->getDefault();
        $val3 = $header->getDefault();

        $this->assertSame('token_0', $val1);
        $this->assertSame('token_1', $val2);
        $this->assertSame('token_2', $val3);
        $this->assertCount(3, $values);
    }
}
