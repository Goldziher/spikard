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


class TestServerInfo:
    """Tests for ServerInfo validation."""

    def test_server_info_empty_url_raises(self) -> None:
        """Test that empty server URL raises ValueError."""
        with pytest.raises(ValueError, match="server url cannot be empty"):
            ServerInfo(url="")


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

    def test_openapi_enabled_without_title(self) -> None:
        """Test that enabled without title raises ValueError."""
        with pytest.raises(ValueError, match="title is required when OpenAPI is enabled"):
            OpenApiConfig(enabled=True, title="", version="1.0.0")

    def test_openapi_enabled_without_version(self) -> None:
        """Test that enabled without version raises ValueError."""
        with pytest.raises(ValueError, match="version is required when OpenAPI is enabled"):
            OpenApiConfig(enabled=True, title="My API", version="")


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
