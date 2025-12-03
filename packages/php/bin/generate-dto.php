#!/usr/bin/env php
<?php

declare(strict_types=1);

/**
 * Generate PHP DTOs for Request/Response from Rust-exported metadata.
 */
final class PhpDtoGenerator
{
    /** @var array<int, array{name: string, kind: string, fields: array<int, array{name: string, php_doc: string, rust_type: string, optional: bool, description: string}>}> */
    private array $metadata;

    public function __construct()
    {
        $this->metadata = $this->loadMetadata();
    }

    public function run(): void
    {
        $targetDir = dirname(__DIR__) . '/src/Generated';
        if (!is_dir($targetDir) && !mkdir($targetDir, 0777, true) && !is_dir($targetDir)) {
            throw new RuntimeException('Failed to create generated directory: ' . $targetDir);
        }

        foreach ($this->metadata as $definition) {
            $kind = $definition['kind'] ?? '';
            $code = match ($kind) {
                'request' => $this->renderRequest($definition),
                'response' => $this->renderResponse($definition),
                default => null,
            };

            if ($code === null) {
                continue;
            }

            $path = $targetDir . '/' . $definition['name'] . '.php';
            file_put_contents($path, $code);
        }
    }

    /** @return array<int, array{name: string, kind: string, fields: array<int, array{name: string, php_doc: string, rust_type: string, optional: bool, description: string}>}> */
    private function loadMetadata(): array
    {
        if (function_exists('spikard_dto_definitions')) {
            $payload = spikard_dto_definitions();
            if (!is_array($payload)) {
                throw new RuntimeException('spikard_dto_definitions() must return an array');
            }
            return $payload;
        }

        return [
            [
                'name' => 'Request',
                'kind' => 'request',
                'fields' => [
                    ['name' => 'method', 'php_doc' => 'string', 'rust_type' => 'String', 'optional' => false, 'description' => 'HTTP method in uppercase form'],
                    ['name' => 'path', 'php_doc' => 'string', 'rust_type' => 'String', 'optional' => false, 'description' => 'Route path with query stripped'],
                    ['name' => 'path_params', 'php_doc' => 'array<string, string>', 'rust_type' => 'HashMap<String, String>', 'optional' => false, 'description' => 'Resolved path parameters'],
                    ['name' => 'query_params', 'php_doc' => 'mixed', 'rust_type' => 'serde_json::Value', 'optional' => false, 'description' => 'Parsed query params preserving typed JSON'],
                    ['name' => 'raw_query_params', 'php_doc' => 'array<string, array<int, string>>', 'rust_type' => 'HashMap<String, Vec<String>>', 'optional' => false, 'description' => 'Lossless multi-map query parameters'],
                    ['name' => 'body', 'php_doc' => 'mixed', 'rust_type' => 'serde_json::Value', 'optional' => false, 'description' => 'Validated JSON body'],
                    ['name' => 'raw_body', 'php_doc' => 'string|null', 'rust_type' => 'Option<Vec<u8>>', 'optional' => true, 'description' => 'Raw request body bytes when available'],
                    ['name' => 'headers', 'php_doc' => 'array<string, string>', 'rust_type' => 'HashMap<String, String>', 'optional' => false, 'description' => 'Normalized header map (lowercase keys)'],
                    ['name' => 'cookies', 'php_doc' => 'array<string, string>', 'rust_type' => 'HashMap<String, String>', 'optional' => false, 'description' => 'Incoming cookies'],
                    ['name' => 'files', 'php_doc' => 'array<string, mixed>', 'rust_type' => 'HashMap<String, Value>', 'optional' => false, 'description' => 'Multipart form/file uploads'],
                    ['name' => 'dependencies', 'php_doc' => 'ResolvedDependencies|null', 'rust_type' => 'Option<ResolvedDependencies>', 'optional' => true, 'description' => 'Dependency injection payload'],
                ],
            ],
            [
                'name' => 'Response',
                'kind' => 'response',
                'fields' => [
                    ['name' => 'status', 'php_doc' => 'int', 'rust_type' => 'u16', 'optional' => false, 'description' => 'HTTP status code'],
                    ['name' => 'body', 'php_doc' => 'mixed', 'rust_type' => 'serde_json::Value', 'optional' => true, 'description' => 'Response body as structured JSON'],
                    ['name' => 'headers', 'php_doc' => 'array<string, string>', 'rust_type' => 'HashMap<String, String>', 'optional' => false, 'description' => 'Outgoing headers'],
                    ['name' => 'cookies', 'php_doc' => 'array<string, string>', 'rust_type' => 'HashMap<String, String>', 'optional' => false, 'description' => 'Outgoing cookies'],
                ],
            ],
        ];
    }

