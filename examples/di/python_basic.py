"""Basic dependency injection example with type-based dependencies.

This example demonstrates:
- Registering type-based dependencies (recommended)
- String-based value dependencies (legacy, still supported)
- Auto-injection by type annotation
- Method chaining with provide()
"""

from spikard import Spikard
from spikard.di import Provide


class AppConfig:
    """Application configuration."""

    def __init__(self) -> None:
        self.app_name = "MyApp"
        self.version = "1.0.0"
        self.max_connections = 100


def main() -> None:
    app = Spikard()

    # Type-based dependency (recommended)
    app.provide(AppConfig, AppConfig())

    @app.get("/config")
    async def get_config(config: AppConfig) -> dict:
        """Handler with auto-injected type-based dependency.

        The parameter 'config' is matched by its AppConfig type annotation
        to the registered dependency.
        """
        return {
            "app": config.app_name,
            "version": config.version,
        }

    @app.get("/stats")
    async def get_stats(config: AppConfig) -> dict:
        """Handler reusing the same injected dependency."""
        return {
            "max_connections": config.max_connections,
            "current_connections": 42,
        }


if __name__ == "__main__":
    main()
