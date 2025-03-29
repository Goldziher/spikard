from __future__ import annotations

from json import dumps
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from collections.abc import Mapping


class SpikardError(Exception):
    """Raised when an internal error occurs."""

    context: Mapping[str, Any]
    """The context of the error."""

    def __init__(self, message: str, **context: Any) -> None:
        self.context = context
        super().__init__(message)

    def __str__(self) -> str:
        """Return a string representation of the exception."""
        ctx = f"\n\nContext: {dumps(self.context)}" if self.context else ""

        return f"{self.__class__.__name__}: {super().__str__()}{ctx}"

    def __repr__(self) -> str:
        """Return a string representation of the exception."""
        return self.__str__()


class JsonSchemaValidationError(SpikardError):
    """Raised when a validation error occurs."""


class ResponseValidationError(SpikardError):
    """Raised when a validation error occurs."""


class SerializationError(SpikardError):
    """Raised when an error occurs during serialization."""


class DeserializationError(SpikardError):
    """Raised when an error occurs during deserialization."""


class RequestError(SpikardError):
    """Raised when an error occurs during a request."""

    wait_internal: float | None
    """
    An amount of time in seconds to wait before retrying the request.
    This value should be set if it can be extracted from a 429 coded response.
    """

    def __init__(self, message: str, context: Any = None, wait_internal: float | None = None) -> None:
        self.wait_internal = wait_internal
        super().__init__(message, **context)


class RetryError(SpikardError):
    """Raised when a retry error occurs."""
