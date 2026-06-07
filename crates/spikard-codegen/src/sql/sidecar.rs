//! Per-language call metadata that crosses the boundary from spikard's SQL
//! module to the per-language handler-stub generators.
//!
//! The OpenAPI spec emitted by [`crate::sql::openapi_from_routes`] stays vanilla
//! — no `x-*` extensions — so any generic OpenAPI consumer sees a normal
//! document. The sidecar JSON carries everything spikard's per-language
//! generators need to replace `raise NotImplementedError("TODO")` stubs with
//! real bodies that call into scythe-generated query functions.

use std::collections::BTreeMap;

use scythe_core::analyzer::AnalyzedQuery;
use scythe_core::parser::QueryCommand;
use serde::{Deserialize, Serialize};

use super::annotations::HttpParamBinding;

/// Top-level sidecar: language → operation_id → entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sidecar {
    pub by_language: BTreeMap<String, BTreeMap<String, SidecarEntry>>,
}

impl Sidecar {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an entry for `(language, operation_id)`.
    pub fn insert(&mut self, language: &str, operation_id: &str, entry: SidecarEntry) {
        self.by_language
            .entry(language.to_string())
            .or_default()
            .insert(operation_id.to_string(), entry);
    }

    pub fn entry_for<'a>(&'a self, language: &str, operation_id: &str) -> Option<&'a SidecarEntry> {
        self.by_language.get(language).and_then(|m| m.get(operation_id))
    }
}

/// One handler's call info in one target language.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarEntry {
    /// Function name emitted by scythe's codegen backend in this language
    /// (already canonicalised: e.g. `get_user` for Python from `@name GetUser`).
    pub scythe_fn: String,
    /// Module/package path the function lives in (e.g. `queries`,
    /// `queries.users`). Per-language generators turn this into an import.
    pub scythe_module: String,
    /// Call arguments in the order scythe expects them, with sources tagged so
    /// the generator knows whether to pull from `request.path`,
    /// `request.query`, the body, or a header.
    pub params: Vec<SidecarParam>,
    /// Resolved return type in this language (e.g. `User` in Python with a
    /// dataclass, `Promise<User | null>` in TS).
    pub return_lang_type: String,
    /// Whether the scythe-generated function is `async fn` (Rust),
    /// `async def` (Python), `async`/`Promise` (TS), etc.
    pub is_async: bool,
    /// Drives how the generator wraps the call result (single row, array, exec,
    /// affected-rows count, etc.).
    pub command: QueryCommand,
}

/// One argument of a sidecar call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarParam {
    /// SQL parameter name as it appears in scythe's `AnalyzedParam` and in the
    /// scythe-generated function signature.
    pub name: String,
    /// Resolved language type (e.g. `int` in Python, `number` in TS,
    /// `Option<i32>` in Rust).
    pub lang_type: String,
    /// Where to pull the value from in the HTTP request.
    pub source: HttpParamBinding,
}

/// Build a sidecar entry from an `AnalyzedQuery` and a per-param binding map.
///
/// `lang_type_for` resolves a `(neutral_type, nullable)` pair to the target
/// language's type string. We deliberately take it as a closure so the SQL
/// module stays language-agnostic — callers supply scythe's own backend-aware
/// resolver per language.
pub fn build_sidecar_entry<F>(
    query: &AnalyzedQuery,
    bindings: &BTreeMap<String, HttpParamBinding>,
    scythe_module: &str,
    scythe_fn: &str,
    is_async: bool,
    lang_type_for: F,
) -> SidecarEntry
where
    F: Fn(&str, bool) -> String,
{
    let params = query
        .params
        .iter()
        .map(|p| {
            let source = bindings.get(&p.name).copied().unwrap_or(HttpParamBinding::Body);
            SidecarParam {
                name: p.name.clone(),
                lang_type: lang_type_for(&p.neutral_type, p.nullable),
                source,
            }
        })
        .collect();

    let return_lang_type = compose_return_type(query, &lang_type_for);

    SidecarEntry {
        scythe_fn: scythe_fn.to_string(),
        scythe_module: scythe_module.to_string(),
        params,
        return_lang_type,
        is_async,
        command: query.command.clone(),
    }
}

