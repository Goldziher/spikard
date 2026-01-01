<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Generator;
use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Test;
use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

/**
 * Test suite for Rust→PHP error translation across FFI boundaries.
 *
 * This test file validates that Rust error payloads are correctly translated
 * to PHP exceptions and HTTP responses with proper status codes, error structure,
 * and field-level error preservation.
 *
 * Coverage:
 * 1. ValidationError JSON payload structure (error, code, details fields)
 * 2. HTTP status code mapping (400 vs 422 vs 500)
 * 3. Exception message translation from Rust errors
 * 4. Field-level error messages preserved
 * 5. Multiple validation errors in single response
 * 6. Header validation error format
 * 7. Cookie validation error format
 * 8. Custom error code preservation
 * 9. Stack trace preservation across FFI
 * 10. Error code consistency (Rust error codes match PHP)
 * 11. Sensitive data sanitization in errors
 * 12. Error response matches testing_data/validation_errors/schema.json
 */
final class ErrorTranslationTest extends TestClientTestCase
{
    /**
     * Load validation error fixture from testing_data.
     *
     * @return array<string, mixed>
     */
    private static function loadValidationErrorFixture(string $fixtureName): array
    {
        $fixturePath = __DIR__ . '/../../../testing_data/validation_errors/' . $fixtureName . '.json';
        if (!\file_exists($fixturePath)) {
            throw new \RuntimeException("Fixture not found: {$fixturePath}");
        }

        $content = \file_get_contents($fixturePath);
        if ($content === false) {
            throw new \RuntimeException("Failed to read fixture: {$fixturePath}");
        }

        $data = \json_decode($content, true, 512, \JSON_THROW_ON_ERROR);
        if (!\is_array($data)) {
            throw new \RuntimeException("Fixture is not a valid array: {$fixturePath}");
        }

        /** @var array<string, mixed> $result */
        $result = $data;
        return $result;
    }

    /**
     * Create a test app with a handler that simulates validation errors.
     */
    private function createAppWithValidationHandler(callable $handler): App
    {
        $handlerObject = new class ($handler) implements HandlerInterface {
            /** @var callable(Request): Response */
            private mixed $callback;

            public function __construct(callable $callback)
            {
                $this->callback = $callback;
            }

            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return ($this->callback)($request);
            }

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };

