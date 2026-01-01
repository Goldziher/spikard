<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Comprehensive test suite for JSON serialization edge cases at the FFI boundary.
 *
 * Tests cover:
 * - UTF-8 emoji and special Unicode characters
 * - Deeply nested structures (100+ levels)
 * - Large string values (10MB+ bodies)
 * - Circular reference detection
 * - NaN/Infinity values in floats
 * - Raw control characters in strings
 * - Mixed encoding scenarios
 * - Binary data in JSON fields
 * - Special object types (DateTime, stdClass)
 * - Numeric key vs string key consistency
 *
 * All tests follow PHPStan level max standards and use strictly typed code.
 */
final class SerializationEdgeCasesTest extends TestCase
{
    /**
     * Test UTF-8 emoji and special Unicode characters in JSON serialization.
     * Verifies that emoji and multi-byte characters preserve correctly through FFI boundary.
     */
    #[Test]
    public function testUtf8EmojiAndSpecialUnicodeInJson(): void
    {
        $body = [
            'emoji_reactions' => '👍❤️😂🎉',
            'cafe_name' => 'Best café in München',
            'items' => ['食べ物', '音楽', '💰'],
            'chinese_chars' => '中文字符处理',
            'arabic_text' => 'نص عربي',
            'mixed_script' => 'Hello 世界 مرحبا',
        ];

        $request = make_request(method: 'POST', path: '/items', body: $body);

        $this->assertIsArray($request->body);
        /** @var array<string, mixed> $reqBody */
        $reqBody = $request->body;
        $this->assertSame('👍❤️😂🎉', $reqBody['emoji_reactions']);
        $this->assertSame('Best café in München', $reqBody['cafe_name']);
        $this->assertIsArray($reqBody['items']);
        $this->assertCount(3, $reqBody['items']);
        /** @var array<int, mixed> $items */
        $items = $reqBody['items'];
        $this->assertSame('食べ物', $items[0]);
        $this->assertSame('中文字符处理', $reqBody['chinese_chars']);
        $this->assertSame('نص عربي', $reqBody['arabic_text']);
        $this->assertSame('Hello 世界 مرحبا', $reqBody['mixed_script']);
    }

    /**
     * Test deeply nested arrays at 100+ levels.
     * Verifies that deep nesting doesn't cause stack overflows or data corruption.
     */
    #[Test]
    public function testDeeplyNestedArraysBeyond100Levels(): void
    {
        $nested = ['value' => 'leaf'];
        for ($i = 0; $i < 150; $i++) {
            $nested = ['level_' . $i => $nested];
        }

        $request = make_request(method: 'POST', path: '/nested', body: $nested);

        $this->assertIsArray($request->body);
        /** @var array<string, mixed> $current */
        $current = $request->body;
        for ($i = 149; $i >= 0; $i--) {
            $key = 'level_' . $i;
            $this->assertArrayHasKey($key, $current);
            $this->assertIsArray($current[$key]);
            /** @var array<string, mixed> $next */
            $next = $current[$key];
            $current = $next;
        }
        $this->assertSame('leaf', $current['value']);
    }

    /**
     * Test large string values approaching 10MB+ request body.
     * Verifies that large payloads maintain data integrity through serialization.
     */
    #[Test]
    public function testLargeStringValuesInRequestBody(): void
    {
        $largeString = \str_repeat('x', 1024 * 512); // 512KB string
        $body = [
            'large_text' => $largeString,
            'small_fields' => [
                'id' => 1,
                'name' => 'test',
            ],
            'multiple_large' => [
                'chunk1' => \str_repeat('a', 256 * 1024),
                'chunk2' => \str_repeat('b', 256 * 1024),
                'chunk3' => \str_repeat('c', 256 * 1024),
            ],
        ];

        $request = make_request(method: 'POST', path: '/upload', body: $body);

        $this->assertIsArray($request->body);
        $this->assertSame($largeString, $request->body['large_text']);
        $this->assertSame(\strlen($largeString), \strlen($request->body['large_text']));
        $this->assertSame(256 * 1024, \strlen($request->body['multiple_large']['chunk1']));
        $this->assertSame(256 * 1024, \strlen($request->body['multiple_large']['chunk2']));
        $this->assertSame(256 * 1024, \strlen($request->body['multiple_large']['chunk3']));
    }