fn compose_return_type<F>(query: &AnalyzedQuery, lang_type_for: &F) -> String
where
    F: Fn(&str, bool) -> String,
{
    match query.command {
        QueryCommand::Exec => "void".to_string(),
        QueryCommand::ExecRows => "rows".to_string(),
        _ => {
            // Compose a tuple-style string of the row's column types; the
            // generator translates this into the language's row struct.
            let cols: Vec<String> = query
                .columns
                .iter()
                .map(|c| lang_type_for(&c.neutral_type, c.nullable))
                .collect();
            cols.join(", ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scythe_core::analyzer::{AnalyzedColumn, AnalyzedParam, AnalyzedQuery};
    use scythe_core::parser::QueryCommand;

    fn fake_query() -> AnalyzedQuery {
        AnalyzedQuery {
            name: "GetUser".to_string(),
            command: QueryCommand::One,
            sql: "SELECT id, name FROM users WHERE id = $1".to_string(),
            columns: vec![
                AnalyzedColumn {
                    name: "id".to_string(),
                    neutral_type: "int32".to_string(),
                    nullable: false,
                },
                AnalyzedColumn {
                    name: "name".to_string(),
                    neutral_type: "string".to_string(),
                    nullable: true,
                },
            ],
            params: vec![AnalyzedParam {
                name: "id".to_string(),
                neutral_type: "int32".to_string(),
                nullable: false,
                position: 1,
            }],
            deprecated: None,
            source_table: Some("users".to_string()),
            composites: vec![],
            enums: vec![],
            optional_params: vec![],
            group_by: None,
            custom: vec![],
        }
    }

    fn py_lang_type(neutral: &str, nullable: bool) -> String {
        let base = match neutral {
            "int32" | "int64" | "int16" => "int",
            "string" => "str",
            "bool" => "bool",
            _ => "Any",
        };
        if nullable {
            format!("{base} | None")
        } else {
            base.to_string()
        }
    }

    #[test]
    fn carries_scythe_module_and_fn() {
        let entry = build_sidecar_entry(
            &fake_query(),
            &BTreeMap::new(),
            "queries",
            "get_user",
            true,
            py_lang_type,
        );
        assert_eq!(entry.scythe_module, "queries");
        assert_eq!(entry.scythe_fn, "get_user");
        assert!(entry.is_async);
    }

    #[test]
    fn binds_params_from_map() {
        let mut bindings = BTreeMap::new();
        bindings.insert("id".to_string(), HttpParamBinding::Path);
        let entry = build_sidecar_entry(&fake_query(), &bindings, "queries", "get_user", true, py_lang_type);
        assert_eq!(entry.params.len(), 1);
        assert_eq!(entry.params[0].name, "id");
        assert_eq!(entry.params[0].source, HttpParamBinding::Path);
        assert_eq!(entry.params[0].lang_type, "int");
    }

    #[test]
    fn unbound_params_default_to_body() {
        let entry = build_sidecar_entry(
            &fake_query(),
            &BTreeMap::new(),
            "queries",
            "get_user",
            true,
            py_lang_type,
        );
        assert_eq!(entry.params[0].source, HttpParamBinding::Body);
    }

    #[test]
    fn return_type_lists_columns_for_one_command() {
        let entry = build_sidecar_entry(
            &fake_query(),
            &BTreeMap::new(),
            "queries",
            "get_user",
            true,
            py_lang_type,
        );
        assert_eq!(entry.return_lang_type, "int, str | None");
    }

    #[test]
    fn return_type_is_void_for_exec() {
        let mut q = fake_query();
        q.command = QueryCommand::Exec;
        let entry = build_sidecar_entry(&q, &BTreeMap::new(), "queries", "f", true, py_lang_type);
        assert_eq!(entry.return_lang_type, "void");
    }

    #[test]
    fn return_type_is_rows_for_exec_rows() {
        let mut q = fake_query();
        q.command = QueryCommand::ExecRows;
        let entry = build_sidecar_entry(&q, &BTreeMap::new(), "queries", "f", true, py_lang_type);
        assert_eq!(entry.return_lang_type, "rows");
    }

    #[test]
    fn sidecar_insert_and_lookup() {
        let mut sidecar = Sidecar::new();
        let entry = build_sidecar_entry(
            &fake_query(),
            &BTreeMap::new(),
            "queries",
            "get_user",
            true,
            py_lang_type,
        );
        sidecar.insert("python", "GetUser", entry);
        assert!(sidecar.entry_for("python", "GetUser").is_some());
        assert!(sidecar.entry_for("typescript", "GetUser").is_none());
    }

    #[test]
    fn sidecar_serializes_to_json() {
        let mut sidecar = Sidecar::new();
        let entry = build_sidecar_entry(
            &fake_query(),
            &BTreeMap::new(),
            "queries",
            "get_user",
            true,
            py_lang_type,
        );
        sidecar.insert("python", "GetUser", entry);
        let json = serde_json::to_string(&sidecar).unwrap();
        assert!(json.contains("\"by_language\""));
        assert!(json.contains("\"python\""));
        assert!(json.contains("\"GetUser\""));
    }
}
