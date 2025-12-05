"""
Comprehensive tests for spikard.config validation paths.

Covers validation failures in:
- LicenseInfo (lines 246-247)
- ServerInfo (lines 276-277)
- SecuritySchemeInfo (lines 336-346)
- OpenApiConfig (lines 429-433)
- ServerConfig (line 576)
"""

import pytest

from spikard.config import (
    LicenseInfo,
    OpenApiConfig,
    SecuritySchemeInfo,
    ServerConfig,
    ServerInfo,
)


class TestLicenseInfo:
    """Tests for LicenseInfo validation."""

    def test_license_info_empty_name_raises(self) -> None:
        """Test that empty license name raises ValueError."""
        with pytest.raises(ValueError, match="license name cannot be empty"):
            LicenseInfo(name="")

    def test_license_info_valid_name(self) -> None:
        """Test that valid license name is accepted."""
        license_info = LicenseInfo(name="MIT")
        assert license_info.name == "MIT"

    def test_license_info_with_url(self) -> None:
        """Test license info with both name and URL."""
        license_info = LicenseInfo(name="Apache 2.0", url="https://apache.org/licenses/LICENSE-2.0")
        assert license_info.name == "Apache 2.0"
        assert license_info.url == "https://apache.org/licenses/LICENSE-2.0"

    def test_license_info_url_optional(self) -> None:
        """Test that URL is optional."""
        license_info = LicenseInfo(name="MIT", url=None)
        assert license_info.name == "MIT"
        assert license_info.url is None

    def test_license_info_various_names(self) -> None:
        """Test various valid license names."""
        valid_names = ["MIT", "Apache 2.0", "GPL-3.0", "BSD-3-Clause", "ISC"]
        for name in valid_names:
            license_info = LicenseInfo(name=name)
            assert license_info.name == name


class TestServerInfo:
    """Tests for ServerInfo validation."""

    def test_server_info_empty_url_raises(self) -> None:
        """Test that empty server URL raises ValueError."""
        with pytest.raises(ValueError, match="server url cannot be empty"):
            ServerInfo(url="")

    def test_server_info_valid_url(self) -> None:
        """Test that valid URL is accepted."""
        server_info = ServerInfo(url="https://api.example.com")
        assert server_info.url == "https://api.example.com"

    def test_server_info_with_description(self) -> None:
        """Test server info with description."""
        server_info = ServerInfo(url="https://api.example.com", description="Production API")
        assert server_info.url == "https://api.example.com"
        assert server_info.description == "Production API"

    def test_server_info_description_optional(self) -> None:
        """Test that description is optional."""
        server_info = ServerInfo(url="https://api.example.com", description=None)
        assert server_info.url == "https://api.example.com"
        assert server_info.description is None

    def test_server_info_localhost_url(self) -> None:
        """Test localhost URL."""
        server_info = ServerInfo(url="http://localhost:8000")
        assert server_info.url == "http://localhost:8000"

    def test_server_info_multiple_servers(self) -> None:
        """Test creating multiple server configurations."""
        servers = [
            ServerInfo(url="https://api.example.com", description="Production"),
            ServerInfo(url="http://localhost:8000", description="Development"),
            ServerInfo(url="https://staging.example.com", description="Staging"),
        ]
        assert len(servers) == 3
        assert servers[0].description == "Production"
        assert servers[1].description == "Development"
        assert servers[2].description == "Staging"


