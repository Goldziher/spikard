"""Database example with cleanup pattern using async generators.

This example demonstrates:
- Factory dependencies that depend on other dependencies
- Async generator pattern for cleanup (close DB connection)
- Singleton vs per-request caching
- Proper resource management
"""

from collections.abc import AsyncGenerator
from spikard import Spikard
from spikard.di import Provide


# Simulated database pool (singleton - shared across requests)
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
        print(f"[DB Pool] Created pool for {self.db_url} (max: {self.max_connections})")

    async def create_session(self) -> "DatabaseSession":
        """Create a new database session.

        Returns:
        -------
        DatabaseSession
            A new session from the pool
        """
        self.connection_count += 1
        print(f"[DB Pool] Creating session #{self.connection_count}")
        return DatabaseSession(self, self.connection_count)


# Simulated database session (per-request with cleanup)
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
        print(f"[DB Session {self.session_id}] Session opened")

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
        print(f"[DB Session {self.session_id}] Executing: {sql}")
        return [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]

    async def close(self) -> None:
        """Close the session and return to pool."""
        if not self.closed:
            self.closed = True
            print(f"[DB Session {self.session_id}] Session closed")


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
    print("[Factory] Creating database pool...")
    return DatabasePool(config)


async def create_db_session(db_pool: DatabasePool) -> AsyncGenerator[DatabaseSession, None]:
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
    print("[Factory] Creating database session...")
    session = await db_pool.create_session()

    # Yield the session to the handler
    yield session

    # Cleanup runs after the handler completes
    print("[Factory] Cleaning up database session...")
    await session.close()


def main() -> None:
    """Run the database example application."""
    app = Spikard()

    # Register configuration (static value)
    app.provide(
        "config",
        {
            "db_url": "postgresql://localhost/myapp",
            "max_connections": 10,
        },
    )

    # Register database pool factory (singleton)
    # This will be created once and shared across all requests
    app.provide(
        "db_pool",
        Provide(
            create_db_pool,
            depends_on=["config"],
            singleton=True,  # Share across all requests
        ),
    )

    # Register database session factory (per-request with cleanup)
    # This will be created fresh for each request and cleaned up after
    app.provide(
        "db_session",
        Provide(
            create_db_session,
            depends_on=["db_pool"],
            use_cache=True,  # Cache within request (but not across requests)
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

    print("\n=== Database DI Example ===")
    print("Routes:")
    print("  GET /users  - Query users (uses db_session with cleanup)")
    print("  GET /stats  - Pool statistics (uses singleton db_pool)")
    print("  GET /complex - Complex query (uses both dependencies)")
    print("\nNote: Watch the console for cleanup logs after each request")
    print("\nStarting server on http://127.0.0.1:8000")
    print("Press Ctrl+C to stop\n")


if __name__ == "__main__":
    main()
