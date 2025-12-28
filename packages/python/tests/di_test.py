"""Advanced tests for spikard.di.Provide auto-detection functionality.

This module provides comprehensive coverage for the Provide class __post_init__ logic
that auto-detects the depends_on parameter from function signatures (lines 136-142).

The auto-detection filters out standard parameter names (self, cls, request, response)
and builds a list of remaining parameters that represent actual dependencies.
"""

from __future__ import annotations

from spikard.di import Provide


def test_explicit_depends_on_takes_precedence() -> None:
    """Explicit depends_on should not be overridden by auto-detection.

    When depends_on is explicitly provided (even if non-empty), the
    auto-detection logic should be skipped entirely.
    """

    def my_func(db: object, cache: object) -> str:
        return "value"

    provider = Provide(dependency=my_func, depends_on=["custom_dep"])

    assert provider.depends_on == ["custom_dep"]


def test_auto_detect_single_parameter() -> None:
    """Auto-detection should find single dependency parameter.

    When a function has one parameter and it's not filtered, it should
    be automatically detected as a dependency.
    """

    def my_func(config: dict[str, object]) -> object:
        return config.get("value")

    provider = Provide(dependency=my_func)

    assert provider.depends_on == ["config"]


def test_auto_detect_filters_self() -> None:
    """The 'self' parameter should be filtered during auto-detection.

    Methods have a 'self' parameter that is not a dependency;
    it should be excluded from the auto-detected list.
    """

    def my_method(self: object, db: object) -> object:
        return db

    provider = Provide(dependency=my_method)

    assert provider.depends_on == ["db"]
    assert "self" not in provider.depends_on


def test_auto_detect_filters_cls() -> None:
    """The 'cls' parameter should be filtered during auto-detection.

    Class methods have a 'cls' parameter that is not a dependency;
    it should be excluded from the auto-detected list.
    """

    def my_classmethod(cls: type, service: object) -> object:
        return service

    provider = Provide(dependency=my_classmethod)

    assert provider.depends_on == ["service"]
    assert "cls" not in provider.depends_on


def test_auto_detect_filters_request() -> None:
    """The 'request' parameter should be filtered during auto-detection.

    HTTP handlers often receive a request object; this is implicitly
    available and should be excluded from dependencies.
    """

    def my_handler(request: object, db: object) -> object:
        return db

    provider = Provide(dependency=my_handler)

    assert provider.depends_on == ["db"]
    assert "request" not in provider.depends_on


def test_auto_detect_filters_response() -> None:
    """The 'response' parameter should be filtered during auto-detection.

    HTTP handlers often receive a response object; this is implicitly
    available and should be excluded from dependencies.
    """

    def my_handler(response: object, cache: object) -> object:
        return cache

    provider = Provide(dependency=my_handler)

    assert provider.depends_on == ["cache"]
    assert "response" not in provider.depends_on


def test_auto_detect_empty_for_no_params() -> None:
    """Auto-detection should return empty list for parameterless functions.

    A function with no parameters has no dependencies and should result
    in an empty depends_on list.
    """

    def constant() -> str:
        return "constant_value"

    provider = Provide(dependency=constant)

    assert provider.depends_on == []


def test_auto_detect_multiple_parameters() -> None:
    """Auto-detection should find multiple dependency parameters.

    A function with multiple parameters should have all of them detected
    as dependencies (unless they are filtered names).
    """

    def my_func(db: object, cache: object, config: object) -> str:
        return str(db) + str(cache) + str(config)

    provider = Provide(dependency=my_func)

    assert provider.depends_on == ["db", "cache", "config"]
    assert len(provider.depends_on) == 3


def test_auto_detect_filters_all_standard_params() -> None:
    """All standard filtered parameters should be excluded together.

    A function with all standard parameters plus real dependencies should
    filter out all four standard names.
    """

    def complex_handler(
        self: object,
        cls: type,
        request: object,
        response: object,
        db: object,
        cache: object,
    ) -> str:
        return str(db) + str(cache)

    provider = Provide(dependency=complex_handler)

    assert provider.depends_on == ["db", "cache"]
    assert "self" not in provider.depends_on
    assert "cls" not in provider.depends_on
    assert "request" not in provider.depends_on
    assert "response" not in provider.depends_on