class TestSecuritySchemeInfo:
    """Tests for SecuritySchemeInfo validation."""

    def test_security_scheme_invalid_type(self) -> None:
        """Test that invalid type raises ValueError."""
        with pytest.raises(ValueError, match="type must be 'http' or 'apiKey'"):
            SecuritySchemeInfo(type="oauth")

    def test_security_scheme_http_without_scheme(self) -> None:
        """Test that HTTP type without scheme raises ValueError."""
        with pytest.raises(ValueError, match="scheme is required for HTTP security"):
            SecuritySchemeInfo(type="http", scheme=None)

    def test_security_scheme_http_with_scheme(self) -> None:
        """Test HTTP security scheme with valid scheme."""
        security = SecuritySchemeInfo(type="http", scheme="bearer")
        assert security.type == "http"
        assert security.scheme == "bearer"

    def test_security_scheme_http_bearer_with_format(self) -> None:
        """Test HTTP Bearer with bearer format."""
        security = SecuritySchemeInfo(type="http", scheme="bearer", bearer_format="JWT")
        assert security.type == "http"
        assert security.scheme == "bearer"
        assert security.bearer_format == "JWT"

    def test_security_scheme_http_basic(self) -> None:
        """Test HTTP Basic authentication scheme."""
        security = SecuritySchemeInfo(type="http", scheme="basic")
        assert security.type == "http"
        assert security.scheme == "basic"

    def test_security_scheme_apikey_without_location(self) -> None:
        """Test that apiKey without location raises ValueError."""
        with pytest.raises(ValueError, match="location and name are required for API key security"):
            SecuritySchemeInfo(type="apiKey", name="X-API-Key", location=None)

    def test_security_scheme_apikey_without_name(self) -> None:
        """Test that apiKey without name raises ValueError."""
        with pytest.raises(ValueError, match="location and name are required for API key security"):
            SecuritySchemeInfo(type="apiKey", location="header", name=None)

    def test_security_scheme_apikey_without_both(self) -> None:
        """Test that apiKey without both location and name raises ValueError."""
        with pytest.raises(ValueError, match="location and name are required for API key security"):
            SecuritySchemeInfo(type="apiKey")

    def test_security_scheme_apikey_invalid_location(self) -> None:
        """Test that invalid location raises ValueError."""
        with pytest.raises(ValueError, match="location must be 'header', 'query', or 'cookie'"):
            SecuritySchemeInfo(type="apiKey", location="body", name="X-API-Key")

    def test_security_scheme_apikey_header_valid(self) -> None:
        """Test valid API key in header."""
        security = SecuritySchemeInfo(type="apiKey", location="header", name="X-API-Key")
        assert security.type == "apiKey"
        assert security.location == "header"
        assert security.name == "X-API-Key"

    def test_security_scheme_apikey_query_valid(self) -> None:
        """Test valid API key in query parameter."""
        security = SecuritySchemeInfo(type="apiKey", location="query", name="api_key")
        assert security.type == "apiKey"
        assert security.location == "query"
        assert security.name == "api_key"

    def test_security_scheme_apikey_cookie_valid(self) -> None:
        """Test valid API key in cookie."""
        security = SecuritySchemeInfo(type="apiKey", location="cookie", name="session")
        assert security.type == "apiKey"
        assert security.location == "cookie"
        assert security.name == "session"

    def test_security_scheme_apikey_invalid_locations(self) -> None:
        """Test various invalid locations."""
        # Only test non-empty invalid locations
        # Empty string is caught by "location and name are required" check first
        invalid_locations = ["body", "path", "form", "jwt"]
        for location in invalid_locations:
            with pytest.raises(
                ValueError,
                match="location must be 'header', 'query', or 'cookie'",
            ):
                SecuritySchemeInfo(type="apiKey", location=location, name="X-API-Key")

    def test_security_scheme_invalid_types(self) -> None:
        """Test various invalid types."""
        invalid_types = ["oauth2", "oauth", "basic", "digest", "bearer", ""]
        for invalid_type in invalid_types:
            with pytest.raises(ValueError, match="type must be 'http' or 'apiKey'"):
                SecuritySchemeInfo(type=invalid_type)


class TestOpenApiConfig:
    """Tests for OpenApiConfig validation."""

    def test_openapi_disabled_default(self) -> None:
        """Test that OpenAPI is disabled by default."""
        config = OpenApiConfig()
        assert config.enabled is False

    def test_openapi_enabled_without_title(self) -> None:
        """Test that enabled without title raises ValueError."""
        with pytest.raises(ValueError, match="title is required when OpenAPI is enabled"):
            OpenApiConfig(enabled=True, title="", version="1.0.0")

    def test_openapi_enabled_without_version(self) -> None:
        """Test that enabled without version raises ValueError."""
        with pytest.raises(ValueError, match="version is required when OpenAPI is enabled"):
            OpenApiConfig(enabled=True, title="My API", version="")

    def test_openapi_disabled_empty_title_version(self) -> None:
        """Test that disabled config doesn't validate title/version."""
        config = OpenApiConfig(enabled=False, title="", version="")
        assert config.enabled is False
        assert config.title == ""
        assert config.version == ""

    def test_openapi_enabled_with_title_and_version(self) -> None:
        """Test valid enabled OpenAPI config."""
        config = OpenApiConfig(enabled=True, title="My API", version="1.0.0")
        assert config.enabled is True
        assert config.title == "My API"
        assert config.version == "1.0.0"

    def test_openapi_with_description(self) -> None:
        """Test OpenAPI config with description."""
        config = OpenApiConfig(
            enabled=True,
            title="My API",
            version="1.0.0",
            description="A comprehensive API",
        )
        assert config.description == "A comprehensive API"

    def test_openapi_default_paths(self) -> None:
        """Test default OpenAPI paths."""
        config = OpenApiConfig(enabled=True, title="My API", version="1.0.0")
        assert config.swagger_ui_path == "/docs"
        assert config.redoc_path == "/redoc"
        assert config.openapi_json_path == "/openapi.json"

    def test_openapi_custom_paths(self) -> None:
        """Test custom OpenAPI paths."""
        config = OpenApiConfig(
            enabled=True,
            title="My API",
            version="1.0.0",
            swagger_ui_path="/swagger",
            redoc_path="/api-docs",
            openapi_json_path="/spec.json",
        )
        assert config.swagger_ui_path == "/swagger"
        assert config.redoc_path == "/api-docs"
        assert config.openapi_json_path == "/spec.json"

    def test_openapi_with_servers(self) -> None:
        """Test OpenAPI config with servers."""
        servers = [
            ServerInfo(url="https://api.example.com", description="Production"),
            ServerInfo(url="http://localhost:8000", description="Development"),
        ]
        config = OpenApiConfig(enabled=True, title="My API", version="1.0.0", servers=servers)
        assert len(config.servers) == 2
        assert config.servers[0].url == "https://api.example.com"

    def test_openapi_with_security_schemes(self) -> None:
        """Test OpenAPI config with security schemes."""
        schemes = {
            "api_key": SecuritySchemeInfo(type="apiKey", location="header", name="X-API-Key"),
            "bearer": SecuritySchemeInfo(type="http", scheme="bearer"),
        }
        config = OpenApiConfig(
            enabled=True,
            title="My API",
            version="1.0.0",
            security_schemes=schemes,
        )
        assert len(config.security_schemes) == 2
        assert "api_key" in config.security_schemes
        assert "bearer" in config.security_schemes

    def test_openapi_enabled_whitespace_title(self) -> None:
        """Test that whitespace-only title is accepted (truthiness check)."""
        # Whitespace strings are truthy, so validation passes
        config = OpenApiConfig(enabled=True, title="   ", version="1.0.0")
        assert config.enabled is True
        assert config.title == "   "

    def test_openapi_disabled_with_full_config(self) -> None:
        """Test disabled config with all optional fields."""
        config = OpenApiConfig(
            enabled=False,
            title="My API",
            version="1.0.0",
            description="An API",
            servers=[ServerInfo(url="https://api.example.com")],
        )
        assert config.enabled is False


