/// Render a Swift deep accessor for `tool_calls[N]...` paths.
///
/// The flat `tool_calls` array is `[StreamToolCallRef]`.  Each element is a
/// swift-bridge opaque ref: the first field after an index (e.g. `function`)
/// is accessed with `.method()` (direct call on the non-optional ref).
/// All subsequent fields use `?.method()` (optional chaining) because each
/// intermediate method returns `Optional`.  The string leaf appends
/// `?.toString()` to convert `RustString?` to `String?`.
///
/// Example: `[0].function.name`
///   → `(root)[0].function()?.name()?.toString()`
pub(super) fn render_swift_tool_calls_deep(root_expr: &str, tail: &str) -> String {
    use heck::ToLowerCamelCase;
    let segs = parse_tail(tail);
    let mut expr = root_expr.to_string();
    // First-class `StreamToolCall` struct: every field is a Codable Swift
    // property (no parens). Index access on `[StreamToolCall]` returns the
    // element directly (non-optional). Subsequent `Optional` properties chain
    // with `?.`. Track whether the prior segment yielded a non-optional value:
    // - after `Index`, the element is non-optional → next field uses `.`
    // - after the first optional property access (`?.`), every subsequent
    //   field also uses `?.` (chained optional)
    let mut prev_is_optional = false;
    for seg in &segs {
        match seg {
            TailSeg::Index(n) => {
                expr = format!("({expr})[{n}]");
                prev_is_optional = false;
            }
            TailSeg::Field(f) => {
                let prop = f.to_lower_camel_case();
                let sep = if prev_is_optional { "?." } else { "." };
                expr = format!("{expr}{sep}{prop}");
                // All `StreamToolCall` fields (function/id/arguments) are
                // `Optional<...>` in the first-class binding, so chaining
                // henceforth uses `?.`.
                prev_is_optional = true;
            }
        }
    }
    expr
}

/// Render a rust deep accessor for `tool_calls[N]...` paths over the flattened
/// stream-chunk `tool_calls` iterator. Handles Option-wrapped fields by chaining
/// `as_ref().and_then(...)` so the final value is a `&str` (for name/id/arguments).
pub(super) fn render_rust_tool_calls_deep(chunks_var: &str, tail: &str) -> String {
    let segs = parse_tail(tail);
    // Locate index segment (rust uses .nth(n) on the iterator instead of [N] on a Vec)
    let idx = segs.iter().find_map(|s| match s {
        TailSeg::Index(n) => Some(*n),
        _ => None,
    });
    let field_segs: Vec<&str> = segs
        .iter()
        .filter_map(|s| match s {
            TailSeg::Field(f) => Some(f.as_str()),
            _ => None,
        })
        .collect();

    let base = format!(
        "{chunks_var}.iter().flat_map(|c| c.choices.iter().flat_map(|ch| ch.delta.tool_calls.iter().flatten()))"
    );
    let with_nth = match idx {
        Some(n) => format!("{base}.nth({n})"),
        None => base,
    };

    // Chain Option-aware field access. Every field on StreamToolCall is Option<...>;
    // the leaf (String fields) uses `.as_deref()` to project to `&str`.
    let mut expr = with_nth;
    for (i, f) in field_segs.iter().enumerate() {
        let is_leaf = i == field_segs.len() - 1;
        if is_leaf {
            expr = format!("{expr}.and_then(|x| x.{f}.as_deref())");
        } else {
            expr = format!("{expr}.and_then(|x| x.{f}.as_ref())");
        }
    }
    format!("{expr}.unwrap_or(\"\")")
}

/// Parse a deep-path tail (e.g. `[0].function.name`) into structured segments.
///
/// The tail always starts with either `[N]` (array index) or `.field`.
/// Returns a list of segments: `TailSeg::Index(N)` or `TailSeg::Field(name)`.
#[derive(Debug, PartialEq)]
pub(super) enum TailSeg {
    Index(usize),
    Field(String),
}

