<?php

declare(strict_types=1);

namespace Spikard\Support;

use PharData;
use RecursiveDirectoryIterator;
use RecursiveIteratorIterator;
use RuntimeException;

final class ExtensionInstaller
{
    private const REPOSITORY = 'Goldziher/spikard';

    /** @var array<string, string> */
    private array $env;
    private string $projectRoot;
    private string $composerVersion;
    private string $osName;
    private string $phpVersion;
    private string $arch;

    /**
     * @param array<string, string>|null $env
     */
    public function __construct(string $projectRoot, ?array $env = null)
    {
        $this->projectRoot = rtrim($projectRoot, '/');
        $this->env = $env ?? $_ENV + $_SERVER;
        $this->composerVersion = $this->detectComposerVersion();
        $this->osName = PHP_OS_FAMILY;
        $this->phpVersion = PHP_MAJOR_VERSION . '.' . PHP_MINOR_VERSION;
        $this->arch = $this->normalizeArch(php_uname('m'));
    }

    public function install(): bool
    {
        $this->printHeader();

        if (extension_loaded('spikard') || extension_loaded('spikard_php')) {
            echo "Extension already loaded.\n";
            return true;
        }

        $archivePath = $this->downloadArchive();
        if ($archivePath === null) {
            return $this->finishUnavailable();
        }

        $libraryPath = $this->extractLibrary($archivePath);
        if ($libraryPath === null) {
            return $this->finishUnavailable();
        }

        $installedPath = $this->installLibrary($libraryPath);
        if ($installedPath === null) {
            return $this->finishUnavailable();
        }

        return $this->activateLibrary($installedPath);
    }

    public function assetFileName(): string
    {
        return sprintf('php-extension-%s-%s.tar.gz', $this->osLabel(), $this->phpVersion);
    }

    public function releaseAssetUrl(): string
    {
        return rtrim($this->releaseBaseUrl(), '/') . '/' . $this->assetFileName();
    }

    public function platformKey(): string
    {
        return sprintf('%s-%s-%s', $this->osLabel(), $this->arch, $this->phpVersion);
    }

    private function printHeader(): void
    {
        echo "Spikard PHP Extension Installer\n";
        echo "Detected platform: {$this->platformKey()}\n";
        echo "Version: {$this->composerVersion}\n";
        echo "Release asset: {$this->assetFileName()}\n";
    }

    private function detectComposerVersion(): string
    {
        $composerJsonPath = $this->projectRoot . '/composer.json';
        if (!is_file($composerJsonPath)) {
            return 'unknown';
        }

        $json = file_get_contents($composerJsonPath);
        if ($json === false) {
            return 'unknown';
        }

        $data = json_decode($json, true);
        return is_array($data) && isset($data['version']) && is_string($data['version'])
            ? $data['version']
            : 'unknown';
    }

    private function normalizeArch(string $arch): string
    {
        return match ($arch) {
            'x86_64', 'AMD64' => 'x86_64',
            'arm64', 'aarch64' => 'arm64',
            default => strtolower($arch),
        };
    }

    private function releaseBaseUrl(): string
    {
        $override = $this->env['SPIKARD_PHP_RELEASE_BASE_URL'] ?? null;
        if (is_string($override) && $override !== '') {
            return $override;
        }

        return sprintf(
            'https://github.com/%s/releases/download/%s',
            self::REPOSITORY,
            $this->releaseTag()
        );
    }

    private function releaseTag(): string
    {
        if ($this->composerVersion === 'unknown') {
            return 'latest';
        }

        return str_starts_with($this->composerVersion, 'v')
            ? $this->composerVersion
            : 'v' . $this->composerVersion;
    }

    private function osLabel(): string
    {
        return match ($this->osName) {
            'Linux' => 'linux',
            'Darwin' => 'macos',
            'Windows' => 'windows',
            default => strtolower($this->osName),
        };
    }

    private function downloadArchive(): ?string
    {
        $target = $this->createTempArchivePath();
        $source = $this->releaseAssetUrl();
        if (!@copy($source, $target)) {
            return null;
        }

        return $target;
    }

    private function createTempArchivePath(): string
    {
        $tmp = tempnam(sys_get_temp_dir(), 'spikard-php-');
        if ($tmp === false) {
            throw new RuntimeException('Unable to allocate temporary archive path');
        }

        unlink($tmp);
        return $tmp . '.tar.gz';
    }

