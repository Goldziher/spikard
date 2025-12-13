//! PHP DTO generator for Request/Response types from Rust-exported metadata
//!
//! This module generates PHP Data Transfer Objects (DTOs) from structured metadata,
//! typically exported from the Rust ext-php-rs binding. It produces readonly classes
//! matching the Spikard Request and Response types.

use anyhow::Result;
use std::collections::HashMap;

/// Field metadata for DTO generation
#[derive(Debug, Clone)]
pub struct DtoField {
    pub name: String,
    pub php_doc: String,
    pub rust_type: String,
    pub optional: bool,
    pub description: String,
}

/// DTO definition (Request or Response)
#[derive(Debug, Clone)]
pub struct DtoDefinition {
    pub name: String,
    pub kind: String,
    pub fields: Vec<DtoField>,
}

/// PHP DTO generator
pub struct PhpDtoGenerator {
    metadata: Vec<DtoDefinition>,
}

impl PhpDtoGenerator {
    /// Create a new PHP DTO generator with default Request/Response metadata
    pub fn new() -> Self {
        Self {
            metadata: Self::default_metadata(),
        }
    }

    /// Create a PHP DTO generator with custom metadata
    pub fn with_metadata(metadata: Vec<DtoDefinition>) -> Self {
        Self { metadata }
    }

    /// Generate all DTOs and write to directory
    ///
    /// Returns a map of filename -> generated PHP code
    pub fn generate_all(&self) -> Result<HashMap<String, String>> {
        let mut generated = HashMap::new();

        for definition in &self.metadata {
            let kind = definition.kind.as_str();
            let code = match kind {
                "request" => self.render_request(definition)?,
                "response" => self.render_response(definition)?,
                _ => continue,
            };

            let filename = format!("{}.php", definition.name);
            generated.insert(filename, code);
        }

        Ok(generated)
    }

    /// Render Request DTO class
    fn render_request(&self, definition: &DtoDefinition) -> Result<String> {
        let headers_doc = self.php_doc_for(&definition.fields, "headers");
        let cookies_doc = self.php_doc_for(&definition.fields, "cookies");
        let query_doc = self.php_doc_for(&definition.fields, "raw_query_params");
        let path_doc = self.php_doc_for(&definition.fields, "path_params");
        let files_doc = self.php_doc_for(&definition.fields, "files");
        let raw_query_doc = self.php_doc_for(&definition.fields, "raw_query_params");
        let dependencies_doc = self.php_doc_for(&definition.fields, "dependencies");

        Ok(format!(
            r#"<?php

declare(strict_types=1);

namespace Spikard\Generated;

use Spikard\DI\ResolvedDependencies;

final class Request
{{
    public function __construct(
        public readonly string $method,
        public readonly string $path,
        public readonly mixed $body,
        {headers_doc}public readonly array $headers = [],
        {cookies_doc}public readonly array $cookies = [],
        {query_doc}public readonly array $queryParams = [],
        {path_doc}public readonly array $pathParams = [],
        {files_doc}public readonly array $files = [],
        public readonly ?string $rawBody = null,
        {raw_query_doc}public readonly ?array $rawQueryParams = null,
        {dependencies_doc}public readonly ?ResolvedDependencies $dependencies = null,
    ) {{
    }}

    /** @param array<string, mixed> $options */
    public static function fromHttp(string $method, string $path, array $options = []): self
    {{
        $headers = self::normalizeStringMap($options['headers'] ?? []);
        $cookies = self::normalizeStringMap($options['cookies'] ?? []);
        $files = self::normalizeMixedMap($options['files'] ?? []);
        $queryParams = self::parseQueryParams($path);
        $pathOnly = \explode('?', $path, 2)[0];
        $body = $options['body'] ?? null;

        if ($body === null && $files !== []) {{
            $body = $files;
        }}

        $rawBody = \is_string($body)
            ? $body
            : ((\is_scalar($body) && !\is_bool($body)) ? (string) $body : null);

        return new self(
            method: \strtoupper($method),
            path: $pathOnly,
            body: $body,
            headers: $headers,
            cookies: $cookies,
            queryParams: $queryParams,
            pathParams: self::normalizeStringMap($options['pathParams'] ?? []),
            files: $files,
            rawBody: $rawBody,
            rawQueryParams: $queryParams,
            dependencies: $options['dependencies'] ?? null,
        );
    }}

    public function query(string $name): ?string
    {{
        $values = $this->queryParams[$name] ?? $this->rawQueryParams[$name] ?? null;
        if (\is_array($values)) {{
            foreach ($values as $value) {{
                if (\is_string($value)) {{
                    return $value;
                }}
            }}
        }}

        return null;
    }}

    /** @return array<string, string> */
    private static function normalizeStringMap(mixed $input): array
    {{
        if (!\is_array($input)) {{
            return [];
        }}

        $normalized = [];
        foreach ($input as $key => $value) {{
            if (!\is_string($key) || (!\is_string($value) && !\is_numeric($value))) {{
                continue;
            }}
            $normalized[$key] = (string) $value;
        }}

        return $normalized;
    }}

    /** @return array<string, mixed> */
    private static function normalizeMixedMap(mixed $input): array
    {{
        if (!\is_array($input)) {{
            return [];
        }}

        $normalized = [];
        foreach ($input as $key => $value) {{
            if (!\is_string($key)) {{
                continue;
            }}
            $normalized[$key] = $value;
        }}

        return $normalized;
    }}

    /** @return array<string, array<int, string>> */
    private static function parseQueryParams(string $path): array
    {{
        $parsed = \parse_url($path, PHP_URL_QUERY);
        if (!\is_string($parsed) || $parsed === '') {{
            return [];
        }}

        $result = [];
        foreach (\explode('&', $parsed) as $pair) {{
            if ($pair === '') {{
                continue;
            }}

            [$rawKey, $rawValue] = \array_pad(\explode('=', $pair, 2), 2, '');
            $key = \urldecode($rawKey);
            $value = \urldecode($rawValue);

            if ($key === '') {{
                continue;
            }}

            if (!\array_key_exists($key, $result)) {{
                $result[$key] = [];
            }}

            $result[$key][] = $value;
        }}

        return $result;
    }}
}}
"#
        ))
    }