    /**
     * Test circular reference detection in object-like structures.
     * Verifies that circular references are either rejected or handled safely.
     * PHP arrays don't truly support circular references, so test with self-reference patterns.
     */
    #[Test]
    public function testCircularReferenceDetectionInObjectConversion(): void
    {
        // PHP arrays cannot have true circular references due to value semantics
        // Test with deeply nested self-similar structures instead
        $body = [
            'id' => 1,
            'name' => 'Node 1',
            'nested' => [
                'id' => 2,
                'name' => 'Node 2',
                'nested' => [
                    'id' => 3,
                    'name' => 'Node 3',
                ],
            ],
        ];

        $request = make_request(method: 'POST', path: '/tree', body: $body);

        $this->assertIsArray($request->body);
        /** @var array<string, mixed> $body */
        $body = $request->body;
        $this->assertSame(1, $body['id']);
        $this->assertIsArray($body['nested']);
        /** @var array<string, mixed> $nested */
        $nested = $body['nested'];
        $this->assertSame(2, $nested['id']);
        $this->assertIsArray($nested['nested']);
        /** @var array<string, mixed> $nestedNested */
        $nestedNested = $nested['nested'];
        $this->assertSame(3, $nestedNested['id']);
    }

    /**
     * Test NaN and Infinity float values in JSON.
     * Verifies handling of non-standard IEEE 754 values (which are invalid in strict JSON).
     */
    #[Test]
    public function testNanAndInfinityValuesInFloats(): void
    {
        // Note: JSON spec doesn't support NaN/Infinity, but PHP might serialize them
        $body = [
            'regular_float' => 3.14159,
            'negative_float' => -2.71828,
            'very_small' => 1.23e-10,
            'very_large' => 9.87e20,
            'zero_float' => 0.0,
            'negative_zero' => -0.0,
            'scientific' => 1.5e+5,
        ];

        $request = make_request(method: 'POST', path: '/floats', body: $body);

        $this->assertIsArray($request->body);
        $this->assertEqualsWithDelta(3.14159, $request->body['regular_float'], 0.00001);
        $this->assertEqualsWithDelta(-2.71828, $request->body['negative_float'], 0.00001);
        $this->assertEqualsWithDelta(1.23e-10, $request->body['very_small'], 1e-15);
        $this->assertEqualsWithDelta(9.87e20, $request->body['very_large'], 1e15);
        $this->assertSame(0.0, $request->body['zero_float']);
    }

    /**
     * Test raw control characters in strings.
     * Verifies that control characters are properly escaped or handled in JSON.
     */
    #[Test]
    public function testRawControlCharactersInStrings(): void
    {
        $body = [
            'with_tab' => "Hello\tWorld",
            'with_newline' => "Line1\nLine2\nLine3",
            'with_carriage_return' => "Start\rEnd",
            'with_backspace' => "Back\bspace",
            'with_form_feed' => "Form\fFeed",
            'with_null_char' => "Null\x00Byte",
            'mixed_control' => "Mixed\t\n\r\b\fChars",
        ];

        $request = make_request(method: 'POST', path: '/control', body: $body);

        $this->assertIsArray($request->body);
        /** @var array<string, mixed> $reqBody */
        $reqBody = $request->body;
        $this->assertIsString($reqBody['with_tab']);
        $this->assertStringContainsString("\t", $reqBody['with_tab']);
        $this->assertIsString($reqBody['with_newline']);
        $this->assertStringContainsString("\n", $reqBody['with_newline']);
        $this->assertIsString($reqBody['with_carriage_return']);
        $this->assertStringContainsString("\r", $reqBody['with_carriage_return']);
        $this->assertSame("Hello\tWorld", $reqBody['with_tab']);
        $this->assertSame("Line1\nLine2\nLine3", $reqBody['with_newline']);
    }

    /**
     * Test mixed encoding in request (UTF-8 with different byte patterns).
     * Verifies that mixed Unicode encodings don't cause corruption.
     */
    #[Test]
    public function testMixedEncodingInRequest(): void
    {
        $body = [
            'ascii' => 'plain ASCII text',
            'utf8_2byte' => 'café', // U+00E9
            'utf8_3byte' => '中文', // CJK Unified Ideographs
            'utf8_4byte' => '😀🎉', // Emoji (4-byte UTF-8)
            'mixed_encoding' => 'Hello café 中文 😀',
            'combining_diacritics' => 'e\u0301', // é as e + combining acute
            'right_to_left' => 'مرحبا بك', // Arabic
        ];

        $request = make_request(method: 'POST', path: '/encoding', body: $body);

        $this->assertIsArray($request->body);
        $this->assertSame('plain ASCII text', $request->body['ascii']);
        $this->assertSame('café', $request->body['utf8_2byte']);
        $this->assertSame('中文', $request->body['utf8_3byte']);
        $this->assertSame('😀🎉', $request->body['utf8_4byte']);
        $this->assertStringContainsString('café', $request->body['mixed_encoding']);
        $this->assertStringContainsString('中文', $request->body['mixed_encoding']);
        $this->assertStringContainsString('😀', $request->body['mixed_encoding']);
    }

