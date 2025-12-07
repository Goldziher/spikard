#!/usr/bin/env php
<?php

declare(strict_types=1);

/**
 * DEPRECATED: PHP DTO Generator
 *
 * This script is deprecated. Please use the Rust CLI instead:
 *
 *   spikard generate php-dto [--output src/Generated]
 *
 * This wrapper is kept for backward compatibility but will be removed in v1.1.0.
 * It now delegates to the Rust CLI implementation.
 *
 * @deprecated Use `spikard generate php-dto` instead
 */

// Try to find the spikard binary
$candidates = [
    // Installed via Composer in vendor/bin
    __DIR__ . '/../../vendor/bin/spikard',
    // Installed globally
    'spikard',
    // Look in common installation paths
    '/usr/local/bin/spikard',
    '/usr/bin/spikard',
];

$spikard = null;
foreach ($candidates as $candidate) {
    if (is_executable($candidate) || (file_exists($candidate) && is_readable($candidate))) {
        $spikard = $candidate;
        break;
    }
}

if ($spikard === null) {
    fwrite(STDERR, "Error: spikard CLI not found. Please install it or add it to your PATH.\n");
    fwrite(STDERR, "You can build it with: cargo build -p spikard-cli --release\n");
    exit(1);
}

// Get the output directory, default to src/Generated
$outputDir = 'src/Generated';
if ($argc > 1) {
    $outputDir = $argv[1];
}

// Ensure output directory exists
if (!is_dir($outputDir) && !mkdir($outputDir, 0755, true)) {
    fwrite(STDERR, "Error: Failed to create output directory: {$outputDir}\n");
    exit(1);
}

// Call the Rust CLI
$command = escapeshellcmd($spikard) . ' generate php-dto -o ' . escapeshellarg($outputDir);
$returnCode = 0;
passthru($command, $returnCode);

exit($returnCode);
