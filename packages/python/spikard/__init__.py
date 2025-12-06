"""Spikard - High-performance HTTP framework with Rust core."""

from _spikard import Response, StreamingResponse  # type: ignore[attr-defined]
from spikard import background
from spikard._internal.converters import register_decoder
from spikard.app import HttpMethod, Spikard
from spikard.config import (
    ApiKeyConfig,
    CompressionConfig,
    JwtConfig,
    RateLimitConfig,
    ServerConfig,
    StaticFilesConfig,
)
from spikard.datastructures import UploadFile
from spikard.di import Provide
from spikard.params import Body, Cookie, Header, Path, Query
from spikard.request import Request
from spikard.routing import delete, get, head, options, patch, post, put, route, trace
from spikard.sse import SseEvent, sse
from spikard.websocket import websocket

__all__ = [
    "ApiKeyConfig",
    "Body",
    "CompressionConfig",
    "Cookie",
    "Header",
    "HttpMethod",
    "JwtConfig",
    "Path",
    "Provide",
    "Query",
    "RateLimitConfig",
    "Request",
    "Response",
    "ServerConfig",
    "Spikard",
    "SseEvent",
    "StaticFilesConfig",
    "StreamingResponse",
    "UploadFile",
    "background",
    "delete",
    "get",
    "head",
    "options",
    "patch",
    "post",
    "put",
    "register_decoder",
    "route",
    "sse",
    "trace",
    "websocket",
]
