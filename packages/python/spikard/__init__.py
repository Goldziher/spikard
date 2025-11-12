"""Spikard - High-performance HTTP framework with Rust core."""

# Import Response from the Rust module (_spikard)
# This is installed as a separate package via maturin
from _spikard import Response, StreamingResponse

from spikard._internal.converters import register_decoder
from spikard.app import Spikard
from spikard.config import (
    ApiKeyConfig,
    CompressionConfig,
    JwtConfig,
    RateLimitConfig,
    ServerConfig,
    StaticFilesConfig,
)
from spikard.params import Body, Cookie, Header, Path, Query
from spikard.request import Request
from spikard.routing import delete, get, head, options, patch, post, put, route, trace
from spikard.sse import SseEvent, SseEventProducer
from spikard.websocket import WebSocketHandler

__all__ = [
    "ApiKeyConfig",
    "Body",
    "CompressionConfig",
    "Cookie",
    "Header",
    "JwtConfig",
    "Path",
    "Query",
    "RateLimitConfig",
    "Request",
    "Response",
    "ServerConfig",
    "Spikard",
    "SseEvent",
    "SseEventProducer",
    "StaticFilesConfig",
    "StreamingResponse",
    "WebSocketHandler",
    "delete",
    "get",
    "head",
    "options",
    "patch",
    "post",
    "put",
    "register_decoder",
    "route",
    "trace",
]
