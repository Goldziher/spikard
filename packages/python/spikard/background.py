"""Python helper for scheduling background tasks."""

import asyncio
import threading
from typing import TYPE_CHECKING, Any

from _spikard import background_run as _background_run  # type: ignore[attr-defined]

if TYPE_CHECKING:
    from collections.abc import Coroutine

_PENDING_TASKS: set[asyncio.Task[object]] = set()


def _cleanup_task(task: asyncio.Task[object]) -> None:
    """Remove completed tasks from the tracking set."""
    _PENDING_TASKS.discard(task)


def run(awaitable: Coroutine[Any, Any, object]) -> None:
    """Schedule an awaitable to run in the background executor."""
    try:
        _background_run(awaitable)
    except RuntimeError:
        pass
    else:
        return

    try:
        loop = asyncio.get_running_loop()
    except RuntimeError:
        loop = asyncio.new_event_loop()

        def runner() -> None:
            asyncio.set_event_loop(loop)
            loop.run_until_complete(awaitable)
            loop.close()

        threading.Thread(target=runner, daemon=True).start()
    else:
        task: asyncio.Task[object] = loop.create_task(awaitable)
        _PENDING_TASKS.add(task)
        task.add_done_callback(_cleanup_task)