class TestServerConfigShutdownTimeout:
    """Tests for ServerConfig shutdown_timeout validation."""

    def test_server_config_shutdown_timeout_zero(self) -> None:
        """Test that shutdown_timeout of 0 raises ValueError."""
        with pytest.raises(ValueError, match="shutdown_timeout must be >= 1"):
            ServerConfig(shutdown_timeout=0)

    def test_server_config_shutdown_timeout_negative(self) -> None:
        """Test that negative shutdown_timeout raises ValueError."""
        with pytest.raises(ValueError, match="shutdown_timeout must be >= 1"):
            ServerConfig(shutdown_timeout=-1)

    def test_server_config_shutdown_timeout_negative_large(self) -> None:
        """Test that large negative shutdown_timeout raises ValueError."""
        with pytest.raises(ValueError, match="shutdown_timeout must be >= 1"):
            ServerConfig(shutdown_timeout=-100)

    def test_server_config_shutdown_timeout_valid(self) -> None:
        """Test valid shutdown_timeout values."""
        config = ServerConfig(shutdown_timeout=30)
        assert config.shutdown_timeout == 30

    def test_server_config_shutdown_timeout_minimum(self) -> None:
        """Test minimum valid shutdown_timeout value."""
        config = ServerConfig(shutdown_timeout=1)
        assert config.shutdown_timeout == 1

    def test_server_config_shutdown_timeout_large(self) -> None:
        """Test large shutdown_timeout value."""
        config = ServerConfig(shutdown_timeout=3600)
        assert config.shutdown_timeout == 3600

    def test_server_config_default_shutdown_timeout(self) -> None:
        """Test default shutdown_timeout value."""
        config = ServerConfig()
        assert config.shutdown_timeout == 30

    def test_server_config_shutdown_timeout_with_other_settings(self) -> None:
        """Test shutdown_timeout validation with other config settings."""
        config = ServerConfig(
            host="0.0.0.0",
            port=8080,
            workers=4,
            graceful_shutdown=True,
            shutdown_timeout=45,
        )
        assert config.shutdown_timeout == 45
        assert config.graceful_shutdown is True
        assert config.port == 8080


class TestCombinedValidation:
    """Tests for combined validation scenarios."""

    def test_openapi_with_license_and_servers(self) -> None:
        """Test OpenAPI with license and server info."""
        config = OpenApiConfig(
            enabled=True,
            title="My API",
            version="1.0.0",
            license=LicenseInfo(name="MIT", url="https://opensource.org/licenses/MIT"),
            servers=[
                ServerInfo(url="https://api.example.com", description="Production"),
            ],
        )
        assert config.license is not None
        assert config.license.name == "MIT"
        assert len(config.servers) == 1

    def test_security_scheme_in_openapi(self) -> None:
        """Test security scheme configuration in OpenAPI."""
        api_key_scheme = SecuritySchemeInfo(type="apiKey", location="header", name="X-API-Key")
        config = OpenApiConfig(
            enabled=True,
            title="Secure API",
            version="1.0.0",
            security_schemes={"api_key": api_key_scheme},
        )
        assert "api_key" in config.security_schemes
        assert config.security_schemes["api_key"].type == "apiKey"

    def test_server_config_with_openapi_and_license(self) -> None:
        """Test ServerConfig with complete OpenAPI setup."""
        openapi = OpenApiConfig(
            enabled=True,
            title="Complete API",
            version="2.0.0",
            description="A full-featured API",
            license=LicenseInfo(name="Apache 2.0"),
            servers=[ServerInfo(url="https://api.example.com")],
        )
        config = ServerConfig(
            host="0.0.0.0",
            port=8080,
            shutdown_timeout=60,
            openapi=openapi,
        )
        assert config.openapi is not None
        assert config.openapi.title == "Complete API"
        assert config.shutdown_timeout == 60
