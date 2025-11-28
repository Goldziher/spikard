#!/usr/bin/env php
<?php
declare(strict_types=1);

/**
 * Post-install hook to fetch and install pre-built Spikard PHP extension.
 * Falls back to PIE source build if binary not available.
 */

// This is a stub for future binary installation
// For now, it simply echoes a status message

class ExtensionInstaller
{
    private string $composerVersion;
    private string $osName;
    private string $phpVersion;
    private string $arch;

    public function __construct()
    {
        $composerJsonPath = __DIR__ . '/../composer.json';
        if (file_exists($composerJsonPath)) {
            $composerData = json_decode(
                file_get_contents($composerJsonPath),
                true
            );
            $this->composerVersion = $composerData['version'] ?? 'unknown';
        } else {
            $this->composerVersion = 'unknown';
        }

        $this->detectEnvironment();
    }

    private function detectEnvironment(): void
    {
        $this->osName = PHP_OS_FAMILY;
        $this->phpVersion = PHP_MAJOR_VERSION . '.' . PHP_MINOR_VERSION;
        $this->arch = php_uname('m');

        // Normalize arch
        if (in_array($this->arch, ['x86_64', 'AMD64'])) {
            $this->arch = 'x86_64';
        }
    }

    public function install(): bool
    {
        $platformKey = $this->getPlatformKey();
        echo "Spikard PHP Extension Installer\n";
        echo "Detected platform: {$platformKey}\n";
        echo "Version: {$this->composerVersion}\n";

        if ($this->installBinary()) {
            return true;
        }

        echo "\nBinary not available for this platform.\n";
        echo "To compile from source, use the PIE (PHP Inspector Extension) build:\n";
        echo "  pie build goldziher/spikard-extension\n";
        return false;
    }

    private function getPlatformKey(): string
    {
        $osMap = [
            'Linux' => 'linux',
            'Darwin' => 'macos',
            'Windows' => 'windows',
        ];
        $os = $osMap[$this->osName] ?? strtolower($this->osName);
        return "{$os}-{$this->arch}-{$this->phpVersion}";
    }

    private function installBinary(): bool
    {
        // TODO: Implement binary download and installation from GitHub Releases
        // This will be populated once release workflow is complete
        return false;
    }
}

$installer = new ExtensionInstaller();
if (!$installer->install()) {
    exit(0); // Don't fail Composer if binary unavailable
}