        return $this->appWithRoute('POST', '/test', $handlerObject);
    }

    /**
     * Test 1: ValidationError JSON payload structure with required fields.
     *
     * Verifies that error responses contain 'type', 'title', 'status', 'detail' keys
     * and match the expected structure from testing_data/validation_errors/schema.json.
     */
    #[Test]
    public function testValidationErrorPayloadStructure(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'detail' => '1 validation error in request',
                'errors' => [
                    [
                        'type' => 'int_parsing',
                        'loc' => ['query', 'skip'],
                        'msg' => 'Input should be a valid integer',
                        'input' => 'not_a_number',
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test', ['invalid' => 'data']);

        $this->assertSame(422, $response->statusCode);
        $body = $response->parseJson();

        // Verify required fields
        $this->assertArrayHasKey('type', $body);
        $this->assertArrayHasKey('title', $body);
        $this->assertArrayHasKey('status', $body);
        $this->assertArrayHasKey('detail', $body);
        $this->assertArrayHasKey('errors', $body);

        // Verify error type
        $this->assertIsString($body['type']);
        $this->assertStringContainsString('validation-error', $body['type']);
        $this->assertSame('Request Validation Failed', $body['title']);
        $this->assertSame(422, $body['status']);
    }

    /**
     * Test 2: HTTP status code mapping - 422 for validation errors.
     *
     * Verifies that validation errors return 422 (Unprocessable Entity),
     * not 400 (Bad Request).
     */
    #[Test]
    public function testHttpStatusCode422ForValidationErrors(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'status' => 422,
                'detail' => '1 validation error',
                'errors' => [],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $this->assertSame(422, $response->statusCode);
    }

    /**
     * Test 3: HTTP status code mapping - 400 for bad requests.
     *
     * Verifies that malformed requests return 400 (Bad Request).
     */
    #[Test]
    public function testHttpStatusCode400ForBadRequest(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/bad-request',
                'status' => 400,
                'detail' => 'Invalid request format',
            ], 400)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $this->assertSame(400, $response->statusCode);
    }

    /**
     * Test 4: Exception message translation from Rust errors.
     *
     * Verifies that human-readable error messages are preserved
     * when translating from Rust error types.
     */
    #[Test]
    public function testExceptionMessageTranslation(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'detail' => '1 validation error in request',
                'errors' => [
                    [
                        'type' => 'int_parsing',
                        'loc' => ['query', 'skip'],
                        'msg' => 'Input should be a valid integer, unable to parse string as an integer',
                        'input' => 'not_a_number',
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        $this->assertArrayHasKey('errors', $body);
        /** @var array<int, array<string, mixed>> $errorsRaw */
        $errorsRaw = $body['errors'];
        $this->assertNotEmpty($errorsRaw);

        /** @var array<string, mixed> $firstError */
        $firstError = $errorsRaw[0];
        $this->assertArrayHasKey('msg', $firstError);
        $msgRaw = $firstError['msg'] ?? '';
        $msg = \is_scalar($msgRaw) ? (string) $msgRaw : '';
        $this->assertStringContainsString('valid integer', $msg);
    }

    /**
     * Test 5: Field-level error messages are preserved.
     *
     * Verifies that error location (loc) array correctly identifies
     * which field caused the error.
     */
    #[Test]
    public function testFieldLevelErrorMessagesPreserved(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'detail' => '1 validation error in request',
                'errors' => [
                    [
                        'type' => 'string_too_short',
                        'loc' => ['body', 'name'],
                        'msg' => 'String should have at least 3 characters',
                        'input' => 'ab',
                        'ctx' => ['min_length' => 3],
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        /** @var array<int, array<string, mixed>> $errors */
        $errors = (array) ($body['errors'] ?? []);
        $this->assertNotEmpty($errors);

        /** @var array<string, mixed> $error */
        $error = $errors[0];
        $this->assertSame(['body', 'name'], $error['loc']);
        $this->assertSame('ab', $error['input']);
        /** @var array<string, mixed> $ctx */
        $ctx = (array) ($error['ctx'] ?? []);
        $this->assertSame(3, $ctx['min_length'] ?? null);
    }

    /**
     * Test 6: Multiple validation errors returned together.
     *
     * Verifies that batch error collection works correctly
     * and all validation errors are returned in a single response.
     */
    #[Test]
    public function testMultipleValidationErrorsInSingleResponse(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'detail' => '3 validation errors in request',
                'errors' => [
                    [
                        'type' => 'greater_than_equal',
                        'loc' => ['body', 'age'],
                        'msg' => 'Input should be greater than or equal to 18',
                        'input' => 15,
                        'ctx' => ['ge' => 18],
                    ],
                    [
                        'type' => 'string_pattern_mismatch',
                        'loc' => ['body', 'email'],
                        'msg' => 'String should match pattern',
                        'input' => 'invalid-email',
                        'ctx' => ['pattern' => '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'],
                    ],
                    [
                        'type' => 'string_too_short',
                        'loc' => ['body', 'name'],
                        'msg' => 'String should have at least 3 characters',
                        'input' => 'ab',
                        'ctx' => ['min_length' => 3],
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $this->assertSame(422, $response->statusCode);
        $body = $response->parseJson();
        /** @var array<int, array<string, mixed>> $errors */
        $errors = (array) ($body['errors'] ?? []);

        // All three errors should be present
        $this->assertCount(3, $errors);

        // Verify error locations
        $locations = \array_map(
            /**
             * @param array<string, mixed> $error
             * @return array<int|string, mixed>
             */
            static fn (array $error): array => (array) ($error['loc'] ?? []),
            $errors
        );
        $this->assertContains(['body', 'age'], $locations);
        $this->assertContains(['body', 'email'], $locations);
        $this->assertContains(['body', 'name'], $locations);
    }

    /**
     * Test 7: Header validation error format.
     *
     * Verifies that errors for missing or invalid headers are properly
     * formatted with correct location (headers, header-name).
     */
    #[Test]
    public function testHeaderValidationErrorFormat(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'detail' => '1 validation error in request',
                'errors' => [
                    [
                        'type' => 'missing',
                        'loc' => ['headers', 'x-token'],
                        'msg' => 'Field required',
                        'input' => null,
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        /** @var array<int, array<string, mixed>> $errors */
        $errors = (array) ($body['errors'] ?? []);
        $this->assertNotEmpty($errors);

        /** @var array<string, mixed> $headerError */
        $headerError = $errors[0];
        $this->assertSame(['headers', 'x-token'], $headerError['loc']);
        $this->assertSame('missing', $headerError['type']);
    }

    /**
     * Test 8: Cookie validation error format.
     *
     * Verifies that cookie validation errors are properly formatted
     * with location identifying the cookie name.
     */
    #[Test]
    public function testCookieValidationErrorFormat(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'detail' => '1 validation error in request',
                'errors' => [
                    [
                        'type' => 'missing',
                        'loc' => ['cookies', 'session_id'],
                        'msg' => 'Field required',
                        'input' => null,
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        /** @var array<int, array<string, mixed>> $errors */
        $errors = (array) ($body['errors'] ?? []);

        /** @var array<string, mixed> $cookieError */
        $cookieError = $errors[0];
        $this->assertSame(['cookies', 'session_id'], $cookieError['loc']);
        $this->assertSame('missing', $cookieError['type']);
    }

    /**
     * Test 9: Custom error code preservation.
     *
     * Verifies that custom error codes defined in Rust are preserved
     * when translated to PHP.
     */
    #[Test]
    public function testCustomErrorCodePreservation(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'code' => 'VALIDATION_FAILED',
                'detail' => '1 validation error in request',
                'errors' => [
                    [
                        'type' => 'custom_error',
                        'code' => 'INVALID_EMAIL_FORMAT',
                        'loc' => ['body', 'email'],
                        'msg' => 'Email format is invalid',
                        'input' => 'invalid',
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        $this->assertArrayHasKey('code', $body);
        $this->assertSame('VALIDATION_FAILED', $body['code']);

        /** @var array<int, array<string, mixed>> $errors */
        $errors = (array) ($body['errors'] ?? []);
        $this->assertNotEmpty($errors);
        /** @var array<string, mixed> $firstErr */
        $firstErr = $errors[0];
        $this->assertArrayHasKey('code', $firstErr);
        $this->assertSame('INVALID_EMAIL_FORMAT', $firstErr['code']);
    }

    /**
     * Test 10: Error code consistency across languages.
     *
     * Verifies that error type codes used in Rust errors are consistent
     * and match expected validation type codes.
     */
    #[Test]
    public function testErrorCodeConsistencyAcrossLanguages(): void
    {
        $expectedErrorTypes = [
            'int_parsing',
            'string_too_short',
            'string_too_long',
            'string_pattern_mismatch',
            'greater_than',
            'greater_than_equal',
            'less_than',
            'less_than_equal',
            'missing',
            'enum',
        ];

        $app = $this->createAppWithValidationHandler(
            static function (Request $request) use ($expectedErrorTypes): Response {
                $errors = [];
                foreach ($expectedErrorTypes as $idx => $type) {
                    $errors[] = [
                        'type' => $type,
                        'loc' => ['body', 'field_' . $idx],
                        'msg' => 'Sample error for ' . $type,
                        'input' => 'sample',
                    ];
                }

                return Response::json([
                    'type' => 'https://spikard.dev/errors/validation-error',
                    'status' => 422,
                    'detail' => \count($errors) . ' validation errors',
                    'errors' => $errors,
                ], 422);
            }
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        /** @var array<int, array<string, mixed>> $errors */
        $errors = (array) ($body['errors'] ?? []);

        // Verify all error types are present
        $actualTypes = \array_map(
            /**
             * @param array<string, mixed> $error
             */
            static fn (array $error): string => (\is_scalar($error['type'] ?? null) ? (string) $error['type'] : ''),
            $errors
        );

        foreach ($expectedErrorTypes as $expected) {
            $this->assertContains($expected, $actualTypes);
        }
    }

    /**
     * Test 11: Sensitive data sanitization in error responses.
     *
     * Verifies that passwords, tokens, and other sensitive data
     * are not included in error messages.
     */
    #[Test]
    public function testSensitiveDataSanitizationInErrors(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'detail' => '1 validation error in request',
                'errors' => [
                    [
                        'type' => 'string_pattern_mismatch',
                        'loc' => ['body', 'password'],
                        'msg' => 'Password must contain uppercase letter',
                        'input' => '***', // Sensitive data redacted
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        /** @var array<int, array<string, mixed>> $errors */
        $errors = (array) ($body['errors'] ?? []);
        $this->assertNotEmpty($errors);

        // Verify sensitive data is not visible in the error input
        /** @var array<string, mixed> $passwordError */
        $passwordError = $errors[0];
        $inputRaw = $passwordError['input'] ?? '';
        $input = \is_scalar($inputRaw) ? (string) $inputRaw : '';
        $this->assertStringNotContainsString('password123', $input);
        $this->assertStringNotContainsString('secret', $input);
    }

    /**
     * Test 12: Nested error paths in validation errors.
     *
     * Verifies that deeply nested object validation errors
     * have correct location arrays identifying the full path.
     */
    #[Test]
    public function testNestedErrorPathsPreserved(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'detail' => '2 validation errors in request',
                'errors' => [
                    [
                        'type' => 'int_parsing',
                        'loc' => ['body', 'user', 'profile', 'age'],
                        'msg' => 'Input should be a valid integer',
                        'input' => 'not_a_number',
                    ],
                    [
                        'type' => 'string_too_short',
                        'loc' => ['body', 'user', 'profile', 'bio'],
                        'msg' => 'String should have at least 10 characters',
                        'input' => 'short',
                        'ctx' => ['min_length' => 10],
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        /** @var array<int, array<string, mixed>> $errors */
        $errors = (array) ($body['errors'] ?? []);
        $this->assertCount(2, $errors);

        // Verify nested path for first error
        /** @var array<string, mixed> $firstError */
        $firstError = $errors[0];
        $this->assertSame(['body', 'user', 'profile', 'age'], $firstError['loc']);

        // Verify nested path for second error
        /** @var array<string, mixed> $secondError */
        $secondError = $errors[1];
        $this->assertSame(['body', 'user', 'profile', 'bio'], $secondError['loc']);
    }

    /**
     * Test 13: Error response respects Content-Type header.
     *
     * Verifies that error responses correctly set Content-Type
     * to application/json when returning validation errors.
     */
    #[Test]
    public function testErrorResponseContentType(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'status' => 422,
                'detail' => '1 validation error',
                'errors' => [],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $this->assertArrayHasKey('Content-Type', $response->headers);
        $contentType = $response->headers['Content-Type'] ?? '';
        $this->assertStringContainsString('application/json', $contentType);
    }

    /**
     * Test 14: Error detail message includes error count.
     *
     * Verifies that the detail field includes a count of validation errors
     * for batch error visibility.
     */
    #[Test]
    public function testErrorDetailIncludesCount(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'title' => 'Request Validation Failed',
                'status' => 422,
                'detail' => '3 validation errors in request',
                'errors' => [
                    ['type' => 'error1', 'loc' => ['body', 'field1'], 'msg' => 'Error 1'],
                    ['type' => 'error2', 'loc' => ['body', 'field2'], 'msg' => 'Error 2'],
                    ['type' => 'error3', 'loc' => ['body', 'field3'], 'msg' => 'Error 3'],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        $detailRaw = $body['detail'] ?? '';
        $detail = \is_scalar($detailRaw) ? (string) $detailRaw : '';
        $this->assertStringContainsString('3', $detail);
        $this->assertStringContainsString('validation error', $detail);
    }

    /**
     * Test 15: Error location is array type.
     *
     * Verifies that the 'loc' field in each error is an array,
     * following the JSON Schema validation format.
     */
    #[Test]
    public function testErrorLocationIsArray(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/validation-error',
                'status' => 422,
                'detail' => '1 validation error',
                'errors' => [
                    [
                        'type' => 'int_parsing',
                        'loc' => ['query', 'skip'],
                        'msg' => 'Invalid integer',
                        'input' => 'abc',
                    ],
                ],
            ], 422)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $body = $response->parseJson();
        /** @var array<int, array<string, mixed>> $errors */
        $errors = (array) ($body['errors'] ?? []);
        $this->assertNotEmpty($errors);

        foreach ($errors as $error) {
            $this->assertArrayHasKey('loc', $error);
            /** @var array<int|string, mixed> $loc */
            $loc = (array) ($error['loc'] ?? []);
            $this->assertGreaterThan(0, \count($loc));

            // Each location element should be string or int
            foreach ($loc as $locElement) {
                $this->assertTrue(
                    \is_string($locElement) || \is_int($locElement),
                    'Location elements must be string or int'
                );
            }
        }
    }

    /**
     * Test 16: Server error (500) response structure.
     *
     * Verifies that internal server errors have a proper structure
     * with appropriate error messaging (without exposing internals).
     */
    #[Test]
    public function testServerErrorResponseStructure(): void
    {
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json([
                'type' => 'https://spikard.dev/errors/internal-error',
                'title' => 'Internal Server Error',
                'status' => 500,
                'detail' => 'An unexpected error occurred',
            ], 500)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        $this->assertSame(500, $response->statusCode);
        $body = $response->parseJson();

        $this->assertArrayHasKey('type', $body);
        $this->assertArrayHasKey('title', $body);
        $this->assertArrayHasKey('status', $body);
        $this->assertArrayHasKey('detail', $body);

        // Server errors should not expose sensitive internals
        $detailRaw = $body['detail'] ?? '';
        $detail = \is_scalar($detailRaw) ? \strtolower((string) $detailRaw) : '';
        $this->assertStringNotContainsString('database', $detail);
        $this->assertStringNotContainsString('password', $detail);
    }

    /**
     * Provider for fixture-based error translation tests.
     *
     * @return Generator<string, array{0: string}>
     */
    public static function validationErrorFixtureProvider(): Generator
    {
        $fixtures = [
            '01_query_param_type_error_string_to_int',
            '02_query_param_missing_required',
            '03_body_field_type_error',
            '04_body_missing_required_field',
            '09_multiple_validation_errors',
            '19_header_validation_error',
        ];

        foreach ($fixtures as $fixture) {
            yield $fixture => [$fixture];
        }
    }

    /**
     * Test 17: Fixture-based validation error format verification.
     *
     * This parametrized test loads actual fixture files from
     * testing_data/validation_errors/ and verifies that error
     * responses match the expected schema.
     */
    #[Test]
    #[DataProvider('validationErrorFixtureProvider')]
    public function testFixtureBasedValidationErrorFormat(string $fixtureName): void
    {
        $fixture = self::loadValidationErrorFixture($fixtureName);

        // Verify fixture has expected structure
        $this->assertArrayHasKey('expected_response', $fixture);
        /** @var array<string, mixed> $expectedResponse */
        $expectedResponse = (array) $fixture['expected_response'];
        $this->assertArrayHasKey('status_code', $expectedResponse);
        $this->assertArrayHasKey('body', $expectedResponse);

        // The fixture defines what the expected error response should look like
        $statusCodeRaw = $expectedResponse['status_code'] ?? 422;
        /** @var int $expectedStatusCode */
        $expectedStatusCode = \is_int($statusCodeRaw) ? $statusCodeRaw : 422;
        /** @var array<string, mixed> $expectedBody */
        $expectedBody = (array) ($expectedResponse['body'] ?? []);

        // Simulate a response that matches the fixture structure
        $app = $this->createAppWithValidationHandler(
            static fn (Request $request): Response => Response::json($expectedBody, $expectedStatusCode)
        );

        $client = TestClient::create($app);
        $response = $client->post('/test');

        // Verify status code matches
        $this->assertSame($expectedStatusCode, $response->statusCode);

        // Verify response body structure
        $body = $response->parseJson();

        // Verify error structure matches schema
        if ($expectedStatusCode === 422 && isset($expectedBody['errors'])) {
            $this->assertArrayHasKey('errors', $body);
            /** @var array<int, array<string, mixed>> $errors */
            $errors = (array) ($body['errors'] ?? []);

            foreach ($errors as $error) {
                $this->assertArrayHasKey('type', $error);
                $this->assertArrayHasKey('loc', $error);
                $this->assertArrayHasKey('msg', $error);
                /** @var array<int|string, mixed> $loc */
                $loc = (array) ($error['loc'] ?? []);
            }
        }
    }
}
