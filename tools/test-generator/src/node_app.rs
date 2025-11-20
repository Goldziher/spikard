//! Node.js test app generator
//!
//! Generates a Spikard Node.js/TypeScript application from fixtures for e2e testing.

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::background::{BackgroundFixtureData, background_data};
use crate::middleware::{MiddlewareMetadata, parse_middleware, write_static_assets};
use crate::streaming::{StreamingFixtureData, streaming_data};
use crate::ts_target::TypeScriptTarget;
use anyhow::{Context, Result, ensure};
use serde_json::{Map as JsonMap, Value};
use spikard_cli::codegen::ts_schema::{TypeScriptDto, generate_typescript_dto, json_value_to_ts_literal};
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

const MAX_SAFE_INTEGER: i128 = 9007199254740991; 

/// Generate Node.js test application from fixtures
pub fn generate_node_app(fixtures_dir: &Path, output_dir: &Path, target: &TypeScriptTarget) -> Result<()> {
    println!("Generating Node.js test app...");

    let app_dir = output_dir.join("app");
    fs::create_dir_all(&app_dir).context("Failed to create app directory")?;
    fs::create_dir_all(app_dir.join("static_assets")).context("Failed to create static assets directory")?;

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

    let mut dto_map: HashMap<String, TypeScriptDto> = HashMap::new();
    for fixture in sse_fixtures.iter().chain(websocket_fixtures.iter()) {
        if dto_map.contains_key(&fixture.name) {
            continue;
        }
        let dto = generate_typescript_dto(&fixture.name, &fixture.schema)
            .with_context(|| format!("Failed to build DTO for {}", fixture.name))?;
        dto_map.insert(fixture.name.clone(), dto);
    }

    let app_content = generate_app_file_per_fixture(
        &fixtures_by_category,
        &app_dir,
        &sse_fixtures,
        &websocket_fixtures,
        &dto_map,
        target,
    )?;
    fs::write(app_dir.join("main.ts"), app_content).context("Failed to write main.ts")?;

    let package_json = generate_package_json(target);
    fs::write(output_dir.join("package.json"), package_json).context("Failed to write package.json")?;

    let tsconfig = generate_tsconfig();
    fs::write(output_dir.join("tsconfig.json"), tsconfig).context("Failed to write tsconfig.json")?;

    let vitest_config = generate_vitest_config();
    fs::write(output_dir.join("vitest.config.ts"), vitest_config).context("Failed to write vitest.config.ts")?;

    println!("  ✓ Generated app/main.ts");
    println!("  ✓ Generated package.json");
    println!("  ✓ Generated tsconfig.json");
    println!("  ✓ Generated vitest.config.ts");

    format_generated_ts(output_dir)?;
    Ok(())
}

/// Generate package.json for the e2e Node.js project
fn generate_package_json(target: &TypeScriptTarget) -> String {
    format!(
        r#"{{
	"name": "{name}",
	"version": "0.1.0",
	"private": true,
	"type": "module",
	"scripts": {{
		"test": "vitest run",
		"test:watch": "vitest"
	}},
	"devDependencies": {{
		"{dependency}": "workspace:*",
        "@types/node": "^24.9.2",
		"@vitest/coverage-v8": "^4.0.6",
		"typescript": "^5.9.3",
		"vitest": "^4.0.6"
	}}
}}
"#,
        name = target.e2e_package_name,
        dependency = target.dependency_package
    )
}

/// Generate tsconfig.json for TypeScript compilation
fn generate_tsconfig() -> String {
    r#"{
	"compilerOptions": {
		"target": "ES2022",
		"module": "ES2022",
		"lib": ["ES2022"],
		"moduleResolution": "bundler",
		"strict": true,
		"esModuleInterop": true,
		"skipLibCheck": true,
		"forceConsistentCasingInFileNames": true,
		"resolveJsonModule": true,
		"types": ["vitest/globals", "node"]
	},
	"include": ["app/**/*", "tests/**/*"]
}
"#
    .to_string()
}

fn format_generated_ts(dir: &Path) -> Result<()> {
    let status = Command::new("pnpm")
        .current_dir(dir)
        .args(["biome", "check", "--write", "--unsafe", "."])
        .status()
        .context("Failed to run `pnpm biome check --write .` in e2e/node app")?;
    ensure!(
        status.success(),
        "`pnpm biome check --write .` exited with non-zero status"
    );

    Ok(())
}

/// Generate vitest.config.ts for test configuration
fn generate_vitest_config() -> String {
    r#"import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		globals: true,
		environment: "node",
	},
});
"#
    .to_string()
}

