"""Minimal Spikard server for E2E testing of published package.

Tests the PUBLISHED spikard package from PyPI (0.10.1).
"""

from spikard import Cookie, Header, Path, Query, Spikard
from spikard.config import ServerConfig

app = Spikard()


@app.get("/health")
def health() -> dict[str, str]:
    """Health check endpoint."""
    return {"status": "ok"}


@app.get("/query")
def query_params(
    name: str = Query(),
    age: int = Query(),
) -> dict[str, str | int]:
    """Query parameter endpoint - tests required query params."""
    return {"name": name, "age": age}


@app.post("/echo")
async def echo(body: dict) -> dict:
    """Echo endpoint - tests JSON body parsing."""
    return {"received": body, "method": "POST"}


@app.get("/users/{user_id}")
def user(user_id: str = Path()) -> dict[str, str]:
    """Path parameter endpoint - tests path extraction."""
    return {"userId": user_id, "type": "string"}


@app.put("/items/{item_id}")
def update_item(item_id: str = Path(), body: dict | None = None) -> dict:
    """PUT endpoint - tests PUT method and path parameters."""
    return {"itemId": item_id, "updated": body, "method": "PUT"}


@app.delete("/items/{item_id}")
def delete_item(item_id: str = Path()) -> dict:
    """DELETE endpoint - tests DELETE method and path parameters."""
    return {"itemId": item_id, "deleted": True, "method": "DELETE"}


@app.patch("/items/{item_id}")
def patch_item(item_id: str = Path(), body: dict | None = None) -> dict:
    """PATCH endpoint - tests PATCH method and path parameters."""
    return {"itemId": item_id, "patched": body, "method": "PATCH"}


@app.get("/headers")
def extract_headers(x_custom_header: str = Header(alias="X-Custom-Header")) -> dict[str, str]:
    """Header extraction endpoint - tests custom header extraction."""
    return {"x-custom-header": x_custom_header}


@app.get("/cookies")
def extract_cookies(session: str = Cookie()) -> dict[str, str]:
    """Cookie extraction endpoint - tests session cookie extraction."""
    return {"session": session}


@app.get("/error")
def trigger_error() -> dict:
    """Error endpoint - tests 500 error handling."""
    raise RuntimeError("Intentional error")


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
