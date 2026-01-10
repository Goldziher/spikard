"""Configuration classes for Spikard server.

All configuration uses dataclasses with msgspec for high-performance serialization.
"""

from dataclasses import dataclass, field, replace
from typing import Any, cast

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
class ContactInfo:
    """Contact information for OpenAPI documentation.

    Attributes:
        name: Name of the contact person/organization
        email: Email address for contact
        url: URL for contact information

    Example:
        ```python
        from spikard.config import ContactInfo

        contact = ContactInfo(
            name="API Support",
            email="support@example.com",
            url="https://example.com/support",
        )
        ```
    """

    name: str | None = None
    email: str | None = None
    url: str | None = None


@dataclass
class LicenseInfo:
    """License information for OpenAPI documentation.

    Attributes:
        name: License name (e.g., "MIT", "Apache 2.0")
        url: URL to the full license text

    Example:
        ```python
        from spikard.config import LicenseInfo

        license_info = LicenseInfo(
            name="MIT",
            url="https://opensource.org/licenses/MIT",
        )
        ```
    """

    name: str
    url: str | None = None

    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        if not self.name:
            raise ValueError("license name cannot be empty")


@dataclass
class ServerInfo:
    """Server information for OpenAPI documentation.

    Multiple servers can be specified for different environments.

    Attributes:
        url: Server URL (e.g., "https://api.example.com")
        description: Description of the server (e.g., "Production", "Staging")

    Example:
        ```python
        from spikard.config import ServerInfo

        servers = [
            ServerInfo(url="https://api.example.com", description="Production"),
            ServerInfo(url="http://localhost:8000", description="Development"),
        ]
        ```
    """

    url: str
    description: str | None = None

    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        if not self.url:
            raise ValueError("server url cannot be empty")


@dataclass
class SecuritySchemeInfo:
    """Security scheme configuration for OpenAPI documentation.

    Supports HTTP (Bearer/JWT) and API Key authentication schemes.
    Security schemes are auto-detected from middleware config if not explicitly provided.

    For HTTP Bearer (JWT):
        ```python
        SecuritySchemeInfo(type="http", scheme="bearer", bearer_format="JWT")
        ```

    For API Key:
        ```python
        SecuritySchemeInfo(
            type="apiKey",
            location="header",  # or "query", "cookie"
            name="X-API-Key",
        )
        ```

    Attributes:
        type: Scheme type - "http" or "apiKey"
        scheme: For HTTP type - "bearer", "basic", etc.
        bearer_format: For HTTP Bearer - format hint like "JWT"
        location: For API key - "header", "query", or "cookie"
        name: For API key - parameter name (e.g., "X-API-Key")

    Example:
        ```python
        from spikard.config import SecuritySchemeInfo

        # JWT/Bearer authentication
        jwt_scheme = SecuritySchemeInfo(
            type="http",
            scheme="bearer",
            bearer_format="JWT",
        )

        # API Key authentication
        api_key_scheme = SecuritySchemeInfo(
            type="apiKey",
            location="header",
            name="X-API-Key",
        )
        ```
    """

    type: str
    scheme: str | None = None
    bearer_format: str | None = None
    location: str | None = None
    name: str | None = None

    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        if self.type not in ("http", "apiKey"):
            raise ValueError("type must be 'http' or 'apiKey'")

        if self.type == "http" and not self.scheme:
            raise ValueError("scheme is required for HTTP security")

        if self.type == "apiKey":
            if not self.location or not self.name:
                raise ValueError("location and name are required for API key security")
            if self.location not in ("header", "query", "cookie"):
                raise ValueError("location must be 'header', 'query', or 'cookie'")


@dataclass
class OpenApiConfig:
    """OpenAPI 3.1.0 documentation configuration.

    Spikard can automatically generate OpenAPI documentation from your routes.
    When enabled, it serves:
    - Swagger UI at /docs (customizable)
    - Redoc at /redoc (customizable)
    - OpenAPI JSON spec at /openapi.json (customizable)

    Security schemes are auto-detected from middleware configuration.
    Schemas are generated from your route type hints and validation.

    Attributes:
        enabled: Enable OpenAPI generation (default: False for zero overhead)
        title: API title (required if enabled)
        version: API version (required if enabled)
        description: API description (supports Markdown)
        swagger_ui_path: Path to serve Swagger UI (default: "/docs")
        redoc_path: Path to serve Redoc (default: "/redoc")
        openapi_json_path: Path to serve OpenAPI JSON spec (default: "/openapi.json")
        contact: Contact information for the API
        license: License information for the API
        servers: List of server URLs for different environments
        security_schemes: Custom security schemes (auto-detected if not provided)

    Example:
        ```python
        from spikard import Spikard
        from spikard.config import (
            ContactInfo,
            LicenseInfo,
            OpenApiConfig,
            ServerConfig,
            ServerInfo,
        )

        openapi = OpenApiConfig(
            enabled=True,
            title="My API",
            version="1.0.0",
            description="A great API built with Spikard",
            contact=ContactInfo(
                name="API Team",
                email="api@example.com",
                url="https://example.com",
            ),
            license=LicenseInfo(
                name="MIT",
                url="https://opensource.org/licenses/MIT",
            ),
            servers=[
                ServerInfo(url="https://api.example.com", description="Production"),
                ServerInfo(url="http://localhost:8000", description="Development"),
            ],
        )

        config = ServerConfig(openapi=openapi)
        app = Spikard(config=config)

        # Swagger UI available at http://localhost:8000/docs
        # Redoc available at http://localhost:8000/redoc
        # OpenAPI spec at http://localhost:8000/openapi.json
        ```
    """

    enabled: bool = False
    title: str = "API"
    version: str = "1.0.0"
    description: str | None = None
    swagger_ui_path: str = "/docs"
    redoc_path: str = "/redoc"
    openapi_json_path: str = "/openapi.json"
    contact: ContactInfo | None = None
    license: LicenseInfo | None = None
    servers: list[ServerInfo] = field(default_factory=list)
    security_schemes: dict[str, SecuritySchemeInfo] = field(default_factory=dict)

    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        if self.enabled:
            if not self.title:
                raise ValueError("title is required when OpenAPI is enabled")
            if not self.version:
                raise ValueError("version is required when OpenAPI is enabled")


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
        openapi: OpenAPI configuration (default: None/disabled)

    Example:
        ```python
        from spikard import Spikard
        from spikard.config import (
            CompressionConfig,
            OpenApiConfig,
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
            openapi=OpenApiConfig(
                enabled=True,
                title="My API",
                version="1.0.0",
            ),
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

    host: str = "127.0.0.1"
    port: int = 8000
    workers: int = 1

    enable_request_id: bool = True
    max_body_size: int | None = 10 * 1024 * 1024
    request_timeout: int | None = 30

    compression: CompressionConfig | None = field(default_factory=CompressionConfig)
    rate_limit: RateLimitConfig | None = None
    jwt_auth: JwtConfig | None = None
    api_key_auth: ApiKeyConfig | None = None
    static_files: list[StaticFilesConfig] = field(default_factory=list)

    graceful_shutdown: bool = True
    shutdown_timeout: int = 30

    openapi: OpenApiConfig | None = None

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
        return cast("dict[str, Any]", msgspec.to_builtins(self))

    def copy(self, **updates: Any) -> ServerConfig:
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
        return replace(self, **updates)
