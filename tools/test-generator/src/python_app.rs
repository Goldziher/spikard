//! Python test app generator
//!
//! Generates a Spikard Python application from fixtures for e2e testing.
//!
//! Rotates through all Python type systems to ensure validation works with:
//! - Plain dict (fastest, no conversion)
//! - TypedDict (typed hints, no runtime conversion)
//! - dataclass (stdlib, mutable)
//! - NamedTuple (stdlib, immutable)
//! - msgspec.Struct (fastest typed)
//! - Pydantic BaseModel (popular, slower)

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::background::{BackgroundFixtureData, background_data};
use crate::middleware::{MiddlewareMetadata, parse_middleware, write_static_assets};
use crate::streaming::{StreamingFixtureData, chunk_bytes, streaming_data};
use anyhow::{Context, Result};
use serde_json::{Value, json};
use spikard_codegen::openapi::{Fixture, FixtureStreamChunk, load_fixtures_from_dir};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Convert JSON string to Python dict syntax
/// Replaces JSON literals (true, false, null) with Python equivalents (True, False, None)
fn json_to_python_dict(json_str: &str) -> String {
    json_str
        .replace(":true", ":True")
        .replace(":false", ":False")
        .replace(":null", ":None")
        .replace("[true", "[True")
        .replace("[false", "[False")
        .replace("[null", "[None")
        .replace(",true", ",True")
        .replace(",false", ",False")
        .replace(",null", ",None")
}

/// Type system to use for request body parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BodyType {
    /// Plain dict[str, Any] - no conversion needed (fastest)
    PlainDict,
    /// TypedDict - type hints only, no runtime conversion (fastest)
    TypedDict,
    /// @dataclass - stdlib mutable typed object
    Dataclass,
    /// NamedTuple - stdlib immutable typed tuple
    NamedTuple,
    /// msgspec.Struct - fastest typed conversion
    MsgspecStruct,
    /// Pydantic BaseModel - popular but slower
    Pydantic,
}

impl BodyType {
    /// Rotate through all type systems to ensure comprehensive testing
    fn for_index(index: usize) -> Self {
        match index % 6 {
            0 => BodyType::PlainDict,
            1 => BodyType::TypedDict,
            2 => BodyType::Dataclass,
            3 => BodyType::NamedTuple,
            4 => BodyType::MsgspecStruct,
            5 => BodyType::Pydantic,
            _ => unreachable!(),
        }
    }
}

/// Generate Python test application from fixtures
pub fn generate_python_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Generating Python test app...");

    let app_dir = output_dir.join("app");
    fs::create_dir_all(&app_dir).context("Failed to create app directory")?;
    let static_root = app_dir.join("static_assets");
    if static_root.exists() {
        fs::remove_dir_all(&static_root).with_context(|| format!("Failed to clear {}", static_root.display()))?;
    }
    fs::create_dir_all(&static_root).context("Failed to create static assets directory")?;

    let mut fixtures_by_category: HashMap<String, Vec<Fixture>> = HashMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let category = path.file_name().unwrap().to_str().unwrap().to_string();
            let fixtures = load_fixtures_from_dir(&path)?;

            if !fixtures.is_empty() {
                fixtures_by_category.insert(category, fixtures);
            }
        }
    }

    let sse_fixtures = load_sse_fixtures(fixtures_dir).context("Failed to load SSE fixtures")?;
    let websocket_fixtures = load_websocket_fixtures(fixtures_dir).context("Failed to load WebSocket fixtures")?;

    let app_content =
        generate_app_file_per_fixture(&fixtures_by_category, &app_dir, &sse_fixtures, &websocket_fixtures)?;
    fs::write(app_dir.join("main.py"), app_content).context("Failed to write main.py")?;

    fs::write(app_dir.join("__init__.py"), "\"\"\"E2E test application.\"\"\"\n")
        .context("Failed to write __init__.py")?;

    println!("  ✓ Generated app/main.py");
    Ok(())
}

/// Generate app file with per-fixture app factory functions (matches Rust pattern)
fn generate_app_file_per_fixture(
    fixtures_by_category: &HashMap<String, Vec<Fixture>>,
    app_dir: &Path,
    sse_fixtures: &[AsyncFixture],
    websocket_fixtures: &[AsyncFixture],
) -> Result<String> {
    let mut needs_background = false;
    let mut needs_static_assets = false;
    let mut needs_asyncio = false;

    for fixtures in fixtures_by_category.values() {
        for fixture in fixtures {
            if !needs_background && background_data(fixture)?.is_some() {
                needs_background = true;
            }
            let middleware_meta = parse_middleware(fixture)?;
            if !needs_static_assets && !middleware_meta.static_dirs.is_empty() {
                needs_static_assets = true;
            }
            if !needs_asyncio
                && middleware_meta
                    .request_timeout
                    .as_ref()
                    .and_then(|cfg| cfg.sleep_ms)
                    .is_some()
            {
                needs_asyncio = true;
            }
        }
    }

    let mut code = String::new();

    code.push_str("\"\"\"Generated E2E test application with per-fixture app factories.\"\"\"\n");
    code.push_str("# ruff: noqa: ARG001, A002\n");
    code.push_str("# mypy: ignore-errors\n");
    code.push('\n');
    code.push_str("from dataclasses import asdict, dataclass\n");
    code.push_str("from datetime import date, datetime\n");
    code.push_str("from enum import Enum\n");
    code.push_str("from typing import Any, Iterator, NamedTuple, TypedDict\n");
    code.push_str("from uuid import UUID\n\n");
    if needs_asyncio {
        code.push_str("import asyncio\n");
    }
    if needs_static_assets {
        code.push_str("from pathlib import Path\n");
    }
    code.push_str("import json\n");
    code.push_str("import msgspec\n");
    code.push_str("from pydantic import BaseModel\n\n");
    if needs_background {
        code.push_str("from collections import defaultdict\n");
    }
    if needs_static_assets {
        code.push_str("BASE_DIR = Path(__file__).parent\n\n");
    }
    let mut spikard_imports = vec![
        "Response",
        "Spikard",
        "StreamingResponse",
        "delete",
        "get",
        "head",
        "options",
        "patch",
        "post",
        "put",
    ];
    if needs_background {
        spikard_imports.push("background");
    }
    if !websocket_fixtures.is_empty() {
        spikard_imports.push("websocket");
    }
    if !sse_fixtures.is_empty() {
        spikard_imports.push("sse");
    }
    code.push_str(&format!("from spikard import {}\n", spikard_imports.join(", ")));
    code.push_str("from spikard.config import (\n");
    code.push_str("    ServerConfig, CompressionConfig, RateLimitConfig,\n");
    code.push_str("    JwtConfig, ApiKeyConfig, StaticFilesConfig,\n");
    code.push_str("    OpenApiConfig, ContactInfo, LicenseInfo, ServerInfo, SecuritySchemeInfo\n");
    code.push_str(")\n\n");
    if needs_background {
        code.push_str("BACKGROUND_STATE = defaultdict(list)\n\n");
    }

    let mut handler_names = HashMap::new();

    let mut all_app_factories = Vec::new();

    for (category, fixtures) in fixtures_by_category.iter() {
        for (index, fixture) in fixtures.iter().enumerate() {
            let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
            let handler_name = make_unique_name(&fixture_id, &mut handler_names);

            let body_type = BodyType::for_index(index);
            let background_info = background_data(fixture)?;
            let middleware_meta = parse_middleware(fixture)?;
            if !middleware_meta.static_dirs.is_empty() {
                write_static_assets(app_dir, &fixture_id, &middleware_meta.static_dirs)?;
            }

            let requires_plain_dict = background_info.is_some()
                || fixture
                    .handler
                    .as_ref()
                    .and_then(|handler| handler.body_schema.as_ref())
                    .and_then(|schema| schema.get("type").and_then(|v| v.as_str()))
                    .is_some_and(|schema_type| schema_type != "object" && schema_type != "array");
            let effective_body_type = if requires_plain_dict {
                BodyType::PlainDict
            } else {
                body_type
            };

            let (handler_code, app_factory_code) = generate_fixture_handler_and_app_python(
                fixture,
                &handler_name,
                &fixture_id,
                effective_body_type,
                &mut handler_names,
                background_info,
                &middleware_meta,
            )?;

            code.push_str(&handler_code);
            code.push_str("\n\n");
            code.push_str(&app_factory_code);
            code.push_str("\n\n");

            all_app_factories.push((
                category.clone(),
                fixture.name.clone(),
                format!("create_app_{}", handler_name),
            ));
        }
    }

    append_sse_factories(&mut code, sse_fixtures, &mut all_app_factories)?;
    append_websocket_factories(&mut code, websocket_fixtures, &mut all_app_factories)?;

    code.push_str("# App factory functions:\n");
    for (category, fixture_name, factory_fn) in all_app_factories {
        code.push_str(&format!("# - {}() for {} / {}\n", factory_fn, category, fixture_name));
    }

    Ok(code)
}

