"""Internal utilities for parameter introspection and type conversion.

This subpackage contains vendored and adapted code from Litestar for
type annotation introspection and parameter schema generation, plus the
runtime type-conversion helpers used by the ergonomic handler adapter.

Original source: https://github.com/litestar-org/litestar
License: MIT (see ATTRIBUTIONS.md in project root)
Copyright (c) 2021-2025 Litestar Org.

Modifications: Adapted for Spikard's JSON Schema-based parameter validation system.
"""

from spikard._internal.constraints import extract_constraints_from_field
from spikard._internal.converters import (
    convert_params,
    convert_value,
    needs_conversion,
    register_decoder,
)
from spikard._internal.field_definition import FieldDefinition
from spikard._internal.json_schema import field_definition_to_json_schema
from spikard._internal.parsed_signature import ParsedSignature, parse_fn_signature
from spikard._internal.serialization import to_builtins
from spikard._internal.types import Empty

__all__ = [
    "Empty",
    "FieldDefinition",
    "ParsedSignature",
    "convert_params",
    "convert_value",
    "extract_constraints_from_field",
    "field_definition_to_json_schema",
    "needs_conversion",
    "parse_fn_signature",
    "register_decoder",
    "to_builtins",
]
