<?php

declare(strict_types=1);

namespace Spikard\TestApp\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;
use Spikard\TestApp\App;

final class AppTest extends TestCase
{
    private static TestClient $client;

    public static function setUpBeforeClass(): void
    {
        self::$client = App::createApp();
    }

    public function testUsesCorrectPackageVersion(): void
    {
        $composerLock = json_decode(
            file_get_contents(__DIR__ . '/../composer.lock'),
            true,
            512,
            JSON_THROW_ON_ERROR
        );

        $spikardPackage = array_filter(
            $composerLock['packages'],
            static fn(array $pkg): bool => $pkg['name'] === 'spikard/spikard'
        );

        $this->assertCount(1, $spikardPackage);
        $package = array_values($spikardPackage)[0];
        $this->assertSame('0.7.0', $package['version']);
    }

    public function testRespondsToHealthCheck(): void
    {
        $response = self::$client->request('GET', '/health');

        $this->assertSame(200, $response->statusCode);
        $this->assertSame(['status' => 'ok'], $response->body);
    }

    public function testHandlesQueryParameters(): void
    {
        $path = '/query?' . http_build_query(['name' => 'Alice', 'age' => '30']);
        $response = self::$client->request('GET', $path);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame(['name' => 'Alice', 'age' => 30], $response->body);
    }

    public function testEchoesJsonRequests(): void
    {
        $payload = ['message' => 'Hello from PHP!'];
        $response = self::$client->request('POST', '/echo', [
            'body' => $payload,
        ]);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame($payload, $response->body['received']);
        $this->assertSame('POST', $response->body['method']);
    }

    public function testExtractsPathParameters(): void
    {
        // Path parameters require the native Rust extension for proper routing.
        // When running with TestClient in PHP mode (without extension),
        // path parameter routes cannot be matched due to exact path matching.
        // This is expected behavior - the native extension handles parameter matching.
        if (!\function_exists('spikard_version')) {
            $this->markTestSkipped('Path parameter routing requires native extension');
        }

        $response = self::$client->request('GET', '/users/42');

        $this->assertSame(200, $response->statusCode);
        $this->assertSame('42', $response->body['userId']);
        $this->assertSame('string', $response->body['type']);
    }
}