fn append_sse_factories(
    code: &mut String,
    fixtures: &[AsyncFixture],
    registry: &mut Vec<(String, String, String)>,
) -> Result<()> {
    use std::collections::BTreeMap;

    if fixtures.is_empty() {
        return Ok(());
    }

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref()
            && (fixture.operations.is_empty()
                || fixture
                    .operations
                    .iter()
                    .any(|op| op.action.eq_ignore_ascii_case("send")))
        {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    for (channel, channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let slug = sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"));
        let factory_name = format!("create_app_sse_{}", slug);
        let handler_name = format!("sse_handler_{}", slug);
        let events_literal = build_sse_events_literal(&channel_fixtures)?;

        code.push_str(&format!(
            r#"


def {factory_name}() -> Spikard:
    """SSE channel for {channel_path}"""
    app = Spikard()

    @sse("{channel_path}")
    async def {handler_name}():
        """SSE event stream for {channel_path}."""
        events = {events_literal}
        for event_data in events:
            yield json.loads(event_data)

    return app
"#,
            factory_name = factory_name,
            channel_path = channel_path,
            events_literal = events_literal,
            handler_name = handler_name
        ));

        registry.push(("sse".to_string(), channel_path.clone(), factory_name));
    }

    Ok(())
}

fn build_sse_events_literal(fixtures: &[&AsyncFixture]) -> Result<String> {
    let mut events = Vec::new();
    for fixture in fixtures {
        for example in &fixture.examples {
            let json_str = serde_json::to_string(example).context("Failed to serialize SSE example")?;
            events.push(format!("\"{}\"", escape_python_string(&json_str)));
        }
    }

    if events.is_empty() {
        events.push("\"{}\"".to_string());
    }

    Ok(format!("[{}]", events.join(", ")))
}

fn infer_message_type(fixture: &AsyncFixture) -> Option<String> {
    for example in &fixture.examples {
        if let Some(obj) = example.as_object()
            && let Some(Value::String(value)) = obj.get("type")
        {
            return Some(value.clone());
        } else if let Some(batch) = example.as_array() {
            for item in batch {
                if let Some(obj) = item.as_object()
                    && let Some(Value::String(value)) = obj.get("type")
                {
                    return Some(value.clone());
                }
            }
        }
    }
    None
}

fn first_example(fixture: &AsyncFixture) -> Option<Value> {
    for example in &fixture.examples {
        if example.is_null() {
            continue;
        }

        if let Some(batch) = example.as_array() {
            if let Some(first) = batch.first() {
                return Some(first.clone());
            }
        } else {
            return Some(example.clone());
        }
    }
    None
}

fn append_websocket_factories(
    code: &mut String,
    fixtures: &[AsyncFixture],
    registry: &mut Vec<(String, String, String)>,
) -> Result<()> {
    use std::collections::BTreeMap;

    if fixtures.is_empty() {
        return Ok(());
    }

    let mut fixture_lookup: HashMap<&str, &AsyncFixture> = HashMap::new();
    for fixture in fixtures {
        fixture_lookup.insert(fixture.name.as_str(), fixture);
    }

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    for (channel, channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let slug = sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"));
        let factory_name = format!("create_app_websocket_{}", slug);
        let handler_name = format!("websocket_handler_{}", slug);

        let mut match_arms = Vec::new();
        for fixture in &channel_fixtures {
            let has_receive_op = fixture
                .operations
                .iter()
                .any(|op| op.action.eq_ignore_ascii_case("receive"));
            if !has_receive_op {
                continue;
            }

            let match_value = infer_message_type(fixture).unwrap_or_else(|| fixture.name.clone());
            let mut reply_values: Vec<Value> = Vec::new();

            for op in fixture
                .operations
                .iter()
                .filter(|op| op.action.eq_ignore_ascii_case("receive"))
            {
                for reply_name in &op.replies {
                    if let Some(reply_fixture) = fixture_lookup.get(reply_name.as_str())
                        && let Some(example) = first_example(reply_fixture)
                    {
                        reply_values.push(example);
                    }
                }
            }

            let reply_literal = if reply_values.is_empty() {
                "            message[\"validated\"] = True\n            return message".to_string()
            } else if reply_values.len() == 1 {
                format!("            return {}", json_to_python(&reply_values[0]))
            } else {
                let joined = reply_values.iter().map(json_to_python).collect::<Vec<_>>().join(", ");
                format!("            return [{}]", joined)
            };

            match_arms.push(format!(
                "        if msg_type == {}:\n{}",
                json_to_python(&Value::String(match_value)),
                reply_literal
            ));
        }

        let match_block = if match_arms.is_empty() {
            "        message[\"validated\"] = True\n        return message".to_string()
        } else {
            format!(
                "{}\n        message[\"validated\"] = True\n        return message",
                match_arms.join("\n")
            )
        };

        code.push_str(&format!(
            r#"


def {factory_name}() -> Spikard:
    """WebSocket channel for {channel_path}"""
    app = Spikard()

    @websocket("{channel_path}")
    async def {handler_name}(message: dict) -> Any:
        """WebSocket handler for {channel_path} - generated from AsyncAPI fixtures."""
        msg_type = message.get("type")
{match_block}

    return app
"#,
            factory_name = factory_name,
            channel_path = channel_path,
            handler_name = handler_name,
            match_block = match_block
        ));

        registry.push(("websocket".to_string(), channel_path.clone(), factory_name));
    }

    Ok(())
}

/// Generate handler and app factory for a single fixture (Python version)
fn generate_fixture_handler_and_app_python(
    fixture: &Fixture,
    handler_name: &str,
    fixture_id: &str,
    body_type: BodyType,
    handler_names: &mut HashMap<String, usize>,
    background: Option<BackgroundFixtureData>,
    metadata: &MiddlewareMetadata,
) -> Result<(String, String)> {
    let route = if let Some(handler) = &fixture.handler {
        handler.route.clone()
    } else {
        fixture.request.path.clone()
    };
    let skip_route_registration = !metadata.static_dirs.is_empty();

    let route_path = route.split('?').next().unwrap_or(&route);
    let method = fixture
        .handler
        .as_ref()
        .map(|h| h.method.as_str())
        .unwrap_or_else(|| fixture.request.method.as_str());

    let (models_code, model_name) = if skip_route_registration {
        (String::new(), None)
    } else {
        generate_models_for_fixture_with_name(fixture, handler_name, body_type, handler_names)?
    };

    let handler_func = if skip_route_registration {
        String::new()
    } else {
        generate_handler_function_for_fixture(
            fixture,
            fixture_id,
            route_path,
            method,
            handler_name,
            body_type,
            model_name.as_deref(),
            background.as_ref(),
            metadata,
        )?
    };

    let hooks_functions = if skip_route_registration {
        String::new()
    } else if let Some(handler) = &fixture.handler {
        if let Some(middleware) = &handler.middleware {
            if let Some(hooks) = middleware.get("lifecycle_hooks") {
                generate_lifecycle_hooks_functions(hooks, handler_name, fixture)?
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let mut handler_code = String::new();
    if !models_code.is_empty() {
        handler_code.push_str(&models_code);
        handler_code.push_str("\n\n");
    }
    if !hooks_functions.is_empty() {
        handler_code.push_str(&hooks_functions);
        handler_code.push_str("\n\n");
    }
    handler_code.push_str(&handler_func);
    if let Some(bg) = &background {
        handler_code.push_str("\n\n");
        handler_code.push_str(&generate_background_state_handler_python(handler_name, fixture_id, bg));
    }

    let app_factory_name = format!("create_app_{}", handler_name);

    let body_schema_str = if let Some(handler) = &fixture.handler {
        if let Some(schema) = &handler.body_schema {
            let is_empty_schema = schema.get("type").and_then(|v| v.as_str()) == Some("object")
                && schema
                    .get("properties")
                    .and_then(|v| v.as_object())
                    .map(|props| props.is_empty())
                    .unwrap_or(true)
                && schema
                    .get("required")
                    .and_then(|v| v.as_array())
                    .map(|req| req.is_empty())
                    .unwrap_or(true);

            if is_empty_schema {
                "None".to_string()
            } else {
                let schema_json = serde_json::to_string(schema)?;
                json_to_python_dict(&schema_json)
            }
        } else {
            "None".to_string()
        }
    } else {
        "None".to_string()
    };

    let parameter_schema_str = if let Some(handler) = &fixture.handler {
        if let Some(params) = &handler.parameters {
            build_parameter_schema_with_sources(params)?
        } else {
            "None".to_string()
        }
    } else {
        "None".to_string()
    };

    let file_params_str = if let Some(handler) = &fixture.handler {
        if let Some(params) = &handler.parameters {
            extract_file_params(params).unwrap_or_else(|| "None".to_string())
        } else {
            "None".to_string()
        }
    } else {
        "None".to_string()
    };

    let method_upper = method.to_uppercase();
    let request_method_upper = fixture.request.method.to_uppercase();
    let primary_registration = if skip_route_registration {
        String::new()
    } else {
        format!(
            "    app.register_route(\"{}\", \"{}\", body_schema={}, parameter_schema={}, file_params={})({})",
            method_upper, route_path, body_schema_str, parameter_schema_str, file_params_str, handler_name
        )
    };
    let additional_registration = if !skip_route_registration
        && request_method_upper != method_upper
        && is_supported_python_http_method(&request_method_upper)
    {
        format!(
            "\n    app.register_route(\"{}\", \"{}\", body_schema={}, parameter_schema={}, file_params={})({})",
            request_method_upper,
            route_path,
            body_schema_str.clone(),
            parameter_schema_str.clone(),
            file_params_str.clone(),
            handler_name
        )
    } else {
        String::new()
    };

    let raw_middleware = fixture.handler.as_ref().and_then(|handler| handler.middleware.as_ref());
    let config_str = generate_server_config_from_metadata(metadata, raw_middleware, fixture_id)?;

    let hooks_code = if let Some(handler) = &fixture.handler {
        if let Some(middleware) = &handler.middleware {
            if let Some(hooks) = middleware.get("lifecycle_hooks") {
                generate_lifecycle_hooks_registration(hooks, handler_name)?
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let state_route_registration = if let Some(bg) = &background {
        format!(
            "\n    app.register_route(\"GET\", \"{}\", body_schema=None, parameter_schema=None, file_params=None)({}_background_state)",
            bg.state_path, handler_name
        )
    } else {
        String::new()
    };
    let mut route_registration = String::new();
    if !primary_registration.is_empty() {
        route_registration.push_str(&primary_registration);
    }
    if !additional_registration.is_empty() {
        route_registration.push_str(&additional_registration);
    }
    if !state_route_registration.is_empty() {
        route_registration.push_str(&state_route_registration);
    }
    let register_comment = if route_registration.trim().is_empty() {
        String::new()
    } else if skip_route_registration {
        "    # Static files served via ServerConfig\n".to_string()
    } else {
        "    # Register handler with this app instance\n".to_string()
    };
    let mut registration_block = String::new();
    if !route_registration.trim().is_empty() {
        registration_block.push_str(&register_comment);
        registration_block.push_str(&route_registration);
        if !route_registration.ends_with('\n') {
            registration_block.push('\n');
        }
    }

    let app_factory_code = if config_str == "None" && hooks_code.is_empty() {
        format!(
            r#"def {}() -> Spikard:
    """App factory for fixture: {}"""
    app = Spikard()
{}{}    return app"#,
            app_factory_name, fixture.name, registration_block, hooks_code
        )
    } else if config_str != "None" && hooks_code.is_empty() {
        format!(
            r#"def {}() -> Spikard:
    """App factory for fixture: {}"""
    config = {}
    app = Spikard(config=config)
{}{}    return app"#,
            app_factory_name, fixture.name, config_str, registration_block, hooks_code
        )
    } else if config_str == "None" && !hooks_code.is_empty() {
        format!(
            r#"def {}() -> Spikard:
    """App factory for fixture: {}"""
    app = Spikard()
{}{}    return app"#,
            app_factory_name, fixture.name, registration_block, hooks_code
        )
    } else {
        format!(
            r#"def {}() -> Spikard:
    """App factory for fixture: {}"""
    config = {}
    app = Spikard(config=config)
{}{}    return app"#,
            app_factory_name, fixture.name, config_str, registration_block, hooks_code
        )
    };

    Ok((handler_code, app_factory_code))
}

/// Generate just the models for a fixture (module-level) and return the model name
fn generate_models_for_fixture_with_name(
    fixture: &Fixture,
    handler_name: &str,
    body_type: BodyType,
    handler_names: &mut HashMap<String, usize>,
) -> Result<(String, Option<String>)> {
    let body_schema = if let Some(handler) = &fixture.handler {
        handler.body_schema.as_ref()
    } else {
        None
    };

    if let Some(schema) = body_schema {
        let model_name_base = format!("{}Body", to_pascal_case(handler_name));
        let model_name = make_unique_name(&model_name_base, handler_names);
        let model_code = extract_body_model(schema, &model_name, body_type)?;
        Ok((model_code, Some(model_name)))
    } else {
        Ok((String::new(), None))
    }
}

/// Generate handler function (without decorator, for manual registration)
#[allow(clippy::too_many_arguments)]
fn generate_handler_function_for_fixture(
    fixture: &Fixture,
    fixture_id: &str,
    route: &str,
    method: &str,
    handler_name: &str,
    body_type: BodyType,
    model_name: Option<&str>,
    background: Option<&BackgroundFixtureData>,
    metadata: &MiddlewareMetadata,
) -> Result<String> {
    let handler_opt = fixture.handler.as_ref();

    let params = if let Some(handler) = handler_opt {
        if let Some(ref param_schema) = handler.parameters {
            extract_parameters(param_schema)?
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let body_schema = if let Some(handler) = handler_opt {
        handler.body_schema.as_ref()
    } else {
        None
    };

    let expected_status = fixture.expected_response.status_code;
    let expected_body_value = fixture.expected_response.body.as_ref();
    let expected_body = expected_body_value.map(json_to_python);
    let expected_body_is_empty = expected_body_value.is_some_and(is_value_effectively_empty);

    let validation_errors_body = if let Some(errors) = fixture.expected_response.validation_errors.as_ref() {
        if errors.is_empty() {
            None
        } else {
            let errors_value = serde_json::to_value(errors)?;
            let body_value = json!({ "errors": errors_value });
            Some(json_to_python(&body_value))
        }
    } else {
        None
    };

    let mut code = String::new();

    let fn_prefix = "async def";
    code.push_str(&format!("{} {}(\n", fn_prefix, handler_name));

    if let Some(schema) = body_schema {
        let is_empty_schema = schema.get("type").and_then(|v| v.as_str()) == Some("object")
            && schema
                .get("properties")
                .and_then(|v| v.as_object())
                .map(|props| props.is_empty())
                .unwrap_or(true)
            && schema
                .get("required")
                .and_then(|v| v.as_array())
                .map(|req| req.is_empty())
                .unwrap_or(true);

        if !is_empty_schema {
            let body_param_type = match body_type {
                BodyType::PlainDict => json_type_to_python(schema).unwrap_or_else(|_| "dict[str, Any]".to_string()),
                _ => model_name.unwrap_or("dict[str, Any]").to_string(),
            };
            code.push_str(&format!("    body: {},\n", body_param_type));
        }
    }

    for (param_name, param_type, is_required) in &params {
        if *is_required {
            code.push_str(&format!("    {}: {},\n", param_name, param_type));
        }
    }
    for (param_name, param_type, is_required) in &params {
        if !*is_required {
            code.push_str(&format!("    {}: {} | None = None,\n", param_name, param_type));
        }
    }

    code.push_str(") -> Any:\n");
    let method_upper = method.to_uppercase();
    code.push_str(&format!("    \"\"\"Handler for {} {}.\"\"\"\n", method_upper, route));
    if let Some(sleep_ms) = metadata.request_timeout.as_ref().and_then(|cfg| cfg.sleep_ms) {
        let sleep_literal = format_sleep_seconds(sleep_ms);
        code.push_str(&format!("    await asyncio.sleep({})\n", sleep_literal));
    }

    let should_return_expected = expected_body.is_some() && !expected_body_is_empty;
    let should_return_validation_errors = validation_errors_body.is_some() && !should_return_expected;
    let mut headers_map = sanitized_expected_headers(fixture);

    if let Some(middleware) = fixture.handler.as_ref().and_then(|h| h.middleware.as_ref())
        && let Some(hooks) = middleware.get("lifecycle_hooks")
        && let Some(on_response) = hooks.get("on_response").and_then(|v| v.as_array())
    {
        for hook in on_response.iter() {
            if let Some(hook_name) = hook.get("name").and_then(|v| v.as_str()) {
                if hook_name.contains("security") {
                    headers_map.remove("X-Content-Type-Options");
                    headers_map.remove("X-Frame-Options");
                    headers_map.remove("X-XSS-Protection");
                    headers_map.remove("Strict-Transport-Security");
                } else if hook_name.contains("timing") || hook_name.contains("timer") {
                    headers_map.remove("X-Response-Time");
                }
            }
        }
    }

    let fixture_has_expected_headers = !headers_map.is_empty();
    let has_request_inputs = body_schema.is_some() || !params.is_empty();
    let handler_status = if metadata.rate_limit.is_some() {
        200
    } else {
        expected_status
    };

    let should_echo_params = handler_status == 200
        && !should_return_expected
        && !should_return_validation_errors
        && has_request_inputs
        && !fixture_has_expected_headers
        && !matches!(method_upper.as_str(), "HEAD" | "OPTIONS");

    let is_json_response = expected_body.is_some() && !expected_body_value.is_some_and(|v| v.is_string());
    if is_json_response && !headers_map.contains_key("Content-Type") && !headers_map.contains_key("content-type") {
        headers_map.insert("Content-Type".to_string(), "application/json".to_string());
    }

    let headers_param = if !headers_map.is_empty() {
        let headers_dict = headers_map
            .iter()
            .map(|(k, v)| format!("\"{}\" : \"{}\"", k, v))
            .collect::<Vec<_>>()
            .join(", ");
        format!(", headers={{{}}}", headers_dict)
    } else {
        String::new()
    };

    if let Some(stream_info) = streaming_data(fixture)? {
        code.push_str(&generate_streaming_handler_body_python(
            fixture,
            &stream_info,
            handler_status,
        )?);
        return Ok(code);
    }

    if let Some(bg) = background {
        code.push_str(&generate_background_handler_body_python(
            fixture_id,
            bg,
            handler_status,
            &headers_param,
        ));
        return Ok(code);
    }

    if should_return_expected {
        if let Some(body_literal) = expected_body.as_ref() {
            if expected_body_value.is_some_and(|v| v.is_string()) {
                if let Some(target_len) = content_length_from_headers(&headers_map) {
                    code.push_str(&format!("    content_value = {}\n", body_literal));
                    code.push_str("    content_bytes = content_value.encode(\"utf-8\")\n");
                    code.push_str(&format!(
                        "    if len(content_bytes) < {}:\n        padding = b\" \" * ({} - len(content_bytes))\n        content_bytes = content_bytes + padding\n",
                        target_len, target_len
                    ));
                    code.push_str("    def stream_content() -> Iterator[bytes]:\n");
                    code.push_str("        yield content_bytes\n");
                    code.push_str(&format!(
                        "    return StreamingResponse(stream_content(), status_code={}{})\n",
                        handler_status, headers_param
                    ));
                } else {
                    code.push_str(&format!(
                        "    return Response(content={}, status_code={}{})\n",
                        body_literal, handler_status, headers_param
                    ));
                }
            } else {
                code.push_str(&format!(
                    "    return Response(content={}, status_code={}{})\n",
                    body_literal, handler_status, headers_param
                ));
            }
        } else {
            code.push_str(&format!(
                "    return Response(status_code={}{})\n",
                handler_status, headers_param
            ));
        }
    } else if should_return_validation_errors {
        if let Some(body_json) = validation_errors_body.as_ref() {
            code.push_str(&format!(
                "    return Response(content={}, status_code={}{})\n",
                body_json, handler_status, headers_param
            ));
        } else {
            code.push_str(&format!(
                "    return Response(status_code={}{})\n",
                handler_status, headers_param
            ));
        }
    } else if should_echo_params {
        code.push_str("    # Echo back parameters for testing\n");
        code.push_str("    result: dict[str, Any] = {}\n");

        if body_schema.is_some() {
            code.push_str("    if body is not None:\n");
            match body_type {
                BodyType::PlainDict | BodyType::TypedDict => {
                    code.push_str("        result.update(body)\n");
                }
                BodyType::Dataclass => {
                    code.push_str("        result.update(asdict(body))\n");
                }
                BodyType::NamedTuple => {
                    code.push_str("        result.update(body._asdict())\n");
                }
                BodyType::MsgspecStruct => {
                    code.push_str("        result.update(msgspec.to_builtins(body))\n");
                }
                BodyType::Pydantic => {
                    code.push_str("        result.update(body.model_dump())\n");
                }
            }
        }

        for (param_name, param_type, _) in &params {
            code.push_str(&format!("    if {} is not None:\n", param_name));
            if param_type.contains("UUID") || param_type.contains("datetime") || param_type.contains("date") {
                code.push_str(&format!("        result[\"{}\"] = str({})\n", param_name, param_name));
            } else {
                code.push_str(&format!("        result[\"{}\"] = {}\n", param_name, param_name));
            }
        }

        code.push_str("    return result\n");
    } else {
        code.push_str(&format!(
            "    return Response(status_code={}{})\n",
            handler_status, headers_param
        ));
    }

    Ok(code)
}

fn generate_streaming_handler_body_python(
    fixture: &Fixture,
    stream_info: &StreamingFixtureData,
    handler_status: u16,
) -> Result<String> {
    let mut body = String::new();
    body.push_str("    async def stream_chunks():\n");
    for chunk in &stream_info.streaming.chunks {
        match chunk {
            FixtureStreamChunk::Text { value } => {
                let literal = json_to_python(&Value::String(value.clone()));
                body.push_str(&format!("        yield {}\n", literal));
            }
            FixtureStreamChunk::Bytes { .. } => {
                let bytes = chunk_bytes(chunk)?;
                let literal = python_bytes_literal(&bytes);
                body.push_str(&format!("        yield {}\n", literal));
            }
        }
    }
    body.push('\n');

    let headers_literal = build_streaming_headers_python(fixture, stream_info);
    body.push_str(&format!(
        "    return StreamingResponse(\n        stream_chunks(),\n        status_code={},\n        headers={}\n    )\n",
        handler_status, headers_literal
    ));
    Ok(body)
}

fn generate_background_handler_body_python(
    fixture_id: &str,
    background: &BackgroundFixtureData,
    handler_status: u16,
    headers_param: &str,
) -> String {
    let mut code = String::new();
    code.push_str(&format!(
        "    state = BACKGROUND_STATE.setdefault(\"{}\", [])\n",
        fixture_id
    ));
    code.push_str(&format!(
        "    value = body.get(\"{}\") if body is not None else None\n",
        background.value_field
    ));
    code.push_str("    if value is None:\n");
    code.push_str("        raise ValueError('background task requires request body value')\n");
    code.push_str("    async def _background_task() -> None:\n");
    code.push_str("        state.append(value)\n");
    code.push_str("    background.run(_background_task())\n");
    code.push_str(&format!(
        "    return Response(status_code={}{} )\n",
        handler_status, headers_param
    ));
    code
}

fn generate_background_state_handler_python(
    handler_name: &str,
    fixture_id: &str,
    background: &BackgroundFixtureData,
) -> String {
    format!(
        r#"def {handler_name}_background_state() -> Any:
    """Background state endpoint."""
    state = BACKGROUND_STATE.get("{fixture_id}", [])
    return {{"{state_key}": state}}
"#,
        handler_name = handler_name,
        fixture_id = fixture_id,
        state_key = background.state_key
    )
}

#[allow(clippy::manual_is_multiple_of)]
fn format_sleep_seconds(ms: u64) -> String {
    if ms % 1000 == 0 {
        return format!("{}", ms / 1000);
    }
    let secs = (ms as f64) / 1000.0;
    let mut literal = format!("{:.3}", secs);
    while literal.contains('.') && literal.ends_with('0') {
        literal.pop();
    }
    if literal.ends_with('.') {
        literal.push('0');
    }
    literal
}

fn build_streaming_headers_python(fixture: &Fixture, stream_info: &StreamingFixtureData) -> String {
    let mut headers = sanitized_expected_headers(fixture);

    if let Some(content_type) = stream_info.streaming.content_type.as_ref() {
        headers.insert("content-type".to_string(), content_type.clone());
    }

    if !headers.contains_key("content-type") {
        headers.insert("content-type".to_string(), "application/octet-stream".to_string());
    }

    if headers.is_empty() {
        "{}".to_string()
    } else {
        let mut map = serde_json::Map::new();
        for (key, value) in headers {
            map.insert(key, Value::String(value));
        }
        json_to_python(&Value::Object(map))
    }
}

fn python_bytes_literal(bytes: &[u8]) -> String {
    let mut literal = String::from("b\"");
    for &byte in bytes {
        match byte {
            b'\\' => literal.push_str("\\\\"),
            b'"' => literal.push_str("\\\""),
            b'\n' => literal.push_str("\\n"),
            b'\r' => literal.push_str("\\r"),
            b'\t' => literal.push_str("\\t"),
            0x20..=0x7e => literal.push(byte as char),
            _ => literal.push_str(&format!("\\x{:02x}", byte)),
        }
    }
    literal.push('"');
    literal
}

fn sanitized_expected_headers(fixture: &Fixture) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    if let Some(expected) = fixture.expected_response.headers.as_ref() {
        for (key, value) in expected {
            if key.eq_ignore_ascii_case("content-encoding") {
                continue;
            }
            if let Some(converted) = normalize_expected_header_value(value) {
                headers.insert(key.clone(), converted);
            }
        }
    }
    headers
}

fn content_length_from_headers(headers: &HashMap<String, String>) -> Option<usize> {
    headers
        .get("Content-Length")
        .or_else(|| headers.get("content-length"))
        .and_then(|value| value.parse::<usize>().ok())
}

fn normalize_expected_header_value(raw: &str) -> Option<String> {
    match raw {
        "<<absent>>" => None,
        "<<present>>" => Some("spikard-test-value".to_string()),
        "<<uuid>>" => Some("00000000-0000-4000-8000-000000000000".to_string()),
        _ => Some(raw.to_string()),
    }
}

/// Sanitize a string to be a valid Python identifier (lowercase snake_case)
fn sanitize_identifier(s: &str) -> String {
    let mut result = s
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '_', "_");

    while result.contains("__") {
        result = result.replace("__", "_");
    }

    result.trim_matches('_').to_string()
}

fn escape_python_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Generate CORS preflight handler
#[allow(dead_code)]
fn generate_cors_preflight_handler(handler_name: &str, route: &str, _cors_config: &Value) -> Result<String> {
    let mut code = String::new();

    code.push_str(&format!("@options(\"{}\")\n", route));
    code.push_str(&format!("def {}() -> dict[str, Any]:\n", handler_name));
    code.push_str("    \"\"\"CORS preflight handler.\"\"\"\n");
    code.push_str("    # CORS is handled by Spikard middleware\n");
    code.push_str("    return {}\n");

    Ok(code)
}

/// Extract parameters from parameter schema
fn extract_parameters(schema: &Value) -> Result<Vec<(String, String, bool)>> {
    let mut params = Vec::new();

    if let Some(obj) = schema.as_object() {
        if let Some(path_params) = obj.get("path").and_then(|v| v.as_object()) {
            for (name, param_schema) in path_params {
                let param_type = json_type_to_python(param_schema)?;
                let python_name = to_python_identifier(name);
                params.push((python_name, param_type, true));
            }
        }

        if let Some(query_params) = obj.get("query").and_then(|v| v.as_object()) {
            for (name, param_schema) in query_params {
                let param_type = json_type_to_python(param_schema)?;
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = !is_optional;
                let python_name = to_python_identifier(name);
                params.push((python_name, param_type, is_required));
            }
        }

        if let Some(headers) = obj.get("headers").and_then(|v| v.as_object()) {
            for (name, param_schema) in headers {
                let param_type = json_type_to_python(param_schema)?;
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = !is_optional;
                let python_name = to_python_identifier(name);
                params.push((python_name, param_type, is_required));
            }
        }

        if let Some(cookies) = obj.get("cookies").and_then(|v| v.as_object()) {
            for (name, param_schema) in cookies {
                let param_type = json_type_to_python(param_schema)?;
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = !is_optional;
                let python_name = to_python_identifier(name);
                params.push((python_name, param_type, is_required));
            }
        }
    }

    Ok(params)
}

/// Convert a name to a valid Python identifier
///
/// NOTE: We do NOT add underscore suffixes for Python builtins because:
/// - FFI passes parameters by their exact schema name (e.g., "filter", "id", "type")
/// - Python allows shadowing builtins in function parameters (only shadows within scope)
/// - Adding suffixes would cause FFI mismatch (Rust passes "filter", Python expects "filter_")
fn to_python_identifier(name: &str) -> String {
    name.replace(['-', '.'], "_").to_lowercase()
}

/// Make a name unique by adding a suffix if needed
fn make_unique_name(base_name: &str, used_names: &mut HashMap<String, usize>) -> String {
    let count = used_names.entry(base_name.to_string()).or_insert(0);
    *count += 1;

    if *count == 1 {
        base_name.to_string()
    } else {
        format!("{}_{}", base_name, *count - 1)
    }
}

/// Extract body model definition - generates code for different type systems
fn extract_body_model(schema: &Value, model_name: &str, body_type: BodyType) -> Result<String> {
    match body_type {
        BodyType::PlainDict => Ok(String::new()),
        BodyType::TypedDict => generate_typed_dict(schema, model_name),
        BodyType::Dataclass => generate_dataclass(schema, model_name),
        BodyType::NamedTuple => generate_namedtuple(schema, model_name),
        BodyType::MsgspecStruct => generate_msgspec_struct(schema, model_name),
        BodyType::Pydantic => generate_pydantic_model(schema, model_name),
    }
}

/// Generate TypedDict definition
fn generate_typed_dict(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str(&format!("class {}(TypedDict):\n", model_name));
    code.push_str("    \"\"\"Request body type (TypedDict - runtime is dict).\"\"\"\n\n");

    let Some(obj) = schema.as_object() else {
        return Ok(code);
    };
    let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) else {
        return Ok(code);
    };

    let required_fields: Vec<String> = obj
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let mut required_props: Vec<(&String, &Value)> = Vec::new();
    let mut optional_props: Vec<(&String, &Value)> = Vec::new();

    for (prop_name, prop_schema) in properties {
        if required_fields.contains(prop_name) {
            required_props.push((prop_name, prop_schema));
        } else {
            optional_props.push((prop_name, prop_schema));
        }
    }

    for (prop_name, prop_schema) in required_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
    }

    for (prop_name, prop_schema) in optional_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {} | None\n", python_prop_name, prop_type));
    }

    Ok(code)
}

/// Generate dataclass definition
fn generate_dataclass(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str("@dataclass\n");
    code.push_str(&format!("class {}:\n", model_name));
    code.push_str("    \"\"\"Request body dataclass.\"\"\"\n\n");

    let Some(obj) = schema.as_object() else {
        return Ok(code);
    };
    let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) else {
        return Ok(code);
    };

    let required_fields: Vec<String> = obj
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let mut required_props: Vec<(&String, &Value)> = Vec::new();
    let mut optional_props: Vec<(&String, &Value)> = Vec::new();

    for (prop_name, prop_schema) in properties {
        if required_fields.contains(prop_name) {
            required_props.push((prop_name, prop_schema));
        } else {
            optional_props.push((prop_name, prop_schema));
        }
    }

    for (prop_name, prop_schema) in required_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
    }

    for (prop_name, prop_schema) in optional_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
    }

    Ok(code)
}

/// Generate NamedTuple definition
fn generate_namedtuple(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str(&format!("class {}(NamedTuple):\n", model_name));
    code.push_str("    \"\"\"Request body NamedTuple (immutable).\"\"\"\n\n");

    let Some(obj) = schema.as_object() else {
        return Ok(code);
    };
    let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) else {
        return Ok(code);
    };

    let required_fields: Vec<String> = obj
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let mut required_props: Vec<(&String, &Value)> = Vec::new();
    let mut optional_props: Vec<(&String, &Value)> = Vec::new();

    for (prop_name, prop_schema) in properties {
        if required_fields.contains(prop_name) {
            required_props.push((prop_name, prop_schema));
        } else {
            optional_props.push((prop_name, prop_schema));
        }
    }

    for (prop_name, prop_schema) in required_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
    }

    for (prop_name, prop_schema) in optional_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
    }

    Ok(code)
}

