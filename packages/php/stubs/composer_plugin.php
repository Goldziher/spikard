<?php

declare(strict_types=1);

namespace Composer;

class Composer
{
    public function getConfig(): Config
    {
        return new Config();
    }
}

class Config
{
    public function get(string $key): mixed
    {
        return null;
    }
}

namespace Composer\EventDispatcher;

interface EventSubscriberInterface
{
    /**
     * @return array<string, string>
     */
    public static function getSubscribedEvents(): array;
}

namespace Composer\IO;

interface IOInterface
{
    /**
     * @param array<int, string>|string $messages
     */
    public function write(array|string $messages, bool $newline = true, int $verbosity = 0): void;

    /**
     * @param array<int, string>|string $messages
     */
    public function writeError(array|string $messages, bool $newline = true, int $verbosity = 0): void;
}

namespace Composer\Plugin;

use Composer\Composer;
use Composer\IO\IOInterface;

interface PluginInterface
{
    public function activate(Composer $composer, IOInterface $io): void;

    public function deactivate(Composer $composer, IOInterface $io): void;

    public function uninstall(Composer $composer, IOInterface $io): void;
}

namespace Composer\Script;

class Event
{
}

final class ScriptEvents
{
    public const POST_INSTALL_CMD = 'post-install-cmd';
    public const POST_UPDATE_CMD = 'post-update-cmd';
}
