from __future__ import annotations

from typing import Generic, TypeVar

T = TypeVar("T")


class Ref(Generic[T]):
    """A reference to a value that can be mutated."""

    value: T | None = None

    def __init__(self, value: T | None = None) -> None:
        self.value = value