/// Generate msgspec.Struct definition
fn generate_msgspec_struct(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str(&format!("class {}(msgspec.Struct):\n", model_name));
    code.push_str("    \"\"\"Request body msgspec.Struct (fast typed).\"\"\"\n\n");

    let Some(obj) = schema.as_object() else {
        return Ok(code);
    };
    let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) else {
        return Ok(code);
    };

    let required_fields: Vec<String> = obj
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let mut required_props: Vec<(&String, &Value)> = Vec::new();
    let mut optional_props: Vec<(&String, &Value)> = Vec::new();

    for (prop_name, prop_schema) in properties {
        if required_fields.contains(prop_name) {
            required_props.push((prop_name, prop_schema));
        } else {
            optional_props.push((prop_name, prop_schema));
        }
    }

    for (prop_name, prop_schema) in required_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
    }

    for (prop_name, prop_schema) in optional_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
    }

    Ok(code)
}

/// Generate Pydantic BaseModel definition
fn generate_pydantic_model(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str(&format!("class {}(BaseModel):\n", model_name));
    code.push_str("    \"\"\"Request body Pydantic model.\"\"\"\n\n");

    let Some(obj) = schema.as_object() else {
        return Ok(code);
    };
    let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) else {
        return Ok(code);
    };

    let required_fields: Vec<String> = obj
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let mut required_props: Vec<(&String, &Value)> = Vec::new();
    let mut optional_props: Vec<(&String, &Value)> = Vec::new();

    for (prop_name, prop_schema) in properties {
        if required_fields.contains(prop_name) {
            required_props.push((prop_name, prop_schema));
        } else {
            optional_props.push((prop_name, prop_schema));
        }
    }

    for (prop_name, prop_schema) in required_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
    }

    for (prop_name, prop_schema) in optional_props {
        let prop_type = json_type_to_python(prop_schema)?;
        let python_prop_name = to_python_identifier(prop_name);
        code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
    }

    Ok(code)
}

