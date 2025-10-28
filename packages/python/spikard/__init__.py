"""Spikard - High-performance HTTP framework with Rust core."""

# Import Response from the Rust module (_spikard)
# This is installed as a separate package via maturin
from _spikard import Response

from spikard.app import Spikard
from spikard.params import Cookie, Header
from spikard.routing import delete, get, patch, post, put

__all__ = [
    "Cookie",
    "Header",
    "Response",
    "Spikard",
    "delete",
    "get",
    "patch",
    "post",
    "put",
]