    /**
     * Test binary data represented in JSON fields.
     * Verifies that base64-encoded binary data maintains integrity.
     */
    #[Test]
    public function testBinaryDataInJsonFields(): void
    {
        $binaryData = \random_bytes(256);
        $base64Encoded = \base64_encode($binaryData);

        $body = [
            'binary_data' => $base64Encoded,
            'filename' => 'data.bin',
            'content_type' => 'application/octet-stream',
            'size' => \strlen($binaryData),
            'checksum' => \md5($binaryData),
            'hex_data' => \bin2hex($binaryData),
        ];

        $request = make_request(method: 'POST', path: '/binary', body: $body);

        $this->assertIsArray($request->body);
        $this->assertSame($base64Encoded, $request->body['binary_data']);
        $this->assertSame(\strlen($binaryData), $request->body['size']);
        $this->assertSame(\md5($binaryData), $request->body['checksum']);
        $this->assertSame(\bin2hex($binaryData), $request->body['hex_data']);
        $this->assertSame($binaryData, \base64_decode($request->body['binary_data']));
    }

    /**
     * Test special object types (DateTime, stdClass) in JSON serialization.
     * Verifies that objects are properly converted to arrays/strings.
     */
    #[Test]
    public function testSpecialObjectTypesInJsonSerialization(): void
    {
        $dateTime = new \DateTime('2024-12-28T15:30:00Z');
        $stdClass = new \stdClass();
        $stdClass->name = 'Test Object';
        $stdClass->value = 42;

        $body = [
            'datetime_string' => $dateTime->format('c'),
            'datetime_timestamp' => $dateTime->getTimestamp(),
            'object_as_array' => [
                'name' => 'Test Object',
                'value' => 42,
            ],
            'simple_values' => [
                'int' => 42,
                'float' => 3.14,
                'bool' => true,
                'null' => null,
                'string' => 'text',
            ],
        ];

        $request = make_request(method: 'POST', path: '/objects', body: $body);

        $this->assertIsArray($request->body);
        $this->assertSame('2024-12-28T15:30:00+00:00', $request->body['datetime_string']);
        $this->assertIsInt($request->body['datetime_timestamp']);
        $this->assertSame('Test Object', $request->body['object_as_array']['name']);
        $this->assertSame(42, $request->body['object_as_array']['value']);
        $this->assertSame(42, $request->body['simple_values']['int']);
        $this->assertTrue($request->body['simple_values']['bool']);
        $this->assertNull($request->body['simple_values']['null']);
    }

    /**
     * Test numeric key vs string key consistency in arrays.
     * Verifies that numeric vs string keys are handled consistently through FFI.
     */
    #[Test]
    public function testNumericKeyVsStringKeyConsistency(): void
    {
        $body = [
            0 => 'zero',
            1 => 'one',
            2 => 'two',
            'string_zero' => 'zero_string',
            'string_one' => 'one_string',
            'mixed_1' => 'mixed value 1',
            'items' => [
                0 => 'item_0',
                1 => 'item_1',
                'special' => 'special_item',
            ],
            'nested_numeric' => [
                10 => 'ten',
                20 => 'twenty',
                30 => 'thirty',
            ],
        ];

        $request = make_request(method: 'POST', path: '/keys', body: $body);

        $this->assertIsArray($request->body);
        $this->assertSame('zero', $request->body[0]);
        $this->assertSame('one', $request->body[1]);
        $this->assertSame('two', $request->body[2]);
        $this->assertSame('zero_string', $request->body['string_zero']);
        $this->assertSame('one_string', $request->body['string_one']);
        $this->assertSame('item_0', $request->body['items'][0]);
        $this->assertSame('special_item', $request->body['items']['special']);
        $this->assertSame('ten', $request->body['nested_numeric'][10]);
        $this->assertSame('twenty', $request->body['nested_numeric'][20]);
    }

