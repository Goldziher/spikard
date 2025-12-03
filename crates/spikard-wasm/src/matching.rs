use crate::types::RouteDefinition;
use percent_encoding::percent_decode_str;
use serde_json::Value;
use std::collections::HashMap;
use url::form_urlencoded;
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
pub struct QueryParams {
    pub normalized: HashMap<String, Value>,
    pub raw: HashMap<String, Vec<String>>,
}

pub fn match_route(
    routes: &[RouteDefinition],
    method: &str,
    path: &str,
) -> Result<(RouteDefinition, HashMap<String, String>, String, QueryParams), JsValue> {
    let (path_only, query_params) = split_path_and_query(path);

    for route in routes {
        if !route.method.eq_ignore_ascii_case(method) {
            continue;
        }
        if let Some(params) = match_path_segments(&route.path, &path_only) {
            return Ok((route.clone(), params, path_only, query_params.clone()));
        }
    }

    Err(JsValue::from_str(&format!("No route for {method} {path}")))
}

fn split_path_and_query(path: &str) -> (String, QueryParams) {
    if let Some((path_only, query)) = path.split_once('?') {
        (path_only.to_string(), parse_query(query))
    } else {
        (path.to_string(), QueryParams::default())
    }
}

fn parse_query(query: &str) -> QueryParams {
    if query.is_empty() {
        return QueryParams::default();
    }

    let mut normalized = HashMap::new();
    let mut raw = HashMap::new();
    for (key, value) in form_urlencoded::parse(query.as_bytes()) {
        let key = key.into_owned();
        let value = value.into_owned();
        raw.entry(key.clone()).or_insert_with(Vec::new).push(value.clone());
        normalized
            .entry(key)
            .and_modify(|v| {
                if let Value::Array(arr) = v {
                    arr.push(Value::String(value.clone()));
                }
            })
            .or_insert_with(|| Value::Array(vec![Value::String(value)]));
    }
    QueryParams { normalized, raw }
}

fn match_path_segments(template: &str, actual: &str) -> Option<HashMap<String, String>> {
    let template_segments = split_segments(template);
    let actual_segments = split_segments(actual);
    let mut params = HashMap::new();
    let mut actual_idx = 0;

    for template_segment in template_segments.iter() {
        if let Some((name, kind)) = parse_template_segment(template_segment) {
            match kind {
                SegmentKind::GreedyPath => {
                    let remaining = actual_segments.get(actual_idx..)?.join("/");
                    let decoded = percent_decode_str(&remaining).decode_utf8().ok()?.to_string();
                    params.insert(name, decoded);
                    actual_idx = actual_segments.len();
                    break;
                }
                SegmentKind::Normal => {
                    let actual_segment = actual_segments.get(actual_idx)?;
                    let decoded = percent_decode_str(actual_segment).decode_utf8().ok()?.to_string();
                    params.insert(name, decoded);
                    actual_idx += 1;
                }
            }
        } else {
            let actual_segment = actual_segments.get(actual_idx)?;
            if template_segment != actual_segment {
                return None;
            }
            actual_idx += 1;
        }
    }

    if actual_idx == actual_segments.len() {
        Some(params)
    } else {
        None
    }
}

fn split_segments(path: &str) -> Vec<&str> {
    path.split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
}

fn parse_template_segment(segment: &str) -> Option<(String, SegmentKind)> {
    if !segment.starts_with('{') || !segment.ends_with('}') {
        return None;
    }
    let inner = segment.trim_start_matches('{').trim_end_matches('}');
    if inner.is_empty() {
        return None;
    }
    let mut parts = inner.splitn(2, ':');
    let name = parts.next()?.to_string();
    let kind = match parts.next() {
        Some("path") => SegmentKind::GreedyPath,
        _ => SegmentKind::Normal,
    };
    Some((name, kind))
}

enum SegmentKind {
    Normal,
    GreedyPath,
}
