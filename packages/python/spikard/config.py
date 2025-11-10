"""Configuration classes for Spikard server.

All configuration uses dataclasses with msgspec for high-performance serialization.
"""

from dataclasses import dataclass, field, replace
from typing import Any

import msgspec


@dataclass
class CompressionConfig:
    """Configuration for response compression middleware.

    Spikard supports gzip and brotli compression for responses.
    Compression is applied based on Accept-Encoding headers.

    Attributes:
        gzip: Enable gzip compression (default: True)
        brotli: Enable brotli compression (default: True)
        min_size: Minimum response size in bytes to compress (default: 1024)
        quality: Compression quality level (0-11 for brotli, 0-9 for gzip, default: 6)

    Example:
        ```python
        from spikard import Spikard
        from spikard.config import CompressionConfig, ServerConfig

        config = ServerConfig(
            compression=CompressionConfig(
                gzip=True,
                brotli=True,
                min_size=2048,  # Only compress responses >= 2KB
                quality=9,  # Maximum compression
            )
        )

        app = Spikard(config=config)
        ```
    """

    gzip: bool = True
    brotli: bool = True
    min_size: int = 1024
    quality: int = 6

    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        if self.min_size < 0:
            raise ValueError("min_size must be >= 0")
        if self.quality < 0 or self.quality > 11:
            raise ValueError("Compression quality must be between 0 and 11")


@dataclass
class RateLimitConfig:
    """Configuration for rate limiting middleware.

    Uses the Generic Cell Rate Algorithm (GCRA) for smooth rate limiting.
    By default, rate limits are applied per IP address.

    Attributes:
        per_second: Maximum requests per second
        burst: Burst allowance - allows temporary spikes
        ip_based: Apply rate limits per IP address (default: True)

    Example:
        ```python
        from spikard import Spikard
        from spikard.config import RateLimitConfig, ServerConfig

        config = ServerConfig(
            rate_limit=RateLimitConfig(
                per_second=10,  # 10 requests per second
                burst=20,  # Allow bursts up to 20
                ip_based=True,  # Per IP address
            )
        )

        app = Spikard(config=config)
        ```
    """

    per_second: int
    burst: int
    ip_based: bool = True

    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        if self.per_second <= 0:
            raise ValueError("per_second must be > 0")
        if self.burst <= 0:
            raise ValueError("burst must be > 0")


@dataclass
class JwtConfig:
    """Configuration for JWT authentication middleware.

    Validates JWT tokens using the specified secret and algorithm.
    Tokens are expected in the Authorization header as "Bearer <token>".

    Attributes:
        secret: Secret key for JWT validation
        algorithm: JWT algorithm (default: "HS256")
        audience: Expected audience claim(s) - can be a string or list of strings
        issuer: Expected issuer claim
        leeway: Time leeway in seconds for exp/nbf/iat claims (default: 0)

    Supported algorithms:
        - HS256, HS384, HS512 (HMAC with SHA)
        - RS256, RS384, RS512 (RSA signatures)
        - ES256, ES384, ES512 (ECDSA signatures)
        - PS256, PS384, PS512 (RSA-PSS signatures)

    Example:
        ```python
        from spikard import Spikard
        from spikard.config import JwtConfig, ServerConfig

        config = ServerConfig(
            jwt_auth=JwtConfig(
                secret="your-secret-key",
                algorithm="HS256",
                audience=["https://api.example.com"],
                issuer="https://auth.example.com",
                leeway=10,
            )
        )

        app = Spikard(config=config)
        ```
    """

    secret: str
    algorithm: str = "HS256"
    audience: list[str] | None = None
    issuer: str | None = None
    leeway: int = 0

    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        valid_algorithms = {
            "HS256",
            "HS384",
            "HS512",
            "RS256",
            "RS384",
            "RS512",
            "ES256",
            "ES384",
            "ES512",
            "PS256",
            "PS384",
            "PS512",
        }
        if self.algorithm not in valid_algorithms:
            raise ValueError(
                f"Invalid JWT algorithm '{self.algorithm}'. Must be one of: {', '.join(sorted(valid_algorithms))}"
            )
        if self.leeway < 0:
            raise ValueError("leeway must be >= 0")


@dataclass
class ApiKeyConfig:
    """Configuration for API key authentication middleware.

    Validates API keys from request headers. Keys are matched exactly.

    Attributes:
        keys: List of valid API keys
        header_name: HTTP header name to check for API key (default: "X-API-Key")

    Example:
        ```python
        from spikard import Spikard
        from spikard.config import ApiKeyConfig, ServerConfig

        config = ServerConfig(api_key_auth=ApiKeyConfig(keys=["secret-key-1", "secret-key-2"], header_name="X-API-Key"))

        app = Spikard(config=config)
        ```
    """

    keys: list[str]
    header_name: str = "X-API-Key"

    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        if not self.keys:
            raise ValueError("keys list cannot be empty")


