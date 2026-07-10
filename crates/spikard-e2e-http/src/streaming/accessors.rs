use super::model::{StreamingFieldResolver, split_streaming_deep_path};
use super::renderers::{render_deep_tail, render_rust_tool_calls_deep, render_swift_tool_calls_deep};

impl StreamingFieldResolver {
    /// Returns the language-specific expression for a streaming-virtual field,
    /// given `chunks_var` (the collected-list local name) and `lang`.
    ///
    /// Returns `None` when the field name is not a known streaming-virtual
    /// field or the language has no streaming support.
    ///
    /// `module_qualifier` carries the per-project module/crate name used by the
    /// Rust and C# `stream.has_*_event` branches to construct the streaming
    /// union type path. Pass the cargo crate name (`snake_case`) for Rust callers
    /// and the C# namespace (`PascalCase`) for C# callers. When `None` is
    /// supplied for those branches, the accessor returns `None` so the call
    /// site can skip the assertion rather than emit code referencing an unknown
    /// type.
    #[must_use]
    pub fn accessor(field: &str, lang: &str, chunks_var: &str) -> Option<String> {
        Self::accessor_with_module_qualifier(field, lang, chunks_var, None)
    }

    /// Same as [`Self::accessor`] but accepts a per-project module qualifier
    /// for the `stream.has_*_event` branches that emit a streaming union type
    /// path.
    ///
    /// This wrapper does not guess an event item type. Event-variant fields
    /// return `None` unless callers use [`Self::accessor_with_streaming_context`]
    /// with an explicit or adapter-inferred `item_type`.
    #[must_use]
    pub fn accessor_with_module_qualifier(
        field: &str,
        lang: &str,
        chunks_var: &str,
        module_qualifier: Option<&str>,
    ) -> Option<String> {
        Self::accessor_with_streaming_context(field, lang, chunks_var, module_qualifier, None)
    }

