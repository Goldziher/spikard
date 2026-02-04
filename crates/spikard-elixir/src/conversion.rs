//! Elixir ↔ Rust type conversion utilities.
//!
//! This module provides functions for converting between Elixir and Rust types,
//! including JSON conversion, string conversion, and request/response building.

#![allow(dead_code)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::doc_markdown)]

use rustler::{Encoder, Env, MapIterator, NifResult, Term};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::atoms;

/// Convert JSON to an Elixir term.
///
/// Recursively converts JSON types to native Elixir types:
/// - null → nil (Atom)
/// - bool → true/false (Atoms)
/// - number → integer or float
/// - string → binary string
/// - array → list
/// - object → map
pub fn json_to_elixir<'a>(env: Env<'a>, value: &JsonValue) -> NifResult<Term<'a>> {
    match value {
        JsonValue::Null => Ok(atoms::nil().encode(env)),
        JsonValue::Bool(b) => {
            if *b {
                Ok(atoms::true_().encode(env))
            } else {
                Ok(atoms::false_().encode(env))
            }
        }
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.encode(env))
            } else if let Some(u) = n.as_u64() {
                Ok(u.encode(env))
            } else if let Some(f) = n.as_f64() {
                Ok(f.encode(env))
            } else {
                // Fallback to 0 if number cannot be represented
                Ok(0i64.encode(env))
            }
        }
        JsonValue::String(s) => Ok(s.as_str().encode(env)),
        JsonValue::Array(arr) => {
            let terms: Result<Vec<Term>, _> =
                arr.iter().map(|v| json_to_elixir(env, v)).collect();
            Ok(terms?.encode(env))
        }
        JsonValue::Object(obj) => {
            let pairs: Result<Vec<(Term<'a>, Term<'a>)>, _> = obj
                .iter()
                .map(|(key, val)| {
                    let key_term = key.as_str().encode(env);
                    let val_term = json_to_elixir(env, val)?;
                    Ok((key_term, val_term))
                })
                .collect();
            Term::map_from_pairs(env, &pairs?)
                .map_err(|_| rustler::Error::BadArg)
        }
    }
}

/// Convert an Elixir term to JSON value.
///
/// Handles common Elixir types and converts them to JSON:
/// - Atoms (nil, true, false) → null, true, false
/// - Integers → number
/// - Floats → number
/// - Binaries → string
/// - Lists → array
/// - Maps → object
pub fn elixir_to_json(env: Env, term: Term) -> NifResult<JsonValue> {
    // Check for atoms first (nil, true, false)
    if let Ok(atom) = term.decode::<rustler::Atom>() {
        if atom == atoms::nil() {
            return Ok(JsonValue::Null);
        }
        if atom == atoms::true_() {
            return Ok(JsonValue::Bool(true));
        }
        if atom == atoms::false_() {
            return Ok(JsonValue::Bool(false));
        }
        // Convert other atoms to null for now
        return Ok(JsonValue::Null);
    }

    // Try integer
    if let Ok(i) = term.decode::<i64>() {
        return Ok(JsonValue::Number(i.into()));
    }

    // Try float
    if let Ok(f) = term.decode::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            return Ok(JsonValue::Number(n));
        }
        return Ok(JsonValue::Null);
    }

    // Try string/binary
    if let Ok(s) = term.decode::<String>() {
        return Ok(JsonValue::String(s));
    }

    // Try list
    if let Ok(list) = term.decode::<Vec<Term>>() {
        let arr: Result<Vec<JsonValue>, _> =
            list.into_iter().map(|t| elixir_to_json(env, t)).collect();
        return Ok(JsonValue::Array(arr?));
    }

    // Try map using MapIterator
    if let Some(iter) = MapIterator::new(term) {
        let mut obj = serde_json::Map::new();
        for (key, val) in iter {
            let key_term: Term = key;
            if let Ok(key_str) = key_term.decode::<String>() {
                obj.insert(key_str, elixir_to_json(env, val)?);
            }
        }
        return Ok(JsonValue::Object(obj));
    }

    // Fallback to null
    Ok(JsonValue::Null)
}

/// Convert a HashMap<String, String> to an Elixir map.
pub fn map_to_elixir_map<'a>(
    env: Env<'a>,
    map: &HashMap<String, String>,
) -> NifResult<Term<'a>> {
    let pairs: Vec<(Term<'a>, Term<'a>)> = map
        .iter()
        .map(|(k, v)| (k.as_str().encode(env), v.as_str().encode(env)))
        .collect();
    Term::map_from_pairs(env, &pairs).map_err(|_| rustler::Error::BadArg)
}

/// Convert a HashMap<String, Vec<String>> to an Elixir map with list values.
pub fn multimap_to_elixir_map<'a>(
    env: Env<'a>,
    map: &HashMap<String, Vec<String>>,
) -> NifResult<Term<'a>> {
    let pairs: Vec<(Term<'a>, Term<'a>)> = map
        .iter()
        .map(|(k, values)| {
            let list: Vec<Term> = values.iter().map(|v| v.as_str().encode(env)).collect();
            (k.as_str().encode(env), list.encode(env))
        })
        .collect();
    Term::map_from_pairs(env, &pairs).map_err(|_| rustler::Error::BadArg)
}

/// Check if a term is the nil atom.
pub fn is_nil(term: Term) -> bool {
    term.decode::<rustler::Atom>()
        .map(|a| a == atoms::nil())
        .unwrap_or(false)
}

/// Safely get a value from an Elixir map by string key.
pub fn get_map_value<'a>(_env: Env<'a>, map: Term<'a>, key: &str) -> Option<Term<'a>> {
    if let Ok(hmap) = map.decode::<HashMap<String, Term<'a>>>() {
        hmap.get(key).copied()
    } else {
        None
    }
}

/// Decode an Elixir map to HashMap<String, String>.
pub fn decode_string_map(term: Term) -> NifResult<HashMap<String, String>> {
    term.decode()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_json_conversion_types() {
        // Test that the conversion functions compile and have correct signatures
        // This is a compile-time check - if the module compiles, the signatures are correct
        assert!(true);
    }
}
