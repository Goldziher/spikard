"""Spikard Rust extension module."""

from pkgutil import extend_path

__path__ = extend_path(__path__, __name__)

from _spikard._spikard import *  # noqa: F403

__version__ = "0.2.0"