/// Generate app file with per-fixture app factory functions
fn generate_app_file_per_fixture(
    fixtures_by_category: &HashMap<String, Vec<Fixture>>,
    app_dir: &Path,
    sse_fixtures: &[AsyncFixture],
    websocket_fixtures: &[AsyncFixture],
    dto_map: &HashMap<String, TypeScriptDto>,
    target: &TypeScriptTarget,
) -> Result<String> {
    let mut needs_background = false;
    let mut needs_static_assets = false;
    let mut needs_server_config_import = false;
    let has_sse = !sse_fixtures.is_empty();
    let mut padded_binary_bodies = false;
    let mut streaming_has_binary_chunks = false;
    let has_websocket = !websocket_fixtures.is_empty();
    for fixtures in fixtures_by_category.values() {
        for fixture in fixtures {
            if !needs_background && background_data(fixture)?.is_some() {
                needs_background = true;
            }
            let metadata = parse_middleware(fixture)?;
            if !needs_static_assets && !metadata.static_dirs.is_empty() {
                needs_static_assets = true;
            }
            if !needs_server_config_import
                && (metadata_requires_server_config(&metadata) || handler_requires_server_config(fixture))
            {
                needs_server_config_import = true;
            }
            if !padded_binary_bodies && fixture_requires_binary_body(fixture) {
                padded_binary_bodies = true;
            }
            if !streaming_has_binary_chunks && streaming_data(fixture)?.map(|info| !info.is_text_only).unwrap_or(false)
            {
                streaming_has_binary_chunks = true;
            }
        }
    }

    let mut code = String::new();

    code.push_str("/**\n");
    code.push_str(" * Generated E2E test application with per-fixture app factories.\n");
    code.push_str(" * @generated\n");
    code.push_str(" */\n\n");

    let mut needs_streaming_import = fixtures_by_category
        .values()
        .flat_map(|fixtures| fixtures.iter())
        .any(|fixture| fixture.streaming.is_some());
    if has_sse {
        needs_streaming_import = true;
    }
    if !needs_streaming_import && padded_binary_bodies {
        needs_streaming_import = true;
    }
    let mut value_imports = Vec::new();
    if needs_streaming_import {
        value_imports.push("StreamingResponse");
    }
    if needs_background {
        value_imports.push("background");
    }
    if !value_imports.is_empty() {
        code.push_str(&format!(
            "import {{ {} }} from \"{}\";\n",
            value_imports.join(", "),
            target.binding_package
        ));
    }

    let mut type_imports = vec!["RouteMetadata".to_string(), "SpikardApp".to_string()];
    if needs_server_config_import {
        type_imports.push("ServerConfig".to_string());
    }

    code.push_str("import type {\n");
    for name in type_imports {
        code.push_str(&format!("\t{},\n", name));
    }
    code.push_str(&format!("}} from \"{}\";\n", target.binding_package));
    let needs_buffer_import = has_websocket || padded_binary_bodies || streaming_has_binary_chunks;
    if needs_buffer_import {
        code.push_str("import { Buffer } from \"node:buffer\";\n");
    }
    code.push_str("import { z } from \"zod\";\n\n");

    code.push_str("type HandlerResponse = {\n");
    code.push_str("\tstatus: number;\n");
    code.push_str("\theaders?: Record<string, string>;\n");
    code.push_str("\tbody?: unknown;\n");
    code.push_str("};\n");
    code.push_str("type HookRequest = {\n");
    code.push_str("\tbody?: unknown;\n");
    code.push_str("\theaders?: Record<string, string>;\n");
    code.push_str("\tparams?: Record<string, unknown>;\n");
    code.push_str("\t[key: string]: unknown;\n");
    code.push_str("};\n");
    code.push_str("type HookResponse = {\n");
    code.push_str("\tstatusCode?: number;\n");
    code.push_str("\tbody?: unknown;\n");
    code.push_str("\theaders?: Record<string, string>;\n");
    code.push_str("\t[key: string]: unknown;\n");
    code.push_str("};\n");
    code.push_str("type HookResult = HookRequest | HookResponse;\n\n");

    if has_websocket {
        code.push_str("function normalizeWebsocketPayload(message: unknown): unknown {\n");
        code.push_str("\tif (Array.isArray(message)) {\n");
        code.push_str("\t\tif (message.length === 1) {\n");
        code.push_str("\t\t\treturn normalizeWebsocketPayload(message[0]);\n");
        code.push_str("\t\t}\n");
        code.push_str("\t\treturn message.map((entry) => normalizeWebsocketPayload(entry));\n");
        code.push_str("\t}\n");
        code.push_str("\tif (typeof message === \"string\") {\n");
        code.push_str("\t\ttry {\n");
        code.push_str("\t\t\treturn JSON.parse(message);\n");
        code.push_str("\t\t} catch {\n");
        code.push_str("\t\t\treturn message;\n");
        code.push_str("\t\t}\n");
        code.push_str("\t}\n");
        code.push_str("\tif (typeof Buffer !== \"undefined\" && Buffer.isBuffer(message)) {\n");
        code.push_str("\t\treturn JSON.parse(message.toString(\"utf-8\"));\n");
        code.push_str("\t}\n");
        code.push_str("\tif (message instanceof ArrayBuffer) {\n");
        code.push_str("\t\treturn JSON.parse(Buffer.from(message).toString(\"utf-8\"));\n");
        code.push_str("\t}\n");
        code.push_str("\tif (message && typeof message === \"object\" && ArrayBuffer.isView(message)) {\n");
        code.push_str("\t\tconst view = message as ArrayBufferView;\n");
        code.push_str("\t\tconst buffer = Buffer.from(view.buffer, view.byteOffset, view.byteLength);\n");
        code.push_str("\t\treturn JSON.parse(buffer.toString(\"utf-8\"));\n");
        code.push_str("\t}\n");
        code.push_str("\treturn message;\n");
        code.push_str("}\n\n");
    }

    for dto in dto_map.values() {
        code.push_str(&dto.schema_declaration);
        code.push('\n');
        code.push_str(&dto.type_declaration);
        code.push('\n');
    }

    if needs_background {
        code.push_str("const BACKGROUND_STATE: Record<string, unknown[]> = {};\n\n");
    }

    let mut handler_names = HashMap::new();

    let mut all_app_factories = Vec::new();

    for (category, fixtures) in fixtures_by_category.iter() {
        for fixture in fixtures.iter() {
            let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
            let handler_name = make_unique_name(&fixture_id, &mut handler_names);
            let metadata = parse_middleware(fixture)?;
            if !metadata.static_dirs.is_empty() {
                write_static_assets(app_dir, &fixture_id, &metadata.static_dirs)?;
            }

            let background_info = background_data(fixture)?;
            let (handler_code, app_factory_code) =
                generate_fixture_handler_and_app_node(fixture, &handler_name, &fixture_id, background_info, &metadata)?;

            code.push_str(&handler_code);
            code.push_str("\n\n");
            code.push_str(&app_factory_code);
            code.push_str("\n\n");

            all_app_factories.push((
                category.clone(),
                fixture.name.clone(),
                format!("createApp{}", to_pascal_case(&handler_name)),
            ));
        }
    }

    append_sse_factories(
        &mut code,
        sse_fixtures,
        &mut all_app_factories,
        &mut handler_names,
        dto_map,
    )?;
    append_websocket_factories(
        &mut code,
        websocket_fixtures,
        &mut all_app_factories,
        &mut handler_names,
        dto_map,
    )?;

    code.push_str("// App factory functions:\n");
    for (category, fixture_name, factory_fn) in &all_app_factories {
        code.push_str(&format!("// - {}() for {} / {}\n", factory_fn, category, fixture_name));
    }
    code.push('\n');
    code.push_str("export {\n");
    for dto in dto_map.values() {
        code.push_str(&format!("\t{},\n", dto.schema_ident));
    }
    code.push_str("};\n");
    code.push_str("export type {\n");
    for dto in dto_map.values() {
        code.push_str(&format!("\t{},\n", dto.type_ident));
    }
    code.push_str("};\n");

    Ok(code)
}

