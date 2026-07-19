//! Python HTTP e2e test generation (server-pattern slice from alef core).
//!
//! This module owns the server-pattern files for spikard's Python e2e suite:
//! - `app_harness.py` — spawns the SUT as an HTTP server via the `PyO3` binding
//! - `conftest.py` (server variant) — pytest conftest that spawns `app_harness.py`
//!
//! The shared client-pattern conftest (mock-server, file-fixture variants) stays
//! generic in alef. Only the server-spawn slice lives here.
//!
//! Sources (alef `src/e2e/codegen/python/`):
//! - `config.rs::render_app_harness` (3-arg: `e2e_config`, groups, `crate_config`)
//! - `config.rs::render_conftest` (`uses_harness` branch only)
//! - `config.rs::build_middleware_value`
//! - `config.rs::render_env_setup_block`
#![allow(dead_code)]

use alef::ResolvedCrateConfig;
use alef::core::hash::{self, CommentStyle};
use alef::e2e::config::E2eConfig;
use alef::e2e::fixture::{FixtureGroup, HttpMiddleware};
use minijinja::{Environment, context};
use serde_json::json;

/// Build the private template environment holding the Python HTTP templates.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "python/app_harness.py.jinja".to_owned(),
        include_str!("../../templates/python/app_harness.py.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env
}

/// Render a named template from the local environment.
fn render(env: &Environment<'static>, name: &str, ctx: minijinja::Value) -> String {
    env.get_template(name)
        .expect("template must exist")
        .render(ctx)
        .unwrap_or_default()
}

/// Convert the fixture's `HttpMiddleware` into a `serde_json::Value` suitable
/// for embedding in the harness fixture JSON.
///
/// Field names are normalised to match each binding's `from_json()` contract:
/// CORS `allow_*` → `allowed_*` to match `CorsConfig.from_json()`.
fn build_middleware_value(middleware: Option<&HttpMiddleware>) -> serde_json::Value {
    let Some(mw) = middleware else {
        return serde_json::Value::Null;
    };

    let mut map = serde_json::Map::new();

    if let Some(cors) = &mw.cors {
        let mut cors_map = serde_json::Map::new();
        cors_map.insert("allowed_origins".to_string(), json!(cors.allow_origins));
        cors_map.insert("allowed_methods".to_string(), json!(cors.allow_methods));
        cors_map.insert("allowed_headers".to_string(), json!(cors.allow_headers));
        if !cors.expose_headers.is_empty() {
            cors_map.insert("expose_headers".to_string(), json!(cors.expose_headers));
        }
        if let Some(max_age) = cors.max_age {
            cors_map.insert("max_age".to_string(), json!(max_age));
        }
        if cors.allow_credentials {
            cors_map.insert("allow_credentials".to_string(), json!(true));
        }
        map.insert("cors".to_string(), serde_json::Value::Object(cors_map));
    }

    for (key, value) in [
        ("jwt_auth", &mw.jwt_auth),
        ("api_key_auth", &mw.api_key_auth),
        ("compression", &mw.compression),
        ("rate_limit", &mw.rate_limit),
        ("request_timeout", &mw.request_timeout),
        ("request_id", &mw.request_id),
    ] {
        if let Some(v) = value {
            map.insert(key.to_string(), v.clone());
        }
    }

    if map.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::Value::Object(map)
    }
}

/// Render the server-pattern `app_harness.py` that spawns the SUT HTTP server.
///
/// Ported verbatim from alef's `python/config.rs::render_app_harness` (3-arg form).
///
/// # Panics
///
/// Panics if the built-in Jinja template fails to parse (indicates a compile-time
/// template authoring error, not a runtime condition).
#[must_use]
pub fn render_app_harness(
    e2e_config: &E2eConfig,
    groups: &[FixtureGroup],
    crate_config: &ResolvedCrateConfig,
) -> String {
    let mut fixtures_map = serde_json::Map::new();

    for group in groups {
        for fixture in &group.fixtures {
            let Some(http_data) = fixture.http.as_ref() else {
                continue;
            };

            let middleware_value = build_middleware_value(http_data.handler.middleware.as_ref());

            let fixture_json = json!({
                "http": {
                    "handler": {
                        "route": &http_data.handler.route,
                        "method": &http_data.handler.method,
                        "body_schema": http_data.handler.body_schema.clone(),
                        "middleware": middleware_value,
                    },
                    "request": {
                        "path": &http_data.request.path,
                    },
                    "expected_response": {
                        "status_code": http_data.expected_response.status_code,
                        "body": &http_data.expected_response.body,
                        "headers": &http_data.expected_response.headers,
                    }
                }
            });
            fixtures_map.insert(fixture.id.clone(), fixture_json);
        }
    }

    let fixtures_json = serde_json::to_string(&fixtures_map).unwrap_or_default();

    let imports = &e2e_config.harness.imports;
    let app_class = e2e_config.harness.app_class_for_lang("python");
    let register_route_method = e2e_config
        .harness
        .register_method_idiomatic("python")
        .unwrap_or_else(|| "register_route".to_string());
    let body_schema_setter = &e2e_config.harness.body_schema_setter;
    let method_enum = &e2e_config.harness.method_enum;
    let run_method = e2e_config.harness.run_method_for_lang("python");
    let host = &e2e_config.harness.host;
    let port = e2e_config.harness.port;

    let header = hash::header(CommentStyle::Hash);

    let route_builder_import = if imports.is_empty() {
        "app._app".to_string()
    } else {
        let module_leaf = imports[0].rsplit('.').next().unwrap_or(&imports[0]).replace('-', "_");
        format!("{}._{}", &imports[0], module_leaf)
    };
    let method_enum_import = route_builder_import.clone();

    let skip_app_config = crate_config.exclude.methods.iter().any(|m| m == "App.config");

    let env = make_env();
    render(
        &env,
        "python/app_harness.py.jinja",
        context! {
            header => header,
            imports => imports,
            app_class => app_class.as_deref().unwrap_or("App"),
            route_builder_import => route_builder_import,
            route_builder_class => "RouteBuilder",
            route_builder_constructor => "__init__",
            route_builder_schema_setter => body_schema_setter.as_deref().unwrap_or("request_schema_json"),
            method_enum_import => method_enum_import,
            method_enum_class => method_enum.as_deref().unwrap_or("Method"),
            register_route_method => register_route_method.as_str(),
            run_method => run_method.as_deref().unwrap_or("run"),
            response_body_field => e2e_config.harness.response_body_field.as_str(),
            host => host,
            port => port,
            fixtures_json => fixtures_json,
            skip_app_config => skip_app_config,
        },
    )
}

