//! Glue between the CLI and `spikard_codegen::sql`.
//!
//! Reads schema DDL + annotated query files from disk, runs scythe's parser
//! and analyzer, builds the handler set via `spikard_codegen::sql`, and writes
//! `handlers.json` (route list), `openapi.json` (the spec), and
//! `spikard-sql.json` (sidecar) to the output directory.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use scythe_core::analyzer::AnalyzedQuery;
use scythe_core::catalog::Catalog;
use scythe_core::dialect::SqlDialect;
use scythe_core::parser::parse_query_with_dialect;
use spikard_codegen::sql::{BuildOptions, DecimalMode, LanguageBackend, OpenApiInfo, build_handler_set};

use super::TargetLanguage;
use super::engine::GeneratedAsset;

/// Output of [`generate_from_sql_dir`] — the three artifacts always written
/// to the output directory.
#[derive(Debug)]
pub struct SqlCodegenOutput {
    pub assets: Vec<GeneratedAsset>,
}

#[derive(Debug, Clone)]
pub struct SqlCodegenConfig {
    pub schema_paths: Vec<PathBuf>,
    pub queries_dir: PathBuf,
    pub output_dir: PathBuf,
    pub dialect: SqlDialect,
    pub languages: Vec<TargetLanguage>,
    pub decimal_mode: DecimalMode,
    pub strict: bool,
    pub emit_openapi: bool,
    pub api_title: String,
    pub api_version: String,
}

pub fn generate_from_sql_dir(config: SqlCodegenConfig) -> Result<SqlCodegenOutput> {
    let catalog = load_catalog(&config.schema_paths, &config.dialect)?;
    let queries = load_queries(&config.queries_dir, &config.dialect, &catalog)?;
    if queries.is_empty() {
        bail!(
            "No queries found in {}. Add at least one .sql file with `-- @name`, `-- @returns`, and `-- @http` annotations.",
            config.queries_dir.display()
        );
    }

    let info = OpenApiInfo::new(config.api_title.clone(), config.api_version.clone());
    let opts = BuildOptions {
        decimal_mode: config.decimal_mode,
        strict: config.strict,
    };

    let backends: Vec<LanguageBackend<'_>> = config.languages.iter().map(|lang| language_backend(*lang)).collect();

    let set = build_handler_set(&catalog, &queries, &info, &opts, &backends)
        .context("Failed to build handler set from SQL annotations")?;

    fs::create_dir_all(&config.output_dir)
        .with_context(|| format!("Failed to create output directory {}", config.output_dir.display()))?;

    let mut assets = Vec::new();

    let routes_path = config.output_dir.join("handlers.json");
    let routes_json = serde_json::to_string_pretty(&set.routes).context("Failed to serialize routes")?;
    fs::write(&routes_path, &routes_json).with_context(|| format!("Failed to write {}", routes_path.display()))?;
    assets.push(GeneratedAsset {
        path: routes_path,
        description: "SQL-derived route metadata".to_string(),
    });

    let sidecar_path = config.output_dir.join("spikard-sql.json");
    let sidecar_json = serde_json::to_string_pretty(&set.sidecar).context("Failed to serialize sidecar")?;
    fs::write(&sidecar_path, &sidecar_json).with_context(|| format!("Failed to write {}", sidecar_path.display()))?;
    assets.push(GeneratedAsset {
        path: sidecar_path,
        description: "Per-language SQL→handler sidecar".to_string(),
    });

    if config.emit_openapi {
        let openapi_path = config.output_dir.join("openapi.json");
        let openapi_json = serde_json::to_string_pretty(&set.openapi).context("Failed to serialize OpenAPI spec")?;
        fs::write(&openapi_path, &openapi_json)
            .with_context(|| format!("Failed to write {}", openapi_path.display()))?;
        assets.push(GeneratedAsset {
            path: openapi_path,
            description: "OpenAPI 3.1 spec derived from SQL annotations".to_string(),
        });
    }

    Ok(SqlCodegenOutput { assets })
}