/// Generate handler and app factory for a single fixture (Node.js version)
fn generate_fixture_handler_and_app_node(
    fixture: &Fixture,
    handler_name: &str,
    fixture_id: &str,
    background: Option<BackgroundFixtureData>,
    metadata: &MiddlewareMetadata,
) -> Result<(String, String)> {
    let route = if let Some(handler) = &fixture.handler {
        handler.route.clone()
    } else {
        fixture.request.path.clone()
    };

    let route_path = route.split('?').next().unwrap_or(&route);
    let method = fixture.request.method.as_str();
    let skip_route_registration = !metadata.static_dirs.is_empty();

    let handler_func = if skip_route_registration {
        String::new()
    } else {
        generate_handler_function(
            fixture,
            route_path,
            method,
            handler_name,
            fixture_id,
            background.as_ref(),
            metadata,
        )?
    };

    let background_state_handler = if skip_route_registration {
        None
    } else {
        background
            .as_ref()
            .map(|bg| generate_background_state_handler(handler_name, fixture_id, bg))
    };

    let hooks_code = if skip_route_registration {
        String::new()
    } else if let Some(handler) = &fixture.handler {
        if let Some(middleware) = &handler.middleware {
            if let Some(hooks) = middleware.get("lifecycle_hooks") {
                generate_lifecycle_hooks_ts(handler_name, hooks, fixture)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let hooks_registration = if skip_route_registration {
        String::new()
    } else if let Some(handler) = &fixture.handler {
        if let Some(middleware) = &handler.middleware {
            if let Some(hooks) = middleware.get("lifecycle_hooks") {
                generate_lifecycle_hooks_registration_ts(handler_name, hooks)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let app_factory_name = format!("createApp{}", to_pascal_case(handler_name));

    let body_schema_str = if let Some(handler) = &fixture.handler {
        if let Some(schema) = &handler.body_schema {
            serde_json::to_string(schema)?
        } else {
            "undefined".to_string()
        }
    } else {
        "undefined".to_string()
    };

    let parameter_schema_str = if let Some(handler) = &fixture.handler {
        if let Some(params) = &handler.parameters {
            build_parameter_schema_json(params)?
        } else {
            "undefined".to_string()
        }
    } else {
        "undefined".to_string()
    };

    let file_params_str = if let Some(handler) = &fixture.handler {
        if let Some(params) = &handler.parameters {
            extract_file_params_json(params).unwrap_or_else(|| "undefined".to_string())
        } else {
            "undefined".to_string()
        }
    } else {
        "undefined".to_string()
    };

    let raw_middleware = fixture.handler.as_ref().and_then(|handler| handler.middleware.as_ref());
    let config_code = generate_server_config_ts(metadata, raw_middleware, fixture_id)?;

    let background_handler_name = if background.is_some() && !skip_route_registration {
        Some(format!("{}_background_state", handler_name))
    } else {
        None
    };
    let mut handler_entries = Vec::new();
    if !skip_route_registration {
        handler_entries.push(format!("\t\t\t{}: {}", handler_name, to_camel_case(handler_name)));
    }
    if let Some(bg_name) = &background_handler_name {
        handler_entries.push(format!("\t\t\t{}: {}", bg_name, to_camel_case(bg_name)));
    }
    let handlers_literal = if handler_entries.is_empty() {
        "{\n\t\t}".to_string()
    } else {
        format!("{{\n{}\n\t\t}}", handler_entries.join(",\n"))
    };

    let background_route_decl = if let (Some(bg), Some(handler_name)) =
        (background.as_ref(), background_handler_name.as_ref())
    {
        Some(format!(
            "\tconst backgroundRoute: RouteMetadata = {{\n\t\tmethod: \"GET\",\n\t\tpath: \"{}\",\n\t\thandler_name: \"{}\",\n\t\trequest_schema: undefined,\n\t\tresponse_schema: undefined,\n\t\tparameter_schema: undefined,\n\t\tfile_params: undefined,\n\t\tis_async: true,\n\t}};\n\n",
            bg.state_path, handler_name
        ))
    } else {
        None
    };

    let routes_literal = if skip_route_registration {
        "[]".to_string()
    } else if background_route_decl.is_some() {
        "[route, backgroundRoute]".to_string()
    } else {
        "[route]".to_string()
    };

    let mut app_factory_code = String::new();
    app_factory_code.push_str(&format!("export function {}(): SpikardApp {{\n", app_factory_name));
    if !config_code.is_empty() {
        app_factory_code.push_str(&config_code);
        if !config_code.ends_with('\n') {
            app_factory_code.push('\n');
        }
        app_factory_code.push('\n');
    }
    if !skip_route_registration {
        app_factory_code.push_str(&format!(
            "\tconst route: RouteMetadata = {{\n\t\tmethod: \"{}\",\n\t\tpath: \"{}\",\n\t\thandler_name: \"{}\",\n\t\trequest_schema: {},\n\t\tresponse_schema: undefined,\n\t\tparameter_schema: {},\n\t\tfile_params: {},\n\t\tis_async: true,\n\t}};\n\n",
            method.to_uppercase(),
            route_path,
            handler_name,
            body_schema_str,
            parameter_schema_str,
            file_params_str
        ));
        if let Some(bg_decl) = &background_route_decl {
            app_factory_code.push_str(bg_decl);
        }
    }

    app_factory_code.push_str("\treturn {\n");
    app_factory_code.push_str(&format!("\t\troutes: {},\n", routes_literal));
    app_factory_code.push_str(&format!("\t\thandlers: {},\n", handlers_literal));
    if !hooks_registration.is_empty() {
        app_factory_code.push_str(&hooks_registration);
    }
    if !config_code.is_empty() {
        app_factory_code.push_str("\t\tconfig,\n");
    }
    app_factory_code.push_str("\t};\n");
    app_factory_code.push_str("}\n");

    let mut full_handler_code = String::new();
    if !hooks_code.is_empty() {
        full_handler_code.push_str(&hooks_code);
    }
    if !handler_func.is_empty() {
        if !full_handler_code.is_empty() {
            full_handler_code.push_str("\n\n");
        }
        full_handler_code.push_str(&handler_func);
    }
    if let Some(state_handler) = background_state_handler.filter(|code| !code.is_empty()) {
        if !full_handler_code.is_empty() {
            full_handler_code.push_str("\n\n");
        }
        full_handler_code.push_str(&state_handler);
    }

    Ok((full_handler_code.trim().to_string(), app_factory_code))
}

/// Generate handler function for a fixture
fn generate_handler_function(
    fixture: &Fixture,
    route: &str,
    method: &str,
    handler_name: &str,
    fixture_id: &str,
    background: Option<&BackgroundFixtureData>,
    metadata: &MiddlewareMetadata,
) -> Result<String> {
    if let Some(stream_info) = streaming_data(fixture)? {
        return generate_streaming_handler(fixture, handler_name, route, method, &stream_info);
    }

    let handler_opt = fixture.handler.as_ref();

    let params = if let Some(handler) = handler_opt {
        if let Some(ref param_schema) = handler.parameters {
            extract_parameters_ts(param_schema)?
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let has_body = handler_opt.and_then(|h| h.body_schema.as_ref()).is_some();

    let expected_status = fixture.expected_response.status_code;
    let expected_body = fixture.expected_response.body.as_ref();
    let expected_body_is_empty = expected_body.is_some_and(is_value_effectively_empty);

    let mut code = String::new();

    code.push_str(&format!(
        "/**\n * Handler for {} {}\n */\n",
        method.to_uppercase(),
        route
    ));
    code.push_str(&format!(
        "async function {}(requestJson: string): Promise<string> {{\n",
        to_camel_case(handler_name)
    ));
    code.push_str("\tconst request = JSON.parse(requestJson);\n");

    let mut uses_body = has_body && ((expected_status == 200 && expected_body.is_none()) || expected_status == 422);
    if background.is_some() {
        uses_body = true;
    }
    let body_var = if uses_body { "body" } else { "_body" };
    code.push_str(&format!("\tconst {} = request.body ?? null;\n", body_var));

    let uses_params =
        !params.is_empty() && ((expected_status == 200 && expected_body.is_none()) || expected_status == 422);
    let params_var = if uses_params { "params" } else { "_params" };
    code.push_str(&format!("\tconst {} = request.params ?? {{}};\n", params_var));
    let handler_status = if metadata.rate_limit.is_some() {
        200
    } else {
        expected_status
    };
    code.push_str(&format!(
        "\tconst response: HandlerResponse = {{ status: {} }};\n",
        handler_status
    ));

    if let Some(headers_literal) = build_expected_headers_literal(fixture)? {
        code.push_str(&format!("\tresponse.headers = {};\n", headers_literal));
    }

    if let Some(timeout) = metadata.request_timeout.as_ref().and_then(|cfg| cfg.sleep_ms) {
        code.push_str(&format!(
            "\tawait new Promise((resolve) => setTimeout(resolve, {}));\n",
            timeout
        ));
    }

    if let Some(bg) = background {
        code.push_str(&format!(
            "\tBACKGROUND_STATE[\"{fixture_id}\"] = BACKGROUND_STATE[\"{fixture_id}\"] ?? [];\n",
            fixture_id = fixture_id
        ));
        code.push_str(&format!(
            "\tconst state = BACKGROUND_STATE[\"{fixture_id}\"] as unknown[];\n",
            fixture_id = fixture_id
        ));
        let value_accessor = format_property_access(body_var, &bg.value_field);
        code.push_str(&format!(
            "\tconst value = {} && typeof {} === \"object\" ? {} : undefined;\n",
            body_var, body_var, value_accessor
        ));
        code.push_str("\tif (value === undefined || value === null) {\n");
        code.push_str("\t\tthrow new Error(\"background task requires request body value\");\n");
        code.push_str("\t}\n");
        code.push_str("\tvoid Promise.resolve().then(() => void state.push(value));\n");
        code.push_str("\tresponse.body = null;\n");
        code.push_str("\treturn JSON.stringify(response);\n");
        code.push('}');
        code.push('\n');
        return Ok(code);
    }

    let should_echo_params = (expected_status == 200 && expected_body.is_none()) || expected_status == 422;
    let should_return_expected = expected_body.is_some() && !expected_body_is_empty && expected_status != 422;

    if should_return_expected {
        if let Some(body_json) = expected_body {
            if body_json.is_string() {
                if let Some(target_len) = expected_content_length(fixture) {
                    let literal = serde_json::to_string(body_json)?;
                    code.push_str(&format!("\tconst contentValue = {};\n", literal));
                    code.push_str("\tlet contentBytes = Buffer.from(contentValue, \"utf-8\");\n");
                    code.push_str(&format!(
                        "\tif (contentBytes.length < {}) {{\n\t\tconst padding = Buffer.alloc({} - contentBytes.length, \" \");\n\t\tcontentBytes = Buffer.concat([contentBytes, padding]);\n\t}}\n",
                        target_len, target_len
                    ));
                    code.push_str("\tasync function* streamContent() {\n");
                    code.push_str("\t\tyield contentBytes;\n");
                    code.push_str("\t}\n");
                    code.push_str("\treturn new StreamingResponse(streamContent(), {\n");
                    code.push_str(&format!("\t\tstatusCode: {},\n", handler_status));
                    code.push_str("\t\theaders: response.headers ?? {},\n");
                    code.push_str("\t});\n");
                    code.push('}');
                    return Ok(code);
                } else {
                    let converted = convert_large_numbers_to_strings(body_json);
                    let json_str = serde_json::to_string(&converted)?;
                    code.push_str(&format!("\tconst responseBody = {};\n", json_str));
                    code.push_str("\tresponse.body = responseBody;\n");
                }
            } else {
                let converted = convert_large_numbers_to_strings(body_json);
                let json_str = serde_json::to_string(&converted)?;
                code.push_str(&format!("\tconst responseBody = {};\n", json_str));
                code.push_str("\tresponse.body = responseBody;\n");
            }
        } else {
            code.push_str("\tresponse.body = null;\n");
        }
    } else if should_echo_params {
        code.push_str("\tconst result: Record<string, unknown> = {};\n");
        for binding in &params {
            let param_access = format_property_access(params_var, &binding.key);
            code.push_str(&format!("\tconst {} = {};\n", binding.var_name, param_access));
        }
        if has_body {
            code.push_str(&format!(
                "\tif ({} !== null && {} !== undefined) {{\n",
                body_var, body_var
            ));
            code.push_str(&format!("\t\tif (typeof {} === \"object\") {{\n", body_var));
            code.push_str(&format!("\t\t\tObject.assign(result, {});\n", body_var));
            code.push_str("\t\t} else {\n");
            code.push_str(&format!(
                "\t\t\t{} = {};\n",
                format_property_access("result", "body"),
                body_var
            ));
            code.push_str("\t\t}\n");
            code.push_str("\t}\n");
        }

        for binding in &params {
            code.push_str(&format!(
                "\tif ({} !== null && {} !== undefined) {{\n",
                binding.var_name, binding.var_name
            ));
            let result_access = format_property_access("result", &binding.key);
            if binding.ty.contains("Date") {
                code.push_str(&format!(
                    "\t\t{} = {}.toISOString();\n",
                    result_access, binding.var_name
                ));
            } else {
                code.push_str(&format!("\t\t{} = {};\n", result_access, binding.var_name));
            }
            code.push_str("\t}\n");
        }

        code.push_str("\tresponse.body = result;\n");
    } else {
        code.push_str("\tresponse.body = null;\n");
    }

    code.push_str("\treturn JSON.stringify(response);\n");
    code.push('}');

    Ok(code)
}

fn generate_streaming_handler(
    fixture: &Fixture,
    handler_name: &str,
    route: &str,
    method: &str,
    stream_info: &StreamingFixtureData,
) -> Result<String> {
    let function_name = to_camel_case(handler_name);
    let expected_status = fixture.expected_response.status_code;
    let headers_literal = build_streaming_headers_ts(fixture, stream_info);

    let mut chunk_lines = Vec::new();
    if let Some(streaming) = &fixture.streaming {
        for chunk in &streaming.chunks {
            match chunk {
                spikard_codegen::openapi::FixtureStreamChunk::Text { value } => {
                    let literal = serde_json::to_string(value)?;
                    chunk_lines.push(format!("\t\tyield {};", literal));
                }
                spikard_codegen::openapi::FixtureStreamChunk::Bytes { base64 } => {
                    chunk_lines.push(format!("\t\tyield Buffer.from(\"{}\", \"base64\");", base64));
                }
            }
        }
    }

    let chunks_block = if chunk_lines.is_empty() {
        "\t\t// No chunks defined\n".to_string()
    } else {
        format!("{}\n", chunk_lines.join("\n"))
    };

    Ok(format!(
        "/**\n * Handler for {} {}\n */\nasync function {}(_requestJson: string): Promise<StreamingResponse> {{\n\tconst stream = async function* () {{\n{}\t}};\n\n\treturn new StreamingResponse(stream(), {{\n\t\tstatusCode: {},\n\t\theaders: {}\n\t}});\n}}",
        method.to_uppercase(),
        route,
        function_name,
        chunks_block,
        expected_status,
        headers_literal
    ))
}

fn generate_background_state_handler(
    handler_name: &str,
    fixture_id: &str,
    background: &BackgroundFixtureData,
) -> String {
    let function_name = to_camel_case(&format!("{}_background_state", handler_name));
    format!(
        "async function {func}(): Promise<string> {{
\tconst state = BACKGROUND_STATE[\"{fixture}\"] ?? [];
\tconst response: HandlerResponse = {{ status: 200 }};
\tresponse.headers = {{ \"content-type\": \"application/json\" }};
\tresponse.body = {{ \"{state_key}\": state }};
\treturn JSON.stringify(response);
}}",
        func = function_name,
        fixture = fixture_id,
        state_key = background.state_key
    )
}

fn build_streaming_headers_ts(fixture: &Fixture, stream_info: &StreamingFixtureData) -> String {
    let mut headers = sanitized_expected_headers(fixture);

    if let Some(content_type) = stream_info.streaming.content_type.as_ref() {
        headers.insert("content-type".to_string(), Value::String(content_type.clone()));
    }

    if !headers.contains_key("content-type") {
        headers.insert(
            "content-type".to_string(),
            Value::String("application/octet-stream".to_string()),
        );
    }

    let literal = Value::Object(headers);
    serde_json::to_string(&literal).unwrap_or_else(|_| "{}".to_string())
}

fn append_sse_factories(
    code: &mut String,
    fixtures: &[AsyncFixture],
    registry: &mut Vec<(String, String, String)>,
    handler_names: &mut HashMap<String, usize>,
    dto_map: &HashMap<String, TypeScriptDto>,
) -> Result<()> {
    use std::collections::BTreeMap;

    if fixtures.is_empty() {
        return Ok(());
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
        let unique = make_unique_name(&format!("sse_{}", slug), handler_names);
        let handler_fn = format!("sseHandler{}", to_pascal_case(&slug));
        let factory_fn = format!("createApp{}", to_pascal_case(&unique));
        let mut union_members = Vec::new();
        let mut event_entries = Vec::new();
        for fixture in &channel_fixtures {
            if let Some(dto) = dto_map.get(&fixture.name) {
                union_members.push(dto.type_ident.clone());
                if fixture.examples.is_empty() {
                    event_entries.push(format!("    {}.parse({{}})", dto.schema_ident));
                } else {
                    for example in &fixture.examples {
                        let literal = json_value_to_ts_literal(example);
                        event_entries.push(format!("    {}.parse({})", dto.schema_ident, literal));
                    }
                }
            }
        }
        if union_members.is_empty() {
            union_members.push("Record<string, unknown>".to_string());
        }
        union_members.sort();
        union_members.dedup();
        if event_entries.is_empty() {
            event_entries.push("    z.record(z.string(), z.unknown()).parse({})".to_string());
        }
        let events_literal = format!("[\n{}\n  ]", event_entries.join(",\n"));

        let handler_code = format!(
            "async function {handler_fn}(_requestJson: string): Promise<StreamingResponse> {{
\tconst events = {events_literal};
\tasync function* eventStream() {{
\t\tfor (const payload of events) {{
\t\t\tyield `data: ${{JSON.stringify(payload)}}\\n\\n`;
\t\t}}
\t}}
\treturn new StreamingResponse(eventStream(), {{
\t\tstatusCode: 200,
\t\theaders: {{
\t\t\t\"content-type\": \"text/event-stream\",
\t\t\t\"cache-control\": \"no-cache\",
\t\t}},
\t}});
}}",
            handler_fn = handler_fn,
            events_literal = events_literal
        );

        let factory_code = format!(
            "export function {factory_fn}(): SpikardApp {{
\tconst route: RouteMetadata = {{
\t\tmethod: \"GET\",
\t\tpath: \"{path}\",
\t\thandler_name: \"{handler_fn}\",
\t\trequest_schema: undefined,
\t\tresponse_schema: undefined,
\t\tparameter_schema: undefined,
\t\tfile_params: undefined,
\t\tis_async: true,
\t}};

\treturn {{
\t\troutes: [route],
\t\thandlers: {{
\t\t\t{handler_fn},
\t\t}},
\t}};
}}",
            factory_fn = factory_fn,
            path = channel_path,
            handler_fn = handler_fn
        );

        code.push_str("\n\n");
        code.push_str(&handler_code);
        code.push_str("\n\n");
        code.push_str(&factory_code);
        code.push('\n');

        registry.push(("asyncapi_sse".to_string(), channel_path.clone(), factory_fn));
    }

    Ok(())
}

fn append_websocket_factories(
    code: &mut String,
    fixtures: &[AsyncFixture],
    registry: &mut Vec<(String, String, String)>,
    handler_names: &mut HashMap<String, usize>,
    dto_map: &HashMap<String, TypeScriptDto>,
) -> Result<()> {
    use std::collections::BTreeMap;

    if fixtures.is_empty() {
        return Ok(());
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
        let unique = make_unique_name(&format!("websocket_{}", slug), handler_names);
        let handler_fn = format!("websocketHandler{}", to_pascal_case(&slug));
        let factory_fn = format!("createApp{}", to_pascal_case(&unique));
        let mut schema_idents = Vec::new();
        for fixture in &channel_fixtures {
            if let Some(dto) = dto_map.get(&fixture.name) {
                schema_idents.push(dto.schema_ident.clone());
            }
        }
        schema_idents.sort();
        schema_idents.dedup();

        let schema_expr = match schema_idents.len() {
            0 => "z.record(z.string(), z.unknown())".to_string(),
            1 => schema_idents[0].clone(),
            _ => format!("z.union([{}])", schema_idents.join(", ")),
        };
        let schema_name = format!("{}ChannelSchema", to_pascal_case(&slug));
        let type_name = format!("{}ChannelMessage", to_pascal_case(&slug));
        let handler_code = format!(
            "const {schema_name} = {schema_expr};
type {type_name} = z.infer<typeof {schema_name}>;
type {type_name}Response = {type_name} & {{ validated: true }};

async function {handler_fn}(message: unknown): Promise<string> {{
\tconst payload: {type_name} = {schema_name}.parse(normalizeWebsocketPayload(message));
\tconst response: {type_name}Response = {{ ...payload, validated: true }};
\treturn JSON.stringify(response);
}}",
            schema_name = schema_name,
            schema_expr = schema_expr,
            type_name = type_name,
            handler_fn = handler_fn
        );

        let factory_code = format!(
            "export function {factory_fn}(): SpikardApp {{
\tconst route: RouteMetadata = {{
\t\tmethod: \"GET\",
\t\tpath: \"{path}\",
\t\thandler_name: \"{handler_fn}\",
\t\trequest_schema: undefined,
\t\tresponse_schema: undefined,
\t\tparameter_schema: undefined,
\t\tfile_params: undefined,
\t\tis_async: true,
\t}};

\treturn {{
\t\troutes: [route],
\t\thandlers: {{
\t\t\t{handler_fn},
\t\t}},
\t}};
}}",
            factory_fn = factory_fn,
            path = channel_path,
            handler_fn = handler_fn
        );

        code.push_str("\n\n");
        code.push_str(&handler_code);
        code.push_str("\n\n");
        code.push_str(&factory_code);
        code.push('\n');

        registry.push(("asyncapi_websocket".to_string(), channel_path.clone(), factory_fn));
    }

    Ok(())
}

fn metadata_requires_server_config(metadata: &MiddlewareMetadata) -> bool {
    metadata.compression.is_some()
        || metadata.rate_limit.is_some()
        || metadata.request_timeout.is_some()
        || metadata.request_id.is_some()
        || metadata.body_limit.is_some()
        || !metadata.static_dirs.is_empty()
}

fn handler_requires_server_config(fixture: &Fixture) -> bool {
    if let Some(handler) = &fixture.handler
        && let Some(middleware) = &handler.middleware
    {
        return middleware.get("openapi").is_some()
            || middleware.get("jwt_auth").is_some()
            || middleware.get("api_key_auth").is_some();
    }
    false
}

fn build_expected_headers_literal(fixture: &Fixture) -> Result<Option<String>> {
    let headers = sanitized_expected_headers(fixture);
    if headers.is_empty() {
        Ok(None)
    } else {
        let literal = Value::Object(headers);
        Ok(Some(serde_json::to_string(&literal)?))
    }
}

fn sanitized_expected_headers(fixture: &Fixture) -> JsonMap<String, Value> {
    let mut headers = JsonMap::new();
    if let Some(expected) = fixture.expected_response.headers.as_ref() {
        for (key, value) in expected {
            let header_name = key.to_ascii_lowercase();
            if header_name == "content-encoding" {
                continue;
            }
            if let Some(converted) = normalize_expected_header_value(value) {
                headers.insert(header_name, Value::String(converted));
            }
        }
    }
    headers
}

fn normalize_expected_header_value(raw: &str) -> Option<String> {
    match raw {
        "<<absent>>" => None,
        "<<present>>" => Some("spikard-test-value".to_string()),
        "<<uuid>>" => Some("00000000-0000-4000-8000-000000000000".to_string()),
        _ => Some(raw.to_string()),
    }
}

fn expected_content_length(fixture: &Fixture) -> Option<usize> {
    fixture.expected_response.headers.as_ref().and_then(|headers| {
        headers.iter().find_map(|(key, value)| {
            if key.eq_ignore_ascii_case("content-length") {
                value.parse::<usize>().ok()
            } else {
                None
            }
        })
    })
}

fn fixture_requires_binary_body(fixture: &Fixture) -> bool {
    expected_content_length(fixture).is_some()
        && fixture
            .expected_response
            .body
            .as_ref()
            .is_some_and(|body| body.is_string())
}

/// Extract parameters from parameter schema (TypeScript types)
struct ParameterBinding {
    var_name: String,
    key: String,
    ty: String,
}

fn extract_parameters_ts(schema: &Value) -> Result<Vec<ParameterBinding>> {
    let mut params = Vec::new();

    if let Some(obj) = schema.as_object() {
        if let Some(path_params) = obj.get("path").and_then(|v| v.as_object()) {
            for (name, param_schema) in path_params {
                let param_type = json_type_to_typescript(param_schema)?;
                params.push(ParameterBinding {
                    var_name: to_camel_case(name),
                    key: name.clone(),
                    ty: param_type,
                });
            }
        }

        if let Some(query_params) = obj.get("query").and_then(|v| v.as_object()) {
            for (name, param_schema) in query_params {
                let param_type = json_type_to_typescript(param_schema)?;
                params.push(ParameterBinding {
                    var_name: to_camel_case(name),
                    key: name.clone(),
                    ty: param_type,
                });
            }
        }

        if let Some(headers) = obj.get("headers").and_then(|v| v.as_object()) {
            for (name, param_schema) in headers {
                let param_type = json_type_to_typescript(param_schema)?;
                params.push(ParameterBinding {
                    var_name: to_camel_case(name),
                    key: name.clone(),
                    ty: param_type,
                });
            }
        }

        if let Some(cookies) = obj.get("cookies").and_then(|v| v.as_object()) {
            for (name, param_schema) in cookies {
                let param_type = json_type_to_typescript(param_schema)?;
                params.push(ParameterBinding {
                    var_name: to_camel_case(name),
                    key: name.clone(),
                    ty: param_type,
                });
            }
        }
    }

    Ok(params)
}

/// Convert JSON schema type to TypeScript type annotation
fn json_type_to_typescript(schema: &Value) -> Result<String> {
    let schema_type = schema.get("type").and_then(|v| v.as_str()).unwrap_or("string");

    let type_str = match schema_type {
        "string" => {
            if let Some(format) = schema.get("format").and_then(|v| v.as_str()) {
                match format {
                    "uuid" => "string",
                    "date" | "date-time" => "Date",
                    _ => "string",
                }
            } else {
                "string"
            }
        }
        "integer" => "number",
        "number" => "number",
        "boolean" => "boolean",
        "array" => {
            if let Some(items) = schema.get("items") {
                let item_type = json_type_to_typescript(items)?;
                return Ok(format!("{}[]", item_type));
            }
            "unknown[]"
        }
        "object" => "Record<string, unknown>",
        _ => "unknown",
    };

    Ok(type_str.to_string())
}

/// Build parameter schema JSON string
fn build_parameter_schema_json(params: &Value) -> Result<String> {
    use serde_json::{Map, Value};

    let mut properties = Map::new();
    let mut required: Vec<Value> = Vec::new();

    if let Some(obj) = params.as_object() {
        for (section_key, source) in [
            ("path", "path"),
            ("query", "query"),
            ("headers", "header"),
            ("cookies", "cookie"),
        ] {
            if let Some(section) = obj.get(section_key).and_then(|v| v.as_object()) {
                for (name, schema_value) in section {
                    let mut schema_obj = match schema_value {
                        Value::Object(map) => map.clone(),
                        Value::String(s) => {
                            let mut map = Map::new();
                            map.insert("type".to_string(), Value::String(s.clone()));
                            map
                        }
                        Value::Bool(_) | Value::Number(_) | Value::Array(_) | Value::Null => {
                            let mut map = Map::new();
                            map.insert("type".to_string(), schema_value.clone());
                            map
                        }
                    };

                    schema_obj
                        .entry("type".to_string())
                        .or_insert_with(|| Value::String("string".to_string()));
                    schema_obj.insert("source".to_string(), Value::String(source.to_string()));

                    let is_optional = schema_obj.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                    let is_required_flag = schema_obj
                        .get("required")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(!is_optional);

                    schema_obj.remove("optional");
                    schema_obj.remove("required");

                    properties.insert(name.clone(), Value::Object(schema_obj));

                    let should_require = if source == "path" {
                        true
                    } else {
                        is_required_flag && !is_optional
                    };

                    if should_require {
                        required.push(Value::String(name.clone()));
                    }
                }
            }
        }
    }

    let mut schema = Map::new();
    schema.insert("type".to_string(), Value::String("object".to_string()));
    schema.insert("properties".to_string(), Value::Object(properties));

    if !required.is_empty() {
        schema.insert("required".to_string(), Value::Array(required));
    }

    serde_json::to_string(&Value::Object(schema)).context("Failed to serialize parameter schema")
}

/// Extract file parameters JSON string
fn extract_file_params_json(params: &Value) -> Option<String> {
    params
        .as_object()
        .and_then(|obj| obj.get("files"))
        .and_then(|files| serde_json::to_string(files).ok())
}

/// Generate ServerConfig TypeScript code from middleware JSON
fn generate_server_config_ts(
    metadata: &MiddlewareMetadata,
    raw_middleware: Option<&Value>,
    fixture_id: &str,
) -> Result<String> {
    let mut config_parts = Vec::new();

    if let Some(compression) = &metadata.compression {
        let mut fields = Vec::new();
        if let Some(gzip) = compression.gzip {
            fields.push(format!("\t\t\tgzip: {}", if gzip { "true" } else { "false" }));
        }
        if let Some(brotli) = compression.brotli {
            fields.push(format!("\t\t\tbrotli: {}", if brotli { "true" } else { "false" }));
        }
        if let Some(min_size) = compression.min_size {
            fields.push(format!("\t\t\tminSize: {}", min_size));
        }
        if let Some(quality) = compression.quality {
            fields.push(format!("\t\t\tquality: {}", quality));
        }
        config_parts.push(format!(
            "\t\tcompression: {{\n{}\n\t\t}}",
            if fields.is_empty() {
                String::new()
            } else {
                fields.join(",\n")
            }
        ));
    }

    if let Some(rate_limit) = &metadata.rate_limit {
        let mut fields = vec![
            format!("\t\t\tperSecond: {}", rate_limit.per_second),
            format!("\t\t\tburst: {}", rate_limit.burst),
        ];
        if let Some(ip_based) = rate_limit.ip_based {
            fields.push(format!("\t\t\tipBased: {}", if ip_based { "true" } else { "false" }));
        }
        config_parts.push(format!("\t\trateLimit: {{\n{}\n\t\t}}", fields.join(",\n")));
    }

    if let Some(timeout) = &metadata.request_timeout {
        config_parts.push(format!("\t\trequestTimeout: {}", timeout.seconds));
    }

    if let Some(request_id) = &metadata.request_id
        && let Some(enabled) = request_id.enabled
    {
        config_parts.push(format!(
            "\t\tenableRequestId: {}",
            if enabled { "true" } else { "false" }
        ));
    }

    if let Some(body_limit) = &metadata.body_limit {
        match body_limit.max_bytes {
            Some(bytes) => config_parts.push(format!("\t\tmaxBodySize: {}", bytes)),
            None => config_parts.push("\t\tmaxBodySize: null".to_string()),
        }
    }

    if !metadata.static_dirs.is_empty() {
        let mut dirs = Vec::new();
        for dir in &metadata.static_dirs {
            let mut fields = vec![format!(
                "\t\t\t{{\n\t\t\t\tdirectory: new URL(\"./static_assets/{}/{}\", import.meta.url).pathname,\n\t\t\t\troutePrefix: \"{}\"",
                fixture_id, dir.directory_name, dir.route_prefix
            )];
            if !dir.index_file {
                fields.push("\t\t\t\tindexFile: false".to_string());
            }
            if let Some(cache) = &dir.cache_control {
                fields.push(format!("\t\t\t\tcacheControl: \"{}\"", cache));
            }
            fields.push("\t\t\t}".to_string());
            dirs.push(fields.join(",\n"));
        }
        config_parts.push(format!("\t\tstaticFiles: [\n{}\n\t\t]", dirs.join(",\n")));
    }

    if let Some(middleware) = raw_middleware {
        if let Some(openapi_obj) = middleware.get("openapi").and_then(|v| v.as_object())
            && let Some(block) = build_openapi_config_ts(openapi_obj)?
        {
            config_parts.push(block);
        }

        if let Some(jwt_obj) = middleware.get("jwt_auth").and_then(|v| v.as_object())
            && let Some(block) = build_jwt_config_ts(jwt_obj)?
        {
            config_parts.push(block);
        }

        if let Some(api_key_obj) = middleware.get("api_key_auth").and_then(|v| v.as_object())
            && let Some(block) = build_api_key_config_ts(api_key_obj)?
        {
            config_parts.push(block);
        }
    }

    if config_parts.is_empty() {
        return Ok(String::new());
    }

    Ok(format!(
        "\tconst config: ServerConfig = {{\n{}\n\t}};",
        config_parts.join(",\n")
    ))
}

fn build_jwt_config_ts(jwt_obj: &serde_json::Map<String, Value>) -> Result<Option<String>> {
    let enabled = jwt_obj.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    if !enabled {
        return Ok(None);
    }
    let mut parts = Vec::new();
    if let Some(secret) = jwt_obj.get("secret").and_then(|v| v.as_str()) {
        parts.push(format!("\t\tsecret: \"{}\"", secret));
    }
    if let Some(algorithm) = jwt_obj.get("algorithm").and_then(|v| v.as_str()) {
        parts.push(format!("\t\talgorithm: \"{}\"", algorithm));
    }
    if let Some(audience) = jwt_obj.get("audience") {
        parts.push(format!("\t\taudience: {}", serde_json::to_string(audience)?));
    }
    if let Some(issuer) = jwt_obj.get("issuer").and_then(|v| v.as_str()) {
        parts.push(format!("\t\tissuer: \"{}\"", issuer));
    }
    if let Some(leeway) = jwt_obj.get("leeway").and_then(|v| v.as_i64()) {
        parts.push(format!("\t\tleeway: {}", leeway));
    }
    if parts.is_empty() {
        return Ok(None);
    }
    Ok(Some(format!("\t\tjwtAuth: {{\n{}\n\t\t}}", parts.join(",\n"))))
}

fn build_api_key_config_ts(api_key_obj: &serde_json::Map<String, Value>) -> Result<Option<String>> {
    let enabled = api_key_obj.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    if !enabled {
        return Ok(None);
    }
    let mut parts = Vec::new();
    if let Some(keys) = api_key_obj.get("keys") {
        parts.push(format!("\t\tkeys: {}", serde_json::to_string(keys)?));
    }
    if let Some(header_name) = api_key_obj.get("header_name").and_then(|v| v.as_str()) {
        parts.push(format!("\t\theaderName: \"{}\"", header_name));
    }
    if parts.is_empty() {
        return Ok(None);
    }
    Ok(Some(format!("\t\tapiKeyAuth: {{\n{}\n\t\t}}", parts.join(",\n"))))
}

fn build_openapi_config_ts(openapi_obj: &serde_json::Map<String, Value>) -> Result<Option<String>> {
    let enabled = openapi_obj.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    if !enabled {
        return Ok(None);
    }
    let mut parts = vec!["\t\tenabled: true".to_string()];
    if let Some(title) = openapi_obj.get("title").and_then(|v| v.as_str()) {
        parts.push(format!("\t\ttitle: \"{}\"", title));
    }
    if let Some(version) = openapi_obj.get("version").and_then(|v| v.as_str()) {
        parts.push(format!("\t\tversion: \"{}\"", version));
    }
    if let Some(description) = openapi_obj.get("description").and_then(|v| v.as_str()) {
        parts.push(format!("\t\tdescription: \"{}\"", description));
    }
    if let Some(swagger_ui_path) = openapi_obj.get("swagger_ui_path").and_then(|v| v.as_str()) {
        parts.push(format!("\t\tswaggerUiPath: \"{}\"", swagger_ui_path));
    }
    if let Some(redoc_path) = openapi_obj.get("redoc_path").and_then(|v| v.as_str()) {
        parts.push(format!("\t\tredocPath: \"{}\"", redoc_path));
    }
    if let Some(openapi_json_path) = openapi_obj.get("openapi_json_path").and_then(|v| v.as_str()) {
        parts.push(format!("\t\topenapiJsonPath: \"{}\"", openapi_json_path));
    }
    if let Some(contact) = openapi_obj.get("contact").and_then(|v| v.as_object()) {
        let mut contact_parts = Vec::new();
        if let Some(name) = contact.get("name").and_then(|v| v.as_str()) {
            contact_parts.push(format!("\t\t\tname: \"{}\"", name));
        }
        if let Some(email) = contact.get("email").and_then(|v| v.as_str()) {
            contact_parts.push(format!("\t\t\temail: \"{}\"", email));
        }
        if let Some(url) = contact.get("url").and_then(|v| v.as_str()) {
            contact_parts.push(format!("\t\t\turl: \"{}\"", url));
        }
        if !contact_parts.is_empty() {
            parts.push(format!("\t\tcontact: {{\n{}\n\t\t}}", contact_parts.join(",\n")));
        }
    }
    if let Some(license) = openapi_obj.get("license").and_then(|v| v.as_object()) {
        let mut license_parts = Vec::new();
        if let Some(name) = license.get("name").and_then(|v| v.as_str()) {
            license_parts.push(format!("\t\t\tname: \"{}\"", name));
        }
        if let Some(url) = license.get("url").and_then(|v| v.as_str()) {
            license_parts.push(format!("\t\t\turl: \"{}\"", url));
        }
        if !license_parts.is_empty() {
            parts.push(format!("\t\tlicense: {{\n{}\n\t\t}}", license_parts.join(",\n")));
        }
    }
    if let Some(servers) = openapi_obj.get("servers").and_then(|v| v.as_array()) {
        let mut entries = Vec::new();
        for server in servers {
            if let Some(obj) = server.as_object() {
                let mut server_parts = Vec::new();
                if let Some(url) = obj.get("url").and_then(|v| v.as_str()) {
                    server_parts.push(format!("\t\t\t\turl: \"{}\"", url));
                }
                if let Some(desc) = obj.get("description").and_then(|v| v.as_str()) {
                    server_parts.push(format!("\t\t\t\tdescription: \"{}\"", desc));
                }
                if !server_parts.is_empty() {
                    entries.push(format!("\t\t\t{{\n{}\n\t\t\t}}", server_parts.join(",\n")));
                }
            }
        }
        if !entries.is_empty() {
            parts.push(format!("\t\tservers: [\n{}\n\t\t]", entries.join(",\n")));
        }
    }
    Ok(Some(format!("\t\topenapi: {{\n{}\n\t\t}}", parts.join(",\n"))))
}

fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c == '_' || c == '$' || c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    for ch in chars {
        if ch == '_' || ch == '$' || ch.is_ascii_alphanumeric() {
            continue;
        }
        return false;
    }
    true
}

fn format_property_access(base: &str, key: &str) -> String {
    if is_valid_identifier(key) {
        format!("{}.{}", base, key)
    } else {
        format!("{}[\"{}\"]", base, key)
    }
}

fn is_large_integer(number: &serde_json::Number) -> bool {
    if let Some(i) = number.as_i64() {
        i128::from(i).abs() > MAX_SAFE_INTEGER
    } else if let Some(u) = number.as_u64() {
        (u as i128) > MAX_SAFE_INTEGER
    } else {
        false
    }
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

fn convert_large_numbers_to_strings(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Number(n) if is_large_integer(n) => serde_json::Value::String(n.to_string()),
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(convert_large_numbers_to_strings).collect())
        }
        serde_json::Value::Object(obj) => serde_json::Value::Object(
            obj.iter()
                .map(|(k, v)| (k.clone(), convert_large_numbers_to_strings(v)))
                .collect(),
        ),
        other => other.clone(),
    }
}

/// Sanitize a string to be a valid identifier (lowercase snake_case)
fn sanitize_identifier(s: &str) -> String {
    let mut result = s
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '_', "_");

    while result.contains("__") {
        result = result.replace("__", "_");
    }

    result.trim_matches('_').to_string()
}

/// Convert to camelCase
fn to_camel_case(s: &str) -> String {
    let parts: Vec<&str> = s.split(&['_', '-'][..]).collect();
    if parts.is_empty() {
        return String::new();
    }

    let mut result = parts[0].to_lowercase();
    for part in &parts[1..] {
        if !part.is_empty() {
            result.push_str(&capitalize(part));
        }
    }
    result
}

/// Convert to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split(&['_', '-'][..])
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect()
}

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
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