/// Emit a Python snippet that copies every `[e2e.env]` entry into `os.environ`
/// using `setdefault`. Returns empty string when no env vars are configured.
fn render_env_setup_block(e2e_config: &E2eConfig) -> String {
    if e2e_config.env.is_empty() {
        return String::new();
    }
    let mut keys: Vec<&String> = e2e_config.env.keys().collect();
    keys.sort();
    let entries = keys
        .iter()
        .map(|k| format!("    {:?}: {:?},", k, &e2e_config.env[*k]))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "import os as _os\n\n_SUITE_ENV = {{\n{entries}\n}}\nfor _k, _v in _SUITE_ENV.items():\n    _os.environ.setdefault(_k, _v)\n\n"
    )
}

/// Resolve the Python module name from e2e config overrides or the call module.
fn resolve_module(e2e_config: &E2eConfig) -> String {
    e2e_config
        .call
        .overrides
        .get("python")
        .and_then(|o| o.module.clone())
        .unwrap_or_else(|| e2e_config.call.module.replace('-', "_"))
}

/// Render the server-pattern `conftest.py` that spawns `app_harness.py`.
///
/// This is the `uses_harness` branch of alef's `python/config.rs::render_conftest`,
/// extracted verbatim so the extension can emit it instead of alef.
#[must_use]
pub fn render_conftest_server(e2e_config: &E2eConfig) -> String {
    let module = resolve_module(e2e_config);
    let host = &e2e_config.harness.host;
    let header = hash::header(CommentStyle::Hash);
    let env_setup = render_env_setup_block(e2e_config);

    format!(
        r#"{header}"""Pytest configuration for e2e tests."""
from __future__ import annotations

import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Generator

import pytest

{env_setup}# Ensure the package is importable.
# The {module} package is expected to be installed in the current environment.

_HERE = Path(__file__).parent
_APP_HARNESS = _HERE / "app_harness.py"


@pytest.fixture(scope="session", autouse=True)
def sut_server() -> Generator[str, None, None]:
    """Spawn the app harness and set SUT_URL.

    If SUT_URL is already set, a parent process started a shared harness.
    Use it as-is and do NOT spawn our own.
    """
    import socket  # noqa: PLC0415

    existing = os.environ.get("SUT_URL")
    if existing:
        yield existing
        return

    # Allocate a free ephemeral port and hand it to the harness via
    # SPIKARD_SERVER_PORT (honored by the core server) so parallel suites and
    # leftover processes never collide on a fixed port.
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as _probe:
        _probe.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        _probe.bind(("{host}", 0))
        port = _probe.getsockname()[1]

    # Spawn the harness script as a subprocess bound to the allocated port.
    proc = subprocess.Popen(
        [sys.executable, str(_APP_HARNESS)],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        stdin=subprocess.PIPE,
        env={{**os.environ, "SPIKARD_SERVER_PORT": str(port)}},
    )

    url = f"http://{host}:{{port}}"
    # Poll until the harness actually accepts TCP connections. The harness
    # may print a listening banner before the runtime has finished binding,
    # so port availability is the authoritative readiness signal.
    deadline = time.time() + 15.0
    ready = False
    while time.time() < deadline:
        if proc.poll() is not None:
            # Process died early; surface stderr in the failure path.
            break
        try:
            with socket.create_connection(("{host}", port), timeout=0.5):
                ready = True
                break
        except OSError:
            time.sleep(0.1)

    if not ready:
        stderr_bytes = proc.stderr.read() if proc.stderr else b""
        proc.terminate()
        raise RuntimeError(
            f"App harness did not become reachable on {host}:{{port}} within 15s; "
            f"stderr={{stderr_bytes[:1000]!r}}"
        )

    os.environ["SUT_URL"] = url
    yield url

    # Cleanup
    if proc.stdin:
        proc.stdin.close()
    proc.terminate()
    proc.wait(timeout=5)


@pytest.fixture(scope="session")
def app(sut_server: str) -> object:
    """Return a simple HTTP helper bound to the SUT server URL."""

    class _App:
        def request(self, path: str, **kwargs: object) -> object:
            import urllib.request  # noqa: PLC0415
            method = str(kwargs.pop("method", "GET"))
            url = f"{{sut_server}}{{path}}"
            data = kwargs.pop("json", None)
            if data is not None:
                import json  # noqa: PLC0415
                body = json.dumps(data).encode()
                headers = dict(kwargs.pop("headers", {{}}))
                headers.setdefault("Content-Type", "application/json")
                req = urllib.request.Request(url, data=body, headers=headers, method=method.upper())
            else:
                headers = dict(kwargs.pop("headers", {{}}))
                req = urllib.request.Request(url, headers=headers, method=method.upper())
            try:
                with urllib.request.urlopen(req) as resp:  # noqa: S310
                    return resp
            except urllib.error.HTTPError as exc:
                return exc

    return _App()
"#
    )
}