pub(super) fn parse_tail(tail: &str) -> Vec<TailSeg> {
    let mut segs = Vec::new();
    let mut rest = tail;
    while !rest.is_empty() {
        if let Some(inner) = rest.strip_prefix('[') {
            // Array index: `[N]`
            if let Some(close) = inner.find(']') {
                let idx_str = &inner[..close];
                if let Ok(idx) = idx_str.parse::<usize>() {
                    segs.push(TailSeg::Index(idx));
                }
                rest = &inner[close + 1..];
            } else {
                break;
            }
        } else if let Some(inner) = rest.strip_prefix('.') {
            // Field name: up to next `.` or `[`
            let end = inner.find(['.', '[']).unwrap_or(inner.len());
            segs.push(TailSeg::Field(inner[..end].to_string()));
            rest = &inner[end..];
        } else {
            break;
        }
    }
    segs
}

/// Render the full deep accessor expression by appending per-language tail
/// segments onto `root_expr`.
pub(super) fn render_deep_tail(root_expr: &str, tail: &str, lang: &str) -> String {
    use heck::{ToLowerCamelCase, ToPascalCase};

    let segs = parse_tail(tail);
    let mut out = root_expr.to_string();

    for seg in &segs {
        match (seg, lang) {
            (TailSeg::Index(n), "rust") => {
                out = format!("({out})[{n}]");
            }
            (TailSeg::Index(n), "java") => {
                out = format!("({out}).get({n})");
            }
            (TailSeg::Index(n), "kotlin") => {
                if *n == 0 {
                    out = format!("({out}).first()");
                } else {
                    out = format!("({out}).get({n})");
                }
            }
            (TailSeg::Index(n), "kotlin_android") => {
                if *n == 0 {
                    out = format!("({out}).first()");
                } else {
                    out = format!("({out})[{n}]");
                }
            }
            (TailSeg::Index(n), "elixir") => {
                out = format!("Enum.at({out}, {n})");
            }
            (TailSeg::Index(n), "zig") => {
                out = format!("({out}).items[{n}]");
            }
            (TailSeg::Index(n), "php") => {
                out = format!("({out})[{n}]");
            }
            (TailSeg::Index(n), _) => {
                // rust-like for go (but we handle Field differently), python, node, ts, kotlin, etc.
                out = format!("({out})[{n}]");
            }
            (TailSeg::Field(f), "rust") => {
                use heck::ToSnakeCase;
                out.push('.');
                out.push_str(&f.to_snake_case());
            }
            (TailSeg::Field(f), "go") => {
                use alef::codegen::naming::to_go_name;
                out.push('.');
                out.push_str(&to_go_name(f));
            }
            (TailSeg::Field(f), "java") => {
                out.push('.');
                out.push_str(&f.to_lower_camel_case());
                out.push_str("()");
            }
            (TailSeg::Field(f), "kotlin") => {
                // Use safe-call `?.` for all field accessors in Kotlin deep paths.
                // All streaming tool-call sub-fields (`function`, `id`, `name`,
                // `arguments`) are nullable in the generated Java records, so `?.`
                // is always correct here and prevents "non-null asserted call on
                // nullable receiver" compile errors.
                out.push_str("?.");
                out.push_str(&f.to_lower_camel_case());
                out.push_str("()");
            }
            (TailSeg::Field(f), "kotlin_android") => {
                // kotlin-android: Kotlin data classes use property access (no parens).
                out.push_str("?.");
                out.push_str(&f.to_lower_camel_case());
            }
            (TailSeg::Field(f), "csharp") => {
                out.push('.');
                out.push_str(&f.to_pascal_case());
            }
            (TailSeg::Field(f), "php") => {
                // Streaming PHP accessors operate on json_decoded stdClass with
                // snake_case property names (JSON wire format), not the camelCase
                // properties exposed on the PHP wrapper class. Use the raw field
                // name verbatim.
                out.push_str("->");
                out.push_str(f);
            }
            (TailSeg::Field(f), "elixir") => {
                out.push('.');
                out.push_str(f);
            }
            (TailSeg::Field(f), "zig") => {
                out.push('.');
                out.push_str(f);
            }
            (TailSeg::Field(f), "python" | "ruby") => {
                out.push('.');
                out.push_str(f);
            }
            // node, wasm, typescript, kotlin, dart, swift all use camelCase
            (TailSeg::Field(f), _) => {
                out.push('.');
                out.push_str(&f.to_lower_camel_case());
            }
        }
    }

    out
}