fn load_catalog(schema_paths: &[PathBuf], dialect: &SqlDialect) -> Result<Catalog> {
    let mut ddl_strings: Vec<String> = Vec::new();
    for path in schema_paths {
        if path.is_dir() {
            for entry in fs::read_dir(path).with_context(|| format!("Failed to read schema dir {}", path.display()))? {
                let entry = entry?;
                if entry.file_type()?.is_file() && has_sql_extension(&entry.path()) {
                    ddl_strings.push(fs::read_to_string(entry.path())?);
                }
            }
        } else {
            ddl_strings.push(
                fs::read_to_string(path).with_context(|| format!("Failed to read schema file {}", path.display()))?,
            );
        }
    }
    if ddl_strings.is_empty() {
        bail!("No schema DDL found at the configured paths");
    }
    let refs: Vec<&str> = ddl_strings.iter().map(String::as_str).collect();
    Catalog::from_ddl_with_dialect(&refs, dialect).map_err(|e| anyhow!("Failed to build catalog: {}", e))
}

fn load_queries(queries_dir: &Path, dialect: &SqlDialect, catalog: &Catalog) -> Result<Vec<AnalyzedQuery>> {
    let mut entries: Vec<PathBuf> = if queries_dir.is_file() {
        vec![queries_dir.to_path_buf()]
    } else {
        fs::read_dir(queries_dir)
            .with_context(|| format!("Failed to read queries dir {}", queries_dir.display()))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file() && has_sql_extension(&e.path()))
            .map(|e| e.path())
            .collect()
    };
    entries.sort();

    let mut out = Vec::new();
    for path in entries {
        let body = fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
        for chunk in split_queries(&body) {
            if chunk.trim().is_empty() {
                continue;
            }
            let query = parse_query_with_dialect(chunk, dialect)
                .map_err(|e| anyhow!("Failed to parse query in {}: {}", path.display(), e))?;
            let analyzed = scythe_core::analyzer::analyze(catalog, &query)
                .map_err(|e| anyhow!("Failed to analyze query in {}: {}", path.display(), e))?;
            out.push(analyzed);
        }
    }
    Ok(out)
}

/// Split a `.sql` file into individual query blocks. Each block starts at the
/// first `-- @name` it contains; everything between two such markers is one
/// query (including its other annotations + SQL body).
fn split_queries(body: &str) -> Vec<&str> {
    let mut chunks = Vec::new();
    let mut start: Option<usize> = None;
    let mut last_pos = 0usize;
    for (idx, _line) in body.match_indices('\n').chain(std::iter::once((body.len(), ""))) {
        let line_start = last_pos;
        let line_end = idx;
        let line = &body[line_start..line_end];
        if line.trim_start().to_ascii_lowercase().starts_with("-- @name")
            || line.trim_start().to_ascii_lowercase().starts_with("--@name")
        {
            if let Some(s) = start {
                chunks.push(body[s..line_start].trim_end_matches('\n'));
            }
            start = Some(line_start);
        }
        last_pos = line_end + 1;
    }
    if let Some(s) = start {
        chunks.push(body[s..].trim_end_matches('\n'));
    }
    chunks
}

fn has_sql_extension(p: &Path) -> bool {
    p.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("sql"))
        .unwrap_or(false)
}

fn language_backend(lang: TargetLanguage) -> LanguageBackend<'static> {
    match lang {
        TargetLanguage::Python => LanguageBackend {
            name: "python",
            scythe_module: "queries",
            is_async: true,
            scythe_fn_for: &python_fn_name,
            lang_type_for: &python_lang_type,
        },
        TargetLanguage::TypeScript => LanguageBackend {
            name: "typescript",
            scythe_module: "./queries",
            is_async: true,
            scythe_fn_for: &camel_fn_name,
            lang_type_for: &typescript_lang_type,
        },
        TargetLanguage::Rust => LanguageBackend {
            name: "rust",
            scythe_module: "crate::queries",
            is_async: true,
            scythe_fn_for: &snake_fn_name,
            lang_type_for: &rust_lang_type,
        },
        TargetLanguage::Ruby => LanguageBackend {
            name: "ruby",
            scythe_module: "Queries",
            is_async: false,
            scythe_fn_for: &snake_fn_name,
            lang_type_for: &ruby_lang_type,
        },
        TargetLanguage::Php => LanguageBackend {
            name: "php",
            scythe_module: "Queries",
            is_async: false,
            scythe_fn_for: &camel_fn_name,
            lang_type_for: &php_lang_type,
        },
        TargetLanguage::Elixir => LanguageBackend {
            name: "elixir",
            scythe_module: "Queries",
            is_async: false,
            scythe_fn_for: &snake_fn_name,
            lang_type_for: &elixir_lang_type,
        },
    }
}

