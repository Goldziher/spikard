"""Minimal Spikard server for E2E testing of published package.

Tests the PUBLISHED spikard package from PyPI (0.6.0).
"""

from spikard import Path, Query, Spikard, get, post
from spikard.config import ServerConfig

app = Spikard()


@get("/health")
def health() -> dict[str, str]:
    """Health check endpoint."""
    return {"status": "ok"}


@get("/query")
def query_params(
    name: str = Query(),
    age: int = Query(),
) -> dict[str, str | int]:
    """Query parameter endpoint - tests required query params."""
    return {"name": name, "age": age}


@post("/echo")
async def echo(body: dict) -> dict:
    """Echo endpoint - tests JSON body parsing."""
    return {"received": body, "method": "POST"}


@get("/users/{user_id}")
def user(user_id: str = Path("id")) -> dict[str, str]:
    """Path parameter endpoint - tests path extraction."""
    return {"userId": user_id, "type": "string"}


def create_app() -> Spikard:
    """Return the configured app instance."""
    return app


if __name__ == "__main__":
    import sys

    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    config = ServerConfig(
        host="127.0.0.1",
        port=port,
        workers=1,
    )
    app.run(config=config)