    /// Render Response DTO class
    fn render_response(&self, definition: &DtoDefinition) -> Result<String> {
        let headers_doc = self.php_doc_for(&definition.fields, "headers");
        let cookies_doc = self.php_doc_for(&definition.fields, "cookies");

        Ok(format!(
            r#"<?php

declare(strict_types=1);

namespace Spikard\Generated;

final class Response
{{
    public function __construct(
        public readonly mixed $body = null,
        public readonly int $statusCode = 200,
        {headers_doc}public readonly array $headers = [],
        {cookies_doc}public readonly array $cookies = [],
    ) {{
    }}

    /** @param array<string, string> $headers */
    public static function json(mixed $data, int $status = 200, array $headers = []): self
    {{
        $mergedHeaders = \array_merge(['Content-Type' => 'application/json'], $headers);
        return new self(body: $data, statusCode: $status, headers: $mergedHeaders);
    }}

    /** @param array<string, string> $headers */
    public static function text(string $body, int $status = 200, array $headers = []): self
    {{
        $mergedHeaders = \array_merge(['Content-Type' => 'text/plain; charset=utf-8'], $headers);
        return new self(body: $body, statusCode: $status, headers: $mergedHeaders);
    }}

    /** @param array<string, string> $cookies */
    public function withCookies(array $cookies): self
    {{
        return new self(
            body: $this->body,
            statusCode: $this->statusCode,
            headers: $this->headers,
            cookies: $cookies
        );
    }}

    public function getStatus(): int
    {{
        return $this->statusCode;
    }}

    public function getStatusCode(): int
    {{
        return $this->statusCode;
    }}

    public function getBody(): string
    {{
        if (\is_string($this->body)) {{
            return $this->body;
        }}

        return (string) \json_encode($this->body);
    }}

    /** @return array<string, string> */
    public function getHeaders(): array
    {{
        return $this->headers;
    }}

    /**
     * Convenience accessor to decode JSON body when returned as a string.
     *
     * @return array<string, mixed>|null
     */
    public function jsonBody(): ?array
    {{
        if (\is_array($this->body)) {{
            return $this->body;
        }}

        if (\is_string($this->body)) {{
            $decoded = \json_decode($this->body, true);
            if (\is_array($decoded)) {{
                return $decoded;
            }}
        }}

        return null;
    }}

    public function __call(string $name, array $args): mixed
    {{
        if ($name === 'json') {{
            return $this->jsonBody();
        }}

        throw new \BadMethodCallException('Undefined method ' . __CLASS__ . '::' . $name);
    }}
}}
"#
        ))
    }

    /// Get PHP doc annotation for a specific field
    fn php_doc_for(&self, fields: &[DtoField], name: &str) -> String {
        for field in fields {
            if field.name == name {
                let doc = field.php_doc.trim();
                if !doc.is_empty() {
                    return format!("/** @var {} */\n        ", doc);
                }
            }
        }
        String::new()
    }

