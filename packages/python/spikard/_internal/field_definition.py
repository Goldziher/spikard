"""Field definition - universal IR for all type systems.

Adapted from Litestar's FieldDefinition system.
Original source: https://github.com/litestar-org/litestar
License: MIT (see ATTRIBUTIONS.md in project root)

Modifications for Spikard:
- Removed dependency injection and route handler specifics
- Simplified for Python 3.10+ only
- Focused on JSON Schema generation
- Removed Litestar-specific kwarg definitions
"""

from collections import abc
from dataclasses import dataclass
from inspect import Parameter, Signature
from typing import Any, Literal, Union, get_args, get_origin

from spikard._internal.types import Empty
from spikard._internal.utils import (
    get_instantiable_origin,
    get_safe_generic_origin,
    is_class_and_subclass,
    is_non_string_iterable,
    is_non_string_sequence,
    unwrap_annotation,
)

try:
    import annotated_types
except ImportError:
    annotated_types = None  # type: ignore[assignment]


__all__ = ("FieldDefinition",)


NoneType = type(None)
UnionTypes: set[Any] = {Union}
try:
    from types import UnionType

    UnionTypes.add(UnionType)
except ImportError:
    pass


def _extract_annotated_types_constraints(meta: Any, is_sequence_container: bool) -> dict[str, Any]:
    """Extract constraints from annotated_types metadata.

    Args:
        meta: Metadata from Annotated type
        is_sequence_container: Whether this is for a sequence/array

    Returns:
        Dictionary of constraint names to values
    """
    if annotated_types is None:
        return {}

    kwargs = {}

    if isinstance(meta, annotated_types.GroupedMetadata):
        for sub_meta in meta:
            kwargs.update(_extract_annotated_types_constraints(sub_meta, is_sequence_container))
        return kwargs

    if isinstance(meta, annotated_types.Gt):
        kwargs["gt"] = meta.gt
    elif isinstance(meta, annotated_types.Ge):
        kwargs["ge"] = meta.ge
    elif isinstance(meta, annotated_types.Lt):
        kwargs["lt"] = meta.lt
    elif isinstance(meta, annotated_types.Le):
        kwargs["le"] = meta.le
    elif isinstance(meta, annotated_types.MultipleOf):
        kwargs["multiple_of"] = meta.multiple_of
    elif isinstance(meta, annotated_types.MinLen):
        if is_sequence_container:
            kwargs["min_items"] = meta.min_length
        else:
            kwargs["min_length"] = meta.min_length
    elif isinstance(meta, annotated_types.MaxLen):
        if is_sequence_container:
            kwargs["max_items"] = meta.max_length
        else:
            kwargs["max_length"] = meta.max_length
    elif isinstance(meta, annotated_types.Predicate):
        if meta.func == str.islower:
            kwargs["lower_case"] = True
        elif meta.func == str.isupper:
            kwargs["upper_case"] = True
        elif meta.func == str.isascii:
            kwargs["pattern"] = "[[:ascii:]]"
        elif meta.func == str.isdigit:
            kwargs["pattern"] = "[[:digit:]]"

    return kwargs


