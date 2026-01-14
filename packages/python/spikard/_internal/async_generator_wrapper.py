"""AsyncGeneratorWrapper - converts async generators to sync iterators.

This allows Rust code to iterate over async generators using simple __next__() calls
instead of having to manage Python event loops in Rust.

Based on Robyn's proven implementation.
"""

import asyncio
from collections.abc import AsyncGenerator, AsyncIterator, Iterator
from typing import TypeVar

T = TypeVar("T")


class AsyncGeneratorWrapper(Iterator[T]):
    """Wraps an async generator to implement the sync iterator protocol.

    This wrapper creates a single event loop when first iterated and reuses it
    for all iterations. Each call to __next__() awaits the next value from the
    async generator using run_until_complete().

    Example:
        >>> async def async_gen():
        ...     for i in range(3):
        ...         await asyncio.sleep(0.1)
        ...         yield f"item {i}"
        >>>
        >>> wrapper = AsyncGeneratorWrapper(async_gen())
        >>> for item in wrapper:  # Sync iteration!
        ...     print(item)
    """

    def __init__(self, async_gen: AsyncGenerator[T]) -> None:
        """Initialize the wrapper with an async generator.

        Args:
            async_gen: An async generator to wrap
        """
        self.async_gen = async_gen
        self._loop: asyncio.AbstractEventLoop | None = None
        self._iterator: AsyncIterator[T] | None = None
        self._exhausted = False

    def __iter__(self) -> AsyncGeneratorWrapper[T]:
        """Return self as iterator."""
        return self

    def __next__(self) -> T:
        """Get the next value from the async generator.

        On first call, initializes the event loop and iterator.
        Subsequent calls reuse the same event loop.

        Returns:
            The next value from the async generator

        Raises:
            StopIteration: When the async generator is exhausted
        """
        if self._exhausted:
            raise StopIteration

        if self._iterator is None:
            self._init_async_iterator()

        try:
            return self._get_next_value()
        except StopIteration:
            self._exhausted = True
            raise

    def _init_async_iterator(self) -> None:
        """Initialize the event loop and async iterator.

        Tries to get the running event loop first. If there isn't one,
        creates a new event loop and sets it as the current one.
        """
        try:
            self._loop = asyncio.get_running_loop()
        except RuntimeError:
            self._loop = asyncio.new_event_loop()
            asyncio.set_event_loop(self._loop)

        self._iterator = self.async_gen.__aiter__()

    def _get_next_value(self) -> T:
        """Get the next value from async generator without buffering.

        Returns:
            The next value from the async generator

        Raises:
            StopIteration: When the generator is exhausted or on error
        """
        if self._loop is None:
            raise RuntimeError("Event loop not initialized")
        if self._iterator is None:
            raise RuntimeError("Iterator not initialized")

        async def get_next() -> T:
            if self._iterator is None:
                raise RuntimeError("Iterator not initialized")
            return await self._iterator.__anext__()

        try:
            return self._loop.run_until_complete(get_next())
        except StopAsyncIteration as e:
            raise StopIteration from e
        except (RuntimeError, OSError) as e:
            raise StopIteration from e
