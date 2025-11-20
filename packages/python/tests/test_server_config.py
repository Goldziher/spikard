"""
Comprehensive tests for ServerConfig and all configuration options.
"""

# ruff: noqa: PT011
import pytest

from spikard.config import (
    ApiKeyConfig,
    CompressionConfig,
    JwtConfig,
    OpenApiConfig,
    RateLimitConfig,
    ServerConfig,
    StaticFilesConfig,
)


class TestCompressionConfig:
    """Tests for CompressionConfig."""

    def test_default_compression_config(self) -> None:
        """Test default compression configuration."""
        config = CompressionConfig()
        assert config.gzip is True
        assert config.brotli is True
        assert config.min_size == 1024
        assert config.quality == 6

    def test_custom_compression_config(self) -> None:
        """Test custom compression configuration."""
        config = CompressionConfig(gzip=False, brotli=True, min_size=2048, quality=9)
        assert config.gzip is False
        assert config.brotli is True
        assert config.min_size == 2048
        assert config.quality == 9

    def test_compression_quality_validation(self) -> None:
        """Test compression quality validation."""
        CompressionConfig(quality=0)
        CompressionConfig(quality=6)
        CompressionConfig(quality=11)

        with pytest.raises(ValueError) as exc_info:
            CompressionConfig(quality=12)
        assert "quality" in str(exc_info.value).lower()

        with pytest.raises(ValueError) as exc_info:
            CompressionConfig(quality=-1)
        assert "quality" in str(exc_info.value).lower()

    def test_compression_min_size_validation(self) -> None:
        """Test min_size validation."""
        CompressionConfig(min_size=0)
        CompressionConfig(min_size=1024)
        CompressionConfig(min_size=1048576)

        with pytest.raises(ValueError):
            CompressionConfig(min_size=-1)


class TestRateLimitConfig:
    """Tests for RateLimitConfig."""

    def test_default_rate_limit_config(self) -> None:
        """Test default rate limit configuration."""
        config = RateLimitConfig(per_second=100, burst=200)
        assert config.per_second == 100
        assert config.burst == 200
        assert config.ip_based is True

    def test_custom_rate_limit_config(self) -> None:
        """Test custom rate limit configuration."""
        config = RateLimitConfig(per_second=10, burst=20, ip_based=False)
        assert config.per_second == 10
        assert config.burst == 20
        assert config.ip_based is False

    def test_rate_limit_validation(self) -> None:
        """Test rate limit validation."""
        RateLimitConfig(per_second=1, burst=1)
        RateLimitConfig(per_second=1000, burst=2000)

        with pytest.raises(ValueError):
            RateLimitConfig(per_second=0, burst=10)

        with pytest.raises(ValueError):
            RateLimitConfig(per_second=10, burst=0)

        with pytest.raises(ValueError):
            RateLimitConfig(per_second=-1, burst=10)


class TestJwtConfig:
    """Tests for JwtConfig."""

    def test_default_jwt_config(self) -> None:
        """Test default JWT configuration."""
        config = JwtConfig(secret="my-secret")
        assert config.secret == "my-secret"
        assert config.algorithm == "HS256"
        assert config.audience is None
        assert config.issuer is None
        assert config.leeway == 0

    def test_custom_jwt_config(self) -> None:
        """Test custom JWT configuration."""
        config = JwtConfig(
            secret="my-secret",
            algorithm="RS256",
            audience=["https://api.example.com"],
            issuer="https://auth.example.com",
            leeway=10,
        )
        assert config.secret == "my-secret"
        assert config.algorithm == "RS256"
        assert config.audience == ["https://api.example.com"]
        assert config.issuer == "https://auth.example.com"
        assert config.leeway == 10

    def test_jwt_algorithm_validation(self) -> None:
        """Test JWT algorithm validation."""
        valid_algorithms = [
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
        ]
        for alg in valid_algorithms:
            config = JwtConfig(secret="test", algorithm=alg)
            assert config.algorithm == alg

        with pytest.raises(ValueError) as exc_info:
            JwtConfig(secret="test", algorithm="INVALID")
        assert "algorithm" in str(exc_info.value).lower()

    def test_jwt_leeway_validation(self) -> None:
        """Test JWT leeway validation."""
        JwtConfig(secret="test", leeway=0)
        JwtConfig(secret="test", leeway=60)

        with pytest.raises(ValueError):
            JwtConfig(secret="test", leeway=-1)


