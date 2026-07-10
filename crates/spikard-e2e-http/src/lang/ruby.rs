//! Ruby HTTP e2e test generation (server-pattern slice from alef core).
//!
//! This module owns the server-pattern files for spikard's Ruby e2e suite:
//! - `app_harness.rb` — spawns the SUT as an HTTP server via the Magnus binding
//! - `spec/spec_helper.rb` (server variant) — `RSpec` `spec_helper` that spawns `app_harness.rb`
//! - `spec/*_spec.rb` — per-category spec files using the SUT harness pattern
//!
//! The shared client-pattern files (Gemfile, .rubocop.yaml, mock-server `spec_helper`,
//! mock-server test bodies) stay generic in alef. Only the server-pattern slice lives here.
//!
//! Sources (alef `src/e2e/codegen/ruby/`):
//! - `project.rs::build_middleware_value`
//! - `project.rs::render_app_harness`
//! - `project.rs::render_spec_helper` (`uses_harness` branch only)
//! - `project.rs::render_env_setup`
//! - `http.rs::render_http_example_sut`
//! - `http.rs::synthesize_multipart_body`
//! - `http.rs::http_method_class`
//! - `values.rs::json_to_ruby`
//! - `values.rs::ruby_module_name`
//! - `spec_file.rs::render_spec_file` (HTTP-fixture slice only)

use alef::GeneratedFile;
use alef::ResolvedCrateConfig;
use alef::core::hash::{self, CommentStyle};
use alef::e2e::config::E2eConfig;
use alef::e2e::escape::{escape_ruby_single, ruby_string_literal, sanitize_filename, sanitize_ident};
use alef::e2e::fixture::{Fixture, FixtureGroup, HttpMiddleware};
use anyhow::Result;
use heck::ToUpperCamelCase;
use minijinja::{Environment, context};
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::path::PathBuf;

