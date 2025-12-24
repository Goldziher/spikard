<?php

declare(strict_types=1);

namespace Spikard\Tests;

use Spikard\DI\ResolvedDependencies;
use Spikard\Http\Request;

/**
 * Create a Request while ensuring the body is stored in a variable
 * so extensions that require pass-by-reference can accept it.
 *
 * @param array<string, string> $headers
 * @param array<string, string> $cookies
 * @param array<string, array<int, string>> $queryParams
 * @param array<string, string> $pathParams
 * @param array<string, mixed>|null $validatedParams
 * @param array<string, mixed> $files
 */
function make_request(
    string $method,
    string $path,
    mixed $body,
    array $headers = [],
    array $cookies = [],
    array $queryParams = [],
    array $pathParams = [],
    ?array $validatedParams = null,
    array $files = [],
    ?ResolvedDependencies $dependencies = null,
): Request {
    $bodyRef = $body;
    return new Request(
        $method,
        $path,
        $bodyRef,
        $headers,
        $cookies,
        $queryParams,
        $pathParams,
        $validatedParams,
        $files,
        $dependencies,
    );
}

/**
 * Execute PHP code in a subprocess without loading the extension.
 *
 * @return array{int, string} [exitCode, output]
 */
function run_without_extension(string $code): array
{
    $autoloadPath = \realpath(__DIR__ . '/../vendor/autoload.php');
    if ($autoloadPath === false) {
        return [1, 'Failed to resolve autoload.php path'];
    }

    $command = \sprintf(
        '%s -n -d detect_unicode=0 -r %s',
        \escapeshellarg(PHP_BINARY),
        \escapeshellarg("require '{$autoloadPath}';" . $code)
    );

    $output = [];
    $exitCode = 0;
    \exec($command . ' 2>&1', $output, $exitCode);

    return [$exitCode, \implode("\n", $output)];
}
