"""Parsed function signature representation.

Adapted from Litestar's signature parsing system.
Original source: https://github.com/litestar-org/litestar
License: MIT (see ATTRIBUTIONS.md in project root)

Modifications for Spikard:
- Removed dependency injection and route handler specifics
- Simplified for Python 3.10+ only
- Focused on parameter extraction for JSON Schema generation
"""

from dataclasses import dataclass
from inspect import Signature
from typing import TYPE_CHECKING, Any, get_type_hints

from spikard._internal.field_definition import FieldDefinition
from spikard._internal.types import Empty

if TYPE_CHECKING:
    from collections.abc import Callable

__all__ = ("ParsedSignature", "parse_fn_signature")


@dataclass(frozen=True)
class ParsedSignature:
    """Parsed signature representing a function's parameters and return type.

    This object is the primary source of handler signature information.
    """

    __slots__ = ("original_signature", "parameters", "return_type")

    parameters: dict[str, FieldDefinition]
    """A mapping of parameter names to FieldDefinition instances."""
    return_type: FieldDefinition
    """The return annotation of the callable."""
    original_signature: Signature
    """The raw signature as returned by :func:`inspect.signature`"""

    @classmethod
    def from_fn(cls, fn: Callable[..., Any]) -> ParsedSignature:
        """Parse a function signature.

        Args:
            fn: Any callable

        Returns:
            ParsedSignature
        """
        signature = Signature.from_callable(fn)

        try:
            fn_type_hints = get_type_hints(fn, include_extras=True)
        except (AttributeError, NameError, TypeError, ValueError, RecursionError):
            fn_type_hints = getattr(fn, "__annotations__", {})

        return cls.from_signature(signature, fn_type_hints)

    @classmethod
    def from_signature(cls, signature: Signature, fn_type_hints: dict[str, type]) -> ParsedSignature:
        """Parse an :class:`inspect.Signature` instance.

        Args:
            signature: An :class:`inspect.Signature` instance
            fn_type_hints: Mapping of names to types

        Returns:
            ParsedSignature
        """
        parameters = tuple(
            FieldDefinition.from_parameter(parameter=parameter, fn_type_hints=fn_type_hints)
            for name, parameter in signature.parameters.items()
            if name not in ("self", "cls")
        )

        return_annotation = fn_type_hints.get("return", Any)
        return_type = FieldDefinition.from_annotation(return_annotation)

        if "return" not in fn_type_hints:
            return_type = FieldDefinition.from_annotation(Empty, name="return", default=Empty)

        return cls(
            parameters={p.name: p for p in parameters},
            return_type=return_type,
            original_signature=signature,
        )


def parse_fn_signature(fn: Callable[..., Any]) -> ParsedSignature:
    """Parse a function's signature into a ParsedSignature.

    This is the main entry point for signature parsing.

    Args:
        fn: The function to parse

    Returns:
        ParsedSignature with all parameters and return type analyzed
    """
    return ParsedSignature.from_fn(fn)
