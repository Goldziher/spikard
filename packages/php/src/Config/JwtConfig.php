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
}