    private function extractLibrary(string $archivePath): ?string
    {
        $extractDir = $this->extractionRoot();
        $tarPath = substr($archivePath, 0, -3);

        try {
            if (!is_dir($extractDir) && !mkdir($extractDir, 0777, true) && !is_dir($extractDir)) {
                return null;
            }

            if (is_file($tarPath)) {
                unlink($tarPath);
            }

            $archive = new PharData($archivePath);
            $archive->decompress();
            $tar = new PharData($tarPath);
            $tar->extractTo($extractDir, null, true);
        } catch (\Throwable) {
            return null;
        }

        return $this->findExtractedLibrary($extractDir);
    }

    private function extractionRoot(): string
    {
        return $this->projectRoot . '/build/php-extension/extracted/' . $this->platformKey();
    }

    private function findExtractedLibrary(string $directory): ?string
    {
        $iterator = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($directory));
        foreach ($iterator as $file) {
            if (!$file instanceof \SplFileInfo) {
                continue;
            }

            if (!$file->isFile()) {
                continue;
            }

            $path = $file->getPathname();
            if ($this->isSharedLibrary($path)) {
                return $path;
            }
        }

        return null;
    }

    private function isSharedLibrary(string $path): bool
    {
        return str_ends_with($path, '.so')
            || str_ends_with($path, '.dylib')
            || str_ends_with($path, '.dll');
    }

    private function installLibrary(string $sourcePath): ?string
    {
        $targetDir = $this->extensionInstallDir();
        if (!is_dir($targetDir) && !mkdir($targetDir, 0777, true) && !is_dir($targetDir)) {
            return null;
        }

        $targetPath = $targetDir . '/' . $this->installedLibraryName($sourcePath);
        if (!copy($sourcePath, $targetPath)) {
            return null;
        }

        return $targetPath;
    }

    private function extensionInstallDir(): string
    {
        $override = $this->env['SPIKARD_PHP_EXTENSION_DIR'] ?? null;
        if (is_string($override) && $override !== '') {
            return rtrim($override, '/');
        }

        return $this->projectRoot . '/build/php-extension/lib';
    }

    private function installedLibraryName(string $sourcePath): string
    {
        return match (true) {
            str_ends_with($sourcePath, '.dll') => 'spikard.dll',
            str_ends_with($sourcePath, '.dylib') => 'spikard.dylib',
            default => 'spikard.so',
        };
    }

    private function activateLibrary(string $installedPath): bool
    {
        $iniPath = $this->resolveIniPath();
        if ($iniPath === null) {
            $this->printManualActivation($installedPath);
            return false;
        }

        $contents = "; generated by Spikard installer\nextension=" . $this->quoteIniPath($installedPath) . "\n";
        if (file_put_contents($iniPath, $contents) === false) {
            $this->printManualActivation($installedPath);
            return false;
        }

        echo "Installed extension library to {$installedPath}\n";
        echo "Wrote activation file to {$iniPath}\n";
        return true;
    }

    private function resolveIniPath(): ?string
    {
        $iniFile = $this->env['SPIKARD_PHP_INI_FILE'] ?? null;
        if (is_string($iniFile) && $iniFile !== '') {
            return $iniFile;
        }

        $iniDir = $this->env['SPIKARD_PHP_INI_DIR'] ?? null;
        if (is_string($iniDir) && $iniDir !== '') {
            return rtrim($iniDir, '/') . '/99-spikard.ini';
        }

        foreach ($this->scannedIniDirectories() as $directory) {
            if (is_dir($directory) && is_writable($directory)) {
                return rtrim($directory, '/') . '/99-spikard.ini';
            }
        }

        return null;
    }

    /**
     * @return list<string>
     */
    private function scannedIniDirectories(): array
    {
        $scanned = php_ini_scanned_files();
        if ($scanned === false) {
            return [];
        }

        $directories = [];
        foreach (explode(',', $scanned) as $file) {
            $file = trim($file);
            if ($file === '') {
                continue;
            }

            $directories[] = dirname($file);
        }

        return array_values(array_unique($directories));
    }

    private function quoteIniPath(string $installedPath): string
    {
        return '"' . str_replace('\\', '/', $installedPath) . '"';
    }

    private function finishUnavailable(): bool
    {
        echo "\nBinary not available for this platform.\n";
        echo "Expected release asset: {$this->releaseAssetUrl()}\n";
        echo "To compile from source, use the PIE (PHP Inspector Extension) build:\n";
        echo "  pie build goldziher/spikard-extension\n";
        return false;
    }

    private function printManualActivation(string $installedPath): void
    {
        echo "Installed extension library to {$installedPath}\n";
        echo "Automatic INI activation was not possible.\n";
        echo "Create a PHP INI file with:\n";
        echo "  extension=" . $this->quoteIniPath($installedPath) . "\n";
    }
}
