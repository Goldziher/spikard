"""Type definitions for internal use.

Adapted from Litestar's type system.
Original source: https://github.com/litestar-org/litestar
License: MIT (see ATTRIBUTIONS.md in project root)
"""

from __future__ import annotations

from typing import Any


class Empty:
    """Sentinel class for empty values."""

    def __bool__(self) -> bool:
        return False

    def __repr__(self) -> str:
        return "Empty"

    def __copy__(self) -> Empty:
        return self

    def __deepcopy__(self, _: Any) -> Empty:
        return self


Empty = Empty()  # type: ignore[assignment,misc]