    /// Same as [`Self::accessor_with_module_qualifier`] but also accepts the
    /// unqualified name of the streaming union item type.
    ///
    /// When `item_type` is `None` the `stream.has_*_event` branches return
    /// `None`, so the call site can skip or diagnose the assertion rather than
    /// emitting a reference to an unknown project type.
    #[must_use]
    pub fn accessor_with_streaming_context(
        field: &str,
        lang: &str,
        chunks_var: &str,
        module_qualifier: Option<&str>,
        item_type: Option<&str>,
    ) -> Option<String> {
        match field {
            "stream.items" | "chunks" => Some(match lang {
                "zig" => format!("{chunks_var}.items"),
                "php" => format!("${chunks_var}"),
                _ => chunks_var.to_string(),
            }),

            "stream.items.length" | "chunks.length" => Some(match lang {
                "rust" => format!("{chunks_var}.len()"),
                "go" => format!("len({chunks_var})"),
                "python" => format!("len({chunks_var})"),
                "php" => format!("count(${chunks_var})"),
                "elixir" => format!("length({chunks_var})"),
                "kotlin" => format!("{chunks_var}.size"),
                "zig" => format!("{chunks_var}.items.len"),
                "swift" => format!("{chunks_var}.count"),
                _ => format!("{chunks_var}.length"),
            }),

            "stream_content" => Some(match lang {
                "rust" => {
                    format!(
                        "{chunks_var}.iter().map(|c| c.choices.first().and_then(|ch| ch.delta.content.as_deref()).unwrap_or(\"\")).collect::<String>()"
                    )
                }
                "go" => {
                    format!(
                        "func() string {{ var s string; for _, c := range {chunks_var} {{ if len(c.Choices) > 0 && c.Choices[0].Delta.Content != nil {{ s += *c.Choices[0].Delta.Content }} }}; return s }}()"
                    )
                }
                "java" => {
                    format!(
                        "{chunks_var}.stream().map(c -> c.choices().stream().findFirst().map(ch -> ch.delta().content() != null ? ch.delta().content() : \"\").orElse(\"\")).collect(java.util.stream.Collectors.joining())"
                    )
                }
                "php" => {
                    format!("implode('', array_map(fn($c) => $c->choices[0]->delta->content ?? '', ${chunks_var}))")
                }
                "kotlin" => {
                    format!(
                        "{chunks_var}.joinToString(\"\") {{ it.choices()?.firstOrNull()?.delta()?.content() ?: \"\" }}"
                    )
                }
                "kotlin_android" => {
                    format!("{chunks_var}.joinToString(\"\") {{ it.choices?.firstOrNull()?.delta?.content ?: \"\" }}")
                }
                "elixir" => {
                    format!(
                        "{chunks_var} |> Enum.map(fn c -> (Enum.at(c.choices, 0) || %{{}}) |> Map.get(:delta, %{{}}) |> Map.get(:content, \"\") end) |> Enum.join(\"\")"
                    )
                }
                "python" => {
                    format!("\"\".join(c.choices[0].delta.content or \"\" for c in {chunks_var} if c.choices)")
                }
                "zig" => {
                    format!("{chunks_var}_content.items")
                }
                "swift" => {
                    format!(
                        "{chunks_var}.map {{ c in c.choices.first.flatMap {{ ch in ch.delta.content }} ?? \"\" }}.joined()"
                    )
                }
                "ruby" => {
                    format!("{chunks_var}.map {{ |c| c.choices.first&.delta&.content || '' }}.join")
                }
                _ => {
                    format!("{chunks_var}.map((c: any) => c.choices?.[0]?.delta?.content ?? '').join('')")
                }
            }),

            "stream_complete" => Some(match lang {
                "rust" => {
                    format!(
                        "{chunks_var}.last().and_then(|c| c.choices.first()).and_then(|ch| ch.finish_reason.as_ref()).is_some()"
                    )
                }
                "go" => {
                    format!(
                        "func() bool {{ if len({chunks_var}) == 0 {{ return false }}; last := {chunks_var}[len({chunks_var})-1]; return len(last.Choices) > 0 && last.Choices[0].FinishReason != nil }}()"
                    )
                }
                "java" => {
                    format!(
                        "!{chunks_var}.isEmpty() && {chunks_var}.get({chunks_var}.size()-1).choices().stream().findFirst().flatMap(ch -> java.util.Optional.ofNullable(ch.finishReason())).isPresent()"
                    )
                }
                "php" => {
                    format!("!empty(${chunks_var}) && isset(end(${chunks_var})->choices[0]->finishReason)")
                }
                "kotlin" => {
                    format!(
                        "{chunks_var}.isNotEmpty() && {chunks_var}.last().choices()?.firstOrNull()?.finishReason() != null"
                    )
                }
                "kotlin_android" => {
                    format!(
                        "{chunks_var}.isNotEmpty() && {chunks_var}.last().choices?.firstOrNull()?.finishReason != null"
                    )
                }
                "python" => {
                    format!("bool({chunks_var}) and {chunks_var}[-1].choices[0].finish_reason is not None")
                }
                "elixir" => {
                    format!("Enum.at(List.last({chunks_var}).choices, 0).finish_reason != nil")
                }
                "zig" => {
                    format!("{chunks_var}.items.len > 0")
                }
                "swift" => {
                    format!("!{chunks_var}.isEmpty && {chunks_var}.last!.choices.first?.finishReason != nil")
                }
                "ruby" => {
                    format!("!{chunks_var}.empty? && !{chunks_var}.last&.choices&.first&.finish_reason.nil?")
                }
                _ => {
                    format!(
                        "{chunks_var}.length > 0 && {chunks_var}[{chunks_var}.length - 1].choices?.[0]?.finishReason != null"
                    )
                }
            }),

            "no_chunks_after_done" => Some(match lang {
                "rust" => "true".to_string(),
                "go" => "true".to_string(),
                "java" => "true".to_string(),
                "php" => "true".to_string(),
                _ => "true".to_string(),
            }),

            "stream.has_page_event" => item_type
                .and_then(|ty| has_event_variant_accessor(lang, chunks_var, EventVariant::Page, ty, module_qualifier)),
            "stream.has_error_event" => item_type
                .and_then(|ty| has_event_variant_accessor(lang, chunks_var, EventVariant::Error, ty, module_qualifier)),
            "stream.has_complete_event" => item_type.and_then(|ty| {
                has_event_variant_accessor(lang, chunks_var, EventVariant::Complete, ty, module_qualifier)
            }),

            "stream.event_count_min" => Some(match lang {
                "java" => format!("{chunks_var}.size()"),
                "go" => format!("len({chunks_var})"),
                "php" => format!("count(${chunks_var})"),
                "kotlin" | "kotlin_android" => format!("{chunks_var}.size"),
                "python" => format!("len({chunks_var})"),
                "rust" => format!("{chunks_var}.len()"),
                "node" | "typescript" | "wasm" => format!("{chunks_var}.length"),
                "swift" => format!("{chunks_var}.count"),
                "zig" => format!("{chunks_var}.items.len"),
                "ruby" => format!("{chunks_var}.length"),
                "elixir" => format!("length({chunks_var})"),
                "c" => format!("vlen({chunks_var})"),
                _ => format!("{chunks_var}.length"),
            }),

            "tool_calls" => Some(match lang {
                "rust" => {
                    format!(
                        "{chunks_var}.iter().flat_map(|c| c.choices.iter().flat_map(|ch| ch.delta.tool_calls.iter().flatten())).collect::<Vec<_>>()"
                    )
                }
                "go" => {
                    format!(
                        "func() []pkg.StreamToolCall {{ var tc []pkg.StreamToolCall; for _, c := range {chunks_var} {{ for _, ch := range c.Choices {{ tc = append(tc, ch.Delta.ToolCalls...) }} }}; return tc }}()"
                    )
                }
                "java" => {
                    format!(
                        "{chunks_var}.stream().flatMap(c -> c.choices().stream()).flatMap(ch -> ch.delta().toolCalls() != null ? ch.delta().toolCalls().stream() : java.util.stream.Stream.empty()).toList()"
                    )
                }
                "php" => {
                    format!(
                        "array_merge(...array_map(fn($c) => $c->choices[0]->delta->toolCalls ?? [], ${chunks_var}))"
                    )
                }
                "kotlin" => {
                    format!(
                        "{chunks_var}.flatMap {{ c -> c.choices()?.flatMap {{ ch -> ch.delta()?.toolCalls() ?: emptyList() }} ?: emptyList() }}"
                    )
                }
                "kotlin_android" => {
                    format!(
                        "{chunks_var}.flatMap {{ c -> c.choices?.flatMap {{ ch -> ch.delta?.toolCalls ?: emptyList() }} ?: emptyList() }}"
                    )
                }
                "python" => {
                    format!(
                        "[t for c in {chunks_var} for ch in (c.choices or []) for t in (ch.delta.tool_calls or [])]"
                    )
                }
                "elixir" => {
                    format!(
                        "{chunks_var} |> Enum.flat_map(fn c -> (List.first(c.choices) || %{{}}).delta |> Map.get(:tool_calls, []) end)"
                    )
                }
                "zig" => {
                    format!("{chunks_var}.items")
                }
                "swift" => {
                    format!(
                        "{chunks_var}.flatMap {{ c -> [StreamToolCall] in guard let ch = c.choices.first, let tcs = ch.delta.toolCalls else {{ return [] }}; return tcs }}"
                    )
                }
                "ruby" => {
                    format!("{chunks_var}.flat_map {{ |c| c.choices&.first&.delta&.tool_calls || [] }}")
                }
                _ => {
                    format!("{chunks_var}.flatMap((c: any) => c.choices?.[0]?.delta?.toolCalls ?? [])")
                }
            }),

            "finish_reason" => Some(match lang {
                "rust" => {
                    format!(
                        "{chunks_var}.last().and_then(|c| c.choices.first()).and_then(|ch| ch.finish_reason.as_ref()).map(|v| v.to_string()).unwrap_or_default()"
                    )
                }
                "go" => {
                    format!(
                        "func() string {{ if len({chunks_var}) == 0 {{ return \"\" }}; last := {chunks_var}[len({chunks_var})-1]; if len(last.Choices) > 0 && last.Choices[0].FinishReason != nil {{ return string(*last.Choices[0].FinishReason) }}; return \"\" }}()"
                    )
                }
                "java" => {
                    format!(
                        "({chunks_var}.isEmpty() ? null : {chunks_var}.get({chunks_var}.size()-1).choices().stream().findFirst().map(ch -> ch.finishReason() == null ? null : ch.finishReason().getValue()).orElse(null))"
                    )
                }
                "php" => {
                    format!("(!empty(${chunks_var}) ? (end(${chunks_var})->choices[0]->finishReason ?? null) : null)")
                }
                "kotlin" => {
                    format!(
                        "(if ({chunks_var}.isEmpty()) null else {chunks_var}.last().choices()?.firstOrNull()?.finishReason()?.getValue())"
                    )
                }
                "kotlin_android" => {
                    format!(
                        "(if ({chunks_var}.isEmpty()) null else {chunks_var}.last().choices?.firstOrNull()?.finishReason?.name?.lowercase())"
                    )
                }
                "python" => {
                    format!(
                        "(str({chunks_var}[-1].choices[0].finish_reason) if {chunks_var} and {chunks_var}[-1].choices else None)"
                    )
                }
                "elixir" => {
                    format!("Enum.at(List.last({chunks_var}).choices, 0).finish_reason")
                }
                "zig" => {
                    format!(
                        "(blk: {{ if ({chunks_var}.items.len == 0) break :blk \"\"; var _lcp = std.json.parseFromSlice(std.json.Value, std.heap.c_allocator, {chunks_var}.items[{chunks_var}.items.len - 1], .{{}}) catch break :blk \"\"; defer _lcp.deinit(); if (_lcp.value.object.get(\"choices\")) |_lchs| if (_lchs.array.items.len > 0) if (_lchs.array.items[0].object.get(\"finish_reason\")) |_fr| if (_fr == .string) break :blk _fr.string; break :blk \"\"; }})"
                    )
                }
                "swift" => {
                    format!("({chunks_var}.isEmpty ? nil : {chunks_var}.last!.choices.first?.finishReason?.rawValue)")
                }
                "ruby" => {
                    format!("({chunks_var}.empty? ? nil : {chunks_var}.last&.choices&.first&.finish_reason&.to_s)")
                }
                _ => {
                    format!(
                        "{chunks_var}.length > 0 ? {chunks_var}[{chunks_var}.length - 1].choices?.[0]?.finishReason : undefined"
                    )
                }
            }),

            "usage" => Some(match lang {
                "python" => {
                    format!("({chunks_var}[-1].usage if {chunks_var} else None)")
                }
                "rust" => {
                    format!("{chunks_var}.last().and_then(|c| c.usage.as_ref())")
                }
                "go" => {
                    format!(
                        "func() interface{{}} {{ if len({chunks_var}) == 0 {{ return nil }}; return {chunks_var}[len({chunks_var})-1].Usage }}()"
                    )
                }
                "java" => {
                    format!("({chunks_var}.isEmpty() ? null : {chunks_var}.get({chunks_var}.size()-1).usage())")
                }
                "kotlin" => {
                    format!("(if ({chunks_var}.isEmpty()) null else {chunks_var}.last().usage())")
                }
                "kotlin_android" => {
                    format!("(if ({chunks_var}.isEmpty()) null else {chunks_var}.last().usage)")
                }
                "php" => {
                    format!("(!empty(${chunks_var}) ? end(${chunks_var})->usage ?? null : null)")
                }
                "elixir" => {
                    format!("(if length({chunks_var}) > 0, do: List.last({chunks_var}).usage, else: nil)")
                }
                "swift" => {
                    format!("({chunks_var}.isEmpty ? nil : {chunks_var}.last!.usage)")
                }
                "ruby" => {
                    format!("({chunks_var}.empty? ? nil : {chunks_var}.last&.usage)")
                }
                _ => {
                    format!("({chunks_var}.length > 0 ? {chunks_var}[{chunks_var}.length - 1].usage : undefined)")
                }
            }),

            _ => {
                if let Some((root, tail)) = split_streaming_deep_path(field) {
                    if lang == "rust" && root == "tool_calls" {
                        return Some(render_rust_tool_calls_deep(chunks_var, tail));
                    }
                    if lang == "swift" && root == "tool_calls" {
                        let root_expr = Self::accessor(root, lang, chunks_var)?;
                        return Some(render_swift_tool_calls_deep(&root_expr, tail));
                    }
                    if lang == "zig" && root == "tool_calls" {
                        return None;
                    }
                    let root_expr = Self::accessor(root, lang, chunks_var)?;
                    Some(render_deep_tail(&root_expr, tail, lang))
                } else {
                    None
                }
            }
        }
    }
}

