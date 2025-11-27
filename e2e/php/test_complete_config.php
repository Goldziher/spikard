<?php
declare(strict_types=1);

/**
 * Test script to verify complete ServerConfig serialization.
 *
 * Tests all middleware configurations including the newly added fields:
 * - Basic server settings (host, port, workers)
 * - Request handling (enableRequestId, maxBodySize, requestTimeout)
 * - Graceful shutdown settings
 * - JWT authentication
 * - API Key authentication
 * - OpenAPI documentation
 * - Compression, rate limiting, CORS
 */

require_once __DIR__ . '/../../packages/php/vendor/autoload.php';

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Config\CompressionConfig;
use Spikard\Config\RateLimitConfig;
use Spikard\Config\CorsConfig;
use Spikard\Config\StaticFilesConfig;
use Spikard\Config\JwtConfig;
use Spikard\Config\ApiKeyConfig;
use Spikard\Config\OpenApiConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

echo "Testing Complete ServerConfig Serialization...\n\n";

// Create a simple handler
$handler = new class implements \Spikard\Handlers\HandlerInterface {
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return new Response(
            statusCode: 200,
            body: ['message' => 'Hello from PHP!', 'authenticated' => true],
            headers: ['Content-Type' => 'application/json']
        );
    }
};

// Create app with complete configuration
$app = new App();
$app->addRoute('GET', '/api/hello', $handler);

// Build comprehensive server configuration using all available options
echo "Building ServerConfig with all fields...\n";

$config = ServerConfig::builder()
    // Basic server settings
    ->withHost('0.0.0.0')
    ->withPort(9000)
    ->withWorkers(4)

    // Request handling
    ->withRequestId(true)
    ->withMaxBodySize(5 * 1024 * 1024) // 5MB
    ->withRequestTimeout(60)

    // Graceful shutdown
    ->withGracefulShutdown(true)
    ->withShutdownTimeout(15)

    // Compression
    ->withCompression(new CompressionConfig(
        enabled: true,
        quality: 6
    ))

    // Rate limiting
    ->withRateLimit(new RateLimitConfig(
        refill: 100,
        burst: 20
    ))

    // CORS
    ->withCors(new CorsConfig(
        enabled: true,
        allowedOrigins: ['http://localhost:3000', 'https://app.example.com'],
        allowedMethods: ['GET', 'POST', 'PUT', 'DELETE'],
        allowedHeaders: ['Content-Type', 'Authorization'],
        exposedHeaders: ['X-Request-ID', 'X-RateLimit-Remaining'],
        allowCredentials: true,
        maxAgeSeconds: 3600
    ))

    // Static files
    ->withStaticFiles(new StaticFilesConfig(
        enabled: true,
        root: '/var/www/static',
        indexFile: 'index.html',
        cache: true
    ))

    // JWT authentication
    ->withJwtAuth(new JwtConfig(
        secret: 'my-secret-key-change-in-production',
        algorithm: 'HS256',
        audience: ['https://api.example.com'],
        issuer: 'https://auth.example.com',
        leeway: 5
    ))

    // API Key authentication
    ->withApiKeyAuth(new ApiKeyConfig(
        keys: ['sk_test_123456', 'sk_test_789012', 'sk_prod_abcdef'],
        headerName: 'X-API-Key'
    ))

    // OpenAPI documentation
    ->withOpenApi(new OpenApiConfig(
        enabled: true,
        title: 'My API',
        version: '2.0.0',
        description: 'A comprehensive API with full authentication and documentation',
        swaggerUiPath: '/docs',
        redocPath: '/redoc',
        openapiJsonPath: '/openapi.json'
    ))

    ->build();

echo "✅ ServerConfig built successfully!\n\n";

// Test the configToNative serialization
echo "Testing configToNative() serialization...\n";

// Access the private method via reflection to test serialization
$reflection = new \ReflectionClass($app);
$method = $reflection->getMethod('configToNative');
$method->setAccessible(true);
$nativeConfig = $method->invoke($app, $config);

echo "✅ Serialization successful!\n\n";

