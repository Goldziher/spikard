"""Python profiling metrics collection module.

This module provides instrumentation to collect GIL, GC, and timing metrics
during benchmark runs. It's designed to be imported by benchmark applications
and writes metrics to a JSON file that the Rust benchmark harness can read.
"""

import atexit
import builtins
import gc
import json
import os
import signal
import tempfile
import time
from collections.abc import Callable, Generator
from contextlib import contextmanager, suppress
from dataclasses import dataclass
from functools import wraps
from pathlib import Path
from threading import Lock
from typing import Optional, ParamSpec, TypeVar


@dataclass
class GCMetrics:
    """Garbage collection metrics."""

    collections_gen0: int = 0
    collections_gen1: int = 0
    collections_gen2: int = 0
    total_collections: int = 0
    gc_time_ms: float = 0.0


@dataclass
class TimingMetrics:
    """Request timing breakdown."""

    handler_time_ms: float = 0.0
    serialization_time_ms: float = 0.0
    ffi_overhead_ms: float = 0.0


@dataclass
class ProfileMetrics:
    """Complete profiling metrics for a benchmark run."""

    gc: GCMetrics
    timing: TimingMetrics
    sample_count: int = 0


class MetricsCollector:
    """Collects profiling metrics during benchmark execution.

    Thread-safe singleton that accumulates metrics and writes them
    to a JSON file on shutdown.
    """

    _instance: Optional["MetricsCollector"] = None
    _lock = Lock()

    def __init__(self) -> None:
        self.metrics = ProfileMetrics(gc=GCMetrics(), timing=TimingMetrics())
        self.gc_enabled = gc.isenabled()
        self.initial_gc_counts = gc.get_count()
        self.request_count = 0

        env_path = os.environ.get("SPIKARD_METRICS_FILE")
        default_path = Path(tempfile.gettempdir()) / f"python-metrics-{os.getpid()}.json"
        self.output_path = Path(env_path) if env_path else default_path

    @classmethod
    def instance(cls) -> "MetricsCollector":
        """Get the singleton instance."""
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = cls()
        return cls._instance

    def record_gc_metrics(self) -> None:
        """Record current GC statistics."""
        if not self.gc_enabled:
            return

        stats = gc.get_stats()
        if stats:
            gen0 = stats[0].get("collections", 0)
            gen1 = stats[1].get("collections", 0) if len(stats) > 1 else 0
            gen2 = stats[2].get("collections", 0) if len(stats) > 2 else 0

            self.metrics.gc.collections_gen0 = gen0
            self.metrics.gc.collections_gen1 = gen1
            self.metrics.gc.collections_gen2 = gen2
            self.metrics.gc.total_collections = gen0 + gen1 + gen2

    @contextmanager
    def measure_handler(self) -> Generator[None, None, None]:
        """Context manager to measure handler execution time."""
        start = time.perf_counter()
        try:
            yield
        finally:
            elapsed_ms = (time.perf_counter() - start) * 1000
            with self._lock:
                self.request_count += 1
                n = self.request_count
                self.metrics.timing.handler_time_ms = (self.metrics.timing.handler_time_ms * (n - 1) + elapsed_ms) / n

    @contextmanager
    def measure_serialization(self) -> Generator[None, None, None]:
        """Context manager to measure serialization time."""
        start = time.perf_counter()
        try:
            yield
        finally:
            elapsed_ms = (time.perf_counter() - start) * 1000
            with self._lock:
                n = self.request_count
                if n > 0:
                    self.metrics.timing.serialization_time_ms = (
                        self.metrics.timing.serialization_time_ms * (n - 1) + elapsed_ms
                    ) / n

    def finalize(self) -> None:
        """Finalize metrics collection and write to file."""
        if self.gc_enabled:
            stats = gc.get_stats()
            if stats and len(stats) >= 3:
                self.metrics.gc.collections_gen0 = stats[0].get("collections", 0)
                self.metrics.gc.collections_gen1 = stats[1].get("collections", 0)
                self.metrics.gc.collections_gen2 = stats[2].get("collections", 0)
                self.metrics.gc.total_collections = (
                    self.metrics.gc.collections_gen0
                    + self.metrics.gc.collections_gen1
                    + self.metrics.gc.collections_gen2
                )

        self.metrics.sample_count = self.request_count

        output = {
            "gc_collections": self.metrics.gc.total_collections,
            "gc_collections_gen0": self.metrics.gc.collections_gen0,
            "gc_collections_gen1": self.metrics.gc.collections_gen1,
            "gc_collections_gen2": self.metrics.gc.collections_gen2,
            "gc_time_ms": self.metrics.gc.gc_time_ms,
            "handler_time_ms": self.metrics.timing.handler_time_ms if self.request_count > 0 else None,
            "serialization_time_ms": self.metrics.timing.serialization_time_ms if self.request_count > 0 else None,
            "ffi_overhead_ms": self.metrics.timing.ffi_overhead_ms,
            "sample_count": self.metrics.sample_count,
        }

        try:
            self.output_path.parent.mkdir(parents=True, exist_ok=True)
            with self.output_path.open("w") as file_handle:
                json.dump(output, file_handle, indent=2)
        except Exception:
            pass

    def __del__(self) -> None:
        """Ensure metrics are written on collector destruction."""
        with suppress(builtins.BaseException):
            self.finalize()


P = ParamSpec("P")
R = TypeVar("R")


def get_collector() -> MetricsCollector:
    """Get or create the global metrics collector."""
    return MetricsCollector.instance()


def enable_profiling() -> MetricsCollector:
    """Enable profiling for this process.

    Call this at application startup to initialize metrics collection.
    """
    collector = get_collector()

    atexit.register(collector.finalize)

    def _finalize_and_exit(_signum: int, _frame: object) -> None:
        with suppress(builtins.BaseException):
            collector.finalize()
        raise SystemExit(0)

    with suppress(builtins.BaseException):
        signal.signal(signal.SIGTERM, _finalize_and_exit)
        signal.signal(signal.SIGINT, _finalize_and_exit)

    return collector


def measure_handler(func: Callable[P, R]) -> Callable[P, R]:
    """Decorator to measure handler execution time.

    Usage:
        @measure_handler
        def my_handler(request):
            return {"status": "ok"}
    """

    @wraps(func)
    def wrapper(*args: P.args, **kwargs: P.kwargs) -> R:
        collector = get_collector()
        with collector.measure_handler():
            return func(*args, **kwargs)

    return wrapper


def measure_serialization(func: Callable[P, R]) -> Callable[P, R]:
    """Decorator to measure serialization time.

    Usage:
        @measure_serialization
        def serialize_response(data):
            return json.dumps(data)
    """

    @wraps(func)
    def wrapper(*args: P.args, **kwargs: P.kwargs) -> R:
        collector = get_collector()
        with collector.measure_serialization():
            return func(*args, **kwargs)

    return wrapper


if __name__ == "__main__":
    collector = enable_profiling()

    for i in range(100):
        with collector.measure_handler():
            time.sleep(0.001)
            with collector.measure_serialization():
                _ = json.dumps({"iteration": i})

    collector.finalize()

    metrics = json.loads(collector.output_path.read_text())