fn snake_fn_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 4);
    let mut prev_lower = false;
    for c in name.chars() {
        if c.is_ascii_uppercase() {
            if prev_lower {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
            prev_lower = false;
        } else {
            out.push(c);
            prev_lower = c.is_ascii_lowercase() || c.is_ascii_digit();
        }
    }
    out
}

fn camel_fn_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) => c.to_ascii_lowercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

fn python_fn_name(name: &str) -> String {
    snake_fn_name(name)
}

fn python_lang_type(neutral: &str, nullable: bool) -> String {
    let base = match neutral {
        n if n.starts_with("array<") => {
            return format!(
                "list[{}]{}",
                python_lang_type(&n[6..n.len() - 1], false),
                if nullable { " | None" } else { "" }
            );
        }
        "int16" | "int32" | "int64" => "int",
        "float32" | "float64" => "float",
        "string" => "str",
        "bool" => "bool",
        "bytes" => "bytes",
        "uuid" => "UUID",
        "date" => "date",
        "datetime" | "datetime_tz" => "datetime",
        "time" | "time_tz" => "time",
        "decimal" => "Decimal",
        "json" => "Any",
        _ => "Any",
    };
    if nullable {
        format!("{} | None", base)
    } else {
        base.to_string()
    }
}

fn typescript_lang_type(neutral: &str, nullable: bool) -> String {
    let base = match neutral {
        n if n.starts_with("array<") => {
            return format!(
                "{}[]{}",
                typescript_lang_type(&n[6..n.len() - 1], false),
                if nullable { " | null" } else { "" }
            );
        }
        "int16" | "int32" | "float32" | "float64" => "number",
        "int64" => "bigint",
        "string" | "uuid" | "date" | "datetime" | "datetime_tz" | "time" | "time_tz" | "decimal" => "string",
        "bool" => "boolean",
        "bytes" => "Uint8Array",
        "json" => "unknown",
        _ => "unknown",
    };
    if nullable {
        format!("{} | null", base)
    } else {
        base.to_string()
    }
}

fn rust_lang_type(neutral: &str, nullable: bool) -> String {
    let base = match neutral {
        n if n.starts_with("array<") => {
            return wrap_nullable_rust(format!("Vec<{}>", rust_lang_type(&n[6..n.len() - 1], false)), nullable);
        }
        "int16" => "i16",
        "int32" => "i32",
        "int64" => "i64",
        "float32" => "f32",
        "float64" => "f64",
        "string" => "String",
        "bool" => "bool",
        "bytes" => "Vec<u8>",
        "uuid" => "uuid::Uuid",
        "date" => "chrono::NaiveDate",
        "datetime" => "chrono::NaiveDateTime",
        "datetime_tz" => "chrono::DateTime<chrono::Utc>",
        "time" => "chrono::NaiveTime",
        "time_tz" => "chrono::NaiveTime",
        "decimal" => "rust_decimal::Decimal",
        "json" => "serde_json::Value",
        _ => "serde_json::Value",
    };
    wrap_nullable_rust(base.to_string(), nullable)
}

fn wrap_nullable_rust(t: String, nullable: bool) -> String {
    if nullable { format!("Option<{}>", t) } else { t }
}

fn ruby_lang_type(neutral: &str, nullable: bool) -> String {
    let base = match neutral {
        n if n.starts_with("array<") => {
            return format!(
                "Array<{}>{}",
                ruby_lang_type(&n[6..n.len() - 1], false),
                if nullable { "?" } else { "" }
            );
        }
        "int16" | "int32" | "int64" => "Integer",
        "float32" | "float64" => "Float",
        "string" | "uuid" => "String",
        "bool" => "Bool",
        "bytes" => "String",
        "date" => "Date",
        "datetime" | "datetime_tz" => "DateTime",
        "time" | "time_tz" => "Time",
        "decimal" => "BigDecimal",
        "json" => "Hash",
        _ => "Object",
    };
    if nullable {
        format!("{}?", base)
    } else {
        base.to_string()
    }
}

