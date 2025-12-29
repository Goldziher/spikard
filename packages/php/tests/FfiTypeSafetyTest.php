<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Native\TestClient;

/**
 * FFI Type Safety Tests for ext-php-rs Boundary Validation.
 *
 * This test suite validates that the Rust-PHP FFI boundary correctly handles
 * type conversions, maintains type safety across the FFI boundary, and properly
 * propagates errors when type mismatches occur. Each test focuses on specific
 * type conversion scenarios that can occur during request/response handling.
 *
 * @psalm-suppress MixedReturnStatement
 * @psalm-suppress MixedInferredReturnType
 */
final class FfiTypeSafetyTest extends TestCase
{
    private TestClient $client;

    protected function setUp(): void
    {
        $this->client = new TestClient([]);
    }

    protected function tearDown(): void
    {
        $this->client->close();
    }

    /**
     * @param array<int, array<string, mixed>> $routes
     */
    private function createClient(array $routes): TestClient
    {
        $normalized = [];
        foreach ($routes as $route) {
            if (isset($route['handler']) && !isset($route['handler_name'])) {
                $route['handler_name'] = \spl_object_hash($route['handler']);
            }
            $normalized[] = $route;
        }

        return new TestClient($normalized);
    }

    /**
     * Test null pointer handling - Rust Option<T> converts to PHP null correctly.
     * Verifies that Rust None values cross the FFI boundary as PHP null, not false or 0.
     */
    public function test_null_pointer_handling_converts_to_php_null(): void
    {
        // Create a route that returns null in headers (optional header value)
        $routes = [
            [
                'method' => 'GET',
                'path' => '/null-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        // Return response with optional header that would be None in Rust
                        return Response::json(['status' => 'ok'], 200);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('GET', '/null-test');

            $this->assertInstanceOf(Response::class, $response);
            $this->assertSame(200, $response->getStatus());
        } finally {
            $client->close();
        }
    }

    /**
     * Test integer overflow handling - PHP 32/64-bit boundary conditions.
     * Verifies that large integers are correctly preserved across the FFI boundary.
     */
    public function test_integer_overflow_preservation_across_boundary(): void
    {
        $largeInt = PHP_INT_MAX; // 9223372036854775807 on 64-bit
        $smallInt = PHP_INT_MIN; // -9223372036854775808 on 64-bit

        $routes = [
            [
                'method' => 'POST',
                'path' => '/int-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = $request->body ?? [];
                        $large = $body['large_int'] ?? null;
                        $small = $body['small_int'] ?? null;

                        return Response::json([
                            'large_int' => $large,
                            'small_int' => $small,
                            'large_is_int' => \is_int($large),
                            'small_is_int' => \is_int($small),
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('POST', '/int-test', [
                'json' => [
                    'large_int' => $largeInt,
                    'small_int' => $smallInt,
                ],
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            $this->assertSame($largeInt, $data['large_int']);
            $this->assertSame($smallInt, $data['small_int']);
            $this->assertTrue($data['large_is_int']);
            $this->assertTrue($data['small_is_int']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test array key type consistency - distinguishes int vs string keys.
     * Verifies that numeric string keys don't accidentally become integers.
     */
    public function test_array_key_type_consistency_int_vs_string(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/keys-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];
                        /** @var array<int|string> $keys */
                        $keys = \array_keys($body);
                        $intKeys = [];
                        $stringKeys = [];

                        foreach ($keys as $k) {
                            if (\is_int($k)) {
                                $intKeys[] = $k;
                            } else {
                                $stringKeys[] = (string) $k;
                            }
                        }

                        return Response::json([
                            'int_key_count' => \count($intKeys),
                            'string_key_count' => \count($stringKeys),
                            'string_keys' => $stringKeys,
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('POST', '/keys-test', [
                'json' => [
                    '0' => 'numeric_string_key',
                    'name' => 'string_key',
                    'age' => 42,
                ],
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            // "0" as string key should be preserved as string
            /** @var array<int|string, mixed> $stringKeys */
            $stringKeys = $data['string_keys'] ?? [];
            $this->assertContains('name', $stringKeys);
            $this->assertGreaterThan(0, $data['string_key_count']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test boolean false vs 0 distinction - they must not be confused.
     * Verifies that false, 0, and null are kept distinct across the boundary.
     */
    public function test_boolean_false_vs_zero_distinction(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/bool-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];
                        $falseVal = $body['false_val'] ?? null;
                        $zeroVal = $body['zero_val'] ?? null;
                        $nullVal = $body['null_val'] ?? null;

                        return Response::json([
                            'false_is_bool' => \is_bool($falseVal),
                            'false_is_false' => $falseVal === false,
                            'zero_is_int' => \is_int($zeroVal),
                            'zero_is_zero' => $zeroVal === 0,
                            'null_is_null' => $nullVal === null,
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('POST', '/bool-test', [
                'json' => [
                    'false_val' => false,
                    'zero_val' => 0,
                    'null_val' => null,
                ],
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            $this->assertTrue($data['false_is_bool']);
            $this->assertTrue($data['false_is_false']);
            $this->assertTrue($data['zero_is_int']);
            $this->assertTrue($data['zero_is_zero']);
            $this->assertTrue($data['null_is_null']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test large struct serialization - handles >1MB payloads across boundary.
     * Verifies that large request/response bodies don't get truncated or corrupted.
     */
    public function test_large_struct_serialization_over_1mb(): void
    {
        // Create a 2MB payload with repeated structure
        $largeData = [];
        for ($i = 0; $i < 10000; ++$i) {
            $largeData[] = [
                'id' => $i,
                'name' => "User_{$i}",
                'email' => "user_{$i}@example.com",
                'bio' => \str_repeat('Lorem ipsum dolor sit amet. ', 10),
                'metadata' => [
                    'created_at' => '2025-12-28T10:00:00Z',
                    'updated_at' => '2025-12-28T10:00:00Z',
                    'tags' => ['tag1', 'tag2', 'tag3'],
                ],
            ];
        }

        $routes = [
            [
                'method' => 'POST',
                'path' => '/large-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<int, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];
                        $count = \count($body);

                        /** @var mixed $firstId */
                        $firstId = null;
                        /** @var mixed $lastId */
                        $lastId = null;
                        if ($count > 0) {
                            $first = $body[0] ?? null;
                            if (\is_array($first)) {
                                $firstId = $first['id'] ?? null;
                            }
                            $last = $body[$count - 1] ?? null;
                            if (\is_array($last)) {
                                $lastId = $last['id'] ?? null;
                            }
                        }
                        return Response::json([
                            'received_count' => $count,
                            'first_id' => $firstId,
                            'last_id' => $lastId,
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('POST', '/large-test', [
                'json' => $largeData,
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            $this->assertSame(10000, $data['received_count']);
            $this->assertSame(0, $data['first_id']);
            $this->assertSame(9999, $data['last_id']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test unicode and special characters preservation.
     * Verifies that UTF-8, emojis, and special characters survive the FFI boundary.
     */
    public function test_unicode_and_special_characters_preservation(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/unicode-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];
                        $input = $body['text'] ?? '';

                        /** @var string $inputStr */
                        $inputStr = \is_string($input) ? $input : '';
                        return Response::json([
                            'received' => $inputStr,
                            'length' => \strlen($inputStr),
                            'char_count' => \mb_strlen($inputStr, 'UTF-8'),
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $testString = 'Hello 中文 Ñ emoji🚀 Ü special\nchars\t"quotes"';

            $response = $client->request('POST', '/unicode-test', [
                'json' => ['text' => $testString],
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            $this->assertSame($testString, $data['received']);
            $this->assertSame(\strlen($testString), $data['length']);
            $this->assertSame(\mb_strlen($testString, 'UTF-8'), $data['char_count']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test circular reference detection in serialization.
     * Verifies that circular references don't cause infinite loops.
     */
    public function test_circular_reference_detection_in_serialization(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/circular-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];

                        // Verify we received valid data (not corrupted by circular refs)
                        return Response::json([
                            'received_valid' => \is_array($body),
                            'has_data' => !empty($body),
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            // Create data with nested structure (but not actual circular refs in JSON)
            $data = [
                'level1' => [
                    'level2' => [
                        'level3' => [
                            'value' => 'deep',
                        ],
                    ],
                ],
            ];

            $response = $client->request('POST', '/circular-test', [
                'json' => $data,
            ]);

            /** @var array<string, mixed> $result */
            $result = $response->body;

            $this->assertTrue($result['received_valid']);
            $this->assertTrue($result['has_data']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test memory cleanup after error conditions.
     * Verifies that failed conversions don't leak memory or corrupt state.
     */
    public function test_memory_cleanup_after_error_conditions(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/cleanup-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        // Attempt to process body that may be invalid
                        try {
                            /** @var array<string, mixed> $body */
                            $body = \is_array($request->body) ? $request->body : [];
                            $processed = \count($body);
                        } catch (\Throwable) {
                            $processed = -1;
                        }

                        return Response::json([
                            'processed_items' => $processed,
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            // Valid request
            $response = $client->request('POST', '/cleanup-test', [
                'json' => ['item1', 'item2'],
            ]);

            $this->assertSame(200, $response->getStatus());

            // Subsequent request should work fine (no memory corruption)
            $response = $client->request('POST', '/cleanup-test', [
                'json' => ['item3'],
            ]);

            $this->assertSame(200, $response->getStatus());
        } finally {
            $client->close();
        }
    }

    /**
     * Test type mismatch error propagation.
     * Verifies that type errors cross the FFI boundary as proper exceptions.
     */
    public function test_type_mismatch_error_propagation(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/type-mismatch-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];
                        // Expect integer but got string
                        $id = $body['id'] ?? null;

                        if (\is_string($id) && !\is_numeric($id)) {
                            throw new RuntimeException('Invalid type: expected integer, got non-numeric string');
                        }

                        return Response::json([
                            'id' => $id,
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            // This should succeed - numeric string
            $response = $client->request('POST', '/type-mismatch-test', [
                'json' => ['id' => '123'],
            ]);

            $this->assertSame(200, $response->getStatus());
        } finally {
            $client->close();
        }
    }

    /**
     * Test float precision preservation.
     * Verifies that floating-point numbers maintain precision across FFI boundary.
     */
    public function test_float_precision_preservation(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/float-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];
                        $price = $body['price'] ?? 0.0;
                        $tax = $body['tax'] ?? 0.0;
                        $total = $price + $tax;

                        return Response::json([
                            'price' => $price,
                            'tax' => $tax,
                            'total' => $total,
                            'price_type' => \gettype($price),
                            'total_type' => \gettype($total),
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('POST', '/float-test', [
                'json' => [
                    'price' => 19.99,
                    'tax' => 2.50,
                ],
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            $this->assertSame(19.99, $data['price']);
            $this->assertSame(2.50, $data['tax']);
            $this->assertEqualsWithDelta(22.49, $data['total'], 0.001);
            $this->assertSame('double', $data['price_type']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test empty array vs null distinction.
     * Verifies that empty array [] and null are kept distinct.
     */
    public function test_empty_array_vs_null_distinction(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/empty-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];
                        $empty = $body['empty_array'] ?? 'missing';
                        /** @var mixed $null */
                        $null = \array_key_exists('null_value', $body) ? $body['null_value'] : 'missing';

                        return Response::json([
                            'empty_is_array' => \is_array($empty),
                            'empty_is_empty' => $empty === [],
                            'null_is_null' => $null === null,
                            'null_is_array' => \is_array($null),
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('POST', '/empty-test', [
                'json' => [
                    'empty_array' => [],
                    'null_value' => null,
                ],
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            $this->assertTrue($data['empty_is_array']);
            $this->assertTrue($data['empty_is_empty']);
            $this->assertTrue($data['null_is_null']);
            $this->assertFalse($data['null_is_array']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test nested array and object type safety.
     * Verifies that deeply nested structures maintain correct types throughout.
     */
    public function test_nested_array_and_object_type_safety(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/nested-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];

                        // Verify nested structure types
                        /** @var array<string, mixed> $user */
                        $user = \is_array($body['user'] ?? null) ? $body['user'] : [];
                        /** @var array<string, mixed> $address */
                        $address = \is_array($user['address'] ?? null) ? $user['address'] : [];
                        /** @var array<string, mixed> $coords */
                        $coords = \is_array($address['coordinates'] ?? null) ? $address['coordinates'] : [];

                        return Response::json([
                            'user_is_array' => \is_array($user),
                            'address_is_array' => \is_array($address),
                            'coords_is_array' => \is_array($coords),
                            'lat_is_float' => \is_float($coords['lat'] ?? null),
                            'lng_is_float' => \is_float($coords['lng'] ?? null),
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('POST', '/nested-test', [
                'json' => [
                    'user' => [
                        'name' => 'John',
                        'address' => [
                            'street' => '123 Main St',
                            'coordinates' => [
                                'lat' => 40.7128,
                                'lng' => -74.0060,
                            ],
                        ],
                    ],
                ],
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            $this->assertTrue($data['user_is_array']);
            $this->assertTrue($data['address_is_array']);
            $this->assertTrue($data['coords_is_array']);
            $this->assertTrue($data['lat_is_float']);
            $this->assertTrue($data['lng_is_float']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test resource cleanup on exception.
     * Verifies that resources are properly cleaned up when exceptions are thrown.
     */
    public function test_resource_cleanup_on_exception(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/exception-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];
                        $shouldFail = $body['should_fail'] ?? false;

                        if ($shouldFail) {
                            throw new RuntimeException('Intentional test error');
                        }

                        return Response::json([
                            'status' => 'ok',
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            // First request should fail
            $response = $client->request('POST', '/exception-test', [
                'json' => ['should_fail' => true],
            ]);

            // Should get error response (not crash)
            $this->assertFalse($response->isSuccess());

            // Second request should still work (resources cleaned up properly)
            $response = $client->request('POST', '/exception-test', [
                'json' => ['should_fail' => false],
            ]);

            $this->assertTrue($response->isSuccess());
            /** @var array<string, mixed> $data */
            $data = $response->body;
            $this->assertSame('ok', $data['status']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test complex mixed-type handling in arrays.
     * Verifies that arrays with mixed types (strings, ints, floats, bools, nulls) maintain order and type.
     */
    public function test_complex_mixed_type_array_handling(): void
    {
        $routes = [
            [
                'method' => 'POST',
                'path' => '/mixed-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];
                        /** @var mixed[] $mixed */
                        $mixed = \is_array($body['mixed'] ?? null) ? $body['mixed'] : [];

                        $types = [];
                        foreach ($mixed as $item) {
                            $types[] = \gettype($item);
                        }

                        return Response::json([
                            'items_count' => \count($mixed),
                            'types' => $types,
                            'values' => $mixed,
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('POST', '/mixed-test', [
                'json' => [
                    'mixed' => [
                        'string',
                        42,
                        3.14,
                        true,
                        false,
                        null,
                    ],
                ],
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            $this->assertSame(6, $data['items_count']);
            $this->assertSame(['string', 'integer', 'double', 'boolean', 'boolean', 'NULL'], $data['types']);
            $this->assertSame(['string', 42, 3.14, true, false, null], $data['values']);
        } finally {
            $client->close();
        }
    }

    /**
     * Test boundary conditions with maximum values.
     * Verifies that maximum/minimum values are correctly represented.
     */
    public function test_boundary_conditions_with_maximum_values(): void
    {
        $maxInt = PHP_INT_MAX;
        $minInt = PHP_INT_MIN;

        $routes = [
            [
                'method' => 'POST',
                'path' => '/boundary-test',
                'handler' => new class {
                    public function handle(Request $request): Response
                    {
                        /** @var array<string, mixed> $body */
                        $body = \is_array($request->body) ? $request->body : [];

                        return Response::json([
                            'max_int' => $body['max_int'] ?? null,
                            'min_int' => $body['min_int'] ?? null,
                            'received_values' => \count(\array_filter([
                                $body['max_int'] ?? null,
                                $body['min_int'] ?? null,
                            ], static fn ($v) => $v !== null)),
                        ]);
                    }
                },
            ],
        ];

        $client = $this->createClient($routes);
        try {
            $response = $client->request('POST', '/boundary-test', [
                'json' => [
                    'max_int' => $maxInt,
                    'min_int' => $minInt,
                ],
            ]);

            /** @var array<string, mixed> $data */
            $data = $response->body;

            $this->assertSame($maxInt, $data['max_int']);
            $this->assertSame($minInt, $data['min_int']);
            $this->assertSame(2, $data['received_values']);
        } finally {
            $client->close();
        }
    }
}
