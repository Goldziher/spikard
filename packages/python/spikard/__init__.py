"""Spikard - High-performance HTTP framework with Rust core."""

from __future__ import annotations

from importlib import import_module
from typing import TYPE_CHECKING

from _spikard import (  # type: ignore[attr-defined]
    GrpcRequest,
    GrpcResponse,
    Response,
    StreamingResponse,
)

if TYPE_CHECKING:
    from spikard import background as background
    from spikard import grpc as grpc
    from spikard._internal.converters import register_decoder as register_decoder
    from spikard.app import HttpMethod as HttpMethod
    from spikard.app import Spikard as Spikard
    from spikard.config import ApiKeyConfig as ApiKeyConfig
    from spikard.config import CompressionConfig as CompressionConfig
    from spikard.config import JwtConfig as JwtConfig
    from spikard.config import OpenApiConfig as OpenApiConfig
    from spikard.config import RateLimitConfig as RateLimitConfig
    from spikard.config import ServerConfig as ServerConfig
    from spikard.config import StaticFilesConfig as StaticFilesConfig
    from spikard.datastructures import UploadFile as UploadFile
    from spikard.di import Provide as Provide
    from spikard.grpc import GrpcHandler as GrpcHandler
    from spikard.grpc import GrpcService as GrpcService
    from spikard.jsonrpc import JsonRpcMethodInfo as JsonRpcMethodInfo
    from spikard.params import Body as Body
    from spikard.params import Cookie as Cookie
    from spikard.params import Header as Header
    from spikard.params import Path as Path
    from spikard.params import Query as Query
    from spikard.request import Request as Request
    from spikard.routing import delete as delete
    from spikard.routing import get as get
    from spikard.routing import head as head
    from spikard.routing import options as options
    from spikard.routing import patch as patch
    from spikard.routing import post as post
    from spikard.routing import put as put
    from spikard.routing import route as route
    from spikard.routing import trace as trace
    from spikard.sse import SseEvent as SseEvent
    from spikard.sse import sse as sse
    from spikard.testing import TestClient as TestClient
    from spikard.websocket import websocket as websocket

__all__ = [
    "ApiKeyConfig",
    "Body",
    "CompressionConfig",
    "Cookie",
    "GrpcHandler",
    "GrpcRequest",
    "GrpcResponse",
    "GrpcService",
    "Header",
    "HttpMethod",
    "JsonRpcMethodInfo",
    "JwtConfig",
    "OpenApiConfig",
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
    "TestClient",
    "UploadFile",
    "background",
    "delete",
    "get",
    "grpc",
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

_LAZY_EXPORTS: dict[str, tuple[str, str]] = {
    "ApiKeyConfig": ("spikard.config", "ApiKeyConfig"),
    "Body": ("spikard.params", "Body"),
    "CompressionConfig": ("spikard.config", "CompressionConfig"),
    "Cookie": ("spikard.params", "Cookie"),
    "GrpcHandler": ("spikard.grpc", "GrpcHandler"),
    "GrpcService": ("spikard.grpc", "GrpcService"),
    "Header": ("spikard.params", "Header"),
    "HttpMethod": ("spikard.app", "HttpMethod"),
    "JsonRpcMethodInfo": ("spikard.jsonrpc", "JsonRpcMethodInfo"),
    "JwtConfig": ("spikard.config", "JwtConfig"),
    "OpenApiConfig": ("spikard.config", "OpenApiConfig"),
    "Path": ("spikard.params", "Path"),
    "Provide": ("spikard.di", "Provide"),
    "Query": ("spikard.params", "Query"),
    "RateLimitConfig": ("spikard.config", "RateLimitConfig"),
    "Request": ("spikard.request", "Request"),
    "ServerConfig": ("spikard.config", "ServerConfig"),
    "Spikard": ("spikard.app", "Spikard"),
    "SseEvent": ("spikard.sse", "SseEvent"),
    "StaticFilesConfig": ("spikard.config", "StaticFilesConfig"),
    "TestClient": ("spikard.testing", "TestClient"),
    "UploadFile": ("spikard.datastructures", "UploadFile"),
    "background": ("spikard.background", "__module__"),
    "delete": ("spikard.routing", "delete"),
    "get": ("spikard.routing", "get"),
    "grpc": ("spikard.grpc", "__module__"),
    "head": ("spikard.routing", "head"),
    "options": ("spikard.routing", "options"),
    "patch": ("spikard.routing", "patch"),
    "post": ("spikard.routing", "post"),
    "put": ("spikard.routing", "put"),
    "register_decoder": ("spikard._internal.converters", "register_decoder"),
    "route": ("spikard.routing", "route"),
    "sse": ("spikard.sse", "sse"),
    "trace": ("spikard.routing", "trace"),
    "websocket": ("spikard.websocket", "websocket"),
}


def __getattr__(name: str) -> object:
    target = _LAZY_EXPORTS.get(name)
    if target is None:
        raise AttributeError(name)

    module_name, attr_name = target
    module = import_module(module_name)
    value = module if attr_name == "__module__" else getattr(module, attr_name)
    globals()[name] = value
    return value


def __dir__() -> list[str]:
    return sorted({*globals().keys(), *_LAZY_EXPORTS.keys()})