/// Convert JSON schema type to Python type annotation
fn json_type_to_python(schema: &Value) -> Result<String> {
    let schema_type = schema.get("type").and_then(|v| v.as_str()).unwrap_or("string");

    let type_str = match schema_type {
        "string" => {
            if let Some(format) = schema.get("format").and_then(|v| v.as_str()) {
                match format {
                    "uuid" => "UUID",
                    "date" => "date",
                    "date-time" => "datetime",
                    _ => "str",
                }
            } else {
                "str"
            }
        }
        "integer" => "int",
        "number" => "float",
        "boolean" => "bool",
        "array" => {
            if let Some(items) = schema.get("items") {
                let item_type = json_type_to_python(items)?;
                return Ok(format!("list[{}]", item_type));
            }
            "list[Any]"
        }
        "object" => "dict[str, Any]",
        _ => "Any",
    };

    Ok(type_str.to_string())
}

/// Extract file parameters from handler.parameters.files
fn extract_file_params(params: &Value) -> Option<String> {
    params
        .as_object()
        .and_then(|obj| obj.get("files"))
        .and_then(|files| serde_json::to_string(files).ok())
        .map(|files_json| json_to_python_dict(&files_json))
}

/// Build parameter schema with source fields for each parameter
fn build_parameter_schema_with_sources(params: &Value) -> Result<String> {
    use serde_json::json;

    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    if let Some(obj) = params.as_object() {
        if let Some(path_params) = obj.get("path").and_then(|v| v.as_object()) {
            for (name, param_schema) in path_params {
                let mut param_with_source = param_schema.clone();
                if let Some(param_obj) = param_with_source.as_object_mut() {
                    param_obj.insert("source".to_string(), json!("path"));
                    param_obj.remove("required");
                    param_obj.remove("optional");
                }
                let python_name = to_python_identifier(name);
                properties.insert(python_name.clone(), param_with_source);
                required.push(python_name);
            }
        }

        if let Some(query_params) = obj.get("query").and_then(|v| v.as_object()) {
            for (name, param_schema) in query_params {
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let mut param_with_source = param_schema.clone();
                if let Some(param_obj) = param_with_source.as_object_mut() {
                    param_obj.insert("source".to_string(), json!("query"));
                    param_obj.remove("required");
                    param_obj.remove("optional");
                }
                let python_name = to_python_identifier(name);
                properties.insert(python_name.clone(), param_with_source);
                if !is_optional {
                    required.push(python_name);
                }
            }
        }

        if let Some(headers) = obj.get("headers").and_then(|v| v.as_object()) {
            for (name, param_schema) in headers {
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let mut param_with_source = param_schema.clone();
                if let Some(param_obj) = param_with_source.as_object_mut() {
                    param_obj.insert("source".to_string(), json!("header"));
                    param_obj.remove("required");
                    param_obj.remove("optional");
                }
                let python_name = to_python_identifier(name);
                properties.insert(python_name.clone(), param_with_source);
                if !is_optional {
                    required.push(python_name);
                }
            }
        }

        if let Some(cookies) = obj.get("cookies").and_then(|v| v.as_object()) {
            for (name, param_schema) in cookies {
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = param_schema.get("required").and_then(|v| v.as_bool()).unwrap_or(true);
                let mut param_with_source = param_schema.clone();
                if let Some(param_obj) = param_with_source.as_object_mut() {
                    param_obj.insert("source".to_string(), json!("cookie"));
                    param_obj.remove("required");
                    param_obj.remove("optional");
                }
                let python_name = to_python_identifier(name);
                properties.insert(python_name.clone(), param_with_source);
                if !is_optional && is_required {
                    required.push(python_name);
                }
            }
        }
    }

    let schema = json!({
        "type": "object",
        "properties": properties,
        "required": required,
    });

    let schema_json = serde_json::to_string(&schema)?;
    Ok(json_to_python_dict(&schema_json))
}