    /** @param array{name: string, kind: string, fields: array<int, array{name: string, php_doc: string, rust_type: string, optional: bool, description: string}>} $definition */
    private function renderRequest(array $definition): string
    {
        $docBlocks = [
            'headers' => $this->phpDocFor($definition['fields'], 'headers'),
            'cookies' => $this->phpDocFor($definition['fields'], 'cookies'),
            'queryParams' => $this->phpDocFor($definition['fields'], 'raw_query_params'),
            'pathParams' => $this->phpDocFor($definition['fields'], 'path_params'),
            'files' => $this->phpDocFor($definition['fields'], 'files'),
            'rawQueryParams' => $this->phpDocFor($definition['fields'], 'raw_query_params'),
            'dependencies' => $this->phpDocFor($definition['fields'], 'dependencies'),
        ];

        $headersDoc = $docBlocks['headers'];
        $cookiesDoc = $docBlocks['cookies'];
        $queryDoc = $docBlocks['queryParams'];
        $pathDoc = $docBlocks['pathParams'];
        $filesDoc = $docBlocks['files'];
        $rawQueryDoc = $docBlocks['rawQueryParams'];
        $dependenciesDoc = $docBlocks['dependencies'];

        return <<<PHP
<?php

declare(strict_types=1);

namespace Spikard\\Generated;

use Spikard\\DI\\ResolvedDependencies;

final class Request
{
    public function __construct(
        public readonly string \$method,
        public readonly string \$path,
        public readonly mixed \$body,
        {$headersDoc}
        public readonly array \$headers = [],
        {$cookiesDoc}
        public readonly array \$cookies = [],
        {$queryDoc}
        public readonly array \$queryParams = [],
        {$pathDoc}
        public readonly array \$pathParams = [],
        {$filesDoc}
        public readonly array \$files = [],
        public readonly ?string \$rawBody = null,
        {$rawQueryDoc}
        public readonly ?array \$rawQueryParams = null,
        {$dependenciesDoc}
        public readonly ?ResolvedDependencies \$dependencies = null,
    ) {
    }

    /** @param array<string, mixed> \$options */
    public static function fromHttp(string \$method, string \$path, array \$options = []): self
    {
        \$headers = self::normalizeStringMap(\$options['headers'] ?? []);
        \$cookies = self::normalizeStringMap(\$options['cookies'] ?? []);
        \$files = self::normalizeMixedMap(\$options['files'] ?? []);
        \$queryParams = self::parseQueryParams(\$path);
        \$pathOnly = \\explode('?', \$path, 2)[0];
        \$body = \$options['body'] ?? null;

        if (\$body === null && \$files !== []) {
            \$body = \$files;
        }

        \$rawBody = \\is_string(\$body)
            ? \$body
            : ((\\is_scalar(\$body) && !\\is_bool(\$body)) ? (string) \$body : null);

        return new self(
            method: \\strtoupper(\$method),
            path: \$pathOnly,
            body: \$body,
            headers: \$headers,
            cookies: \$cookies,
            queryParams: \$queryParams,
            pathParams: self::normalizeStringMap(\$options['pathParams'] ?? []),
            files: \$files,
            rawBody: \$rawBody,
            rawQueryParams: \$queryParams,
            dependencies: \$options['dependencies'] ?? null,
        );
    }

    public function query(string \$name): ?string
    {
        \$values = \$this->queryParams[\$name] ?? \$this->rawQueryParams[\$name] ?? null;
        if (\\is_array(\$values)) {
            foreach (\$values as \$value) {
                if (\\is_string(\$value)) {
                    return \$value;
                }
            }
        }

        return null;
    }

    /** @return array<string, string> */
    private static function normalizeStringMap(mixed \$input): array
    {
        if (!\\is_array(\$input)) {
            return [];
        }

        \$normalized = [];
        foreach (\$input as \$key => \$value) {
            if (!\\is_string(\$key) || (!\\is_string(\$value) && !\\is_numeric(\$value))) {
                continue;
            }
            \$normalized[\$key] = (string) \$value;
        }

        return \$normalized;
    }

    /** @return array<string, mixed> */
    private static function normalizeMixedMap(mixed \$input): array
    {
        if (!\\is_array(\$input)) {
            return [];
        }

        \$normalized = [];
        foreach (\$input as \$key => \$value) {
            if (!\\is_string(\$key)) {
                continue;
            }
            \$normalized[\$key] = \$value;
        }

        return \$normalized;
    }

    /** @return array<string, array<int, string>> */
    private static function parseQueryParams(string \$path): array
    {
        \$parsed = \\parse_url(\$path, PHP_URL_QUERY);
        if (!\\is_string(\$parsed) || \$parsed === '') {
            return [];
        }

        \$result = [];
        foreach (\\explode('&', \$parsed) as \$pair) {
            if (\$pair === '') {
                continue;
            }

            [\$rawKey, \$rawValue] = \\array_pad(\\explode('=', \$pair, 2), 2, '');
            \$key = \\urldecode(\$rawKey);
            \$value = \\urldecode(\$rawValue);

            if (\$key === '') {
                continue;
            }

            if (!\\array_key_exists(\$key, \$result)) {
                \$result[\$key] = [];
            }

            \$result[\$key][] = \$value;
        }

        return \$result;
    }
}

PHP;
    }