/// Identifies a tagged stream event variant for `stream.has_*_event` accessors.
#[derive(Debug, Clone, Copy)]
enum EventVariant {
    Page,
    Error,
    Complete,
}

impl EventVariant {
    /// Lower-case JSON-wire tag value for the `type` discriminator.
    const fn tag(self) -> &'static str {
        match self {
            Self::Page => "page",
            Self::Error => "error",
            Self::Complete => "complete",
        }
    }

    /// Upper-camel variant name as used in most language bindings.
    const fn upper_camel(self) -> &'static str {
        match self {
            Self::Page => "Page",
            Self::Error => "Error",
            Self::Complete => "Complete",
        }
    }
}

/// Emit a language-native boolean expression that is `true` iff any chunk in
/// `chunks_var` matches the given streaming-union variant.
///
/// `item_type` is the unqualified name of the streaming union type.
/// `module_qualifier` is the per-project
/// module/namespace prefix required by Rust and C# to form a fully-qualified
/// type path.
///
/// Returns `None` for languages where typed streaming-union matching is not
/// expressible (PHP — eager-JSON, WASM — no streaming on wasm32).
fn has_event_variant_accessor(
    lang: &str,
    chunks_var: &str,
    variant: EventVariant,
    item_type: &str,
    module_qualifier: Option<&str>,
) -> Option<String> {
    let tag = variant.tag();
    let camel = variant.upper_camel();
    match lang {
        "python" => Some(format!("any(e.type == \"{tag}\" for e in {chunks_var})")),
        "node" | "typescript" => Some(format!("{chunks_var}.some((e: any) => e?.type === \"{tag}\")")),
        "ruby" => Some(format!("{chunks_var}.any? {{ |e| e.{tag}? }}")),
        "go" => Some(format!(
            "func() bool {{ for _, e := range {chunks_var} {{ if _, _ok := e.(pkg.{item_type}{camel}); _ok {{ return true }} }}; return false }}()"
        )),
        "java" => Some(format!(
            "{chunks_var}.stream().anyMatch(e -> e instanceof {item_type}.{camel})"
        )),
        "csharp" => module_qualifier.map(|ns| format!("{chunks_var}.Any(e => e is global::{ns}.{item_type}.{camel})")),
        "swift" => Some(format!(
            "{chunks_var}.contains(where: {{ e in e.to_string().toString().contains(\"{tag}\") }})"
        )),
        "elixir" => Some(format!(
            "Enum.any?({chunks_var}, fn e -> Map.get(e, :type) == \"{tag}\" end)"
        )),
        "kotlin" => Some(format!("{chunks_var}.any {{ it is {item_type}.{camel} }}")),
        "kotlin_android" => Some(format!("{chunks_var}.any {{ it is {item_type}.{camel} }}")),
        "dart" => Some(format!("{chunks_var}.any((e) => e is {item_type}_{camel})")),
        "zig" => Some(format!(
            "blk: {{ for ({chunks_var}.items) |_e| {{ if (std.mem.indexOf(u8, _e, \"\\\"type\\\":\\\"{tag}\\\"\") != null) break :blk true; }} break :blk false; }}"
        )),
        // Rust: {item_type} is a tagged enum (`#[serde(tag = "type")]`).
        "rust" => module_qualifier.map(|crate_name| {
            format!("{chunks_var}.iter().any(|e| matches!(e, {crate_name}::{item_type}::{camel} {{ .. }}))")
        }),
        "php" | "wasm" => None,
        _ => None,
    }
}