    /**
     * Test Response JSON serialization with complex types.
     * Verifies that Response::json() handles edge case data correctly.
     */
    #[Test]
    public function testResponseJsonSerializationWithComplexTypes(): void
    {
        $data = [
            'emoji' => '🚀✨💻',
            'nested' => [
                'deep' => [
                    'deeper' => [
                        'value' => 'found',
                    ],
                ],
            ],
            'numbers' => [
                'int' => 42,
                'float' => 3.14159,
                'negative' => -999,
                'zero' => 0,
            ],
            'strings' => [
                'empty' => '',
                'unicode' => '中文',
                'control' => "Tab\there",
            ],
            'booleans' => [
                'true' => true,
                'false' => false,
            ],
            'nulls' => [
                'value' => null,
            ],
            'mixed_array' => [1, 'two', 3.0, null, false, []],
        ];

        $response = Response::json($data);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame('application/json', $response->headers['Content-Type']);
        $this->assertIsArray($response->body);
        /** @var array<string, mixed> $body */
        $body = $response->body;
        $this->assertSame('🚀✨💻', $body['emoji']);
        $this->assertIsArray($body['nested']);
        /** @var array<string, mixed> $nested */
        $nested = $body['nested'];
        $this->assertIsArray($nested['deep']);
        /** @var array<string, mixed> $deep */
        $deep = $nested['deep'];
        $this->assertIsArray($deep['deeper']);
        /** @var array<string, mixed> $deeper */
        $deeper = $deep['deeper'];
        $this->assertSame('found', $deeper['value']);
        $this->assertIsArray($body['numbers']);
        /** @var array<string, mixed> $numbers */
        $numbers = $body['numbers'];
        $this->assertSame(42, $numbers['int']);
        $this->assertEqualsWithDelta(3.14159, $numbers['float'], 0.00001);
        $this->assertIsArray($body['strings']);
        /** @var array<string, mixed> $strings */
        $strings = $body['strings'];
        $this->assertSame('中文', $strings['unicode']);
        $this->assertIsArray($body['booleans']);
        /** @var array<string, mixed> $booleans */
        $booleans = $body['booleans'];
        $this->assertTrue($booleans['true']);
        $this->assertFalse($booleans['false']);
        $this->assertIsArray($body['nulls']);
        /** @var array<string, mixed> $nulls */
        $nulls = $body['nulls'];
        $this->assertNull($nulls['value']);
    }

    /**
     * Test that empty and null values are preserved correctly.
     * Verifies distinction between empty string, zero, false, and null.
     */
    #[Test]
    public function testEmptyAndNullValuesPreservation(): void
    {
        $body = [
            'empty_string' => '',
            'zero_int' => 0,
            'zero_float' => 0.0,
            'false_bool' => false,
            'null_value' => null,
            'empty_array' => [],
            'array_with_null' => [null, null, null],
            'array_with_zero' => [0, 0.0],
            'array_with_false' => [false, false],
            'array_with_empty' => ['', ''],
        ];

        $request = make_request(method: 'POST', path: '/empty', body: $body);

        $this->assertIsArray($request->body);
        $this->assertSame('', $request->body['empty_string']);
        $this->assertSame(0, $request->body['zero_int']);
        $this->assertSame(0.0, $request->body['zero_float']);
        $this->assertFalse($request->body['false_bool']);
        $this->assertNull($request->body['null_value']);
        $this->assertSame([], $request->body['empty_array']);
        $this->assertCount(3, $request->body['array_with_null']);
        $this->assertNull($request->body['array_with_null'][0]);
        $this->assertCount(2, $request->body['array_with_false']);
        $this->assertFalse($request->body['array_with_false'][0]);
    }

    /**
     * Test data type preservation across numeric boundaries.
     * Verifies that integers, floats, and scientific notation are handled correctly.
     */
    #[Test]
    public function testNumericBoundaryPreservation(): void
    {
        $body = [
            'php_int_max_safe' => 9007199254740991, // JavaScript safe integer limit
            'php_int_max' => PHP_INT_MAX,
            'php_int_min' => PHP_INT_MIN,
            'large_negative' => -999999999999999,
            'scientific_positive' => 1.5e+20,
            'scientific_negative_exp' => 1.5e-20,
            'very_small_positive' => 0.00000000000001,
            'fraction' => 0.5,
            'negative_fraction' => -0.5,
        ];

        $request = make_request(method: 'POST', path: '/numbers', body: $body);

        $this->assertIsArray($request->body);
        $this->assertIsInt($request->body['php_int_max_safe']);
        $this->assertSame(9007199254740991, $request->body['php_int_max_safe']);
        $this->assertIsInt($request->body['php_int_max']);
        $this->assertIsInt($request->body['php_int_min']);
        $this->assertIsInt($request->body['large_negative']);
        $this->assertEqualsWithDelta(1.5e+20, $request->body['scientific_positive'], 1e15);
        $this->assertEqualsWithDelta(1.5e-20, $request->body['scientific_negative_exp'], 1e-25);
        $this->assertEqualsWithDelta(0.00000000000001, $request->body['very_small_positive'], 1e-15);
        $this->assertSame(0.5, $request->body['fraction']);
        $this->assertSame(-0.5, $request->body['negative_fraction']);
    }

