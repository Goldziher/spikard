<?php

declare(strict_types=1);

namespace Spikard\Http;

/**
 * JSON-RPC 2.0 Method Metadata.
 *
 * This class encapsulates metadata for a JSON-RPC 2.0 method, used to register
 * handlers and generate documentation (OpenRPC, OpenAPI).
 *
 * @see https://www.jsonrpc.org/specification
 */
final class JsonRpcMethodInfo
{
    /**
     * @param string $methodName The JSON-RPC method name (e.g., "user.create", "math.add")
     * @param string|null $description Human-readable description of the method
     * @param array<string, mixed>|null $paramsSchema JSON Schema defining parameters
     * @param array<string, mixed>|null $resultSchema JSON Schema defining the result
     * @param bool $deprecated Whether this method is deprecated
     * @param array<int, string> $tags Tags for organizing/categorizing methods
     */
    public function __construct(
        public readonly string $methodName,
        public readonly ?string $description = null,
        public readonly ?array $paramsSchema = null,
        public readonly ?array $resultSchema = null,
        public readonly bool $deprecated = false,
        public readonly array $tags = [],
    ) {
        if (empty($methodName)) {
            throw new \InvalidArgumentException('methodName cannot be empty');
        }

        // Validate method name format: alphanumeric, dots, and underscores only
        if (!\preg_match('/^[a-zA-Z0-9_.]+$/', $methodName)) {
            throw new \InvalidArgumentException(
                \sprintf(
                    'Invalid JSON-RPC method name "%s". Must contain only alphanumeric characters, dots, and underscores.',
                    $methodName,
                )
            );
        }
    }

    /**
     * Convert to dictionary for serialization.
     *
     * @return array<string, mixed> Dictionary representation for JSON serialization
     */
    public function toArray(): array
    {
        $result = [
            'method_name' => $this->methodName,
        ];

        if ($this->description !== null) {
            $result['description'] = $this->description;
        }

        if ($this->paramsSchema !== null) {
            $result['params_schema'] = $this->paramsSchema;
        }

        if ($this->resultSchema !== null) {
            $result['result_schema'] = $this->resultSchema;
        }

        if ($this->deprecated) {
            $result['deprecated'] = true;
        }

        if (!empty($this->tags)) {
            $result['tags'] = $this->tags;
        }

        return $result;
    }

    /**
     * Create from dictionary.
     *
     * @param array<string, mixed> $data Dictionary with method metadata
     *
     * @throws \InvalidArgumentException If required fields are missing or invalid
     * @throws \TypeError If field types don't match expectations
     */
    public static function fromArray(array $data): self
    {
        $methodName = $data['method_name'] ?? null;

        if ($methodName === null) {
            throw new \InvalidArgumentException('method_name is required');
        }

        if (!\is_string($methodName)) {
            throw new \TypeError(\sprintf(
                'method_name must be string, got %s',
                \get_debug_type($methodName),
            ));
        }

        $description = $data['description'] ?? null;
        if ($description !== null && !\is_string($description)) {
            throw new \TypeError(\sprintf(
                'description must be string or null, got %s',
                \get_debug_type($description),
            ));
        }

        $paramsSchema = $data['params_schema'] ?? null;
        if ($paramsSchema !== null && !\is_array($paramsSchema)) {
            throw new \TypeError(\sprintf(
                'params_schema must be array or null, got %s',
                \get_debug_type($paramsSchema),
            ));
        }
        /** @var array<string, mixed>|null $paramsSchema */

        $resultSchema = $data['result_schema'] ?? null;
        if ($resultSchema !== null && !\is_array($resultSchema)) {
            throw new \TypeError(\sprintf(
                'result_schema must be array or null, got %s',
                \get_debug_type($resultSchema),
            ));
        }
        /** @var array<string, mixed>|null $resultSchema */

        $deprecated = $data['deprecated'] ?? false;
        if (!\is_bool($deprecated)) {
            throw new \TypeError(\sprintf(
                'deprecated must be bool, got %s',
                \get_debug_type($deprecated),
            ));
        }

        $tags = $data['tags'] ?? [];
        if (!\is_array($tags)) {
            throw new \TypeError(\sprintf(
                'tags must be array, got %s',
                \get_debug_type($tags),
            ));
        }

        // Validate that all tags are strings
        foreach ($tags as $i => $tag) {
            if (!\is_string($tag)) {
                throw new \TypeError(\sprintf(
                    'tags[%d] must be string, got %s',
                    $i,
                    \get_debug_type($tag),
                ));
            }
        }
        /** @var array<int, string> $tags */

        return new self(
            methodName: $methodName,
            description: $description,
            paramsSchema: $paramsSchema,
            resultSchema: $resultSchema,
            deprecated: $deprecated,
            tags: $tags,
        );
    }
}