fn php_lang_type(neutral: &str, nullable: bool) -> String {
    let base = match neutral {
        n if n.starts_with("array<") => "array",
        "int16" | "int32" | "int64" => "int",
        "float32" | "float64" => "float",
        "string" | "uuid" | "date" | "datetime" | "datetime_tz" | "time" | "time_tz" | "decimal" | "bytes" => "string",
        "bool" => "bool",
        "json" => "mixed",
        _ => "mixed",
    };
    if nullable {
        format!("?{}", base)
    } else {
        base.to_string()
    }
}

fn elixir_lang_type(neutral: &str, nullable: bool) -> String {
    let base = match neutral {
        n if n.starts_with("array<") => {
            return format!(
                "[{}]{}",
                elixir_lang_type(&n[6..n.len() - 1], false),
                if nullable { " | nil" } else { "" }
            );
        }
        "int16" | "int32" | "int64" => "integer()",
        "float32" | "float64" => "float()",
        "string" | "uuid" | "date" | "datetime" | "datetime_tz" | "time" | "time_tz" | "decimal" => "String.t()",
        "bool" => "boolean()",
        "bytes" => "binary()",
        "json" => "map()",
        _ => "any()",
    };
    if nullable {
        format!("{} | nil", base)
    } else {
        base.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write(path: &Path, body: &str) {
        std::fs::write(path, body).unwrap();
    }

    #[test]
    fn split_queries_separates_at_at_name() {
        let body = "-- @name First\n-- @returns :one\nSELECT 1;\n\n-- @name Second\n-- @returns :many\nSELECT 2;\n";
        let chunks = split_queries(body);
        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].contains("First"));
        assert!(chunks[1].contains("Second"));
    }

    #[test]
    fn split_queries_handles_single_query() {
        let body = "-- @name Only\n-- @returns :one\nSELECT 1;";
        let chunks = split_queries(body);
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn end_to_end_smoke_writes_three_files() {
        let dir = tempdir().unwrap();
        let schema_path = dir.path().join("schema.sql");
        write(
            &schema_path,
            "CREATE TABLE users (id BIGSERIAL PRIMARY KEY, email TEXT NOT NULL);",
        );
        let queries_dir = dir.path().join("queries");
        std::fs::create_dir_all(&queries_dir).unwrap();
        write(
            &queries_dir.join("users.sql"),
            "-- @name GetUser\n-- @returns :one\n-- @http GET /users/{id}\nSELECT id, email FROM users WHERE id = $1;",
        );
        let output_dir = dir.path().join("out");
        let output = generate_from_sql_dir(SqlCodegenConfig {
            schema_paths: vec![schema_path],
            queries_dir,
            output_dir: output_dir.clone(),
            dialect: SqlDialect::PostgreSQL,
            languages: vec![TargetLanguage::Python],
            decimal_mode: DecimalMode::StringPattern,
            strict: false,
            emit_openapi: true,
            api_title: "Demo".into(),
            api_version: "0.1.0".into(),
        })
        .unwrap();
        assert_eq!(output.assets.len(), 3);
        assert!(output_dir.join("handlers.json").exists());
        assert!(output_dir.join("openapi.json").exists());
        assert!(output_dir.join("spikard-sql.json").exists());

        let openapi: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(output_dir.join("openapi.json")).unwrap()).unwrap();
        assert_eq!(openapi["openapi"], "3.1.0");
        assert!(openapi["paths"]["/users/{id}"]["get"].is_object());

        let sidecar: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(output_dir.join("spikard-sql.json")).unwrap()).unwrap();
        let entry = &sidecar["by_language"]["python"]["GetUser"];
        assert_eq!(entry["scythe_fn"], "get_user");
        assert_eq!(entry["scythe_module"], "queries");
    }

    #[test]
    fn snake_and_camel_helpers() {
        assert_eq!(snake_fn_name("GetUser"), "get_user");
        assert_eq!(snake_fn_name("ListActiveUsers"), "list_active_users");
        assert_eq!(camel_fn_name("GetUser"), "getUser");
    }

    #[test]
    fn python_lang_type_optional_wraps_with_none() {
        assert_eq!(python_lang_type("string", true), "str | None");
        assert_eq!(python_lang_type("int64", false), "int");
    }

    #[test]
    fn rust_lang_type_wraps_option() {
        assert_eq!(rust_lang_type("string", true), "Option<String>");
        assert_eq!(rust_lang_type("int32", false), "i32");
    }
}