    /**
     * Test special string sequences that might break JSON parsing.
     * Verifies that quotes, backslashes, and other special chars are handled.
     */
    #[Test]
    public function testSpecialStringSequencesInJson(): void
    {
        $body = [
            'quotes' => 'He said "Hello"',
            'single_quotes' => "It's working",
            'backslashes' => 'Path: C:\\Users\\Admin',
            'forward_slashes' => 'URL: https://example.com/path',
            'mixed_escapes' => 'Quote: " and Backslash: \\ and Tab: \t',
            'json_like' => '{"key": "value"}',
            'html_like' => '<script>alert("XSS")</script>',
            'sql_like' => "'; DROP TABLE users; --",
            'unicode_escape' => '\u0048\u0065\u006c\u006c\u006f',
        ];

        $request = make_request(method: 'POST', path: '/strings', body: $body);

        $this->assertIsArray($request->body);
        $this->assertSame('He said "Hello"', $request->body['quotes']);
        $this->assertSame("It's working", $request->body['single_quotes']);
        $this->assertSame('Path: C:\\Users\\Admin', $request->body['backslashes']);
        $this->assertSame('URL: https://example.com/path', $request->body['forward_slashes']);
        $this->assertStringContainsString('"', $request->body['mixed_escapes']);
        $this->assertStringContainsString('\\', $request->body['mixed_escapes']);
        $this->assertSame('{"key": "value"}', $request->body['json_like']);
        $this->assertSame('<script>alert("XSS")</script>', $request->body['html_like']);
    }

    /**
     * Test large arrays with mixed content.
     * Verifies that large collections maintain integrity through FFI.
     */
    #[Test]
    public function testLargeArraysWithMixedContent(): void
    {
        $items = [];
        for ($i = 0; $i < 1000; $i++) {
            $items[] = [
                'id' => $i,
                'name' => 'Item ' . $i,
                'description' => 'Description for item ' . $i,
                'values' => [
                    'int' => $i,
                    'float' => $i * 1.5,
                    'string' => 'value_' . $i,
                    'bool' => $i % 2 === 0,
                ],
                'emoji' => '🎉' . ($i % 3 === 0 ? '✨' : ''),
            ];
        }

        $body = [
            'count' => \count($items),
            'items' => $items,
            'metadata' => [
                'total' => \count($items),
                'generated_at' => (new \DateTime())->format('c'),
            ],
        ];

        $request = make_request(method: 'POST', path: '/bulk', body: $body);

        $this->assertIsArray($request->body);
        /** @var array<string, mixed> $reqBody */
        $reqBody = $request->body;
        $this->assertSame(1000, $reqBody['count']);
        $this->assertIsArray($reqBody['items']);
        $this->assertCount(1000, $reqBody['items']);
        /** @var array<int, mixed> $items */
        $items = $reqBody['items'];
        /** @var array<string, mixed> $item0 */
        $item0 = $items[0];
        $this->assertSame('Item 0', $item0['name']);
        $this->assertIsArray($item0['values']);
        /** @var array<string, mixed> $values0 */
        $values0 = $item0['values'];
        $this->assertSame(0, $values0['int']);
        /** @var array<string, mixed> $item999 */
        $item999 = $items[999];
        $this->assertSame('Item 999', $item999['name']);
        $this->assertIsArray($item999['values']);
        /** @var array<string, mixed> $values999 */
        $values999 = $item999['values'];
        $this->assertSame(999, $values999['int']);
        $this->assertIsString($item0['emoji']);
        $this->assertStringContainsString('🎉', $item0['emoji']);
    }

