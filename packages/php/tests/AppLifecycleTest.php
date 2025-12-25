<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;

final class AppLifecycleTest extends TestCase
{
    public function test_run_without_extension_throws(): void
    {
        [$exitCode, $output] = run_without_extension(
            '$config = \\Spikard\\Config\\ServerConfig::builder()->build();'
            . '$app = (new \\Spikard\\App())->registerController(new class () {'
            . '    #[\\Spikard\\Attributes\\Get(\'/hello\')]'
            . '    public function hello(): array { return [\'ok\' => true]; }'
            . '});'
            . '$app->run($config);'
        );

        $this->assertNotSame(0, $exitCode);
        $this->assertStringContainsString('extension is not loaded', $output);
    }

    public function test_close_is_noop_without_extension(): void
    {
        [$exitCode, $output] = run_without_extension(
            '$app = (new \\Spikard\\App())->registerController(new class () {'
            . '    #[\\Spikard\\Attributes\\Get(\'/hello\')]'
            . '    public function hello(): array { return [\'ok\' => true]; }'
            . '});'
            . '$client = \\Spikard\\Testing\\TestClient::create($app);'
            . '$response = $client->get(\'/hello\');'
            . 'if ($response->statusCode !== 200) { throw new \\RuntimeException(\'Unexpected status\'); }'
            . '$client->close();'
        );

        $this->assertSame(0, $exitCode, $output);
    }
}