    /** @param array{name: string, kind: string, fields: array<int, array{name: string, php_doc: string, rust_type: string, optional: bool, description: string}>} $definition */
    private function renderResponse(array $definition): string
    {
        $headersDoc = $this->phpDocFor($definition['fields'], 'headers');
        $cookiesDoc = $this->phpDocFor($definition['fields'], 'cookies');

        return <<<PHP
<?php

declare(strict_types=1);

namespace Spikard\\Generated;

final class Response
{
    public function __construct(
        public readonly mixed \$body = null,
        public readonly int \$statusCode = 200,
        {$headersDoc}
        public readonly array \$headers = [],
        {$cookiesDoc}
        public readonly array \$cookies = [],
    ) {
    }

    /** @param array<string, string> \$headers */
    public static function json(mixed \$data, int \$status = 200, array \$headers = []): self
    {
        \$mergedHeaders = \\array_merge(['Content-Type' => 'application/json'], \$headers);
        return new self(body: \$data, statusCode: \$status, headers: \$mergedHeaders);
    }

    /** @param array<string, string> \$headers */
    public static function text(string \$body, int \$status = 200, array \$headers = []): self
    {
        \$mergedHeaders = \\array_merge(['Content-Type' => 'text/plain; charset=utf-8'], \$headers);
        return new self(body: \$body, statusCode: \$status, headers: \$mergedHeaders);
    }

    /** @param array<string, string> \$cookies */
    public function withCookies(array \$cookies): self
    {
        return new self(
            body: \$this->body,
            statusCode: \$this->statusCode,
            headers: \$this->headers,
            cookies: \$cookies
        );
    }

    public function getStatus(): int
    {
        return \$this->statusCode;
    }

    public function getStatusCode(): int
    {
        return \$this->statusCode;
    }

    public function getBody(): string
    {
        if (\\is_string(\$this->body)) {
            return \$this->body;
        }

        return (string) \\json_encode(\$this->body);
    }

    /** @return array<string, string> */
    public function getHeaders(): array
    {
        return \$this->headers;
    }

    /**
     * Convenience accessor to decode JSON body when returned as a string.
     *
     * @return array<string, mixed>|null
     */
    public function jsonBody(): ?array
    {
        if (\\is_array(\$this->body)) {
            return \$this->body;
        }

        if (\\is_string(\$this->body)) {
            \$decoded = \\json_decode(\$this->body, true);
            if (\\is_array(\$decoded)) {
                return \$decoded;
            }
        }

        return null;
    }

    public function __call(string \$name, array \$args): mixed
    {
        if (\$name === 'json') {
            return \$this->jsonBody();
        }

        throw new \\BadMethodCallException('Undefined method ' . __CLASS__ . '::' . \$name);
    }
}

PHP;
    }

    /** @param array<int, array{name: string, php_doc: string, rust_type: string, optional: bool, description: string}> $fields */
    private function phpDocFor(array $fields, string $name): string
    {
        foreach ($fields as $field) {
            if (($field['name'] ?? '') === $name && isset($field['php_doc'])) {
                $doc = trim((string) $field['php_doc']);
                if ($doc === '') {
                    return '';
                }
                return "/** @var {$doc} */\n        ";
            }
        }

        return '';
    }
}

(new PhpDtoGenerator())->run();
