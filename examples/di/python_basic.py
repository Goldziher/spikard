"""Basic dependency injection example with value dependencies.

This example demonstrates:
- Registering static value dependencies
- Auto-injection by parameter name
- Method chaining with provide()
"""

from spikard import Spikard


def main() -> None:
    app = Spikard()

    app.provide("app_name", "MyApp")
    app.provide("version", "1.0.0")
    app.provide("max_connections", 100)

    @app.get("/config")
    async def get_config(app_name: str, version: str) -> dict:
        """Handler with auto-injected dependencies.

        The parameters app_name and version are automatically matched
        to registered dependencies by name.
        """
        return {
            "app": app_name,
            "version": version,
        }

    @app.get("/stats")
    async def get_stats(max_connections: int) -> dict:
        """Handler with integer dependency injection."""
        return {
            "max_connections": max_connections,
            "current_connections": 42,
        }

    @app.get("/all")
    async def get_all(app_name: str, version: str, max_connections: int) -> dict:
        """Handler with multiple injected dependencies."""
        return {
            "app": app_name,
            "version": version,
            "max_connections": max_connections,
        }


if __name__ == "__main__":
    main()
