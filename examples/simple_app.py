"""Simple example Spikard application."""

from spikard import Spikard

app = Spikard()


@app.get("/")
async def index():
    """Root endpoint."""
    return {"message": "Hello from Spikard!"}


@app.get("/health")
async def health():
    """Health check endpoint."""
    return {"status": "ok"}


@app.get("/users/{user_id}")
async def get_user(user_id: int):
    """Get user by ID."""
    return {"user_id": user_id, "name": f"User {user_id}"}


if __name__ == "__main__":
    app.run(host="127.0.0.1", port=8000)
