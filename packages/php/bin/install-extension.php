#!/usr/bin/env php
<?php

declare(strict_types=1);

$autoloadPath = __DIR__ . '/../vendor/autoload.php';
if (is_file($autoloadPath)) {
    require_once $autoloadPath;
}

if (!class_exists(\Spikard\Support\ExtensionInstaller::class)) {
    fwrite(STDERR, "Spikard extension installer classes are unavailable.\n");
    exit(0);
}

$installer = new \Spikard\Support\ExtensionInstaller(dirname(__DIR__));
exit($installer->install() ? 0 : 0);
