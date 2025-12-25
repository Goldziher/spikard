<?php

declare(strict_types=1);

namespace Spikard\Attributes;

use Attribute;
use Spikard\Http\JsonRpcMethodInfo;

/**
 * JSON-RPC metadata attribute for HTTP controller methods.
 *
 * Attach this to a controller method that is already annotated with a Route
 * attribute. The metadata is forwarded to the Rust core for OpenRPC/OpenAPI.
 *
 * Example:
 * ```php
 * use Spikard\Attributes\{Post, JsonRpcMethod};
 *
 * class RpcController {
 *     #[Post('/rpc')]
 *     #[JsonRpcMethod(
 *         methodName: 'math.add',
 *         description: 'Add two numbers',
 *         paramsSchema: [
 *             'type' => 'object',
 *             'properties' => [
 *                 'a' => ['type' => 'number'],
 *                 'b' => ['type' => 'number'],
 *             ],
 *             'required' => ['a', 'b'],
 *         ],
 *         resultSchema: ['type' => 'number'],
 *         tags: ['math'],
 *     )]
 *     public function add(array $params = new Body()): array {
 *         return ['result' => $params['a'] + $params['b']];
 *     }
 * }
 * ```
 */
#[Attribute(Attribute::TARGET_METHOD)]
final class JsonRpcMethod
{
    /**
     * @param string $methodName The JSON-RPC method name (e.g., "user.create")
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
    }

    public function toMethodInfo(): JsonRpcMethodInfo
    {
        return new JsonRpcMethodInfo(
            methodName: $this->methodName,
            description: $this->description,
            paramsSchema: $this->paramsSchema,
            resultSchema: $this->resultSchema,
            deprecated: $this->deprecated,
            tags: $this->tags,
        );
    }
}