/// Generate TypeScript lifecycle hook function implementations
fn generate_lifecycle_hooks_ts(fixture_id: &str, hooks: &Value, fixture: &Fixture) -> String {
    let mut code = String::new();

    if let Some(on_request) = hooks.get("on_request").and_then(|v| v.as_array()) {
        for (idx, hook) in on_request.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_on_request_{}", fixture_id, hook_name, idx));

            code.push_str(&format!(
                r#"async function {}(request: HookRequest): Promise<HookResult> {{
	// Mock onRequest hook: {}
	return request;
}}

"#,
                func_name, hook_name
            ));
        }
    }

    if let Some(pre_validation) = hooks.get("pre_validation").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_validation.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_pre_validation_{}", fixture_id, hook_name, idx));

            let should_short_circuit = hook_name.contains("rate_limit") && fixture.expected_response.status_code == 429;

            if should_short_circuit {
                code.push_str(&format!(
                    r#"async function {}(_request: HookRequest): Promise<HookResult> {{
	// preValidation hook: {} - Short circuits with 429
	return {{
		statusCode: 429,
		body: {{
			error: "Rate limit exceeded",
			message: "Too many requests, please try again later"
		}},
		headers: {{
			"Retry-After": "60"
		}}
	}};
}}

"#,
                    func_name, hook_name
                ));
            } else {
                code.push_str(&format!(
                    r#"async function {}(request: HookRequest): Promise<HookResult> {{
	// Mock preValidation hook: {}
	return request;
}}

"#,
                    func_name, hook_name
                ));
            }
        }
    }

    if let Some(pre_handler) = hooks.get("pre_handler").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_handler.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_pre_handler_{}", fixture_id, hook_name, idx));

            let auth_fails = hook_name.contains("auth")
                && (fixture.expected_response.status_code == 401 || fixture.expected_response.status_code == 403);

            if auth_fails {
                let (status_code, error_msg, detail_msg) = if fixture.expected_response.status_code == 401 {
                    (401, "Unauthorized", "Invalid or expired authentication token")
                } else {
                    (403, "Forbidden", "Admin role required for this endpoint")
                };

                code.push_str(&format!(
                    r#"async function {}(_request: HookRequest): Promise<HookResult> {{
	// preHandler hook: {} - Short circuits with {}
	return {{
		statusCode: {},
		body: {{
			error: "{}",
			message: "{}"
		}}
	}};
}}

"#,
                    func_name, hook_name, status_code, status_code, error_msg, detail_msg
                ));
            } else {
                code.push_str(&format!(
                    r#"async function {}(request: HookRequest): Promise<HookResult> {{
	// Mock preHandler hook: {}
	return request;
}}

"#,
                    func_name, hook_name
                ));
            }
        }
    }

    if let Some(on_response) = hooks.get("on_response").and_then(|v| v.as_array()) {
        for (idx, hook) in on_response.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_on_response_{}", fixture_id, hook_name, idx));

            if hook_name.contains("security") {
                code.push_str(&format!(
                    r#"async function {}(response: HookResponse): Promise<HookResponse> {{
	// onResponse hook: {} - Adds security headers
	if (!response.headers) response.headers = {{}};
	response.headers["X-Content-Type-Options"] = "nosniff";
	response.headers["X-Frame-Options"] = "DENY";
	response.headers["X-XSS-Protection"] = "1; mode=block";
	response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains";
	return response;
}}

"#,
                    func_name, hook_name
                ));
            } else if hook_name.contains("timing") || hook_name.contains("timer") {
                code.push_str(&format!(
                    r#"async function {}(response: HookResponse): Promise<HookResponse> {{
	// onResponse hook: {} - Adds timing header
	if (!response.headers) response.headers = {{}};
	response.headers["X-Response-Time"] = "0ms";
	return response;
}}

"#,
                    func_name, hook_name
                ));
            } else {
                code.push_str(&format!(
                    r#"async function {}(response: HookResponse): Promise<HookResponse> {{
	// Mock onResponse hook: {}
	return response;
}}

"#,
                    func_name, hook_name
                ));
            }
        }
    }

    if let Some(on_error) = hooks.get("on_error").and_then(|v| v.as_array()) {
        for (idx, hook) in on_error.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_on_error_{}", fixture_id, hook_name, idx));

            code.push_str(&format!(
                r#"async function {}(response: HookResponse): Promise<HookResponse> {{
	// onError hook: {} - Format error response
	if (!response.headers) response.headers = {{}};
	response.headers["Content-Type"] = "application/json";
	return response;
}}

"#,
                func_name, hook_name
            ));
        }
    }

    code
}