@dataclass
class StaticFilesConfig:
    """Configuration for serving static files.

    Serves files from a directory at a given route prefix.
    Multiple static file configurations can be registered.

    Attributes:
        directory: Directory path containing static files
        route_prefix: URL prefix for serving static files (e.g., "/static")
        index_file: Serve index.html for directory requests (default: True)
        cache_control: Optional Cache-Control header value (e.g., "public, max-age=3600")

    Example:
        ```python
        from spikard import Spikard
        from spikard.config import StaticFilesConfig, ServerConfig

        config = ServerConfig(
            static_files=[
                StaticFilesConfig(
                    directory="./public", route_prefix="/static", index_file=True, cache_control="public, max-age=3600"
                ),
                StaticFilesConfig(directory="./assets", route_prefix="/assets", cache_control="public, max-age=86400"),
            ]
        )

        app = Spikard(config=config)
        ```
    """

    directory: str
    route_prefix: str
    index_file: bool = True
    cache_control: str | None = None


@dataclass
class ServerConfig:
    """Complete server configuration for Spikard.

    This is the main configuration object that controls all aspects of the server
    including network settings, middleware, authentication, and more.

    Network Configuration:
        host: Host address to bind to (default: "127.0.0.1")
        port: Port number to listen on (default: 8000, range: 1-65535)
        workers: Number of worker processes (default: 1)

    Request Handling:
        enable_request_id: Add X-Request-ID header to responses (default: True)
        max_body_size: Maximum request body size in bytes (default: 10MB, 0 or None for unlimited)
        request_timeout: Request timeout in seconds (default: 30, None for no timeout)

    Middleware:
        compression: Response compression configuration (default: enabled with defaults)
        rate_limit: Rate limiting configuration (default: None/disabled)
        jwt_auth: JWT authentication configuration (default: None/disabled)
        api_key_auth: API key authentication configuration (default: None/disabled)
        static_files: List of static file serving configurations (default: empty list)

    Lifecycle:
        graceful_shutdown: Enable graceful shutdown (default: True)
        shutdown_timeout: Graceful shutdown timeout in seconds (default: 30)

    OpenAPI/Documentation:
        enable_openapi: Enable OpenAPI documentation generation (default: False)
        openapi_title: API title for OpenAPI docs
        openapi_version: API version for OpenAPI docs

    Example:
        ```python
        from spikard import Spikard
        from spikard.config import (
            CompressionConfig,
            RateLimitConfig,
            ServerConfig,
            StaticFilesConfig,
        )

        config = ServerConfig(
            host="0.0.0.0",
            port=8080,
            workers=4,
            compression=CompressionConfig(quality=9),
            rate_limit=RateLimitConfig(per_second=100, burst=200),
            static_files=[StaticFilesConfig(directory="./public", route_prefix="/static")],
            enable_openapi=True,
            openapi_title="My API",
            openapi_version="1.0.0",
        )

        app = Spikard(config=config)
        app.run()
        ```

        You can also pass config to run():
        ```python
        app = Spikard()
        app.run(config=config)
        ```

        Or use backwards-compatible individual parameters:
        ```python
        app = Spikard()
        app.run(host="0.0.0.0", port=8080)
        ```
    """

    # Network configuration
    host: str = "127.0.0.1"
    port: int = 8000
    workers: int = 1

    # Request handling
    enable_request_id: bool = True
    max_body_size: int | None = 10 * 1024 * 1024  # 10MB default
    request_timeout: int | None = 30  # 30 seconds default

    # Middleware
    compression: CompressionConfig | None = field(default_factory=CompressionConfig)
    rate_limit: RateLimitConfig | None = None
    jwt_auth: JwtConfig | None = None
    api_key_auth: ApiKeyConfig | None = None
    static_files: list[StaticFilesConfig] = field(default_factory=list)

    # Lifecycle
    graceful_shutdown: bool = True
    shutdown_timeout: int = 30

    # OpenAPI
    enable_openapi: bool = False
    openapi_title: str | None = None
    openapi_version: str | None = None

    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        if self.port < 1 or self.port > 65535:
            raise ValueError("port must be between 1 and 65535")
        if self.workers < 1:
            raise ValueError("workers must be >= 1")
        if self.request_timeout is not None and self.request_timeout < 1:
            raise ValueError("request_timeout must be >= 1 or None")
        if self.max_body_size is not None and self.max_body_size < 0:
            raise ValueError("max_body_size must be >= 0 or None")
        if self.shutdown_timeout < 1:
            raise ValueError("shutdown_timeout must be >= 1")

    def to_dict(self) -> dict[str, Any]:
        """Convert config to dictionary (for JSON serialization).

        Uses msgspec for fast, efficient serialization.
        """
        return msgspec.to_builtins(self)  # type: ignore[no-any-return]

    def copy(self, **updates: Any) -> "ServerConfig":
        """Create a copy of the config with updates applied.

        Args:
            **updates: Fields to update in the new config

        Returns:
            New ServerConfig instance with updates applied

        Example:
            ```python
            config = ServerConfig(host="127.0.0.1", port=8000)
            new_config = config.copy(host="0.0.0.0", port=8080)
            ```
        """
        # Use dataclasses.replace() for efficient copying
        return replace(self, **updates)