def test_auto_detect_with_defaults() -> None:
    """Auto-detection should detect parameters regardless of defaults.

    Parameters with default values are still dependencies; they should
    be included in the auto-detected list.
    """

    def my_func(db: object, cache: str = "default") -> str:
        return str(db) + cache

    provider = Provide(dependency=my_func)

    assert "db" in provider.depends_on
    assert "cache" in provider.depends_on
    assert len(provider.depends_on) == 2


def test_auto_detect_with_annotations() -> None:
    """Auto-detection should work regardless of type annotations.

    Type hints on parameters should not affect dependency detection;
    all parameters should be detected.
    """

    def typed_func(db: str, config: dict[str, object]) -> str:
        return db + str(config)

    provider = Provide(dependency=typed_func)

    assert provider.depends_on == ["db", "config"]


def test_auto_detect_lambda() -> None:
    """Auto-detection should work with lambda functions.

    Lambda functions can have parameters that should be auto-detected
    as dependencies.
    """
    provider = Provide(dependency=lambda db: str(db))

    assert provider.depends_on == ["db"]


def test_auto_detect_lambda_multiple() -> None:
    """Auto-detection should detect multiple parameters in lambdas.

    Multi-parameter lambdas should have all parameters detected.
    """
    provider = Provide(dependency=lambda db, cache, config: str(db) + str(cache) + str(config))

    assert provider.depends_on == ["db", "cache", "config"]


def test_auto_detect_preserves_order() -> None:
    """Auto-detection should preserve parameter order.

    The order of parameters in the function signature should match
    the order in the auto-detected depends_on list.
    """

    def ordered_func(first: object, second: object, third: object) -> str:
        return str(first) + str(second) + str(third)

    provider = Provide(dependency=ordered_func)

    assert provider.depends_on == ["first", "second", "third"]


def test_explicit_empty_depends_on() -> None:
    """Empty list triggers auto-detection due to falsy check.

    Note: The code uses 'if not self.depends_on:' which treats empty list
    as falsy, so empty list also triggers auto-detection. This documents
    current behavior.
    """

    def my_func(db: object, cache: object) -> str:
        return "value"

    provider = Provide(dependency=my_func, depends_on=[])

    assert provider.depends_on == ["db", "cache"]


def test_auto_detect_with_kwargs() -> None:
    """Auto-detection should detect explicit parameters with **kwargs.

    Functions that accept **kwargs should still detect explicit parameters.
    """

    def func_with_kwargs(db: object, **kwargs: object) -> object:
        return db

    provider = Provide(dependency=func_with_kwargs)

    assert "db" in provider.depends_on


def test_auto_detect_args_and_kwargs() -> None:
    """Auto-detection should handle *args and **kwargs parameters.

    Functions with *args and **kwargs should detect explicit positional
    and keyword parameters.
    """

    def variadic_func(db: object, *args: object, **kwargs: object) -> object:
        return db

    provider = Provide(dependency=variadic_func)

    assert "db" in provider.depends_on


def test_auto_detect_keyword_only() -> None:
    """Auto-detection should detect keyword-only parameters.

    Parameters after * in a function signature are keyword-only and
    should still be detected as dependencies.
    """

    def keyword_only(db: object, *, cache: object) -> str:
        return str(db) + str(cache)

    provider = Provide(dependency=keyword_only)

    assert "db" in provider.depends_on
    assert "cache" in provider.depends_on
    assert len(provider.depends_on) == 2


def test_auto_detect_mixed_filtered_and_real_deps() -> None:
    """Auto-detection should handle mix of filtered names and dependencies.

    When some parameters are filtered names and others are dependencies,
    only the non-filtered ones should be in the result.
    """

    def mixed_func(self: object, db: object, request: object, config: object) -> str:
        return str(db) + str(config)

    provider = Provide(dependency=mixed_func)

    assert provider.depends_on == ["db", "config"]


