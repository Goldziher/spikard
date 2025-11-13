use anyhow::{Context, Result, bail};
use spikard_codegen::openapi::Fixture;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct MiddlewareMetadata {
    pub compression: Option<CompressionSettings>,
    pub rate_limit: Option<RateLimitSettings>,
    pub request_timeout: Option<RequestTimeoutSettings>,
    pub request_id: Option<RequestIdSettings>,
    pub body_limit: Option<BodyLimitSettings>,
    pub static_dirs: Vec<StaticDirectory>,
}

#[derive(Debug, Clone)]
pub struct CompressionSettings {
    pub gzip: Option<bool>,
    pub brotli: Option<bool>,
    pub min_size: Option<usize>,
    pub quality: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct RateLimitSettings {
    pub per_second: u64,
    pub burst: u32,
    pub ip_based: Option<bool>,
    pub warmup_requests: usize,
    pub warmup_expect_status: Option<u16>,
    pub sleep_ms_between: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RequestTimeoutSettings {
    pub seconds: u64,
    pub sleep_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RequestIdSettings {
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct BodyLimitSettings {
    pub max_bytes: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct StaticDirectory {
    pub route_prefix: String,
    pub directory_name: String,
    pub index_file: bool,
    pub cache_control: Option<String>,
    pub files: Vec<StaticFile>,
}

#[derive(Debug, Clone)]
pub struct StaticFile {
    pub relative_path: String,
    pub content: String,
}

pub fn parse_middleware(fixture: &Fixture) -> Result<MiddlewareMetadata> {
    let mut metadata = MiddlewareMetadata::default();
    let handler = match &fixture.handler {
        Some(handler) => handler,
        None => return Ok(metadata),
    };

    let middleware = match &handler.middleware {
        Some(value) => value,
        None => return Ok(metadata),
    };

    if let Some(compression) = middleware.get("compression").and_then(|v| v.as_object()) {
        metadata.compression = Some(CompressionSettings {
            gzip: compression.get("gzip").and_then(|v| v.as_bool()),
            brotli: compression.get("brotli").and_then(|v| v.as_bool()),
            min_size: compression.get("min_size").and_then(|v| v.as_u64()).map(|v| v as usize),
            quality: compression.get("quality").and_then(|v| v.as_u64()).map(|v| v as u32),
        });
    }

    if let Some(rate_limit) = middleware.get("rate_limit").and_then(|v| v.as_object()) {
        let per_second = rate_limit
            .get("per_second")
            .and_then(|v| v.as_u64())
            .context("rate_limit.per_second must be an integer")?;
        let burst = rate_limit
            .get("burst")
            .and_then(|v| v.as_u64())
            .context("rate_limit.burst must be an integer")? as u32;
        let warmup_requests = rate_limit.get("warmup_requests").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let warmup_expect_status = rate_limit
            .get("warmup_expect_status")
            .and_then(|v| v.as_u64())
            .map(|v| v as u16);
        let sleep_ms_between = rate_limit.get("sleep_ms_between").and_then(|v| v.as_u64());

        let ip_based = rate_limit.get("ip_based").and_then(|v| v.as_bool()).unwrap_or(false);

        metadata.rate_limit = Some(RateLimitSettings {
            per_second,
            burst,
            ip_based: Some(ip_based),
            warmup_requests,
            warmup_expect_status,
            sleep_ms_between,
        });
    }

    if let Some(timeout) = middleware.get("request_timeout").and_then(|v| v.as_object()) {
        let seconds = timeout
            .get("seconds")
            .and_then(|v| v.as_u64())
            .context("request_timeout.seconds must be an integer")?;
        let sleep_ms = timeout.get("sleep_ms").and_then(|v| v.as_u64());
        metadata.request_timeout = Some(RequestTimeoutSettings { seconds, sleep_ms });
    }

    if let Some(request_id) = middleware.get("request_id").and_then(|v| v.as_object()) {
        metadata.request_id = Some(RequestIdSettings {
            enabled: request_id.get("enabled").and_then(|v| v.as_bool()),
        });
    }

    if let Some(body_limit) = middleware.get("body_limit").and_then(|v| v.as_object()) {
        metadata.body_limit = Some(BodyLimitSettings {
            max_bytes: body_limit.get("max_bytes").and_then(|v| v.as_u64()).map(|v| v as usize),
        });
    }

    if let Some(static_files) = middleware.get("static_files").and_then(|v| v.as_array()) {
        let mut seen_dirs = HashSet::new();
        let mut dirs = Vec::new();
        for (index, entry) in static_files.iter().enumerate() {
            let obj = entry.as_object().context("static_files entries must be objects")?;
            let route_prefix = obj
                .get("route_prefix")
                .and_then(|v| v.as_str())
                .context("static_files.route_prefix must be a string")?
                .to_string();
            let directory_name = obj
                .get("directory_name")
                .and_then(|v| v.as_str())
                .map(sanitize_segment)
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| {
                    let base = route_prefix.trim_matches('/').replace('/', "_");
                    let fallback = if base.is_empty() { "static" } else { base.as_str() };
                    format!("{}_{index}", sanitize_segment(fallback))
                });
            if !seen_dirs.insert(directory_name.clone()) {
                bail!("static_files directory_name '{}' is duplicated", directory_name);
            }
            let index_file = obj.get("index_file").and_then(|v| v.as_bool()).unwrap_or(true);
            let cache_control = obj.get("cache_control").and_then(|v| v.as_str()).map(|s| s.to_string());
            let files_val = obj
                .get("files")
                .and_then(|v| v.as_array())
                .context("static_files entry missing files array")?;
            if files_val.is_empty() {
                bail!(
                    "static_files entry for route_prefix '{}' must declare at least one file",
                    route_prefix
                );
            }
            let mut files = Vec::new();
            for file in files_val {
                let file_obj = file.as_object().context("static file entries must be objects")?;
                let relative_path = file_obj
                    .get("path")
                    .and_then(|v| v.as_str())
                    .context("static file entry missing path")?;
                if relative_path.contains("..") {
                    bail!("static file path '{}' may not contain '..'", relative_path);
                }
                let content = file_obj
                    .get("content")
                    .and_then(|v| v.as_str())
                    .context("static file entry missing content")?;
                files.push(StaticFile {
                    relative_path: relative_path.to_string(),
                    content: content.to_string(),
                });
            }
            dirs.push(StaticDirectory {
                route_prefix,
                directory_name,
                index_file,
                cache_control,
                files,
            });
        }
        metadata.static_dirs = dirs;
    }

    Ok(metadata)
}

pub fn write_static_assets(base_dir: &Path, fixture_slug: &str, dirs: &[StaticDirectory]) -> Result<()> {
    if dirs.is_empty() {
        return Ok(());
    }

    let root = base_dir.join("static_assets").join(fixture_slug);
    fs::create_dir_all(&root).with_context(|| format!("Failed to create {}", root.display()))?;

    for dir in dirs {
        let dir_path = root.join(&dir.directory_name);
        fs::create_dir_all(&dir_path).with_context(|| format!("Failed to create static dir {}", dir_path.display()))?;
        for file in &dir.files {
            let file_path = dir_path.join(&file.relative_path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).with_context(|| format!("Failed to create {}", parent.display()))?;
            }
            fs::write(&file_path, &file.content).with_context(|| format!("Failed to write {}", file_path.display()))?;
        }
    }

    Ok(())
}

fn sanitize_segment(input: &str) -> String {
    let mut result = input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>();
    while result.contains("--") {
        result = result.replace("--", "-");
    }
    result.trim_matches('-').to_string()
}
