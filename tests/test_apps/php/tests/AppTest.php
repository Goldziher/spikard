<?php

declare(strict_types=1);

namespace Spikard\TestApp\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\TestApp\App;

final class AppTest extends TestCase
{
    private static mixed $server = null;
    private static string $baseUrl = '';

    public static function setUpBeforeClass(): void
    {
        self::$server = App::createApp();
        self::$server->start();
        $address = self::$server->address();
        self::$baseUrl = sprintf('http://%s:%d', $address['host'], $address['port']);
    }

    public static function tearDownAfterClass(): void
    {
        self::$server?->stop();
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
        $this->assertSame('0.6.0', $package['version']);
    }

    public function testRespondsToHealthCheck(): void
    {
        $response = file_get_contents(self::$baseUrl . '/health');
        $this->assertNotFalse($response);

        $data = json_decode($response, true, 512, JSON_THROW_ON_ERROR);
        $this->assertSame(['status' => 'ok'], $data);
    }

    public function testHandlesQueryParameters(): void
    {
        $url = self::$baseUrl . '/query?' . http_build_query(['name' => 'Alice', 'age' => '30']);
        $response = file_get_contents($url);
        $this->assertNotFalse($response);

        $data = json_decode($response, true, 512, JSON_THROW_ON_ERROR);
        $this->assertSame(['name' => 'Alice', 'age' => 30], $data);
    }

    public function testEchoesJsonRequests(): void
    {
        $payload = ['message' => 'Hello from PHP!'];
        $context = stream_context_create([
            'http' => [
                'method' => 'POST',
                'header' => 'Content-Type: application/json',
                'content' => json_encode($payload, JSON_THROW_ON_ERROR),
            ],
        ]);

        $response = file_get_contents(self::$baseUrl . '/echo', false, $context);
        $this->assertNotFalse($response);

        $data = json_decode($response, true, 512, JSON_THROW_ON_ERROR);
        $this->assertSame($payload, $data['received']);
        $this->assertSame('POST', $data['method']);
    }

    public function testExtractsPathParameters(): void
    {
        $response = file_get_contents(self::$baseUrl . '/users/42');
        $this->assertNotFalse($response);

        $data = json_decode($response, true, 512, JSON_THROW_ON_ERROR);
        $this->assertSame('42', $data['userId']);
        $this->assertSame('string', $data['type']);
    }
}