fn is_value_effectively_empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Bool(_) | Value::Number(_) => false,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
    }
}

fn is_supported_python_http_method(method: &str) -> bool {
    matches!(method, "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "OPTIONS" | "HEAD")
}

/// Convert JSON value to Python dict literal
fn json_to_python(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
                .replace('\0', "\\0");
            format!("\"{}\"", escaped)
        }
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(json_to_python).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(obj) => {
            let items: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, json_to_python(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert snake_case or kebab-case to PascalCase
/// E.g., "post_cookies_samesite_strict" -> "PostCookiesSamesiteStrict"
fn to_pascal_case(s: &str) -> String {
    s.split(&['_', '-'][..])
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect()
}

/// Generate Python ServerConfig code from middleware metadata and raw middleware JSON
fn generate_server_config_from_metadata(
    metadata: &MiddlewareMetadata,
    raw_middleware: Option<&Value>,
    fixture_id: &str,
) -> Result<String> {
    let mut fields = Vec::new();

    if let Some(compression) = &metadata.compression {
        let mut args = Vec::new();
        if let Some(gzip) = compression.gzip {
            args.push(format!("gzip={}", if gzip { "True" } else { "False" }));
        }
        if let Some(brotli) = compression.brotli {
            args.push(format!("brotli={}", if brotli { "True" } else { "False" }));
        }
        if let Some(min_size) = compression.min_size {
            args.push(format!("min_size={}", min_size));
        }
        if let Some(quality) = compression.quality {
            args.push(format!("quality={}", quality));
        }
        if args.is_empty() {
            fields.push("        compression=CompressionConfig()\n".to_string());
        } else {
            fields.push(format!("        compression=CompressionConfig({})\n", args.join(", ")));
        }
    }

    if let Some(rate_limit) = &metadata.rate_limit {
        let mut rl_args = vec![
            format!("per_second={}", rate_limit.per_second),
            format!("burst={}", rate_limit.burst),
        ];
        if let Some(ip_based) = rate_limit.ip_based {
            rl_args.push(format!("ip_based={}", if ip_based { "True" } else { "False" }));
        }
        fields.push(format!("        rate_limit=RateLimitConfig({})\n", rl_args.join(", ")));
    }

    if let Some(timeout) = &metadata.request_timeout {
        fields.push(format!("        request_timeout={}\n", timeout.seconds));
    }

    if let Some(request_id) = &metadata.request_id
        && let Some(enabled) = request_id.enabled
    {
        fields.push(format!(
            "        enable_request_id={}\n",
            if enabled { "True" } else { "False" }
        ));
    }

    if let Some(body_limit) = &metadata.body_limit {
        match body_limit.max_bytes {
            Some(bytes) => fields.push(format!("        max_body_size={}\n", bytes)),
            None => fields.push("        max_body_size=None\n".to_string()),
        }
    }

    if !metadata.static_dirs.is_empty() {
        let mut entries = Vec::new();
        for dir in &metadata.static_dirs {
            let mut args = vec![format!(
                "directory=str(BASE_DIR / \"static_assets\" / \"{}\" / \"{}\")",
                fixture_id, dir.directory_name
            )];
            args.push(format!(
                "route_prefix={}",
                json_to_python(&Value::String(dir.route_prefix.clone()))
            ));
            if !dir.index_file {
                args.push("index_file=False".to_string());
            }
            if let Some(cache) = &dir.cache_control {
                args.push(format!(
                    "cache_control={}",
                    json_to_python(&Value::String(cache.clone()))
                ));
            }
            entries.push(format!("            StaticFilesConfig({})", args.join(", ")));
        }
        if !entries.is_empty() {
            fields.push("        static_files=[\n".to_string());
            for entry in entries {
                fields.push(format!("{}\n", entry));
            }
            fields.push("        ]\n".to_string());
        }
    }

    if let Some(middleware) = raw_middleware {
        if let Some(openapi) = middleware.get("openapi") {
            fields.push(build_openapi_config_block(openapi)?);
        }

        if let Some(jwt) = middleware.get("jwt_auth") {
            fields.push(build_jwt_config_block(jwt)?);
        }

        if let Some(api_key) = middleware.get("api_key_auth") {
            fields.push(build_api_key_config_block(api_key)?);
        }
    }

    if fields.is_empty() {
        return Ok("None".to_string());
    }

    let mut config = String::from("ServerConfig(\n");
    for field in fields {
        config.push_str(&field);
    }
    config.push_str("    )");
    Ok(config)
}

fn build_openapi_config_block(openapi: &Value) -> Result<String> {
    let mut block = String::from("        openapi=OpenApiConfig(\n");
    if let Some(enabled) = openapi.get("enabled") {
        block.push_str(&format!("            enabled={},\n", json_to_python(enabled)));
    }
    if let Some(title) = openapi.get("title") {
        block.push_str(&format!("            title={},\n", json_to_python(title)));
    }
    if let Some(version) = openapi.get("version") {
        block.push_str(&format!("            version={},\n", json_to_python(version)));
    }
    if let Some(description) = openapi.get("description") {
        block.push_str(&format!("            description={},\n", json_to_python(description)));
    }
    if let Some(swagger_ui_path) = openapi.get("swagger_ui_path") {
        block.push_str(&format!(
            "            swagger_ui_path={},\n",
            json_to_python(swagger_ui_path)
        ));
    }
    if let Some(redoc_path) = openapi.get("redoc_path") {
        block.push_str(&format!("            redoc_path={},\n", json_to_python(redoc_path)));
    }
    if let Some(openapi_json_path) = openapi.get("openapi_json_path") {
        block.push_str(&format!(
            "            openapi_json_path={},\n",
            json_to_python(openapi_json_path)
        ));
    }
    if let Some(contact) = openapi.get("contact") {
        block.push_str("            contact=ContactInfo(\n");
        if let Some(name) = contact.get("name") {
            block.push_str(&format!("                name={},\n", json_to_python(name)));
        }
        if let Some(email) = contact.get("email") {
            block.push_str(&format!("                email={},\n", json_to_python(email)));
        }
        if let Some(url) = contact.get("url") {
            block.push_str(&format!("                url={},\n", json_to_python(url)));
        }
        block.push_str("            ),\n");
    }
    if let Some(license) = openapi.get("license") {
        block.push_str("            license=LicenseInfo(\n");
        if let Some(name) = license.get("name") {
            block.push_str(&format!("                name={},\n", json_to_python(name)));
        }
        if let Some(url) = license.get("url") {
            block.push_str(&format!("                url={},\n", json_to_python(url)));
        }
        block.push_str("            ),\n");
    }
    if let Some(servers) = openapi.get("servers").and_then(|v| v.as_array()) {
        block.push_str("            servers=[\n");
        for server in servers {
            block.push_str("                ServerInfo(\n");
            if let Some(url) = server.get("url") {
                block.push_str(&format!("                    url={},\n", json_to_python(url)));
            }
            if let Some(description) = server.get("description") {
                block.push_str(&format!(
                    "                    description={},\n",
                    json_to_python(description)
                ));
            }
            block.push_str("                ),\n");
        }
        block.push_str("            ],\n");
    }
    block.push_str("        ),\n");
    Ok(block)
}

fn build_jwt_config_block(jwt: &Value) -> Result<String> {
    let mut block = String::from("        jwt_auth=JwtConfig(\n");
    if let Some(secret) = jwt.get("secret") {
        block.push_str(&format!("            secret={},\n", json_to_python(secret)));
    }
    if let Some(algorithm) = jwt.get("algorithm") {
        block.push_str(&format!("            algorithm={},\n", json_to_python(algorithm)));
    }
    if let Some(audience) = jwt.get("audience") {
        block.push_str(&format!("            audience={},\n", json_to_python(audience)));
    }
    if let Some(issuer) = jwt.get("issuer") {
        block.push_str(&format!("            issuer={},\n", json_to_python(issuer)));
    }
    if let Some(leeway) = jwt.get("leeway") {
        block.push_str(&format!("            leeway={},\n", json_to_python(leeway)));
    }
    block.push_str("        ),\n");
    Ok(block)
}

fn build_api_key_config_block(api_key: &Value) -> Result<String> {
    let mut block = String::from("        api_key_auth=ApiKeyConfig(\n");
    if let Some(keys) = api_key.get("keys") {
        block.push_str(&format!("            keys={},\n", json_to_python(keys)));
    }
    if let Some(header_name) = api_key.get("header_name") {
        block.push_str(&format!("            header_name={},\n", json_to_python(header_name)));
    }
    block.push_str("        ),\n");
    Ok(block)
}

/// Generate Python lifecycle hooks registration code
fn generate_lifecycle_hooks_registration(hooks: &Value, handler_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str("    # Register lifecycle hooks\n");

    if let Some(on_request) = hooks.get("on_request").and_then(|v| v.as_array()) {
        for (idx, hook) in on_request.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_request_{}", handler_name, hook_name, idx);
            code.push_str(&format!("    app.on_request({})\n", func_name));
        }
    }

    if let Some(pre_validation) = hooks.get("pre_validation").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_validation.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_pre_validation_{}", handler_name, hook_name, idx);
            code.push_str(&format!("    app.pre_validation({})\n", func_name));
        }
    }

    if let Some(pre_handler) = hooks.get("pre_handler").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_handler.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_pre_handler_{}", handler_name, hook_name, idx);
            code.push_str(&format!("    app.pre_handler({})\n", func_name));
        }
    }

    if let Some(on_response) = hooks.get("on_response").and_then(|v| v.as_array()) {
        for (idx, hook) in on_response.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_response_{}", handler_name, hook_name, idx);
            code.push_str(&format!("    app.on_response({})\n", func_name));
        }
    }

    if let Some(on_error) = hooks.get("on_error").and_then(|v| v.as_array()) {
        for (idx, hook) in on_error.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_error_{}", handler_name, hook_name, idx);
            code.push_str(&format!("    app.on_error({})\n", func_name));
        }
    }

    Ok(code)
}