class TestApiKeyConfig:
    """Tests for ApiKeyConfig."""

    def test_default_api_key_config(self) -> None:
        """Test default API key configuration."""
        config = ApiKeyConfig(keys=["key1", "key2"])
        assert config.keys == ["key1", "key2"]
        assert config.header_name == "X-API-Key"

    def test_custom_api_key_config(self) -> None:
        """Test custom API key configuration."""
        config = ApiKeyConfig(keys=["secret-key"], header_name="X-Custom-API-Key")
        assert config.keys == ["secret-key"]
        assert config.header_name == "X-Custom-API-Key"

    def test_api_key_validation(self) -> None:
        """Test API key validation."""
        ApiKeyConfig(keys=["key1"])
        ApiKeyConfig(keys=["key1", "key2", "key3"])

        with pytest.raises(ValueError) as exc_info:
            ApiKeyConfig(keys=[])
        assert "keys" in str(exc_info.value).lower()


class TestStaticFilesConfig:
    """Tests for StaticFilesConfig."""

    def test_default_static_files_config(self) -> None:
        """Test default static files configuration."""
        config = StaticFilesConfig(directory="./public", route_prefix="/static")
        assert config.directory == "./public"
        assert config.route_prefix == "/static"
        assert config.index_file is True
        assert config.cache_control is None

    def test_custom_static_files_config(self) -> None:
        """Test custom static files configuration."""
        config = StaticFilesConfig(
            directory="./assets",
            route_prefix="/assets",
            index_file=False,
            cache_control="public, max-age=3600",
        )
        assert config.directory == "./assets"
        assert config.route_prefix == "/assets"
        assert config.index_file is False
        assert config.cache_control == "public, max-age=3600"


