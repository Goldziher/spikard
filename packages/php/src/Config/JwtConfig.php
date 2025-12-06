<?php

declare(strict_types=1);

namespace Spikard\Config;

/**
 * JWT authentication configuration.
 *
 * Configures JSON Web Token authentication middleware for the server.
 */
final class JwtConfig
{
    /**
     * @param string $secret Secret key for JWT verification
     * @param string $algorithm Required algorithm (HS256, HS384, HS512, RS256, etc.)
     * @param list<string>|null $audience Required audience claim
     * @param string|null $issuer Required issuer claim
     * @param int $leeway Leeway for expiration checks (seconds)
     */
    public function __construct(
        public readonly string $secret,
        public readonly string $algorithm = 'HS256',
        public readonly ?array $audience = null,
        public readonly ?string $issuer = null,
        public readonly int $leeway = 0,
    ) {
    }

    public static function builder(): JwtConfigBuilder
    {
        return new JwtConfigBuilder();
    }
}

/**
 * Builder for JwtConfig.
 *
 * Provides a fluent interface for constructing JwtConfig instances.
 */
final class JwtConfigBuilder
{
    private string $secret = '';
    private string $algorithm = 'HS256';
    /** @var list<string>|null */
    private ?array $audience = null;
    private ?string $issuer = null;
    private int $leeway = 0;

    public function withSecret(string $secret): self
    {
        $this->secret = $secret;
        return $this;
    }

    public function withAlgorithm(string $algorithm): self
    {
        $this->algorithm = $algorithm;
        return $this;
    }

    /**
     * @param list<string>|null $audience
     */
    public function withAudience(?array $audience): self
    {
        $this->audience = $audience;
        return $this;
    }

    public function withIssuer(?string $issuer): self
    {
        $this->issuer = $issuer;
        return $this;
    }

    public function withLeeway(int $leeway): self
    {
        $this->leeway = $leeway;
        return $this;
    }

    public function build(): JwtConfig
    {
        return new JwtConfig(
            secret: $this->secret,
            algorithm: $this->algorithm,
            audience: $this->audience,
            issuer: $this->issuer,
            leeway: $this->leeway,
        );
    }
}
