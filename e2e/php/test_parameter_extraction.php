<?php

declare(strict_types=1);

/**
 * Test parameter extraction helpers
 *
 * This test demonstrates the usage of Query, Path, Header, Cookie, and Body
 * parameter extraction helpers.
 */

require_once __DIR__ . '/../../packages/php/src/Http/Params/ParamBase.php';
require_once __DIR__ . '/../../packages/php/src/Http/Params/Query.php';
require_once __DIR__ . '/../../packages/php/src/Http/Params/Path.php';
require_once __DIR__ . '/../../packages/php/src/Http/Params/Header.php';
require_once __DIR__ . '/../../packages/php/src/Http/Params/Cookie.php';
require_once __DIR__ . '/../../packages/php/src/Http/Params/Body.php';

use Spikard\Http\Params\{Query, Path, Header, Cookie, Body};

echo "Testing Parameter Extraction Helpers\n";
echo "=====================================\n\n";

// Test 1: Query parameter with default
echo "Test 1: Query with default value\n";
$queryParam = new Query(default: 10);
echo "Has default: " . ($queryParam->hasDefault() ? 'yes' : 'no') . "\n";
echo "Default value: " . $queryParam->getDefault() . "\n";
assert($queryParam->getDefault() === 10);
echo "✓ Passed\n\n";

// Test 2: Query parameter with default factory
echo "Test 2: Query with default factory\n";
$queryParam = new Query(defaultFactory: fn() => ['tag1', 'tag2']);
echo "Has default: " . ($queryParam->hasDefault() ? 'yes' : 'no') . "\n";
$tags = $queryParam->getDefault();
echo "Default value: " . json_encode($tags) . "\n";
assert($tags === ['tag1', 'tag2']);
echo "✓ Passed\n\n";

// Test 3: Query parameter with schema
echo "Test 3: Query with JSON schema\n";
$queryParam = new Query(
    default: 10,
    schema: ['minimum' => 1, 'maximum' => 100]
);
$schema = $queryParam->getSchema();
echo "Schema: " . json_encode($schema) . "\n";
assert($schema['minimum'] === 1);
assert($schema['maximum'] === 100);
echo "✓ Passed\n\n";

// Test 4: Path parameter
echo "Test 4: Path parameter\n";
$pathParam = new Path(schema: ['minimum' => 1]);
echo "Has default: " . ($pathParam->hasDefault() ? 'yes' : 'no') . "\n";
echo "Schema: " . json_encode($pathParam->getSchema()) . "\n";
echo "✓ Passed\n\n";

// Test 5: Header parameter with alias
echo "Test 5: Header with alias\n";
$headerParam = new Header(
    alias: 'X-API-Key',
    schema: ['minLength' => 32]
);
echo "Alias: " . $headerParam->getAlias() . "\n";
echo "Convert underscores: " . ($headerParam->shouldConvertUnderscores() ? 'yes' : 'no') . "\n";
echo "Schema: " . json_encode($headerParam->getSchema()) . "\n";
assert($headerParam->getAlias() === 'X-API-Key');
assert($headerParam->shouldConvertUnderscores() === true);
echo "✓ Passed\n\n";

// Test 6: Header with default
echo "Test 6: Header with default value\n";
$headerParam = new Header(default: 'unknown');
echo "Default: " . $headerParam->getDefault() . "\n";
assert($headerParam->getDefault() === 'unknown');
echo "✓ Passed\n\n";

// Test 7: Cookie parameter with constraints
echo "Test 7: Cookie with validation constraints\n";
$cookieParam = new Cookie(
    minLength: 10,
    maxLength: 100,
    pattern: '/^[a-zA-Z0-9]+$/'
);
echo "Min length: " . $cookieParam->getMinLength() . "\n";
echo "Max length: " . $cookieParam->getMaxLength() . "\n";
echo "Pattern: " . $cookieParam->getPattern() . "\n";
assert($cookieParam->getMinLength() === 10);
assert($cookieParam->getMaxLength() === 100);
assert($cookieParam->getPattern() === '/^[a-zA-Z0-9]+$/');
echo "✓ Passed\n\n";

// Test 8: Cookie with default factory
echo "Test 8: Cookie with default factory\n";
$cookieParam = new Cookie(defaultFactory: fn() => ['user_id' => 123]);
$sessionData = $cookieParam->getDefault();
echo "Default: " . json_encode($sessionData) . "\n";
assert($sessionData['user_id'] === 123);
echo "✓ Passed\n\n";

// Test 9: Body parameter with schema
echo "Test 9: Body with JSON schema\n";
$bodyParam = new Body(
    schema: [
        'type' => 'object',
        'required' => ['name', 'price'],
        'properties' => [
            'name' => ['type' => 'string'],
            'price' => ['type' => 'number', 'minimum' => 0],
        ],
    ]
);
$schema = $bodyParam->getSchema();
echo "Schema: " . json_encode($schema, JSON_PRETTY_PRINT) . "\n";
assert($schema['type'] === 'object');
assert(in_array('name', $schema['required']));
echo "✓ Passed\n\n";

// Test 10: Body with default factory
echo "Test 10: Body with default factory\n";
$bodyParam = new Body(defaultFactory: fn() => []);
$data = $bodyParam->getDefault();
echo "Default: " . json_encode($data) . "\n";
assert($data === []);
echo "✓ Passed\n\n";

// Test 11: Callable invocation (__invoke)
echo "Test 11: Callable invocation with __invoke\n";
$queryParam = new Query(default: 42);
$value = $queryParam(); // Call as a function
echo "Invoked value: " . $value . "\n";
assert($value === 42);
echo "✓ Passed\n\n";

// Test 12: Error handling - both default and defaultFactory
echo "Test 12: Error handling - cannot have both default and defaultFactory\n";
try {
    new Query(default: 10, defaultFactory: fn() => 20);
    echo "✗ Failed - should have thrown exception\n";
} catch (\InvalidArgumentException $e) {
    echo "Caught expected exception: " . $e->getMessage() . "\n";
    echo "✓ Passed\n\n";
}

echo "=====================================\n";
echo "All tests passed! ✓\n";