    /// Create default Request and Response metadata
    fn default_metadata() -> Vec<DtoDefinition> {
        vec![
            DtoDefinition {
                name: "Request".to_string(),
                kind: "request".to_string(),
                fields: vec![
                    DtoField {
                        name: "method".to_string(),
                        php_doc: "string".to_string(),
                        rust_type: "String".to_string(),
                        optional: false,
                        description: "HTTP method in uppercase form".to_string(),
                    },
                    DtoField {
                        name: "path".to_string(),
                        php_doc: "string".to_string(),
                        rust_type: "String".to_string(),
                        optional: false,
                        description: "Route path with query stripped".to_string(),
                    },
                    DtoField {
                        name: "path_params".to_string(),
                        php_doc: "array<string, string>".to_string(),
                        rust_type: "HashMap<String, String>".to_string(),
                        optional: false,
                        description: "Resolved path parameters".to_string(),
                    },
                    DtoField {
                        name: "query_params".to_string(),
                        php_doc: "mixed".to_string(),
                        rust_type: "serde_json::Value".to_string(),
                        optional: false,
                        description: "Parsed query params preserving typed JSON".to_string(),
                    },
                    DtoField {
                        name: "raw_query_params".to_string(),
                        php_doc: "array<string, array<int, string>>".to_string(),
                        rust_type: "HashMap<String, Vec<String>>".to_string(),
                        optional: false,
                        description: "Lossless multi-map query parameters".to_string(),
                    },
                    DtoField {
                        name: "body".to_string(),
                        php_doc: "mixed".to_string(),
                        rust_type: "serde_json::Value".to_string(),
                        optional: false,
                        description: "Validated JSON body".to_string(),
                    },
                    DtoField {
                        name: "raw_body".to_string(),
                        php_doc: "string|null".to_string(),
                        rust_type: "Option<Vec<u8>>".to_string(),
                        optional: true,
                        description: "Raw request body bytes when available".to_string(),
                    },
                    DtoField {
                        name: "headers".to_string(),
                        php_doc: "array<string, string>".to_string(),
                        rust_type: "HashMap<String, String>".to_string(),
                        optional: false,
                        description: "Normalized header map (lowercase keys)".to_string(),
                    },
                    DtoField {
                        name: "cookies".to_string(),
                        php_doc: "array<string, string>".to_string(),
                        rust_type: "HashMap<String, String>".to_string(),
                        optional: false,
                        description: "Incoming cookies".to_string(),
                    },
                    DtoField {
                        name: "files".to_string(),
                        php_doc: "array<string, mixed>".to_string(),
                        rust_type: "HashMap<String, Value>".to_string(),
                        optional: false,
                        description: "Multipart form/file uploads".to_string(),
                    },
                    DtoField {
                        name: "dependencies".to_string(),
                        php_doc: "ResolvedDependencies|null".to_string(),
                        rust_type: "Option<ResolvedDependencies>".to_string(),
                        optional: true,
                        description: "Dependency injection payload".to_string(),
                    },
                ],
            },
            DtoDefinition {
                name: "Response".to_string(),
                kind: "response".to_string(),
                fields: vec![
                    DtoField {
                        name: "status".to_string(),
                        php_doc: "int".to_string(),
                        rust_type: "u16".to_string(),
                        optional: false,
                        description: "HTTP status code".to_string(),
                    },
                    DtoField {
                        name: "body".to_string(),
                        php_doc: "mixed".to_string(),
                        rust_type: "serde_json::Value".to_string(),
                        optional: true,
                        description: "Response body as structured JSON".to_string(),
                    },
                    DtoField {
                        name: "headers".to_string(),
                        php_doc: "array<string, string>".to_string(),
                        rust_type: "HashMap<String, String>".to_string(),
                        optional: false,
                        description: "Outgoing headers".to_string(),
                    },
                    DtoField {
                        name: "cookies".to_string(),
                        php_doc: "array<string, string>".to_string(),
                        rust_type: "HashMap<String, String>".to_string(),
                        optional: false,
                        description: "Outgoing cookies".to_string(),
                    },
                ],
            },
        ]
    }
}

impl Default for PhpDtoGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generates_request_dto() {
        let generator = PhpDtoGenerator::new();
        let code = generator.render_request(&generator.metadata[0]).unwrap();

        assert!(code.contains("namespace Spikard\\Generated;"));
        assert!(code.contains("final class Request"));
        assert!(code.contains("public readonly string $method"));
        assert!(code.contains("public readonly string $path"));
        assert!(code.contains("public readonly mixed $body"));
    }

    #[test]
    fn test_generates_response_dto() {
        let generator = PhpDtoGenerator::new();
        let code = generator.render_response(&generator.metadata[1]).unwrap();

        assert!(code.contains("namespace Spikard\\Generated;"));
        assert!(code.contains("final class Response"));
        assert!(code.contains("public readonly mixed $body"));
        assert!(code.contains("public readonly int $statusCode"));
    }

    #[test]
    fn test_generate_all_returns_both_dtos() {
        let generator = PhpDtoGenerator::new();
        let generated = generator.generate_all().unwrap();

        assert!(generated.contains_key("Request.php"));
        assert!(generated.contains_key("Response.php"));
        assert_eq!(generated.len(), 2);
    }
}