@dataclass(frozen=True)
class FieldDefinition:
    """Represents a function parameter or type annotation.

    This is the universal intermediate representation for all Python type systems:
    - Pydantic models and Field()
    - Dataclasses
    - TypedDict
    - Native Python types
    - msgspec (optional)
    - attrs (optional)
    """

    __slots__ = (
        "annotation",
        "args",
        "default",
        "extra",
        "inner_types",
        "instantiable_origin",
        "metadata",
        "name",
        "origin",
        "raw",
        "safe_generic_origin",
        "type_wrappers",
    )

    raw: Any
    """The annotation exactly as received."""
    annotation: Any
    """The annotation with any "wrapper" types removed, e.g. Annotated."""
    type_wrappers: tuple[type, ...]
    """A set of all "wrapper" types, e.g. Annotated."""
    origin: Any
    """The result of calling ``get_origin(annotation)`` after unwrapping Annotated, e.g. list."""
    args: tuple[Any, ...]
    """The result of calling ``get_args(annotation)`` after unwrapping Annotated, e.g. (int,)."""
    metadata: tuple[Any, ...]
    """Any metadata associated with the annotation via ``Annotated``."""
    instantiable_origin: Any
    """An equivalent type to ``origin`` that can be safely instantiated. E.g., ``Sequence`` -> ``list``."""
    safe_generic_origin: Any
    """An equivalent type to ``origin`` that can be safely used as a generic type across all supported Python versions."""
    inner_types: tuple["FieldDefinition", ...]
    """The type's generic args parsed as ``FieldDefinition``, if applicable."""
    default: Any
    """Default value of the field."""
    extra: dict[str, Any]
    """A mapping of extra values (constraints, etc.)."""
    name: str
    """Field name."""

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, FieldDefinition):
            return False

        if self.origin:
            return self.origin == other.origin and self.inner_types == other.inner_types

        return self.annotation == other.annotation  # type: ignore[no-any-return]

    def __hash__(self) -> int:
        return hash((self.name, self.raw, self.annotation, self.origin, self.inner_types))

    @property
    def has_default(self) -> bool:
        """Check if the field has a default value.

        Returns:
            True if the default is not Empty or Ellipsis otherwise False.
        """
        return self.default is not Empty and self.default is not Ellipsis

    @property
    def is_non_string_iterable(self) -> bool:
        """Check if the field type is an Iterable."""
        annotation = self.annotation
        if self.is_optional:
            annotation = self._make_non_optional_union(annotation)
        return is_non_string_iterable(annotation)

    @property
    def is_non_string_sequence(self) -> bool:
        """Check if the field type is a non-string Sequence."""
        annotation = self.annotation
        if self.is_optional:
            annotation = self._make_non_optional_union(annotation)
        return is_non_string_sequence(annotation)

    @property
    def is_any(self) -> bool:
        """Check if the field type is Any."""
        return self.annotation is Any or str(self.annotation) == "typing.Any"

    @property
    def is_simple_type(self) -> bool:
        """Check if the field type is a singleton value (e.g. int, str etc.)."""
        return not (self.is_optional or self.is_union or self.is_non_string_iterable)

    @property
    def is_required(self) -> bool:
        """Check if the field should be marked as a required parameter."""
        # Check if the default value is a Spikard Query/Body/Path parameter wrapper
        if hasattr(self.default, "has_default") and callable(self.default.has_default):
            # It's a Query/Body/Path/etc wrapper
            return not self.default.has_default()

        return bool(not self.is_optional and not self.is_any and not self.has_default)

    @property
    def is_annotated(self) -> bool:
        """Check if the field type is Annotated."""
        return bool(self.metadata)

    @property
    def is_literal(self) -> bool:
        """Check if the field type is Literal."""
        return self.origin is Literal

    @property
    def is_union(self) -> bool:
        """Whether the annotation is a union type or not."""
        return self.origin in UnionTypes

    @property
    def is_optional(self) -> bool:
        """Whether the annotation is Optional or not."""
        return bool(self.is_union and NoneType in self.args)

    @property
    def is_none_type(self) -> bool:
        """Whether the annotation is NoneType or not."""
        return self.annotation is NoneType

    def is_subclass_of(self, cl: type[Any] | tuple[type[Any], ...]) -> bool:
        """Whether the annotation is a subclass of the given type.

        Args:
            cl: The type to check, or tuple of types.

        Returns:
            Whether the annotation is a subtype of the given type(s).
        """
        if self.origin:
            if self.origin in UnionTypes:
                return all(t.is_subclass_of(cl) for t in self.inner_types)

            return self.origin not in UnionTypes and is_class_and_subclass(self.origin, cl)

        return self.annotation is not Any and is_class_and_subclass(self.annotation, cl)

    def _make_non_optional_union(self, annotation: Any) -> Any:
        """Remove None from a Union type."""
        if get_origin(annotation) in UnionTypes:
            args = tuple(arg for arg in get_args(annotation) if arg is not NoneType)
            if len(args) == 1:
                return args[0]
            return Union[args]
        return annotation

    @classmethod
    def from_annotation(cls, annotation: Any, **kwargs: Any) -> "FieldDefinition":
        """Initialize FieldDefinition from a type annotation.

        Args:
            annotation: The type annotation
            **kwargs: Additional keyword arguments

        Returns:
            FieldDefinition
        """
        unwrapped, metadata, wrappers = unwrap_annotation(annotation if annotation is not Empty else Any)
        origin = get_origin(unwrapped)

        annotation_args = () if origin is abc.Callable else get_args(unwrapped)

        # Extract constraints from metadata (annotated_types)
        if metadata:
            is_sequence_container = is_non_string_sequence(annotation)
            extra_constraints = kwargs.get("extra", {}).copy()

            for meta in metadata:
                constraints = _extract_annotated_types_constraints(meta, is_sequence_container)
                extra_constraints.update(constraints)

            if extra_constraints:
                kwargs["extra"] = extra_constraints

        # Also check if the default is a Pydantic Field and extract its constraints
        if hasattr(kwargs.get("default"), "metadata"):
            field_info = kwargs["default"]
            extra_constraints = kwargs.get("extra", {}).copy()

            # Import the constraint extractor from our existing code
            from spikard._internal.constraints import extract_constraints_from_field

            field_constraints = extract_constraints_from_field(field_info)
            extra_constraints.update(field_constraints)

            if extra_constraints:
                kwargs["extra"] = extra_constraints

        kwargs.setdefault("annotation", unwrapped)
        kwargs.setdefault("args", annotation_args)
        kwargs.setdefault("default", Empty)
        kwargs.setdefault("extra", {})
        kwargs.setdefault("inner_types", tuple(FieldDefinition.from_annotation(arg) for arg in annotation_args))
        kwargs.setdefault("instantiable_origin", get_instantiable_origin(origin, unwrapped))
        kwargs.setdefault("metadata", metadata)
        kwargs.setdefault("name", "")
        kwargs.setdefault("origin", origin)
        kwargs.setdefault("raw", annotation)
        kwargs.setdefault("safe_generic_origin", get_safe_generic_origin(origin, unwrapped))
        kwargs.setdefault("type_wrappers", wrappers)

        return FieldDefinition(**kwargs)

    @classmethod
    def from_parameter(cls, parameter: Parameter, fn_type_hints: dict[str, Any]) -> "FieldDefinition":
        """Initialize FieldDefinition from an inspect.Parameter.

        Args:
            parameter: inspect.Parameter instance
            fn_type_hints: Mapping of names to types (from get_type_hints)

        Returns:
            FieldDefinition
        """
        if parameter.name not in fn_type_hints:
            raise ValueError(
                f"'{parameter.name}' does not have a type annotation. If it should receive any value, use 'Any'."
            )

        annotation = fn_type_hints[parameter.name]

        return FieldDefinition.from_annotation(
            annotation=annotation,
            name=parameter.name,
            default=Empty if parameter.default is Signature.empty else parameter.default,
        )
