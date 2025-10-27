"""Spikard - High-performance HTTP framework with Rust core."""

from spikard.app import Spikard
from spikard.routing import get, post, put, patch, delete

__all__ = ["Spikard", "get", "post", "put", "patch", "delete"]
