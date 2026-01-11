"""Database example with cleanup pattern using async generators.

This example demonstrates:
- Factory dependencies that depend on other dependencies
- Async generator pattern for cleanup (close DB connection)
- Singleton vs per-request caching
- Proper resource management
"""

import logging
from typing import TYPE_CHECKING

from spikard import Spikard
from spikard.di import Provide

if TYPE_CHECKING:
    from collections.abc import AsyncGenerator

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class DatabasePool:
    """Simulated database connection pool."""

    def __init__(self, config: dict) -> None:
        """Initialize pool from config.

        Parameters
        ----------
        config : dict
            Configuration dictionary with db_url
        """
        self.db_url = config["db_url"]
        self.max_connections = config.get("max_connections", 10)
        self.connection_count = 0
        logger.info("[DB Pool] Created pool for %s (max: %s)", self.db_url, self.max_connections)

    async def create_session(self) -> DatabaseSession:
        """Create a new database session.

        Returns:
        -------
        DatabaseSession
            A new session from the pool
        """
        self.connection_count += 1
        logger.info("[DB Pool] Creating session #%s", self.connection_count)
        return DatabaseSession(self, self.connection_count)


class DatabaseSession:
    """Simulated database session."""

    def __init__(self, pool: DatabasePool, session_id: int) -> None:
        """Initialize session.

        Parameters
        ----------
        pool : DatabasePool
            The pool this session belongs to
        session_id : int
            Unique session identifier
        """
        self.pool = pool
        self.session_id = session_id
        self.closed = False
        logger.info("[DB Session %s] Session opened", self.session_id)

    async def query(self, sql: str) -> list[dict]:
        """Execute a query.

        Parameters
        ----------
        sql : str
            SQL query to execute

        Returns:
        -------
        list[dict]
            Query results
        """
        if self.closed:
            raise RuntimeError("Session is closed")
        logger.info("[DB Session %s] Executing: %s", self.session_id, sql)
        return [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]

    async def close(self) -> None:
        """Close the session and return to pool."""
        if not self.closed:
            self.closed = True
            logger.info("[DB Session %s] Session closed", self.session_id)


async def create_db_pool(config: dict) -> DatabasePool:
    """Factory for database pool (singleton).

    This dependency is created once and shared across all requests.

    Parameters
    ----------
    config : dict
        Application configuration

    Returns:
    -------
    DatabasePool
        The singleton database pool
    """
    logger.info("[Factory] Creating database pool...")
    return DatabasePool(config)


async def create_db_session(db_pool: DatabasePool) -> AsyncGenerator[DatabaseSession]:
    """Factory for database session with cleanup (per-request).

    This uses the async generator pattern to ensure cleanup.
    The session is yielded to the handler, then automatically closed
    after the request completes.

    Parameters
    ----------
    db_pool : DatabasePool
        The singleton database pool

    Yields:
    ------
    DatabaseSession
        A database session for this request
    """
    logger.info("[Factory] Creating database session...")
    session = await db_pool.create_session()

    yield session

    logger.info("[Factory] Cleaning up database session...")
    await session.close()


def main() -> None:
    """Run the database example application."""
    app = Spikard()

    app.provide(
        "config",
        {
            "db_url": "postgresql://localhost/myapp",
            "max_connections": 10,
        },
    )

    app.provide(
        "db_pool",
        Provide(
            create_db_pool,
            depends_on=["config"],
            singleton=True,
        ),
    )

    app.provide(
        "db_session",
        Provide(
            create_db_session,
            depends_on=["db_pool"],
            use_cache=True,
        ),
    )

    @app.get("/users")
    async def get_users(db_session: DatabaseSession) -> dict:
        """Get all users from the database.

        The db_session is automatically injected and will be cleaned up
        after this handler completes.

        Parameters
        ----------
        db_session : DatabaseSession
            Injected database session

        Returns:
        -------
        dict
            Response with user list
        """
        users = await db_session.query("SELECT * FROM users")
        return {"users": users}

    @app.get("/stats")
    async def get_stats(db_pool: DatabasePool) -> dict:
        """Get database pool statistics.

        The db_pool is the singleton instance shared across all requests.

        Parameters
        ----------
        db_pool : DatabasePool
            Injected database pool

        Returns:
        -------
        dict
            Response with pool statistics
        """
        return {
            "db_url": db_pool.db_url,
            "max_connections": db_pool.max_connections,
            "total_sessions_created": db_pool.connection_count,
        }

    @app.get("/complex")
    async def complex_query(db_session: DatabaseSession, db_pool: DatabasePool) -> dict:
        """Handler that uses multiple dependencies.

        Both db_session and db_pool are injected automatically.

        Parameters
        ----------
        db_session : DatabaseSession
            Injected database session
        db_pool : DatabasePool
            Injected database pool

        Returns:
        -------
        dict
            Response with query results and stats
        """
        users = await db_session.query("SELECT * FROM users WHERE active = true")
        return {
            "users": users,
            "pool_info": {
                "db_url": db_pool.db_url,
                "max_connections": db_pool.max_connections,
            },
        }

    logger.info("=== Database DI Example ===")
    logger.info("Routes:")
    logger.info("  GET /users  - Query users (uses db_session with cleanup)")
    logger.info("  GET /stats  - Pool statistics (uses singleton db_pool)")
    logger.info("  GET /complex - Complex query (uses both dependencies)")
    logger.info("Note: Watch the console for cleanup logs after each request")
    logger.info("Starting server on http://127.0.0.1:8000")
    logger.info("Press Ctrl+C to stop")


if __name__ == "__main__":
    main()