/// Generate lifecycle hooks registration code for TypeScript
fn generate_lifecycle_hooks_registration_ts(fixture_id: &str, hooks: &Value) -> String {
    let mut registrations = Vec::new();

    if let Some(on_request) = hooks.get("on_request").and_then(|v| v.as_array()) {
        let mut hook_funcs = Vec::new();
        for (idx, hook) in on_request.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_on_request_{}", fixture_id, hook_name, idx));
            hook_funcs.push(func_name);
        }
        if !hook_funcs.is_empty() {
            registrations.push(format!("\t\tonRequest: [{}]", hook_funcs.join(", ")));
        }
    }

    if let Some(pre_validation) = hooks.get("pre_validation").and_then(|v| v.as_array()) {
        let mut hook_funcs = Vec::new();
        for (idx, hook) in pre_validation.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_pre_validation_{}", fixture_id, hook_name, idx));
            hook_funcs.push(func_name);
        }
        if !hook_funcs.is_empty() {
            registrations.push(format!("\t\tpreValidation: [{}]", hook_funcs.join(", ")));
        }
    }

    if let Some(pre_handler) = hooks.get("pre_handler").and_then(|v| v.as_array()) {
        let mut hook_funcs = Vec::new();
        for (idx, hook) in pre_handler.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_pre_handler_{}", fixture_id, hook_name, idx));
            hook_funcs.push(func_name);
        }
        if !hook_funcs.is_empty() {
            registrations.push(format!("\t\tpreHandler: [{}]", hook_funcs.join(", ")));
        }
    }

    if let Some(on_response) = hooks.get("on_response").and_then(|v| v.as_array()) {
        let mut hook_funcs = Vec::new();
        for (idx, hook) in on_response.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_on_response_{}", fixture_id, hook_name, idx));
            hook_funcs.push(func_name);
        }
        if !hook_funcs.is_empty() {
            registrations.push(format!("\t\tonResponse: [{}]", hook_funcs.join(", ")));
        }
    }

    if let Some(on_error) = hooks.get("on_error").and_then(|v| v.as_array()) {
        let mut hook_funcs = Vec::new();
        for (idx, hook) in on_error.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = to_camel_case(&format!("{}_{}_on_error_{}", fixture_id, hook_name, idx));
            hook_funcs.push(func_name);
        }
        if !hook_funcs.is_empty() {
            registrations.push(format!("\t\tonError: [{}]", hook_funcs.join(", ")));
        }
    }

    if registrations.is_empty() {
        String::new()
    } else {
        format!("\tlifecycleHooks: {{\n{}\n\t}},\n", registrations.join(",\n"))
    }
}
