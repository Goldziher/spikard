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

from spikard._internal.constraints import extract_constraints_from_field
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


def _combine_union_args(args: tuple[Any, ...]) -> Any:
    """Reconstruct a union type using the ``|`` operator."""
    union_type: Any = args[0]
    for arg in args[1:]:
        union_type = union_type | arg
    return union_type


def _merge_grouped_metadata(meta: Any, is_sequence_container: bool) -> dict[str, Any]:
    """Flatten grouped metadata entries into a single dictionary."""
    merged: dict[str, Any] = {}
    for sub_meta in meta:
        merged.update(_extract_annotated_types_constraints(sub_meta, is_sequence_container))
    return merged


def _extract_numeric_constraints(meta: Any) -> dict[str, Any]:
    """Extract simple numeric constraints from annotated_types metadata."""
    if annotated_types is None:
        return {}

    numeric_constraints: tuple[tuple[type[Any], str], ...] = (
        (annotated_types.Gt, "gt"),
        (annotated_types.Ge, "ge"),
        (annotated_types.Lt, "lt"),
        (annotated_types.Le, "le"),
        (annotated_types.MultipleOf, "multiple_of"),
    )

    for constraint_type, key in numeric_constraints:
        if isinstance(meta, constraint_type):
            return {key: getattr(meta, key)}

    return {}


def _extract_length_constraints(meta: Any, is_sequence_container: bool) -> dict[str, Any]:
    """Extract length-related constraints."""
    if annotated_types is None:
        return {}

    if isinstance(meta, annotated_types.MinLen):
        key = "min_items" if is_sequence_container else "min_length"
        return {key: meta.min_length}
    if isinstance(meta, annotated_types.MaxLen):
        key = "max_items" if is_sequence_container else "max_length"
        return {key: meta.max_length}
    return {}


_PREDICATE_CONSTRAINT_MAPPING: dict[Any, dict[str, Any]] = {
    str.islower: {"lower_case": True},
    str.isupper: {"upper_case": True},
    str.isascii: {"pattern": "[[:ascii:]]"},
    str.isdigit: {"pattern": "[[:digit:]]"},
}


def _extract_predicate_constraints(meta: Any) -> dict[str, Any]:
    """Extract predicate-based constraints such as str.islower/isupper."""
    if annotated_types is None:
        return {}

    if isinstance(meta, annotated_types.Predicate):
        return _PREDICATE_CONSTRAINT_MAPPING.get(meta.func, {})
    return {}


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

    if isinstance(meta, annotated_types.GroupedMetadata):
        return _merge_grouped_metadata(meta, is_sequence_container)

    numeric_constraints = _extract_numeric_constraints(meta)
    if numeric_constraints:
        return numeric_constraints

    length_constraints = _extract_length_constraints(meta, is_sequence_container)
    if length_constraints:
        return length_constraints

    return _extract_predicate_constraints(meta)


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
    inner_types: tuple[FieldDefinition, ...]
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
        if hasattr(self.default, "is_required") and callable(self.default.is_required):
            return bool(self.default.is_required())

        if hasattr(self.default, "has_default") and callable(self.default.has_default):
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
            return _combine_union_args(args)
        return annotation

    @classmethod
    def from_annotation(cls, annotation: Any, **kwargs: Any) -> FieldDefinition:
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

        if metadata:
            is_sequence_container = is_non_string_sequence(annotation)
            extra_constraints = kwargs.get("extra", {}).copy()

            for meta in metadata:
                constraints = _extract_annotated_types_constraints(meta, is_sequence_container)
                extra_constraints.update(constraints)

            if extra_constraints:
                kwargs["extra"] = extra_constraints

        if hasattr(kwargs.get("default"), "metadata"):
            field_info = kwargs["default"]
            extra_constraints = kwargs.get("extra", {}).copy()
            field_constraints = extract_constraints_from_field(field_info)
            extra_constraints.update(field_constraints)

            if extra_constraints:
                kwargs["extra"] = extra_constraints

        default_val = kwargs.get("default")
        if default_val is not Empty and hasattr(default_val, "__class__"):
            class_name = default_val.__class__.__name__
            if class_name in ("Header", "Cookie", "Query", "Path", "Body"):
                extra_constraints = kwargs.get("extra", {}).copy()
                source_map = {
                    "Header": "header",
                    "Cookie": "cookie",
                    "Query": "query",
                    "Path": "path",
                    "Body": "body",
                }
                extra_constraints["source"] = source_map.get(class_name, "query")

                if hasattr(default_val, "schema") and getattr(default_val, "schema", None):
                    schema_dict = default_val.schema  # type: ignore[union-attr]
                    if schema_dict:
                        extra_constraints.update(schema_dict)

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
    def from_parameter(cls, parameter: Parameter, fn_type_hints: dict[str, Any]) -> FieldDefinition:
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
