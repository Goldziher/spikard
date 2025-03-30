"""Exception hierarchy for the Spikard library.

This module defines custom exceptions used throughout the Spikard library to provide
clear error messages and context for different types of failures when interacting
with LLM providers.

All exceptions inherit from the base SpikardError class, which includes context
information to aid in debugging. Specific subclasses are provided for different
error categories.
"""

from __future__ import annotations

from json import dumps
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from collections.abc import Mapping


class SpikardError(Exception):
    """Base exception class for all Spikard-specific errors.

    This class extends the standard Exception class and adds support for context
    information that can be used to provide additional details about the error.

    Attributes:
        context: Dictionary containing additional information about the error context.
            This can include request parameters, response data, or other debugging info.

    Args:
        message: The error message.
        **context: Additional contextual information about the error.
    """

    context: Mapping[str, Any]
    """The context of the error."""

    def __init__(self, message: str, **context: Any) -> None:
        self.context = context
        super().__init__(message)

    def __str__(self) -> str:
        """Return a string representation of the exception.

        Returns:
            A formatted string with the exception class name, message, and context.
        """
        ctx = f"\n\nContext: {dumps(self.context)}" if self.context else ""

        return f"{self.__class__.__name__}: {super().__str__()}{ctx}"

    def __repr__(self) -> str:
        """Return a string representation of the exception for debugging.

        Returns:
            The same string as __str__ to maintain consistency.
        """
        return self.__str__()


class JsonSchemaValidationError(SpikardError):
    """Raised when JSON schema validation fails.

    This exception is raised when data does not conform to the expected JSON schema,
    typically when validating tool inputs or outputs.
    """


class ResponseValidationError(SpikardError):
    """Raised when LLM response validation fails.

    This exception is raised when the response from an LLM provider doesn't match
    the expected format or cannot be properly validated against the expected schema.
    """


class SerializationError(SpikardError):
    """Raised when data serialization fails.

    This exception is raised when attempting to convert data structures to formats
    that can be sent to LLM providers (typically JSON).
    """


class DeserializationError(SpikardError):
    """Raised when data deserialization fails.

    This exception is raised when attempting to parse responses from LLM providers
    into expected data structures.
    """


class RequestError(SpikardError):
    """Raised when an error occurs during a request to an LLM provider.

    This can include network errors, API errors, rate limiting, or other failures
    when communicating with LLM services.

    Attributes:
        wait_interval: Time in seconds to wait before retrying the request,
            typically extracted from rate limit headers in 429 responses.

    Args:
        message: The error message.
        context: Additional contextual information about the error.
        wait_interval: Time in seconds to wait before retrying the request.
    """

    wait_interval: float | None
    """
    An amount of time in seconds to wait before retrying the request.
    This value should be set if it can be extracted from a 429 coded response.
    """

    def __init__(self, message: str, context: Any = None, wait_interval: float | None = None) -> None:
        self.wait_interval = wait_interval
        super().__init__(message, **context)


class RetryError(SpikardError):
    """Raised when maximum retry attempts have been exhausted.

    This exception is raised when the library has attempted to retry a failed
    request the maximum number of times and still cannot succeed.
    """


class ConfigurationError(SpikardError):
    """Raised when the configuration is invalid or incomplete."""


class MissingDependencyError(SpikardError):
    """Raised when a dependency is missing."""

    @classmethod
    def create_for_package(
        cls, *, dependency_group: str, functionality: str, package_name: str
    ) -> MissingDependencyError:
        """Creates a MissingDependencyError for a specified package and functionality.

        This class method generates an error message to notify users about a
        missing package dependency required for specific functionality. The error
        message includes details about the missing package and the optional
        dependency group required for installation.

        Args:
            dependency_group: The name of the optional dependency group that includes
                the required package.
            functionality: The functionality that requires the missing package.
            package_name: The name of the missing package.

        Returns:
            MissingDependencyError: A customized error indicating the missing
            dependency and how to resolve it.
        """
        return MissingDependencyError(
            f"The package '{package_name}' is required to use {functionality}. You can install using the provided optional dependency group by installing `spikard['{dependency_group}']`."
        )
