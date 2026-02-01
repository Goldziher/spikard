"""Database example with cleanup pattern using async generators.

This example demonstrates:
- Type-based dependency injection (recommended pattern)
- Factory dependencies that depend on other dependencies
- Async generator pattern for cleanup (close DB connection)
- Singleton vs per-request caching
"""

import logging
from typing import TYPE_CHECKING

from spikard import Spikard
from spikard.di import Provide

if TYPE_CHECKING:
    from collections.abc import AsyncGenerator

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class AppConfig:
    """Application configuration."""

    def __init__(self, db_url: str, max_connections: int = 10) -> None:
        self.db_url = db_url
        self.max_connections = max_connections


class DatabasePool:
    """Simulated database connection pool."""

    def __init__(self, config: AppConfig) -> None:
        self.db_url = config.db_url
        self.max_connections = config.max_connections
        self.connection_count = 0
        logger.info("[DB Pool] Created pool for %s (max: %s)", self.db_url, self.max_connections)

    async def create_session(self) -> DatabaseSession:
        self.connection_count += 1
        logger.info("[DB Pool] Creating session #%s", self.connection_count)
        return DatabaseSession(self, self.connection_count)


class DatabaseSession:
    """Simulated database session."""

    def __init__(self, pool: DatabasePool, session_id: int) -> None:
        self.pool = pool
        self.session_id = session_id
        self.closed = False

    async def query(self, sql: str) -> list[dict]:
        if self.closed:
            raise RuntimeError("Session is closed")
        logger.info("[DB Session %s] Executing: %s", self.session_id, sql)
        return [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]

    async def close(self) -> None:
        if not self.closed:
            self.closed = True
            logger.info("[DB Session %s] Session closed", self.session_id)


async def create_db_pool(config: AppConfig) -> DatabasePool:
    """Factory for database pool (singleton)."""
    logger.info("[Factory] Creating database pool...")
    return DatabasePool(config)


async def create_db_session(db_pool: DatabasePool) -> AsyncGenerator[DatabaseSession]:
    """Factory for database session with cleanup (per-request)."""
    logger.info("[Factory] Creating database session...")
    session = await db_pool.create_session()
    yield session
    logger.info("[Factory] Cleaning up database session...")
    await session.close()


def main() -> None:
    """Run the database example application."""
    app = Spikard()

    # Type-based dependencies (recommended)
    app.provide(AppConfig, AppConfig(db_url="postgresql://localhost/myapp", max_connections=10))

    app.provide(
        DatabasePool,
        Provide(
            create_db_pool,
            depends_on=[AppConfig],
            singleton=True,
        ),
    )

    app.provide(
        DatabaseSession,
        Provide(
            create_db_session,
            depends_on=[DatabasePool],
            use_cache=True,
        ),
    )

    @app.get("/users")
    async def get_users(db_session: DatabaseSession) -> dict:
        """Get all users. db_session is injected and cleaned up automatically."""
        users = await db_session.query("SELECT * FROM users")
        return {"users": users}

    @app.get("/stats")
    async def get_stats(db_pool: DatabasePool) -> dict:
        """Get pool statistics. db_pool is the singleton instance."""
        return {
            "db_url": db_pool.db_url,
            "max_connections": db_pool.max_connections,
            "total_sessions_created": db_pool.connection_count,
        }

    @app.get("/complex")
    async def complex_query(db_session: DatabaseSession, db_pool: DatabasePool) -> dict:
        """Handler using multiple type-based dependencies."""
        users = await db_session.query("SELECT * FROM users WHERE active = true")
        return {
            "users": users,
            "pool_info": {
                "db_url": db_pool.db_url,
                "max_connections": db_pool.max_connections,
            },
        }


if __name__ == "__main__":
    main()