def test_auto_detect_singleton_flag_preserved() -> None:
    """Auto-detection should not affect singleton flag.

    The singleton flag and use_cache flag should be set independently
    of whether depends_on is auto-detected or explicit.
    """

    def my_func(db: object) -> object:
        return db

    provider = Provide(dependency=my_func, singleton=True)

    assert provider.depends_on == ["db"]
    assert provider.singleton is True


def test_auto_detect_use_cache_flag_preserved() -> None:
    """Auto-detection should not affect use_cache flag.

    The use_cache flag should be preserved regardless of auto-detection.
    """

    def my_func(db: object) -> object:
        return db

    provider = Provide(dependency=my_func, use_cache=True)

    assert provider.depends_on == ["db"]
    assert provider.use_cache is True


def test_provide_repr_with_auto_detected_deps() -> None:
    """The __repr__ should show auto-detected dependencies.

    String representation should include the auto-detected depends_on.
    """

    def my_func(db: object, cache: object) -> str:
        return str(db) + str(cache)

    provider = Provide(dependency=my_func)

    repr_str = repr(provider)
    assert "my_func" in repr_str
    assert "depends_on" in repr_str
    assert "db" in repr_str
    assert "cache" in repr_str


def test_auto_detect_with_positional_only() -> None:
    """Auto-detection should handle positional-only parameters.

    Parameters before / in a function signature are positional-only
    and should still be detected as dependencies.
    """

    def positional_only(db: object, /, cache: object) -> str:
        return str(db) + str(cache)

    provider = Provide(dependency=positional_only)

    assert "db" in provider.depends_on
    assert "cache" in provider.depends_on
    assert len(provider.depends_on) == 2


def test_auto_detect_complex_signature() -> None:
    """Auto-detection should handle complex function signatures.

    A function with mixed parameter types: positional-only, regular,
    keyword-only, *args, and **kwargs. Note: *args and **kwargs are
    also included in the auto-detected list.
    """

    def complex_sig(
        pos_only: object,
        /,
        regular: object,
        *args: object,
        kw_only: object,
        **kwargs: object,
    ) -> str:
        return str(pos_only) + str(regular) + str(kw_only)

    provider = Provide(dependency=complex_sig)

    assert "pos_only" in provider.depends_on
    assert "regular" in provider.depends_on
    assert "kw_only" in provider.depends_on
    assert "args" in provider.depends_on
    assert "kwargs" in provider.depends_on


def test_cacheable_parameter_precedence() -> None:
    """The cacheable parameter should override use_cache.

    When both cacheable and use_cache are provided, cacheable takes
    precedence. This should work alongside auto-detection.
    """

    def my_func(db: object) -> object:
        return db

    provider = Provide(dependency=my_func, use_cache=False, cacheable=True)

    assert provider.use_cache is True
    assert provider.depends_on == ["db"]


__all__ = [
    "test_auto_detect_args_and_kwargs",
    "test_auto_detect_complex_signature",
    "test_auto_detect_empty_for_no_params",
    "test_auto_detect_filters_all_standard_params",
    "test_auto_detect_filters_cls",
    "test_auto_detect_filters_request",
    "test_auto_detect_filters_response",
    "test_auto_detect_filters_self",
    "test_auto_detect_keyword_only",
    "test_auto_detect_lambda",
    "test_auto_detect_lambda_multiple",
    "test_auto_detect_mixed_filtered_and_real_deps",
    "test_auto_detect_multiple_parameters",
    "test_auto_detect_preserves_order",
    "test_auto_detect_single_parameter",
    "test_auto_detect_singleton_flag_preserved",
    "test_auto_detect_use_cache_flag_preserved",
    "test_auto_detect_with_annotations",
    "test_auto_detect_with_defaults",
    "test_auto_detect_with_kwargs",
    "test_auto_detect_with_positional_only",
    "test_cacheable_parameter_precedence",
    "test_explicit_depends_on_takes_precedence",
    "test_explicit_empty_depends_on",
    "test_provide_repr_with_auto_detected_deps",
]