/// Build the private template environment holding the Ruby HTTP templates.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "ruby/app_harness.rb.jinja".to_owned(),
        include_str!("../../templates/ruby/app_harness.rb.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "ruby/http_test_sut.jinja".to_owned(),
        include_str!("../../templates/ruby/http_test_sut.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "ruby/http_101_skip.jinja".to_owned(),
        include_str!("../../templates/ruby/http_101_skip.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "ruby/test_file.jinja".to_owned(),
        include_str!("../../templates/ruby/test_file.jinja").to_owned(),
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

/// Convert a module path (e.g., "spikard") to Ruby `PascalCase` module name
/// (e.g., "Spikard").
fn ruby_module_name(module_path: &str) -> String {
    module_path.to_upper_camel_case()
}

/// Convert a `serde_json::Value` to a Ruby literal string, preferring single quotes.
fn json_to_ruby(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => ruby_string_literal(s),
        serde_json::Value::Bool(true) => "true".to_string(),
        serde_json::Value::Bool(false) => "false".to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Null => "nil".to_string(),
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(json_to_ruby).collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(map) => {
            let items: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{} => {}", ruby_string_literal(k), json_to_ruby(v)))
                .collect();
            format!("{{ {} }}", items.join(", "))
        }
    }
}

/// Convert an uppercase HTTP method string to Ruby's `Net::HTTP` class name.
/// Ruby uses title-cased names: Get, Post, Put, Delete, Patch, Head, Options, Trace.
fn http_method_class(method: &str) -> String {
    let mut chars = method.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

/// Convert the fixture's `HttpMiddleware` into a `serde_json::Value` suitable
/// for embedding in the harness fixture JSON.
///
/// CORS `allow_*` → `allowed_*` to match `CorsConfig.from_json()`.
fn build_middleware_value(middleware: &Option<HttpMiddleware>) -> serde_json::Value {
    let Some(mw) = middleware else {
        return serde_json::Value::Null;
    };

    let mut map = serde_json::Map::new();

    if let Some(cors) = &mw.cors {
        let mut cors_map = serde_json::Map::new();
        cors_map.insert("allowed_origins".to_string(), serde_json::json!(cors.allow_origins));
        cors_map.insert("allowed_methods".to_string(), serde_json::json!(cors.allow_methods));
        cors_map.insert("allowed_headers".to_string(), serde_json::json!(cors.allow_headers));
        if !cors.expose_headers.is_empty() {
            cors_map.insert("expose_headers".to_string(), serde_json::json!(cors.expose_headers));
        }
        if let Some(max_age) = cors.max_age {
            cors_map.insert("max_age".to_string(), serde_json::json!(max_age));
        }
        if cors.allow_credentials {
            cors_map.insert("allow_credentials".to_string(), serde_json::json!(true));
        }
        map.insert("cors".to_string(), serde_json::Value::Object(cors_map));
    }

    if map.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::Value::Object(map)
    }
}

/// Render the server-pattern `app_harness.rb` that spawns the SUT HTTP server.
///
/// Ported verbatim from alef's `ruby/project.rs::render_app_harness`.
#[must_use]
pub fn render_app_harness(e2e_config: &E2eConfig, groups: &[FixtureGroup]) -> String {
    let mut fixtures_map = serde_json::Map::new();

    for group in groups {
        for fixture in &group.fixtures {
            if fixture.http.is_none() {
                continue;
            }
            let http_data = fixture.http.as_ref().unwrap();
            let middleware_value = build_middleware_value(&http_data.handler.middleware);
            let fixture_json = serde_json::json!({
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

    let fixtures_json_raw = serde_json::to_string(&fixtures_map).unwrap_or_default();
    let fixtures_json = ruby_string_literal(&fixtures_json_raw);

    let imports = e2e_config
        .harness
        .imports_for_lang("ruby")
        .into_iter()
        .collect::<Vec<_>>();
    let imports_ref = if imports.is_empty() {
        &e2e_config.harness.imports
    } else {
        &imports
    };

    let app_class_override = e2e_config.harness.app_class_for_lang("ruby");
    let app_class_str = if let Some(ref ac) = app_class_override {
        ac.as_str().to_owned()
    } else if let Some(ref ac) = e2e_config.harness.app_class {
        ac.as_str().to_owned()
    } else {
        String::new()
    };

    let register_route_method = e2e_config
        .harness
        .register_method_idiomatic("ruby")
        .unwrap_or_else(|| "register_route".to_string());

    let body_schema_setter = &e2e_config.harness.body_schema_setter;
    let method_enum = &e2e_config.harness.method_enum;

    let run_method_override = e2e_config.harness.run_method_for_lang("ruby");
    let run_method_str = if let Some(ref rm) = run_method_override {
        rm.as_str().to_owned()
    } else if let Some(ref rm) = e2e_config.harness.run_method {
        rm.as_str().to_owned()
    } else {
        "run".to_string()
    };
    let host = &e2e_config.harness.host;
    let port = e2e_config.harness.port;

    let header = hash::header(CommentStyle::Hash);

    let module_prefix = if imports_ref.is_empty() {
        String::new()
    } else {
        format!("{}::", ruby_module_name(&imports_ref[0]))
    };
    let method_enum_module = method_enum
        .as_deref()
        .map_or_else(|| format!("{module_prefix}Method"), str::to_string);
    let derived_app_class = format!("{module_prefix}App");
    let derived_route_builder_class = format!("{module_prefix}RouteBuilder");
    let derived_server_config_class = format!("{module_prefix}ServerConfig");

    let env = make_env();
    render(
        &env,
        "ruby/app_harness.rb.jinja",
        context! {
            header => header,
            imports => imports_ref,
            app_class => if app_class_str.is_empty() { derived_app_class.as_str() } else { app_class_str.as_str() },
            route_builder_class => derived_route_builder_class.as_str(),
            server_config_class => derived_server_config_class.as_str(),
            route_builder_schema_setter => body_schema_setter.as_deref().unwrap_or("request_schema_json"),
            method_enum_module => method_enum_module,
            register_route_method => register_route_method.as_str(),
            run_method => run_method_str.as_str(),
            response_body_field => e2e_config.harness.response_body_field.as_str(),
            host => host,
            port => port,
            fixtures_json => fixtures_json,
        },
    )
}

/// Render environment variable setup lines for `spec_helper.rb`.
/// Returns empty string if env is empty; otherwise returns alphabetically-sorted
/// `ENV[k] ||= v` assignments, each on its own line.
fn render_env_setup(env: &HashMap<String, String>) -> String {
    if env.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    let mut sorted_keys: Vec<_> = env.keys().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        let value = &env[key];
        let _ = writeln!(out, "ENV[{key:?}] ||= {value:?}");
    }

    out
}

/// Render the server-pattern `spec/spec_helper.rb` that spawns `app_harness.rb`.
///
/// This is the `uses_harness = true` branch of alef's `render_spec_helper`,
/// extracted verbatim so the extension can emit it instead of alef.
#[must_use]
pub fn render_spec_helper_server(e2e_config: &E2eConfig) -> String {
    let header = hash::header(CommentStyle::Hash);
    let mut out = header;
    out.push_str("# frozen_string_literal: true\n");

    let env_setup = render_env_setup(&e2e_config.env);
    if !env_setup.is_empty() {
        let _ = writeln!(out);
        out.push_str(&env_setup);
    }

    let harness_host = &e2e_config.harness.host;

    let _ = writeln!(out);
    let _ = writeln!(out, "require 'socket'");
    let _ = writeln!(out, "require 'open3'");
    let _ = writeln!(out, "require 'timeout'");
    let _ = writeln!(out);
    let harness_setup = format!(
        r#"# Spawn the app harness for server-pattern e2e tests.
# If SUT_URL is already set, a parent process started a shared harness.
# Use it as-is and do NOT spawn our own.
RSpec.configure do |config|
  config.before(:suite) do
    next if ENV['SUT_URL'] && !ENV['SUT_URL'].empty?
    harness_bin = File.expand_path('../app_harness.rb', __dir__)
    unless File.exist?(harness_bin)
      raise "app_harness.rb not found at #{{harness_bin}}"
    end
    # Spawn the harness and read its stdout to extract the dynamic port.
    @_harness_stdin, @_harness_stdout, @_harness_stderr, @_harness_thread = Open3.popen3('ruby', harness_bin)
    @_harness_pid = @_harness_thread.pid
    harness_port = nil
    deadline = Time.now + 15.0
    # Read stdout, collecting all HARNESS_PORT lines. The harness retries on bind
    # failure, so we may see multiple ports. Keep the latest one and verify it's reachable.
    latest_port = nil
    while Time.now < deadline
      if @_harness_thread.status.nil?
        # Process died; use the latest port if available
        harness_port = latest_port if latest_port
        break
      end
      begin
        Timeout.timeout(0.1) do
          line = @_harness_stdout.readline
          if line =~ /^HARNESS_PORT=(\d+)/
            latest_port = $1.to_i
          end
        end
      rescue Timeout::Error, EOFError, Errno::EAGAIN
        # Try to verify the latest port if we have one
        if latest_port
          begin
            TCPSocket.new('{harness_host}', latest_port).close
            harness_port = latest_port
            break  # Success: port is reachable
          rescue Errno::ECONNREFUSED, Errno::EHOSTUNREACH
            # Port not yet listening; keep polling
            sleep(0.05)
          end
        else
          sleep(0.05)
        end
      end
    end
    unless harness_port
      Process.kill('TERM', @_harness_pid) rescue nil
      msg = latest_port ? "App harness did not become reachable on {harness_host}:#{{latest_port}} within 15s" : "App harness did not report port within 15s"
      raise msg
    end
    url = "http://{harness_host}:#{{harness_port}}"
    ENV['SUT_URL'] = url
  end

  config.after(:suite) do
    if @_harness_pid
      Process.kill('TERM', @_harness_pid) rescue nil
      Process.wait(@_harness_pid, 5) rescue nil
    end
  end
end
"#,
    );
    out.push_str(&harness_setup);
    out
}

/// Synthesize a multipart body from the handler's body schema properties.
fn synthesize_multipart_body(props: &serde_json::Map<String, serde_json::Value>) -> String {
    const BOUNDARY: &str = "alef-boundary";
    let mut body = String::new();

    for (prop_name, prop_schema) in props {
        let is_binary = prop_schema
            .get("format")
            .and_then(|f| f.as_str())
            .is_some_and(|f| f == "binary");

        body.push_str(&format!(
            "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"{prop_name}\""
        ));

        if is_binary {
            body.push_str(&format!(
                "; filename=\"{prop_name}.txt\"\r\nContent-Type: text/plain\r\n\r\n"
            ));
            body.push_str("placeholder content");
        } else {
            body.push_str("\r\n\r\nsample");
        }

        body.push_str("\r\n");
    }

    body.push_str(&format!("--{BOUNDARY}--\r\n"));

    ruby_string_literal(&body)
}

/// Render an `RSpec` example for an HTTP server-pattern test fixture (SUT harness).
///
/// Ported verbatim from alef's `ruby/http.rs::render_http_example_sut`.
fn render_http_example_sut(out: &mut String, fixture: &Fixture, env: &Environment<'static>) {
    let Some(http) = &fixture.http else {
        return;
    };

    if http.expected_response.status_code == 101 {
        let description_literal = ruby_string_literal(&fixture.description);
        let method = http.request.method.to_uppercase();
        let path = &http.request.path;
        let rendered = render(
            env,
            "ruby/http_101_skip.jinja",
            context! {
                method => method,
                path => path,
                description => description_literal,
            },
        );
        out.push_str(&rendered);
        return;
    }

    let fn_name = sanitize_ident(&fixture.id);
    let description = &fixture.description;
    let desc_with_period = if description.ends_with('.') {
        description.clone()
    } else {
        format!("{description}.")
    };
    let description_literal = ruby_string_literal(&desc_with_period);

    let mut header_entries: Vec<String> = http
        .request
        .headers
        .iter()
        .map(|(k, v)| format!("      '{k}' => '{v}',"))
        .collect();
    header_entries.sort();
    let headers_ruby = if header_entries.is_empty() {
        "{}".to_string()
    } else {
        format!("{{\n{}\n    }}", header_entries.join("\n"))
    };

    let method = http.request.method.to_uppercase();
    let method_class = http_method_class(&method);
    let path = format!("/fixtures/{}{}", &fixture.id, &http.request.path);

    let content_type_lower = http
        .request
        .headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-type"))
        .map_or_else(
            || {
                http.request
                    .content_type
                    .as_ref()
                    .map(|ct| ct.to_ascii_lowercase())
                    .unwrap_or_default()
            },
            |(_, v)| v.to_ascii_lowercase(),
        );
    let is_multipart = content_type_lower
        .split(';')
        .next()
        .map(str::trim)
        .is_some_and(|t| t.eq_ignore_ascii_case("multipart/form-data"));

    let multipart_body_ruby = if is_multipart && http.request.body.is_none() {
        if let Some(schema) = &http.handler.body_schema {
            if schema.get("type").and_then(|t| t.as_str()) == Some("object") {
                if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
                    synthesize_multipart_body(props)
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let (has_body, body_ruby, is_raw_body) = if let Some(body) = &http.request.body {
        let is_raw = body.is_string();
        (true, json_to_ruby(body), is_raw)
    } else if is_multipart && !multipart_body_ruby.is_empty() {
        (true, multipart_body_ruby, true)
    } else {
        (false, String::new(), false)
    };

    let (has_text_body, text_ruby) = if let Some(serde_json::Value::String(s)) = &http.expected_response.body {
        (true, ruby_string_literal(s))
    } else {
        (false, String::new())
    };

    let (has_json_body, json_ruby) = if let Some(body) = &http.expected_response.body {
        if body.is_null() || body.is_string() && body.as_str() == Some("") {
            (false, String::new())
        } else if matches!(body, serde_json::Value::String(_)) {
            (false, String::new())
        } else {
            (true, json_to_ruby(body))
        }
    } else {
        (false, String::new())
    };

    let (has_partial_body, partial_body_checks) = if let Some(partial) = &http.expected_response.body_partial {
        if let Some(obj) = partial.as_object() {
            let checks: Vec<minijinja::Value> = obj
                .iter()
                .map(|(key, val)| {
                    let ruby_val = json_to_ruby(val);
                    context! {
                        key => key,
                        value => ruby_val,
                    }
                })
                .collect();
            (true, checks)
        } else {
            (false, Vec::new())
        }
    } else {
        (false, Vec::new())
    };

    let mut header_assertions: Vec<minijinja::Value> = Vec::new();
    let mut header_names: Vec<String> = http.expected_response.headers.keys().cloned().collect();
    header_names.sort();

    for name in header_names {
        let value = &http.expected_response.headers[&name];
        header_assertions.push(context! {
            name => name,
            assertion_type => "eq",
            value => value,
        });
    }

    let (has_validation_errors, validation_errors) = if http.expected_response.status_code == 422 {
        if let Some(body) = &http.expected_response.body {
            if let Some(obj) = body.as_object() {
                if let Some(errs) = obj.get("errors").and_then(|v| v.as_array()) {
                    let ve: Vec<minijinja::Value> = errs
                        .iter()
                        .filter_map(|err| {
                            let loc = err.get("loc").and_then(|l| l.as_array())?;
                            let msg = err.get("msg").and_then(|m| m.as_str())?;
                            let loc_ruby = loc.iter().map(json_to_ruby).collect::<Vec<_>>().join(", ");
                            let escaped = escape_ruby_single(msg);
                            Some(context! {
                                loc_ruby => loc_ruby,
                                escaped_msg => escaped,
                            })
                        })
                        .collect();
                    (true, ve)
                } else {
                    (false, Vec::new())
                }
            } else {
                (false, Vec::new())
            }
        } else {
            (false, Vec::new())
        }
    } else {
        (false, Vec::new())
    };

    let rendered = render(
        env,
        "ruby/http_test_sut.jinja",
        context! {
            fn_name => fn_name,
            description => description_literal,
            method => method,
            method_class => method_class,
            path => path,
            headers_ruby => headers_ruby,
            has_body => has_body,
            body_ruby => body_ruby,
            is_raw_body => is_raw_body,
            expected_status => http.expected_response.status_code,
            has_text_body => has_text_body,
            text_ruby => text_ruby,
            has_json_body => has_json_body,
            json_ruby => json_ruby,
            has_partial_body => has_partial_body,
            partial_body_checks => partial_body_checks,
            header_assertions => header_assertions,
            has_validation_errors => has_validation_errors,
            validation_errors => validation_errors,
        },
    );
    out.push_str(&rendered);
}

/// Render one `spec/{category}_spec.rb` for the server-pattern (`uses_harness` = true).
///
/// Only the HTTP-fixture path is exercised for server-pattern Ruby e2e suites:
/// non-HTTP fixtures go through the same skip-block handling as in alef.
fn render_spec_file_server(
    category: &str,
    fixtures: &[&Fixture],
    module_path: &str,
    gem_name: &str,
    env: &Environment<'static>,
) -> String {
    let has_http = fixtures.iter().any(|f| f.http.is_some());
    let mut requires = Vec::new();

    if has_http {
        requires.push("spec_helper".to_string());
    }

    let require_name = if module_path.is_empty() { gem_name } else { module_path };
    requires.push(require_name.replace('-', "_"));
    requires.push("json".to_string());

    let mut examples: Vec<String> = Vec::new();
    for fixture in fixtures {
        if fixture.http.is_some() {
            let mut out = String::new();
            render_http_example_sut(&mut out, fixture, env);
            examples.push(out);
        } else {
            let test_name = sanitize_ident(&fixture.id);
            let description_literal = ruby_string_literal(&format!("{test_name}: {}", fixture.description));
            let mut out = String::new();
            out.push_str(&format!("  it {description_literal} do\n"));
            out.push_str("    skip 'Fixture has no assertions to validate'\n");
            out.push_str("  end\n");
            examples.push(out);
        }
    }

    let header = hash::header(CommentStyle::Hash);
    render(
        env,
        "ruby/test_file.jinja",
        context! {
            category => category,
            requires => requires,
            has_array_contains => false,
            has_http => has_http,
            examples => examples,
            header => header,
        },
    )
}

/// Emit Ruby's server-pattern files.
///
/// Returns the server-pattern `GeneratedFile`s at `e2e/ruby/...`, gated identically
/// to alef's prior emission: HTTP fixtures present and a harness import configured.
///
/// Files produced:
/// - `e2e/ruby/app_harness.rb`
/// - `e2e/ruby/spec/spec_helper.rb`
/// - `e2e/ruby/spec/{category}_spec.rb` for every fixture group with active fixtures
pub fn emit(
    groups: &[FixtureGroup],
    e2e_config: &E2eConfig,
    config: &ResolvedCrateConfig,
) -> Result<Vec<GeneratedFile>> {
    let has_http = groups.iter().flat_map(|g| g.fixtures.iter()).any(|f| f.http.is_some());
    if !has_http || e2e_config.harness.imports.is_empty() {
        return Ok(vec![]);
    }

    let base = PathBuf::from(e2e_config.effective_output()).join("ruby");
    let spec_base = base.join("spec");

    let mut files: Vec<GeneratedFile> = Vec::new();

    files.push(GeneratedFile {
        path: base.join("app_harness.rb"),
        content: render_app_harness(e2e_config, groups),
        generated_header: true,
    });

    files.push(GeneratedFile {
        path: spec_base.join("spec_helper.rb"),
        content: render_spec_helper_server(e2e_config),
        generated_header: true,
    });

    let lang = "ruby";
    let call = &e2e_config.call;
    let overrides = call.overrides.get(lang);
    let module_path = overrides
        .and_then(|o| o.module.as_ref())
        .cloned()
        .unwrap_or_else(|| call.module.clone());
    let ruby_pkg = e2e_config.resolve_package("ruby");
    let gem_name = ruby_pkg
        .as_ref()
        .and_then(|p| p.name.as_ref())
        .cloned()
        .unwrap_or_else(|| config.name.replace('-', "_"));

    let env = make_env();

    for group in groups {
        let active: Vec<&Fixture> = group
            .fixtures
            .iter()
            .filter(|f| {
                if !e2e_config.exclude_categories.is_empty()
                    && e2e_config.exclude_categories.contains(&f.resolved_category())
                {
                    return false;
                }
                !f.skip.as_ref().is_some_and(|s| s.should_skip(lang))
            })
            .collect();

        if active.is_empty() {
            continue;
        }

        let has_any_output = active.iter().any(|f| f.http.is_some() || !f.assertions.is_empty());
        if !has_any_output {
            continue;
        }

        let filename = format!("{}_spec.rb", sanitize_filename(&group.category));
        let content = render_spec_file_server(&group.category, &active, &module_path, &gem_name, &env);
        files.push(GeneratedFile {
            path: spec_base.join(filename),
            content,
            generated_header: true,
        });
    }

    Ok(files)
}