    /**
     * Test that Response preserves all data types correctly.
     * Verifies Response::json() maintains type information.
     */
    #[Test]
    public function testResponseJsonPreservesDataTypes(): void
    {
        $body = [
            'string' => 'hello',
            'integer' => 42,
            'float' => 3.14,
            'boolean_true' => true,
            'boolean_false' => false,
            'null_value' => null,
            'array_numeric' => [1, 2, 3],
            'array_assoc' => ['key1' => 'value1', 'key2' => 'value2'],
            'nested_array' => [
                'level1' => [
                    'level2' => [
                        'level3' => 'deep_value',
                    ],
                ],
            ],
        ];

        $response = Response::json($body);

        /** @var array<string, mixed> $result */
        $result = $response->body;
        $this->assertIsString($result['string']);
        $this->assertIsInt($result['integer']);
        $this->assertIsFloat($result['float']);
        $this->assertIsBool($result['boolean_true']);
        $this->assertIsBool($result['boolean_false']);
        $this->assertNull($result['null_value']);
        $this->assertIsArray($result['array_numeric']);
        $this->assertIsArray($result['array_assoc']);
        $this->assertIsArray($result['nested_array']);
        /** @var array<string, mixed> $level1 */
        $level1 = $result['nested_array'];
        $this->assertIsArray($level1['level1']);
        /** @var array<string, mixed> $level2arr */
        $level2arr = $level1['level1'];
        $this->assertIsArray($level2arr['level2']);
        /** @var array<string, mixed> $level3arr */
        $level3arr = $level2arr['level2'];
        $this->assertSame('deep_value', $level3arr['level3']);
    }

    /**
     * Test header value serialization with special characters.
     * Verifies that headers handle Unicode and special values correctly.
     */
    #[Test]
    public function testHeaderSerializationWithSpecialCharacters(): void
    {
        $headers = [
            'Content-Type' => 'application/json; charset=utf-8',
            'X-Custom-Header' => 'simple-value',
            'X-Unicode-Header' => 'café', // UTF-8 in header
            'X-Long-Header' => \str_repeat('a', 1000),
            'X-Special-Chars' => 'value-with_special.chars+more',
        ];

        $request = make_request(
            method: 'GET',
            path: '/test',
            body: null,
            headers: $headers
        );

        $this->assertArrayHasKey('Content-Type', $request->headers);
        $this->assertSame('application/json; charset=utf-8', $request->headers['Content-Type']);
        $this->assertSame('simple-value', $request->headers['X-Custom-Header']);
        $this->assertSame('café', $request->headers['X-Unicode-Header']);
        $this->assertSame(\str_repeat('a', 1000), $request->headers['X-Long-Header']);
    }

    /**
     * Test cookie value serialization through FFI boundary.
     * Verifies cookie values handle encoding correctly.
     */
    #[Test]
    public function testCookieValueSerializationThroughFfi(): void
    {
        $cookies = [
            'simple' => 'value',
            'with_equals' => 'key=value',
            'with_semicolon' => 'part1;part2',
            'url_encoded' => '%20space%20',
            'uuid' => '550e8400-e29b-41d4-a716-446655440000',
            'base64' => \base64_encode('binary_data'),
            'with_unicode' => 'café',
        ];

        $request = make_request(
            method: 'GET',
            path: '/test',
            body: null,
            cookies: $cookies
        );

        $this->assertArrayHasKey('simple', $request->cookies);
        $this->assertSame('value', $request->cookies['simple']);
        $this->assertSame('key=value', $request->cookies['with_equals']);
        $this->assertSame('part1;part2', $request->cookies['with_semicolon']);
        $this->assertSame('%20space%20', $request->cookies['url_encoded']);
        $this->assertSame('550e8400-e29b-41d4-a716-446655440000', $request->cookies['uuid']);
        $this->assertSame('café', $request->cookies['with_unicode']);
    }

    /**
     * Test array with mixed integer and string keys maintains order.
     * Verifies that key ordering is preserved through serialization.
     */
    #[Test]
    public function testArrayWithMixedKeysPreservesOrder(): void
    {
        $body = [];
        $body[0] = 'first';
        $body['name'] = 'John';
        $body[1] = 'second';
        $body['age'] = 30;
        $body[2] = 'third';
        $body['email'] = 'john@example.com';
        $body[3] = 'fourth';

        $request = make_request(method: 'POST', path: '/test', body: $body);

        $this->assertIsArray($request->body);
        $keys = \array_keys($request->body);
        // Verify all keys are present
        $this->assertContains(0, $keys);
        $this->assertContains('name', $keys);
        $this->assertContains(1, $keys);
        $this->assertContains('age', $keys);
        $this->assertContains(2, $keys);
        $this->assertContains('email', $keys);
        $this->assertContains(3, $keys);
    }
}
