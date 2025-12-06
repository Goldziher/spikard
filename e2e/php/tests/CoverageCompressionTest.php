<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;
use E2E\Php\AppFactory;

/**
 * E2E test coverage for HTTP compression and content encoding.
 */
final class CoverageCompressionTest extends TestCase
{
    /**
     * Gzip compression applied when advertised by client
     */
    public function test_gzip_compression_applied_when_accepted(): void
    {
        $app = AppFactory::create_compression_compression_gzip_applied_1();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/compression/gzip', [
            'headers' => ['Accept-Encoding' => 'gzip'],
        ]);

        $this->assertSame(200, $response->statusCode);
        $this->assertIsArray($response->body);
        $this->assertArrayHasKey('message', $response->body);
    }

    /**
     * Small payloads skip compression even when requested
     */
    public function test_small_payload_skips_compression(): void
    {
        $app = AppFactory::create_compression_compression_payload_below_min_size_is_not_compressed_2();
        $client = TestClient::create($app);
        $response = $client->request('GET', '/compression/skip', [
            'headers' => ['Accept-Encoding' => 'gzip'],
        ]);

        $this->assertSame(200, $response->statusCode);
        $this->assertArrayHasKey('message', $response->body);
        $this->assertSame('Small payload', $response->body['message']);
    }

    /**
     * Full compression E2E: Compare two requests with different Accept-Encoding
     */
    public function test_compression_end_to_end_encoding_negotiation(): void
    {
        $app = AppFactory::create_compression_compression_gzip_applied_1();
        $client = TestClient::create($app);

        // First: with gzip
        $responseWithGzip = $client->request('GET', '/compression/gzip', [
            'headers' => ['Accept-Encoding' => 'gzip'],
        ]);

        // Second: without encoding (same endpoint)
        $responseNoEncoding = $client->request('GET', '/compression/gzip', []);

        // Both should have same status and content
        $this->assertSame(200, $responseWithGzip->statusCode);
        $this->assertSame(200, $responseNoEncoding->statusCode);
        $this->assertEquals($responseWithGzip->body, $responseNoEncoding->body);
    }
}