/// Generate Python lifecycle hook function implementations
fn generate_lifecycle_hooks_functions(hooks: &Value, handler_name: &str, fixture: &Fixture) -> Result<String> {
    let mut code = String::new();

    if let Some(on_request) = hooks.get("on_request").and_then(|v| v.as_array()) {
        for (idx, hook) in on_request.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_request_{}", handler_name, hook_name, idx);

            code.push_str(&format!(
                r#"async def {}(request: Any) -> Any:
    """onRequest hook: {}"""
    # Mock implementation for testing
    return request


"#,
                func_name, hook_name
            ));
        }
    }

    if let Some(pre_validation) = hooks.get("pre_validation").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_validation.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_pre_validation_{}", handler_name, hook_name, idx);

            let should_short_circuit = hook_name.contains("rate_limit") && fixture.expected_response.status_code == 429;

            if should_short_circuit {
                code.push_str(&format!(
                    r#"async def {}(request: Any) -> Any:
    """preValidation hook: {} - Short circuits with 429"""
    from spikard import Response
    return Response(
        content={{"error": "Rate limit exceeded", "message": "Too many requests, please try again later"}},
        status_code=429,
        headers={{"Retry-After": "60"}}
    )


"#,
                    func_name, hook_name
                ));
            } else {
                code.push_str(&format!(
                    r#"async def {}(request: Any) -> Any:
    """preValidation hook: {}"""
    # Mock implementation for testing
    return request


"#,
                    func_name, hook_name
                ));
            }
        }
    }

    if let Some(pre_handler) = hooks.get("pre_handler").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_handler.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_pre_handler_{}", handler_name, hook_name, idx);

            let auth_fails = hook_name.contains("auth")
                && (fixture.expected_response.status_code == 401 || fixture.expected_response.status_code == 403);

            if auth_fails {
                let error_msg = if fixture.expected_response.status_code == 401 {
                    "Unauthorized"
                } else {
                    "Forbidden"
                };
                let detail_msg = if fixture.expected_response.status_code == 401 {
                    "Invalid or expired authentication token"
                } else {
                    "Admin role required for this endpoint"
                };

                code.push_str(&format!(
                    r#"async def {}(request: Any) -> Any:
    """preHandler hook: {} - Short circuits with {}"""
    from spikard import Response
    return Response(
        content={{"error": "{}", "message": "{}"}},
        status_code={}
    )


"#,
                    func_name,
                    hook_name,
                    fixture.expected_response.status_code,
                    error_msg,
                    detail_msg,
                    fixture.expected_response.status_code
                ));
            } else {
                code.push_str(&format!(
                    r#"async def {}(request: Any) -> Any:
    """preHandler hook: {}"""
    # Mock implementation for testing
    return request


"#,
                    func_name, hook_name
                ));
            }
        }
    }

    if let Some(on_response) = hooks.get("on_response").and_then(|v| v.as_array()) {
        for (idx, hook) in on_response.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_response_{}", handler_name, hook_name, idx);

            if hook_name.contains("security") {
                code.push_str(&format!(
                    r#"async def {}(response: Any) -> Any:
    """onResponse hook: {} - Adds security headers"""
    if hasattr(response, 'headers'):
        response.headers["X-Content-Type-Options"] = "nosniff"
        response.headers["X-Frame-Options"] = "DENY"
        response.headers["X-XSS-Protection"] = "1; mode=block"
        response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains"
    return response


"#,
                    func_name, hook_name
                ));
            } else if hook_name.contains("timing") || hook_name.contains("timer") {
                code.push_str(&format!(
                    r#"async def {}(response: Any) -> Any:
    """onResponse hook: {} - Adds timing header"""
    if hasattr(response, 'headers'):
        response.headers["X-Response-Time"] = "0ms"
    return response


"#,
                    func_name, hook_name
                ));
            } else {
                code.push_str(&format!(
                    r#"async def {}(response: Any) -> Any:
    """onResponse hook: {}"""
    # Mock implementation for testing
    return response


"#,
                    func_name, hook_name
                ));
            }
        }
    }

    if let Some(on_error) = hooks.get("on_error").and_then(|v| v.as_array()) {
        for (idx, hook) in on_error.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_error_{}", handler_name, hook_name, idx);

            code.push_str(&format!(
                r#"async def {}(response: Any) -> Any:
    """onError hook: {}"""
    # Mock implementation for testing - format error response
    if hasattr(response, 'headers'):
        response.headers["Content-Type"] = "application/json"
    return response


"#,
                func_name, hook_name
            ));
        }
    }

    Ok(code)
}
