"""Spikard - High-performance HTTP framework with Rust core."""

# Import Response from the Rust module (_spikard)
# This is installed as a separate package via maturin
from _spikard import Response

from spikard._internal.converters import register_decoder
from spikard.app import Spikard
from spikard.params import Body, Cookie, Header, Path, Query
from spikard.routing import delete, get, patch, post, put

__all__ = [
    "Body",
    "Cookie",
    "Header",
    "Path",
    "Query",
    "Response",
    "Spikard",
    "delete",
    "get",
    "patch",
    "post",
    "put",
    "register_decoder",
]
