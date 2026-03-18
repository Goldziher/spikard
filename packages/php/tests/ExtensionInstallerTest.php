<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Support\ExtensionInstaller;

final class ExtensionInstallerTest extends TestCase
{
    public function testInstallerDownloadsAndConfiguresLocalReleaseAsset(): void
    {
        $root = \sys_get_temp_dir() . '/spikard-installer-' . \bin2hex(\random_bytes(8));
        $releaseDir = $root . '/release';
        $extDir = $root . '/ext';
        $iniDir = $root . '/ini';

        \mkdir($releaseDir, 0777, true);
        \mkdir($iniDir, 0777, true);
        \file_put_contents($root . '/composer.json', \json_encode(['version' => '0.12.0'], JSON_THROW_ON_ERROR));

        $installer = new ExtensionInstaller($root, [
            'SPIKARD_PHP_RELEASE_BASE_URL' => 'file://' . $releaseDir,
            'SPIKARD_PHP_EXTENSION_DIR' => $extDir,
            'SPIKARD_PHP_INI_DIR' => $iniDir,
        ]);

        $archivePath = $releaseDir . '/' . $installer->assetFileName();
        $this->createReleaseArchive($archivePath);

        \ob_start();
        $result = $installer->install();
        \ob_end_clean();

        $this->assertTrue($result);
        $this->assertFileExists($extDir . '/spikard.so');
        $this->assertFileExists($iniDir . '/99-spikard.ini');

        $iniContents = \file_get_contents($iniDir . '/99-spikard.ini');
        $this->assertIsString($iniContents);
        $this->assertStringContainsString('extension="', $iniContents);
        $this->assertStringContainsString('/spikard.so"', $iniContents);
    }

    public function testInstallerUsesExplicitPackageVersionOverride(): void
    {
        $root = \sys_get_temp_dir() . '/spikard-installer-version-' . \bin2hex(\random_bytes(8));
        \mkdir($root, 0777, true);
        \file_put_contents($root . '/composer.json', \json_encode(['name' => 'consumer/app'], JSON_THROW_ON_ERROR));

        $installer = new ExtensionInstaller($root, [], '0.12.0');
        $assetFile = $installer->assetFileName();

        $this->assertStringContainsString('/v0.12.0/', $installer->releaseAssetUrl());
        $this->assertNotSame('', $assetFile);
        /** @var non-empty-string $assetFile */
        $this->assertStringEndsWith($assetFile, $installer->releaseAssetUrl());
    }

    private function createReleaseArchive(string $archivePath): void
    {
        $archiveDir = \dirname($archivePath);
        $tempDir = $archiveDir . '/package';
        $tarPath = \substr($archivePath, 0, -3);
        $packageDir = $tempDir . '/payload';

        \mkdir($packageDir, 0777, true);
        \file_put_contents($packageDir . '/libspikard_php.so', 'fake-binary');

        @\unlink($tarPath);
        @\unlink($archivePath);

        $tar = new \PharData($tarPath);
        $tar->buildFromDirectory($tempDir);
        $tar->compress(\Phar::GZ);
        unset($tar);

        \rename($tarPath . '.gz', $archivePath);
    }
}
