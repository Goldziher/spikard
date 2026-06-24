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
                // Zig ArrayList does not expose .len directly; must use .items
                "zig" => format!("{chunks_var}.items"),
                // PHP variables require `$` sigil — bareword `chunks` is parsed as a
                // constant reference and triggers "Undefined constant" errors.
                "php" => format!("${chunks_var}"),
                _ => chunks_var.to_string(),
            }),

            "stream.items.length" | "chunks.length" => Some(match lang {
                "rust" => format!("{chunks_var}.len()"),
                "go" => format!("len({chunks_var})"),
                "python" => format!("len({chunks_var})"),
                "php" => format!("count(${chunks_var})"),
                "elixir" => format!("length({chunks_var})"),
                // kotlin List.size is a property (not .length)
                "kotlin" => format!("{chunks_var}.size"),
                // zig: chunks_var is ArrayList([]u8); use .items.len
                "zig" => format!("{chunks_var}.items.len"),
                // Swift Array uses .count (Collection protocol)
                "swift" => format!("{chunks_var}.count"),
                // node/wasm/typescript use .length
                _ => format!("{chunks_var}.length"),
            }),

            "stream_content" => Some(match lang {
                "rust" => {
                    format!(
                        "{chunks_var}.iter().map(|c| c.choices.first().and_then(|ch| ch.delta.content.as_deref()).unwrap_or(\"\")).collect::<String>()"
                    )
                }
                "go" => {
                    // Go: chunks is []pkg.<adapter item type>.
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
                    // Kotlin: chunks is List<adapter item type> (Java records via typealias).
                    // choices() / delta() / content() are Java record accessor methods.
                    format!(
                        "{chunks_var}.joinToString(\"\") {{ it.choices()?.firstOrNull()?.delta()?.content() ?: \"\" }}"
                    )
                }
                "kotlin_android" => {
                    // kotlin-android: data classes use Kotlin property access (no parens).
                    format!("{chunks_var}.joinToString(\"\") {{ it.choices?.firstOrNull()?.delta?.content ?: \"\" }}")
                }
                "elixir" => {
                    // StreamDelta has all fields optional with skip_serializing_if, so
                    // absent fields are not present as keys in the Jason-decoded map.
                    // Use Map.get with defaults to avoid KeyError on absent :content.
                    format!(
                        "{chunks_var} |> Enum.map(fn c -> (Enum.at(c.choices, 0) || %{{}}) |> Map.get(:delta, %{{}}) |> Map.get(:content, \"\") end) |> Enum.join(\"\")"
                    )
                }
                "python" => {
                    format!("\"\".join(c.choices[0].delta.content or \"\" for c in {chunks_var} if c.choices)")
                }
                "zig" => {
                    // Zig: `{chunks_var}_content` is a `std.ArrayList(u8)` populated by
                    // the collect snippet. `.items` gives a `[]u8` slice of the content.
                    format!("{chunks_var}_content.items")
                }
                // Swift: chunks is [<Module>.<adapter item type>] (first-class
                // Codable struct emitted by alef-backend-swift). choices is
                // `[StreamChoice]` (property), delta is `StreamDelta` (property),
                // content is `String?` (property). No `.toString()` wrapping —
                // first-class fields are already native Swift values.
                "swift" => {
                    format!(
                        "{chunks_var}.map {{ c in c.choices.first.flatMap {{ ch in ch.delta.content }} ?? \"\" }}.joined()"
                    )
                }
                // Ruby: choices returns Array<StreamChoice>, use .first with safe navigation
                "ruby" => {
                    format!("{chunks_var}.map {{ |c| c.choices.first&.delta&.content || '' }}.join")
                }
                // node/wasm/typescript
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
                    // PHP streaming chunks come from `json_decode` of the binding's JSON
                    // string return. The PHP binding serializes with rename_all = "camelCase",
                    // so use `finishReason` (camelCase) to match the emitted JSON keys.
                    format!("!empty(${chunks_var}) && isset(end(${chunks_var})->choices[0]->finishReason)")
                }
                "kotlin" => {
                    // Kotlin: use isNotEmpty() + last() + safe-call chain
                    format!(
                        "{chunks_var}.isNotEmpty() && {chunks_var}.last().choices()?.firstOrNull()?.finishReason() != null"
                    )
                }
                "kotlin_android" => {
                    // kotlin-android: data classes use Kotlin property access (no parens).
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
                // zig: the collect snippet exhausts the stream; check last chunk JSON
                // was collected (chunks.items is non-empty) as a proxy for completion.
                "zig" => {
                    format!("{chunks_var}.items.len > 0")
                }
                // Swift: chunks is [<Module>.<adapter item type>] first-class
                // struct. `choices` is `[StreamChoice]` (property), `finishReason`
                // is `FinishReason?` (property, camelCase).
                "swift" => {
                    format!("!{chunks_var}.isEmpty && {chunks_var}.last!.choices.first?.finishReason != nil")
                }
                // Ruby: choices/finish_reason are Magnus method accessors; use safe navigation
                "ruby" => {
                    format!("!{chunks_var}.empty? && !{chunks_var}.last&.choices&.first&.finish_reason.nil?")
                }
                // node/wasm/typescript
                _ => {
                    format!(
                        "{chunks_var}.length > 0 && {chunks_var}[{chunks_var}.length - 1].choices?.[0]?.finishReason != null"
                    )
                }
            }),

            // no_chunks_after_done is a structural invariant: once the stream
            // closes (channel drained / iterator exhausted), no further chunks
            // can arrive.  We assert `true` as a compile-time proof of intent.
            "no_chunks_after_done" => Some(match lang {
                "rust" => "true".to_string(),
                "go" => "true".to_string(),
                "java" => "true".to_string(),
                "php" => "true".to_string(),
                _ => "true".to_string(),
            }),

            // Streaming union event-variant predicates.
            //
            // Each chunk is a tagged union whose concrete type name is given by
            // `item_type`. The accessor
            // returns a language-native boolean expression that is `true` iff
            // any chunk in the collected list matches the named variant.
            //
            // When `item_type` is `None` the helper returns `None` so the
            // assertion is silently skipped — callers must supply the type name
            // to emit working code.
            //
            // PHP and WASM intentionally return `None`: PHP's crawl-stream is
            // exposed as eager JSON (see `chunks_var` collect_snippet) and WASM
            // does not support streaming on `wasm32` targets.
            "stream.has_page_event" => item_type
                .and_then(|ty| has_event_variant_accessor(lang, chunks_var, EventVariant::Page, ty, module_qualifier)),
            "stream.has_error_event" => item_type
                .and_then(|ty| has_event_variant_accessor(lang, chunks_var, EventVariant::Error, ty, module_qualifier)),
            "stream.has_complete_event" => item_type.and_then(|ty| {
                has_event_variant_accessor(lang, chunks_var, EventVariant::Complete, ty, module_qualifier)
            }),

            // event_count_min is the collected chunks count — used with
            // `greater_than_or_equal` assertions on the chunk count.  Render the
            // language-appropriate length/size accessor.
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
                    // StreamDelta.ToolCalls is `[]StreamToolCall` (slice, not pointer).
                    // Return the typed slice so deep-path accessors like `tool_calls[0].function.name`
                    // can index and access typed fields.
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
                    // PHP streaming chunks are json_decoded stdClass. The PHP binding
                    // serializes with rename_all = "camelCase", so use `toolCalls`.
                    format!(
                        "array_merge(...array_map(fn($c) => $c->choices[0]->delta->toolCalls ?? [], ${chunks_var}))"
                    )
                }
                "kotlin" => {
                    // Kotlin: flatten tool_calls from all chunk deltas into one list
                    format!(
                        "{chunks_var}.flatMap {{ c -> c.choices()?.flatMap {{ ch -> ch.delta()?.toolCalls() ?: emptyList() }} ?: emptyList() }}"
                    )
                }
                "kotlin_android" => {
                    // kotlin-android: data classes use Kotlin property access (no parens).
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
                // Zig: tool_calls count from all chunk deltas
                "zig" => {
                    format!("{chunks_var}.items")
                }
                // Swift: chunks is [<Module>.<adapter item type>] first-class
                // Codable struct. choices is `[StreamChoice]`, delta is
                // `StreamDelta`, toolCalls is `[StreamToolCall]?`.
                "swift" => {
                    format!(
                        "{chunks_var}.flatMap {{ c -> [StreamToolCall] in guard let ch = c.choices.first, let tcs = ch.delta.toolCalls else {{ return [] }}; return tcs }}"
                    )
                }
                // Ruby: choices/delta/tool_calls are Magnus method accessors;
                // delta.tool_calls returns nil when absent — default to empty array.
                "ruby" => {
                    format!("{chunks_var}.flat_map {{ |c| c.choices&.first&.delta&.tool_calls || [] }}")
                }
                _ => {
                    format!("{chunks_var}.flatMap((c: any) => c.choices?.[0]?.delta?.toolCalls ?? [])")
                }
            }),

            "finish_reason" => Some(match lang {
                "rust" => {
                    // The stream item finish_reason is Option<FinishReason> (enum, not
                    // String). Display impl writes the JSON wire form (e.g. "tool_calls").
                    format!(
                        "{chunks_var}.last().and_then(|c| c.choices.first()).and_then(|ch| ch.finish_reason.as_ref()).map(|v| v.to_string()).unwrap_or_default()"
                    )
                }
                "go" => {
                    // FinishReason is a typed alias (`type FinishReason string`) in bindings,
                    // so cast to string explicitly to match the assertion target type.
                    format!(
                        "func() string {{ if len({chunks_var}) == 0 {{ return \"\" }}; last := {chunks_var}[len({chunks_var})-1]; if len(last.Choices) > 0 && last.Choices[0].FinishReason != nil {{ return string(*last.Choices[0].FinishReason) }}; return \"\" }}()"
                    )
                }
                "java" => {
                    // FinishReason.getValue() returns the JSON wire string (e.g. "tool_calls").
                    // Without it, assertEquals(String, FinishReason) fails because Object.equals
                    // doesn't cross types even when toString() matches.
                    format!(
                        "({chunks_var}.isEmpty() ? null : {chunks_var}.get({chunks_var}.size()-1).choices().stream().findFirst().map(ch -> ch.finishReason() == null ? null : ch.finishReason().getValue()).orElse(null))"
                    )
                }
                "php" => {
                    // PHP streaming chunks are json_decoded stdClass. The PHP binding
                    // serializes with rename_all = "camelCase", so use `finishReason`.
                    format!("(!empty(${chunks_var}) ? (end(${chunks_var})->choices[0]->finishReason ?? null) : null)")
                }
                "kotlin" => {
                    // Returns the string value of the finish_reason enum from the last chunk.
                    // FinishReason.getValue() returns the JSON wire string (e.g. "tool_calls").
                    format!(
                        "(if ({chunks_var}.isEmpty()) null else {chunks_var}.last().choices()?.firstOrNull()?.finishReason()?.getValue())"
                    )
                }
                "kotlin_android" => {
                    // kotlin-android: plain Kotlin enum class uses .name.lowercase() for wire string.
                    format!(
                        "(if ({chunks_var}.isEmpty()) null else {chunks_var}.last().choices?.firstOrNull()?.finishReason?.name?.lowercase())"
                    )
                }
                "python" => {
                    // FinishReason is a PyO3 enum object, not a plain string.
                    // Wrap in str() so callers can do `.strip()` / string comparisons
                    // without `AttributeError: 'FinishReason' has no attribute 'strip'`.
                    format!(
                        "(str({chunks_var}[-1].choices[0].finish_reason) if {chunks_var} and {chunks_var}[-1].choices else None)"
                    )
                }
                "elixir" => {
                    format!("Enum.at(List.last({chunks_var}).choices, 0).finish_reason")
                }
                // Zig: finish_reason from the last chunk's JSON via an inline labeled block.
                // Returns `[]const u8` (unwrapped with orelse "" for expectEqualStrings).
                "zig" => {
                    format!(
                        "(blk: {{ if ({chunks_var}.items.len == 0) break :blk \"\"; var _lcp = std.json.parseFromSlice(std.json.Value, std.heap.c_allocator, {chunks_var}.items[{chunks_var}.items.len - 1], .{{}}) catch break :blk \"\"; defer _lcp.deinit(); if (_lcp.value.object.get(\"choices\")) |_lchs| if (_lchs.array.items.len > 0) if (_lchs.array.items[0].object.get(\"finish_reason\")) |_fr| if (_fr == .string) break :blk _fr.string; break :blk \"\"; }})"
                    )
                }
                // Swift: first-class `StreamChoice.finishReason: FinishReason?`
                // where FinishReason is a Codable Swift enum with `String` raw
                // values matching serde wire strings. `.rawValue` yields e.g.
                // "tool_calls" for cross-language fixture parity.
                "swift" => {
                    format!("({chunks_var}.isEmpty ? nil : {chunks_var}.last!.choices.first?.finishReason?.rawValue)")
                }
                // Ruby: FinishReason is a Magnus-wrapped enum. Call .to_s to get the
                // wire string (e.g. "tool_calls") for cross-language fixture parity.
                "ruby" => {
                    format!("({chunks_var}.empty? ? nil : {chunks_var}.last&.choices&.first&.finish_reason&.to_s)")
                }
                _ => {
                    format!(
                        "{chunks_var}.length > 0 ? {chunks_var}[{chunks_var}.length - 1].choices?.[0]?.finishReason : undefined"
                    )
                }
            }),

            // `usage` is a stream-level virtual root: resolves against the last
            // chunk that carried a usage payload.  Deep paths like `usage.total_tokens`
            // are handled by the deep-path logic in the `_` arm below (root=`usage`,
            // tail=`.total_tokens`), which calls this base accessor and appends the tail.
            "usage" => Some(match lang {
                "python" => {
                    // Access the last chunk's usage object (may be None).
                    // Deep paths like usage.total_tokens are rendered as:
                    //   (chunks[-1].usage if chunks else None).total_tokens
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
                    // kotlin-android: data classes use Kotlin property access (no parens).
                    format!("(if ({chunks_var}.isEmpty()) null else {chunks_var}.last().usage)")
                }
                "php" => {
                    format!("(!empty(${chunks_var}) ? end(${chunks_var})->usage ?? null : null)")
                }
                "elixir" => {
                    format!("(if length({chunks_var}) > 0, do: List.last({chunks_var}).usage, else: nil)")
                }
                // Swift: first-class stream item usage property.
                // (Codable struct property — no method call).
                "swift" => {
                    format!("({chunks_var}.isEmpty ? nil : {chunks_var}.last!.usage)")
                }
                // Ruby: usage is a Magnus method accessor; returns nil when absent.
                "ruby" => {
                    format!("({chunks_var}.empty? ? nil : {chunks_var}.last&.usage)")
                }
                _ => {
                    format!("({chunks_var}.length > 0 ? {chunks_var}[{chunks_var}.length - 1].usage : undefined)")
                }
            }),

            _ => {
                // Deep-path: e.g. `tool_calls[0].function.name`
                // Split into root + tail, get the root's inline expression, then
                // render the tail (index + fields) in a per-language style on top.
                if let Some((root, tail)) = split_streaming_deep_path(field) {
                    // Rust needs Option-aware chaining for the StreamToolCall fields
                    // (function/id are Option<...>). The generic tail renderer can't
                    // infer Option-wrapping, so we emit rust-specific idiom here.
                    if lang == "rust" && root == "tool_calls" {
                        return Some(render_rust_tool_calls_deep(chunks_var, tail));
                    }
                    // Swift: StreamToolCallRef fields are swift-bridge methods returning
                    // Optional.  The generic render_deep_tail doesn't know to add `()`
                    // or optional-chain with `?.`, so use a dedicated renderer.
                    if lang == "swift" && root == "tool_calls" {
                        let root_expr = Self::accessor(root, lang, chunks_var)?;
                        return Some(render_swift_tool_calls_deep(&root_expr, tail));
                    }
                    // Zig stores stream chunks as JSON strings (`[]const u8`) in
                    // `chunks: ArrayList([]u8)`, not typed stream item
                    // records. A deep `tool_calls[N].function.name` access would
                    // require parsing each chunk's JSON inline — rather than
                    // emit code that won't compile, signal "unsupported" so the
                    // assertion is skipped at the call site.
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
        // Python: tagged-union exposes `.type` returning the lower-case wire tag.
        "python" => Some(format!("any(e.type == \"{tag}\" for e in {chunks_var})")),
        // Node / TypeScript: deserialized union objects expose a `type`
        // discriminator field with the lower-case wire tag.
        "node" | "typescript" => Some(format!("{chunks_var}.some((e: any) => e?.type === \"{tag}\")")),
        // Ruby: each variant class exposes `<tag>?` predicates.
        "ruby" => Some(format!("{chunks_var}.any? {{ |e| e.{tag}? }}")),
        // Go: variants are concrete struct types ({item_type}{Camel}) that
        // implement the {item_type} interface.  Use a type switch via an
        // anonymous IIFE so the accessor remains an expression.
        "go" => Some(format!(
            "func() bool {{ for _, e := range {chunks_var} {{ if _, _ok := e.(pkg.{item_type}{camel}); _ok {{ return true }} }}; return false }}()"
        )),
        // Java: sealed interface {item_type} with nested records.
        "java" => Some(format!(
            "{chunks_var}.stream().anyMatch(e -> e instanceof {item_type}.{camel})"
        )),
        // C#: abstract record {item_type} with nested sealed records.
        // The qualifier is the project's C# namespace (e.g. `DemoCrawler`).
        "csharp" => module_qualifier.map(|ns| format!("{chunks_var}.Any(e => e is global::{ns}.{item_type}.{camel})")),
        // Swift: the swift-bridge `to_string()` impl on the bridge enum returns the
        // serde-serialized variant name (i.e. the same wire tag the JSON discriminator
        // uses). Match on `tag` (e.g. "page", "error", "complete") rather than the raw
        // Rust identifier so the comparison aligns with whatever `rename_all` the source
        // enum declares. `to_string()` returns `RustString`; convert via `.toString()`
        // before calling `.contains()`.
        "swift" => Some(format!(
            "{chunks_var}.contains(where: {{ e in e.to_string().toString().contains(\"{tag}\") }})"
        )),
        // Elixir: each event is a map with a `:type` key whose value is a string (from JSON).
        "elixir" => Some(format!(
            "Enum.any?({chunks_var}, fn e -> Map.get(e, :type) == \"{tag}\" end)"
        )),
        // Kotlin (Java records via typealias): same shape as Java.
        "kotlin" => Some(format!("{chunks_var}.any {{ it is {item_type}.{camel} }}")),
        // kotlin-android: native sealed class with the same nested variants.
        "kotlin_android" => Some(format!("{chunks_var}.any {{ it is {item_type}.{camel} }}")),
        // Dart (freezed): variants are {item_type}_{Camel} (underscored).
        "dart" => Some(format!("{chunks_var}.any((e) => e is {item_type}_{camel})")),
        // Zig: collected chunks are JSON strings (see Zig collect_snippet); check
        // for the wire-format `"type":"<tag>"` substring on any item.  Substring
        // matching is safe because the JSON is produced by the FFI marshaller
        // with a fixed key ordering and the tag values do not collide.
        "zig" => Some(format!(
            "blk: {{ for ({chunks_var}.items) |_e| {{ if (std.mem.indexOf(u8, _e, \"\\\"type\\\":\\\"{tag}\\\"\") != null) break :blk true; }} break :blk false; }}"
        )),
        // Rust: {item_type} is a tagged enum (`#[serde(tag = "type")]`).
        // Use `matches!` for the predicate so we don't bind the variant payload.
        // The qualifier is the project's cargo crate name (snake_case).
        "rust" => module_qualifier.map(|crate_name| {
            format!("{chunks_var}.iter().any(|e| matches!(e, {crate_name}::{item_type}::{camel} {{ .. }}))")
        }),
        // PHP: crawl-stream is delivered as eager JSON (see PHP collect_snippet)
        // and the PHP binding does not expose typed union objects.
        // WASM: streaming is unavailable on wasm32 targets.
        "php" | "wasm" => None,
        _ => None,
    }
}
