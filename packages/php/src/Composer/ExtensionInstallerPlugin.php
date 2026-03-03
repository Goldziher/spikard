<?php

declare(strict_types=1);

namespace Spikard\Composer;

use Composer\Composer;
use Composer\EventDispatcher\EventSubscriberInterface;
use Composer\IO\IOInterface;
use Composer\Plugin\PluginInterface;
use Composer\Script\Event;
use Composer\Script\ScriptEvents;
use Spikard\Support\ExtensionInstaller;
use Throwable;

final class ExtensionInstallerPlugin implements EventSubscriberInterface, PluginInterface
{
    private Composer $composer;
    private IOInterface $io;
    private bool $hasRun = false;

    public function activate(Composer $composer, IOInterface $io): void
    {
        $this->composer = $composer;
        $this->io = $io;
    }

    public function deactivate(Composer $composer, IOInterface $io): void
    {
    }

    public function uninstall(Composer $composer, IOInterface $io): void
    {
    }

    /**
     * @return array<string, string>
     */
    public static function getSubscribedEvents(): array
    {
        return [
            ScriptEvents::POST_INSTALL_CMD => 'installExtension',
            ScriptEvents::POST_UPDATE_CMD => 'installExtension',
        ];
    }

    public function installExtension(Event $event): void
    {
        if ($this->hasRun) {
            return;
        }
        $this->hasRun = true;

        try {
            $projectRoot = $this->projectRoot();
            $packageRoot = self::packageRoot();
            $installer = new ExtensionInstaller(
                $projectRoot,
                $_ENV + $_SERVER,
                $this->detectPackageVersion($packageRoot),
            );

            ob_start();
            $installed = $installer->install();
            $output = ob_get_clean();

            if (is_string($output) && $output !== '') {
                $this->io->write($output, false);
            }

            if (!$installed) {
                $this->io->writeError('<warning>Spikard extension auto-install did not complete. See installer output above.</warning>');
            }
        } catch (Throwable $exception) {
            if (ob_get_level() > 0) {
                ob_end_clean();
            }
            $this->io->writeError(
                sprintf(
                    '<warning>Spikard extension auto-install failed: %s</warning>',
                    $exception->getMessage(),
                ),
            );
        }
    }

    private function projectRoot(): string
    {
        $vendorDir = $this->composer->getConfig()->get('vendor-dir');
        if (!is_string($vendorDir) || $vendorDir === '') {
            return getcwd() ?: self::packageRoot();
        }

        return dirname($vendorDir);
    }

    private function detectPackageVersion(string $packageRoot): ?string
    {
        $composerJsonPath = $packageRoot . '/composer.json';
        if (!is_file($composerJsonPath)) {
            return null;
        }

        $contents = file_get_contents($composerJsonPath);
        if (!is_string($contents) || $contents === '') {
            return null;
        }

        $data = json_decode($contents, true);
        if (!is_array($data)) {
            return null;
        }

        $version = $data['version'] ?? null;
        return is_string($version) && $version !== '' ? $version : null;
    }

    private static function packageRoot(): string
    {
        return dirname(__DIR__, 2);
    }
}