// Verify all fields are present
echo "Verifying serialized config structure:\n";

$requiredFields = [
    'host', 'port', 'workers',
    'enable_request_id', 'max_body_size', 'request_timeout',
    'graceful_shutdown', 'shutdown_timeout',
    'compression', 'rate_limit', 'cors', 'static_files',
    'jwt_auth', 'api_key_auth', 'openapi'
];

$missingFields = [];
foreach ($requiredFields as $field) {
    if (!array_key_exists($field, $nativeConfig)) {
        $missingFields[] = $field;
    }
}

if (empty($missingFields)) {
    echo "✅ All required fields present in serialized config\n\n";
} else {
    echo "❌ Missing fields: " . implode(', ', $missingFields) . "\n\n";
    exit(1);
}

// Verify field values and snake_case naming
echo "Verifying field values:\n";

$checks = [
    ['field' => 'host', 'expected' => '0.0.0.0', 'actual' => $nativeConfig['host']],
    ['field' => 'port', 'expected' => 9000, 'actual' => $nativeConfig['port']],
    ['field' => 'workers', 'expected' => 4, 'actual' => $nativeConfig['workers']],
    ['field' => 'enable_request_id', 'expected' => true, 'actual' => $nativeConfig['enable_request_id']],
    ['field' => 'max_body_size', 'expected' => 5 * 1024 * 1024, 'actual' => $nativeConfig['max_body_size']],
    ['field' => 'request_timeout', 'expected' => 60, 'actual' => $nativeConfig['request_timeout']],
    ['field' => 'graceful_shutdown', 'expected' => true, 'actual' => $nativeConfig['graceful_shutdown']],
    ['field' => 'shutdown_timeout', 'expected' => 15, 'actual' => $nativeConfig['shutdown_timeout']],
];

$failed = false;
foreach ($checks as $check) {
    if ($check['actual'] !== $check['expected']) {
        echo "❌ {$check['field']}: expected {$check['expected']}, got {$check['actual']}\n";
        $failed = true;
    } else {
        echo "✅ {$check['field']}: {$check['actual']}\n";
    }
}

// Verify nested structures use snake_case
echo "\nVerifying snake_case in nested structures:\n";

// Compression
if (isset($nativeConfig['compression']['min_size'])) {
    echo "✅ compression.min_size (snake_case)\n";
} else {
    echo "❌ compression should use snake_case (min_size)\n";
    $failed = true;
}

// Rate limit
if (isset($nativeConfig['rate_limit']['per_second'])) {
    echo "✅ rate_limit.per_second (snake_case)\n";
} else {
    echo "❌ rate_limit should use snake_case (per_second)\n";
    $failed = true;
}

// CORS
if (isset($nativeConfig['cors']['allowed_origins'])) {
    echo "✅ cors.allowed_origins (snake_case)\n";
} else {
    echo "❌ cors should use snake_case (allowed_origins)\n";
    $failed = true;
}

// JWT auth
if (isset($nativeConfig['jwt_auth']['secret'])) {
    echo "✅ jwt_auth.secret present\n";
} else {
    echo "❌ jwt_auth should have secret field\n";
    $failed = true;
}

// API key auth
if (isset($nativeConfig['api_key_auth']['keys'])) {
    echo "✅ api_key_auth.keys present\n";
} else {
    echo "❌ api_key_auth should have keys field\n";
    $failed = true;
}

// OpenAPI
if (isset($nativeConfig['openapi']['swagger_ui_path'])) {
    echo "✅ openapi.swagger_ui_path (snake_case)\n";
} else {
    echo "❌ openapi should use snake_case (swagger_ui_path)\n";
    $failed = true;
}

if ($failed) {
    echo "\n❌ Some checks failed!\n";
    echo "\nDumping full config for debugging:\n";
    print_r($nativeConfig);
    exit(1);
}

echo "\n🎉 All checks passed!\n";
echo "\n✅ ServerConfig serialization is complete and correct\n";
echo "✅ All fields use proper snake_case naming\n";
echo "✅ Ready for Rust FFI extract_server_config_from_php()\n";