class TestServerConfig:
    """Tests for complete ServerConfig."""

    def test_default_server_config(self) -> None:
        """Test default server configuration."""
        config = ServerConfig()
        assert config.host == "127.0.0.1"
        assert config.port == 8000
        assert config.workers == 1
        assert config.enable_request_id is True
        assert config.max_body_size == 10 * 1024 * 1024
        assert config.request_timeout == 30
        assert config.compression is not None
        assert config.compression.gzip is True
        assert config.rate_limit is None
        assert config.jwt_auth is None
        assert config.api_key_auth is None
        assert config.static_files == []
        assert config.graceful_shutdown is True
        assert config.shutdown_timeout == 30
        assert config.openapi is None

    def test_custom_server_config(self) -> None:
        """Test custom server configuration."""
        config = ServerConfig(
            host="0.0.0.0",
            port=8080,
            workers=4,
            enable_request_id=False,
            max_body_size=5 * 1024 * 1024,
            request_timeout=60,
            compression=CompressionConfig(quality=9),
            rate_limit=RateLimitConfig(per_second=10, burst=20),
            jwt_auth=JwtConfig(secret="my-secret"),
            api_key_auth=ApiKeyConfig(keys=["key1"]),
            static_files=[StaticFilesConfig(directory="./public", route_prefix="/static")],
            graceful_shutdown=False,
            shutdown_timeout=60,
            openapi=OpenApiConfig(
                enabled=True,
                title="My API",
                version="1.0.0",
            ),
        )
        assert config.host == "0.0.0.0"
        assert config.port == 8080
        assert config.workers == 4
        assert config.enable_request_id is False
        assert config.max_body_size == 5 * 1024 * 1024
        assert config.request_timeout == 60
        assert config.compression is not None
        assert config.compression.quality == 9
        assert config.rate_limit is not None
        assert config.rate_limit.per_second == 10
        assert config.jwt_auth is not None
        assert config.jwt_auth.secret == "my-secret"
        assert config.api_key_auth is not None
        assert config.api_key_auth.keys == ["key1"]
        assert len(config.static_files) == 1
        assert config.graceful_shutdown is False
        assert config.shutdown_timeout == 60
        assert config.openapi is not None
        assert config.openapi.enabled is True
        assert config.openapi.title == "My API"
        assert config.openapi.version == "1.0.0"
        assert config.compression is not None
        assert config.rate_limit is not None
        assert config.jwt_auth is not None
        assert config.api_key_auth is not None

    def test_server_config_port_validation(self) -> None:
        """Test port validation."""
        ServerConfig(port=1)
        ServerConfig(port=8000)
        ServerConfig(port=65535)

        with pytest.raises(ValueError):
            ServerConfig(port=0)

        with pytest.raises(ValueError):
            ServerConfig(port=65536)

        with pytest.raises(ValueError):
            ServerConfig(port=-1)

    def test_server_config_workers_validation(self) -> None:
        """Test workers validation."""
        ServerConfig(workers=1)
        ServerConfig(workers=4)
        ServerConfig(workers=16)

        with pytest.raises(ValueError):
            ServerConfig(workers=0)

        with pytest.raises(ValueError):
            ServerConfig(workers=-1)

    def test_server_config_timeout_validation(self) -> None:
        """Test timeout validation."""
        ServerConfig(request_timeout=1)
        ServerConfig(request_timeout=30)
        ServerConfig(request_timeout=None)

        with pytest.raises(ValueError):
            ServerConfig(request_timeout=0)

        with pytest.raises(ValueError):
            ServerConfig(request_timeout=-1)

    def test_server_config_body_size_validation(self) -> None:
        """Test max_body_size validation."""
        ServerConfig(max_body_size=0)
        ServerConfig(max_body_size=1024)
        ServerConfig(max_body_size=None)

        with pytest.raises(ValueError):
            ServerConfig(max_body_size=-1)

    def test_server_config_model_copy(self) -> None:
        """Test msgspec copy() for config updates."""
        config = ServerConfig(host="127.0.0.1", port=8000)

        new_config = config.copy(host="0.0.0.0", port=8080)

        assert config.host == "127.0.0.1"
        assert config.port == 8000

        assert new_config.host == "0.0.0.0"
        assert new_config.port == 8080

    def test_server_config_multiple_static_files(self) -> None:
        """Test multiple static file configurations."""
        config = ServerConfig(
            static_files=[
                StaticFilesConfig(directory="./public", route_prefix="/static"),
                StaticFilesConfig(
                    directory="./assets",
                    route_prefix="/assets",
                    cache_control="public, max-age=86400",
                ),
            ]
        )
        assert len(config.static_files) == 2
        assert config.static_files[0].directory == "./public"
        assert config.static_files[1].directory == "./assets"
        assert config.static_files[1].cache_control == "public, max-age=86400"

    def test_server_config_disable_compression(self) -> None:
        """Test disabling compression."""
        config = ServerConfig(compression=None)
        assert config.compression is None

    def test_server_config_partial_compression(self) -> None:
        """Test partial compression configuration."""
        config = ServerConfig(compression=CompressionConfig(gzip=True, brotli=False))
        assert config.compression is not None
        assert config.compression.gzip is True
        assert config.compression.brotli is False

    def test_server_config_json_serialization(self) -> None:
        """Test JSON serialization of config."""
        import msgspec

        config = ServerConfig(
            host="0.0.0.0",
            port=8080,
            compression=CompressionConfig(quality=9),
            rate_limit=RateLimitConfig(per_second=100, burst=200),
        )

        config_dict = config.to_dict()
        assert config_dict["host"] == "0.0.0.0"
        assert config_dict["port"] == 8080
        assert config_dict["compression"]["quality"] == 9
        assert config_dict["rate_limit"]["per_second"] == 100

        json_bytes = msgspec.json.encode(config)
        assert json_bytes is not None
        assert b'"host":"0.0.0.0"' in json_bytes
        assert b'"port":8080' in json_bytes
