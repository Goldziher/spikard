use super::*;

#[test]
fn is_streaming_virtual_field_recognizes_all_fields() {
    for field in STREAMING_VIRTUAL_FIELDS {
        assert!(
            is_streaming_virtual_field(field),
            "field '{field}' not recognized as streaming virtual"
        );
    }
}

#[test]
fn is_streaming_virtual_field_rejects_real_fields() {
    assert!(!is_streaming_virtual_field("content"));
    assert!(!is_streaming_virtual_field("choices"));
    assert!(!is_streaming_virtual_field("model"));
    assert!(!is_streaming_virtual_field(""));
}

#[test]
fn is_streaming_virtual_field_rejects_non_root_paths_with_matching_tail() {
    assert!(!is_streaming_virtual_field("choices[0].finish_reason"));
    assert!(!is_streaming_virtual_field("choices[0].message.content"));
    assert!(!is_streaming_virtual_field("data[0].embedding"));
}

#[test]
fn is_streaming_virtual_field_does_not_match_usage() {
    assert!(!is_streaming_virtual_field("usage"));
    assert!(!is_streaming_virtual_field("usage.total_tokens"));
    assert!(!is_streaming_virtual_field("usage.prompt_tokens"));
}

#[test]
fn event_variant_accessor_requires_stream_item_type() {
    assert_eq!(
        StreamingFieldResolver::accessor_with_module_qualifier(
            "stream.has_page_event",
            "rust",
            "chunks",
            Some("sample_recipe"),
        ),
        None
    );
}

#[test]
fn event_variant_accessor_uses_configured_stream_item_type() {
    let rust = StreamingFieldResolver::accessor_with_streaming_context(
        "stream.has_page_event",
        "rust",
        "chunks",
        Some("sample_recipe"),
        Some("Event"),
    )
    .expect("configured item type emits rust event predicate");
    assert!(rust.contains("sample_recipe::Event::Page"), "rust predicate: {rust}");

    let go = StreamingFieldResolver::accessor_with_streaming_context(
        "stream.has_complete_event",
        "go",
        "chunks",
        None,
        Some("Event"),
    )
    .expect("configured item type emits go event predicate");
    assert!(go.contains("pkg.EventComplete"), "go predicate: {go}");
}

#[test]
fn accessor_chunks_returns_var_name() {
    assert_eq!(
        StreamingFieldResolver::accessor("chunks", "rust", "chunks"),
        Some("chunks".to_string())
    );
    assert_eq!(
        StreamingFieldResolver::accessor("chunks", "node", "chunks"),
        Some("chunks".to_string())
    );
}

#[test]
fn accessor_chunks_length_uses_language_idiom() {
    let rust = StreamingFieldResolver::accessor("chunks.length", "rust", "chunks").unwrap();
    assert!(rust.contains(".len()"), "rust: {rust}");
    let neutral_rust = StreamingFieldResolver::accessor("stream.items.length", "rust", "items").unwrap();
    assert_eq!(neutral_rust, "items.len()", "rust neutral items count: {neutral_rust}");

    let go = StreamingFieldResolver::accessor("chunks.length", "go", "chunks").unwrap();
    assert!(go.starts_with("len("), "go: {go}");

    let node = StreamingFieldResolver::accessor("chunks.length", "node", "chunks").unwrap();
    assert!(node.contains(".length"), "node: {node}");

    let php = StreamingFieldResolver::accessor("chunks.length", "php", "chunks").unwrap();
    assert!(php.starts_with("count("), "php: {php}");
}

#[test]
fn accessor_chunks_length_zig_uses_items_len() {
    let zig = StreamingFieldResolver::accessor("chunks.length", "zig", "chunks").unwrap();
    assert_eq!(zig, "chunks.items.len", "zig chunks.length: {zig}");
}

#[test]
fn accessor_stream_content_zig_uses_content_items() {
    let zig = StreamingFieldResolver::accessor("stream_content", "zig", "chunks").unwrap();
    assert_eq!(zig, "chunks_content.items", "zig stream_content: {zig}");
}

#[test]
fn collect_snippet_zig_drains_via_ffi() {
    assert!(
        StreamingFieldResolver::collect_snippet("zig", "_stream_handle", "chunks").is_none(),
        "generic zig collect must require adapter metadata"
    );
    let snip = StreamingFieldResolver::collect_snippet_zig(
        "_stream_handle",
        "chunks",
        "sample",
        "sample",
        "StreamClient",
        "events",
        "StreamEvent",
    );
    assert!(snip.contains("std.ArrayList([]u8)"), "zig collect: {snip}");
    assert!(
        snip.contains("stream_client_events_next(_stream_handle)"),
        "zig collect: {snip}"
    );
    assert!(snip.contains("stream_event_to_json(_nc)"), "zig collect: {snip}");
    assert!(snip.contains("chunks_content"), "zig collect: {snip}");
    assert!(
        snip.contains("chunks.append(std.heap.c_allocator"),
        "zig collect: {snip}"
    );
    assert!(snip.contains(".empty;"), "zig collect (Zig 0.16 unmanaged): {snip}");
}

#[test]
fn accessor_stream_content_rust_uses_iterator() {
    let expr = StreamingFieldResolver::accessor("stream_content", "rust", "chunks").unwrap();
    assert!(expr.contains(".collect::<String>()"), "rust stream_content: {expr}");
}

#[test]
fn accessor_no_chunks_after_done_returns_true() {
    for lang in ["rust", "go", "java", "php", "node", "wasm", "elixir"] {
        let expr = StreamingFieldResolver::accessor("no_chunks_after_done", lang, "chunks").unwrap();
        assert_eq!(expr, "true", "lang {lang}: expected 'true', got '{expr}'");
    }
}

#[test]
fn accessor_elixir_chunks_length_uses_length_function() {
    let expr = StreamingFieldResolver::accessor("chunks.length", "elixir", "chunks").unwrap();
    assert_eq!(expr, "length(chunks)", "elixir chunks.length: {expr}");
}

#[test]
fn accessor_elixir_stream_content_uses_pipe() {
    let expr = StreamingFieldResolver::accessor("stream_content", "elixir", "chunks").unwrap();
    assert!(expr.contains("|> Enum.join"), "elixir stream_content: {expr}");
    assert!(expr.contains("|> Enum.map"), "elixir stream_content: {expr}");
    assert!(
        !expr.contains("choices[0]"),
        "elixir stream_content must not use bracket access on list: {expr}"
    );
    assert!(
        expr.contains("Enum.at("),
        "elixir stream_content must use Enum.at for list index: {expr}"
    );
}

#[test]
fn accessor_elixir_stream_complete_uses_list_last() {
    let expr = StreamingFieldResolver::accessor("stream_complete", "elixir", "chunks").unwrap();
    assert!(expr.contains("List.last(chunks)"), "elixir stream_complete: {expr}");
    assert!(expr.contains("finish_reason != nil"), "elixir stream_complete: {expr}");
    assert!(
        !expr.contains("choices[0]"),
        "elixir stream_complete must not use bracket access on list: {expr}"
    );
    assert!(
        expr.contains("Enum.at("),
        "elixir stream_complete must use Enum.at for list index: {expr}"
    );
}

#[test]
fn accessor_elixir_finish_reason_uses_list_last() {
    let expr = StreamingFieldResolver::accessor("finish_reason", "elixir", "chunks").unwrap();
    assert!(expr.contains("List.last(chunks)"), "elixir finish_reason: {expr}");
    assert!(expr.contains("finish_reason"), "elixir finish_reason: {expr}");
    assert!(
        !expr.contains("choices[0]"),
        "elixir finish_reason must not use bracket access on list: {expr}"
    );
    assert!(
        expr.contains("Enum.at("),
        "elixir finish_reason must use Enum.at for list index: {expr}"
    );
}

#[test]
fn collect_snippet_elixir_uses_enum_to_list() {
    let snip = StreamingFieldResolver::collect_snippet("elixir", "result", "chunks").unwrap();
    assert!(snip.contains("Enum.to_list(result)"), "elixir: {snip}");
    assert!(snip.contains("chunks ="), "elixir: {snip}");
}

#[test]
fn collect_snippet_rust_uses_tokio_stream() {
    let snip = StreamingFieldResolver::collect_snippet("rust", "result", "chunks").unwrap();
    assert!(snip.contains("tokio_stream::StreamExt::collect"), "rust: {snip}");
    assert!(snip.contains("let chunks"), "rust: {snip}");
    assert!(snip.contains(".expect("), "rust must unwrap Result items: {snip}");
}

#[test]
fn collect_snippet_go_drains_channel() {
    assert!(
        StreamingFieldResolver::collect_snippet("go", "stream", "chunks").is_none(),
        "typed Go collect must require an item type"
    );
    let snip = StreamingFieldResolver::collect_snippet_typed("go", "stream", "chunks", Some("StreamEvent")).unwrap();
    assert!(snip.contains("for chunk := range stream"), "go: {snip}");
    assert!(snip.contains("[]pkg.StreamEvent"), "go: {snip}");
}

#[test]
fn collect_snippet_java_uses_iterator() {
    assert!(
        StreamingFieldResolver::collect_snippet("java", "result", "chunks").is_none(),
        "typed Java collect must require an item type"
    );
    let snip = StreamingFieldResolver::collect_snippet_typed("java", "result", "chunks", Some("StreamEvent")).unwrap();
    assert!(
        snip.contains(".iterator()"),
        "java snippet must call .iterator() on stream: {snip}"
    );
    assert!(snip.contains("ArrayList<StreamEvent>"), "java: {snip}");
    assert!(snip.contains("hasNext()"), "java: {snip}");
    assert!(snip.contains(".next()"), "java: {snip}");
}

#[test]
fn collect_snippet_php_decodes_json_or_iterates() {
    let snip = StreamingFieldResolver::collect_snippet("php", "result", "chunks").unwrap();
    assert!(snip.contains("json_decode"), "php must decode JSON: {snip}");
    assert!(
        snip.contains("iterator_to_array"),
        "php must keep iterator_to_array fallback: {snip}"
    );
    assert!(snip.contains("$chunks ="), "php must bind $chunks: {snip}");
}

#[test]
fn collect_snippet_node_uses_for_await() {
    let snip = StreamingFieldResolver::collect_snippet("node", "result", "chunks").unwrap();
    assert!(snip.contains("for await"), "node: {snip}");
}

#[test]
fn collect_snippet_python_uses_async_for() {
    let snip = StreamingFieldResolver::collect_snippet("python", "result", "chunks").unwrap();
    assert!(snip.contains("async for chunk in result"), "python: {snip}");
    assert!(snip.contains("chunks.append(chunk)"), "python: {snip}");
}

#[test]
fn accessor_stream_content_python_uses_join() {
    let expr = StreamingFieldResolver::accessor("stream_content", "python", "chunks").unwrap();
    assert!(expr.contains("\"\".join("), "python stream_content: {expr}");
    assert!(expr.contains("c.choices"), "python stream_content: {expr}");
}

#[test]
fn accessor_stream_complete_python_uses_finish_reason() {
    let expr = StreamingFieldResolver::accessor("stream_complete", "python", "chunks").unwrap();
    assert!(
        expr.contains("finish_reason is not None"),
        "python stream_complete: {expr}"
    );
}

#[test]
fn accessor_finish_reason_python_uses_last_chunk() {
    let expr = StreamingFieldResolver::accessor("finish_reason", "python", "chunks").unwrap();
    assert!(expr.contains("chunks[-1]"), "python finish_reason: {expr}");
    assert!(
        expr.starts_with("(str(") || expr.contains("str(chunks"),
        "python finish_reason must wrap in str(): {expr}"
    );
}

#[test]
fn accessor_tool_calls_python_uses_list_comprehension() {
    let expr = StreamingFieldResolver::accessor("tool_calls", "python", "chunks").unwrap();
    assert!(expr.contains("for c in chunks"), "python tool_calls: {expr}");
    assert!(expr.contains("tool_calls"), "python tool_calls: {expr}");
}

#[test]
fn accessor_usage_python_uses_last_chunk() {
    let expr = StreamingFieldResolver::accessor("usage", "python", "chunks").unwrap();
    assert!(
        expr.contains("chunks[-1].usage"),
        "python usage: expected chunks[-1].usage, got: {expr}"
    );
}

#[test]
fn accessor_usage_total_tokens_does_not_route_via_chunks() {
    assert!(StreamingFieldResolver::accessor("usage.total_tokens", "python", "chunks").is_none());
}

#[test]
fn accessor_unknown_field_returns_none() {
    assert_eq!(
        StreamingFieldResolver::accessor("nonexistent_field", "rust", "chunks"),
        None
    );
}

#[test]
fn is_streaming_virtual_field_recognizes_deep_tool_calls_paths() {
    assert!(
        is_streaming_virtual_field("tool_calls[0].function.name"),
        "tool_calls[0].function.name should be recognized"
    );
    assert!(
        is_streaming_virtual_field("tool_calls[0].id"),
        "tool_calls[0].id should be recognized"
    );
    assert!(
        is_streaming_virtual_field("tool_calls[1].function.arguments"),
        "tool_calls[1].function.arguments should be recognized"
    );
    assert!(is_streaming_virtual_field("tool_calls"));
    assert!(!is_streaming_virtual_field("tool_calls_extra.name"));
    assert!(!is_streaming_virtual_field("nonexistent[0].field"));
}

/// Snapshot: `tool_calls[0].function.name` for Rust, Kotlin, TypeScript.
///
/// These three languages cover the main accessor styles:
/// - Rust: `snake_case` field, explicit `[0]` index on collected Vec
/// - Kotlin: camelCase method calls with `.first()` for index 0
/// - TypeScript/Node: camelCase properties with `[0]` bracket
#[test]
fn deep_tool_calls_function_name_snapshot_rust_kotlin_ts() {
    let field = "tool_calls[0].function.name";

    let rust = StreamingFieldResolver::accessor(field, "rust", "chunks").unwrap();
    assert!(
        rust.contains(".nth(0)"),
        "rust deep tool_calls: expected .nth(0) iterator index, got: {rust}"
    );
    assert!(
        rust.contains("x.function.as_ref()"),
        "rust deep tool_calls: expected Option-aware function access, got: {rust}"
    );
    assert!(
        rust.contains("x.name.as_deref()"),
        "rust deep tool_calls: expected Option-aware name leaf, got: {rust}"
    );
    assert!(
        !rust.contains("// skipped"),
        "rust deep tool_calls: must not emit skip comment, got: {rust}"
    );

    let kotlin = StreamingFieldResolver::accessor(field, "kotlin", "chunks").unwrap();
    assert!(
        kotlin.contains(".first()"),
        "kotlin deep tool_calls: expected .first() for index 0, got: {kotlin}"
    );
    assert!(
        kotlin.contains(".function()"),
        "kotlin deep tool_calls: expected .function() method call, got: {kotlin}"
    );
    assert!(
        kotlin.contains(".name()"),
        "kotlin deep tool_calls: expected .name() method call, got: {kotlin}"
    );

    let ts = StreamingFieldResolver::accessor(field, "node", "chunks").unwrap();
    assert!(
        ts.contains("[0]"),
        "ts/node deep tool_calls: expected [0] index, got: {ts}"
    );
    assert!(
        ts.contains(".function"),
        "ts/node deep tool_calls: expected .function segment, got: {ts}"
    );
    assert!(
        ts.contains(".name"),
        "ts/node deep tool_calls: expected .name segment, got: {ts}"
    );
}

#[test]
fn deep_tool_calls_id_snapshot_all_langs() {
    let field = "tool_calls[0].id";

    let rust = StreamingFieldResolver::accessor(field, "rust", "chunks").unwrap();
    assert!(rust.contains(".nth(0)"), "rust: {rust}");
    assert!(rust.contains("x.id.as_deref()"), "rust: {rust}");

    let go = StreamingFieldResolver::accessor(field, "go", "chunks").unwrap();
    assert!(go.contains("[0]"), "go: {go}");
    assert!(go.contains(".ID"), "go: expected .ID initialism, got: {go}");

    let python = StreamingFieldResolver::accessor(field, "python", "chunks").unwrap();
    assert!(python.contains("[0]"), "python: {python}");
    assert!(python.contains(".id"), "python: {python}");

    let php = StreamingFieldResolver::accessor(field, "php", "chunks").unwrap();
    assert!(php.contains("[0]"), "php: {php}");
    assert!(php.contains("->id"), "php: expected ->id, got: {php}");

    let java = StreamingFieldResolver::accessor(field, "java", "chunks").unwrap();
    assert!(java.contains(".get(0)"), "java: expected .get(0), got: {java}");
    assert!(java.contains(".id()"), "java: expected .id() method call, got: {java}");

    let csharp = StreamingFieldResolver::accessor(field, "csharp", "chunks").unwrap();
    assert!(csharp.contains("[0]"), "csharp: {csharp}");
    assert!(
        csharp.contains(".Id"),
        "csharp: expected .Id (PascalCase), got: {csharp}"
    );

    let elixir = StreamingFieldResolver::accessor(field, "elixir", "chunks").unwrap();
    assert!(elixir.contains("Enum.at("), "elixir: expected Enum.at(, got: {elixir}");
    assert!(elixir.contains(".id"), "elixir: {elixir}");
}

#[test]
fn deep_tool_calls_function_name_snapshot_python_elixir_zig() {
    let field = "tool_calls[0].function.name";

    let python = StreamingFieldResolver::accessor(field, "python", "chunks").unwrap();
    assert!(python.contains("[0]"), "python: {python}");
    assert!(python.contains(".function"), "python: {python}");
    assert!(python.contains(".name"), "python: {python}");

    let elixir = StreamingFieldResolver::accessor(field, "elixir", "chunks").unwrap();
    assert!(elixir.contains("Enum.at("), "elixir: {elixir}");
    assert!(elixir.contains(".function"), "elixir: {elixir}");
    assert!(elixir.contains(".name"), "elixir: {elixir}");

    assert!(
        StreamingFieldResolver::accessor(field, "zig", "chunks").is_none(),
        "zig: expected None for deep tool_calls path"
    );
}

#[test]
fn parse_tail_parses_index_then_field_segments() {
    let segs = parse_tail("[0].function.name");
    assert_eq!(segs.len(), 3, "expected 3 segments, got: {segs:?}");
    assert_eq!(segs[0], TailSeg::Index(0));
    assert_eq!(segs[1], TailSeg::Field("function".to_string()));
    assert_eq!(segs[2], TailSeg::Field("name".to_string()));
}

#[test]
fn parse_tail_parses_simple_index_field() {
    let segs = parse_tail("[0].id");
    assert_eq!(segs.len(), 2, "expected 2 segments, got: {segs:?}");
    assert_eq!(segs[0], TailSeg::Index(0));
    assert_eq!(segs[1], TailSeg::Field("id".to_string()));
}

#[test]
fn parse_tail_handles_nonzero_index() {
    let segs = parse_tail("[2].function.arguments");
    assert_eq!(segs[0], TailSeg::Index(2));
    assert_eq!(segs[1], TailSeg::Field("function".to_string()));
    assert_eq!(segs[2], TailSeg::Field("arguments".to_string()));
}

#[test]
fn accessor_chunks_length_swift_uses_count() {
    let swift = StreamingFieldResolver::accessor("chunks.length", "swift", "chunks").unwrap();
    assert_eq!(swift, "chunks.count", "swift chunks.length: {swift}");
}

#[test]
fn accessor_stream_content_swift_uses_swift_closures() {
    let expr = StreamingFieldResolver::accessor("stream_content", "swift", "chunks").unwrap();
    assert!(
        expr.contains("{ c in"),
        "swift stream_content must use Swift closure syntax, got: {expr}"
    );
    assert!(
        !expr.contains("=>"),
        "swift stream_content must not contain JS arrow `=>`, got: {expr}"
    );
    assert!(
        expr.contains("c.choices"),
        "swift stream_content must use property access for choices, got: {expr}"
    );
    assert!(
        expr.contains("ch.delta"),
        "swift stream_content must use property access for delta, got: {expr}"
    );
    assert!(
        expr.contains("ch.delta.content"),
        "swift stream_content must use property access for content, got: {expr}"
    );
    assert!(
        !expr.contains(".toString()"),
        "swift stream_content must NOT wrap first-class String fields with .toString(), got: {expr}"
    );
    assert!(
        expr.contains(".joined()"),
        "swift stream_content must join with .joined(), got: {expr}"
    );
    assert!(
        !expr.contains(".length"),
        "swift stream_content must not use JS .length, got: {expr}"
    );
    assert!(
        !expr.contains(".join("),
        "swift stream_content must not use JS .join(, got: {expr}"
    );
}

#[test]
fn accessor_stream_complete_swift_uses_swift_syntax() {
    let expr = StreamingFieldResolver::accessor("stream_complete", "swift", "chunks").unwrap();
    assert!(
        expr.contains("isEmpty"),
        "swift stream_complete must use .isEmpty, got: {expr}"
    );
    assert!(
        expr.contains(".last!"),
        "swift stream_complete must use .last!, got: {expr}"
    );
    assert!(
        expr.contains(".choices.first"),
        "swift stream_complete must use property access on choices, got: {expr}"
    );
    assert!(
        expr.contains("finishReason"),
        "swift stream_complete must reference lowerCamelCase finishReason, got: {expr}"
    );
    assert!(
        !expr.contains(".length"),
        "swift stream_complete must not use JS .length, got: {expr}"
    );
    assert!(
        !expr.contains("!= null"),
        "swift stream_complete must not use JS `!= null`, got: {expr}"
    );
}

#[test]
fn accessor_tool_calls_swift_uses_swift_flatmap() {
    let expr = StreamingFieldResolver::accessor("tool_calls", "swift", "chunks").unwrap();
    assert!(
        !expr.contains("=>"),
        "swift tool_calls must not contain JS arrow `=>`, got: {expr}"
    );
    assert!(
        expr.contains("flatMap"),
        "swift tool_calls must use flatMap, got: {expr}"
    );
    assert!(
        expr.contains("c.choices.first"),
        "swift tool_calls must use property access on choices, got: {expr}"
    );
    assert!(
        expr.contains("ch.delta.toolCalls"),
        "swift tool_calls must use lowerCamelCase toolCalls property, got: {expr}"
    );
}

#[test]
fn accessor_tool_calls_deep_path_swift_uses_method_calls_with_optional_chain() {
    let expr = StreamingFieldResolver::accessor("tool_calls[0].function.name", "swift", "chunks").unwrap();
    assert!(
        expr.contains("[0].function"),
        "swift deep tool_calls must use plain `.function` directly after array index (non-optional), got: {expr}"
    );
    assert!(
        expr.contains("?.name"),
        "swift deep tool_calls must use ?.name property access, got: {expr}"
    );
    assert!(
        !expr.contains(".toString()"),
        "swift deep tool_calls must NOT wrap first-class String fields with .toString(), got: {expr}"
    );
    assert!(
        !expr.contains("=>"),
        "swift deep tool_calls must not use JS arrow syntax, got: {expr}"
    );
}

#[test]
fn accessor_finish_reason_swift_uses_swift_syntax() {
    let expr = StreamingFieldResolver::accessor("finish_reason", "swift", "chunks").unwrap();
    assert!(
        expr.contains("isEmpty"),
        "swift finish_reason must use .isEmpty, got: {expr}"
    );
    assert!(
        expr.contains(".last!"),
        "swift finish_reason must use .last!, got: {expr}"
    );
    assert!(
        expr.contains("finishReason"),
        "swift finish_reason must use lowerCamelCase finishReason property, got: {expr}"
    );
    assert!(
        expr.contains(".rawValue"),
        "swift finish_reason must read enum .rawValue, got: {expr}"
    );
    assert!(
        !expr.contains("undefined"),
        "swift finish_reason must not use JS `undefined`, got: {expr}"
    );
    assert!(
        !expr.contains(".length"),
        "swift finish_reason must not use JS .length, got: {expr}"
    );
}

#[test]
fn accessor_usage_swift_uses_swift_syntax() {
    let expr = StreamingFieldResolver::accessor("usage", "swift", "chunks").unwrap();
    assert!(expr.contains("isEmpty"), "swift usage must use .isEmpty, got: {expr}");
    assert!(expr.contains(".last!"), "swift usage must use .last!, got: {expr}");
    assert!(
        expr.contains(".usage"),
        "swift usage must reference .usage property, got: {expr}"
    );
    assert!(
        !expr.contains("usage()"),
        "swift usage must NOT use method-call syntax, got: {expr}"
    );
    assert!(
        !expr.contains("undefined"),
        "swift usage must not use JS `undefined`, got: {expr}"
    );
    assert!(
        !expr.contains(".length"),
        "swift usage must not use JS .length, got: {expr}"
    );
}

#[test]
fn kotlin_android_collect_snippet_uses_flow_to_list() {
    let snip = StreamingFieldResolver::collect_snippet("kotlin_android", "result", "chunks").unwrap();
    assert!(
        snip.contains("result.toList()"),
        "kotlin_android collect must use Flow.toList(), got: {snip}"
    );
    assert!(
        !snip.contains("asSequence()"),
        "kotlin_android collect must NOT use asSequence(), got: {snip}"
    );
}

#[test]
fn kotlin_android_stream_content_uses_property_access() {
    let expr = StreamingFieldResolver::accessor("stream_content", "kotlin_android", "chunks").unwrap();
    assert!(
        expr.contains(".choices"),
        "kotlin_android stream_content must use .choices property, got: {expr}"
    );
    assert!(
        !expr.contains(".choices()"),
        "kotlin_android stream_content must NOT use .choices() getter, got: {expr}"
    );
    assert!(
        expr.contains(".delta"),
        "kotlin_android stream_content must use .delta property, got: {expr}"
    );
    assert!(
        !expr.contains(".delta()"),
        "kotlin_android stream_content must NOT use .delta() getter, got: {expr}"
    );
    assert!(
        expr.contains(".content"),
        "kotlin_android stream_content must use .content property, got: {expr}"
    );
    assert!(
        !expr.contains(".content()"),
        "kotlin_android stream_content must NOT use .content() getter, got: {expr}"
    );
}

#[test]
fn kotlin_android_finish_reason_uses_name_lowercase_not_get_value() {
    let expr = StreamingFieldResolver::accessor("finish_reason", "kotlin_android", "chunks").unwrap();
    assert!(
        expr.contains(".finishReason"),
        "kotlin_android finish_reason must use .finishReason property, got: {expr}"
    );
    assert!(
        !expr.contains(".finishReason()"),
        "kotlin_android finish_reason must NOT use .finishReason() getter, got: {expr}"
    );
    assert!(
        expr.contains(".name"),
        "kotlin_android finish_reason must use .name for enum wire value, got: {expr}"
    );
    assert!(
        expr.contains(".lowercase()"),
        "kotlin_android finish_reason must use .lowercase(), got: {expr}"
    );
    assert!(
        !expr.contains(".getValue()"),
        "kotlin_android finish_reason must NOT use .getValue(), got: {expr}"
    );
}

#[test]
fn kotlin_android_usage_uses_property_access() {
    let expr = StreamingFieldResolver::accessor("usage", "kotlin_android", "chunks").unwrap();
    assert!(
        expr.contains(".usage"),
        "kotlin_android usage must use .usage property, got: {expr}"
    );
    assert!(
        !expr.contains(".usage()"),
        "kotlin_android usage must NOT use .usage() getter, got: {expr}"
    );
}

#[test]
fn kotlin_android_deep_tool_calls_uses_property_access() {
    let expr = StreamingFieldResolver::accessor("tool_calls[0].function.name", "kotlin_android", "chunks").unwrap();
    assert!(
        expr.contains(".function"),
        "kotlin_android deep tool_calls must use .function property, got: {expr}"
    );
    assert!(
        !expr.contains(".function()"),
        "kotlin_android deep tool_calls must NOT use .function() getter, got: {expr}"
    );
    assert!(
        expr.contains(".name"),
        "kotlin_android deep tool_calls must use .name property, got: {expr}"
    );
    assert!(
        !expr.contains(".name()"),
        "kotlin_android deep tool_calls must NOT use .name() getter, got: {expr}"
    );
}

#[test]
fn ruby_stream_content_uses_ruby_block_syntax() {
    let expr = StreamingFieldResolver::accessor("stream_content", "ruby", "chunks").unwrap();
    assert!(
        !expr.contains("=>"),
        "ruby stream_content must not contain JS arrow `=>`, got: {expr}"
    );
    assert!(
        expr.contains("{ |c|"),
        "ruby stream_content must use Ruby block `{{ |c|`, got: {expr}"
    );
    assert!(
        expr.contains(".join"),
        "ruby stream_content must use .join, got: {expr}"
    );
    assert!(
        expr.contains("c.choices"),
        "ruby stream_content must access .choices, got: {expr}"
    );
    assert!(
        expr.contains(".delta"),
        "ruby stream_content must access .delta, got: {expr}"
    );
    assert!(
        expr.contains(".content"),
        "ruby stream_content must access .content, got: {expr}"
    );
    assert!(
        !expr.contains("?.["),
        "ruby stream_content must not use TS optional chaining `?.[`, got: {expr}"
    );
}

#[test]
fn ruby_stream_complete_uses_ruby_nil_predicate() {
    let expr = StreamingFieldResolver::accessor("stream_complete", "ruby", "chunks").unwrap();
    assert!(
        !expr.contains("!= null"),
        "ruby stream_complete must not use JS `!= null`, got: {expr}"
    );
    assert!(
        expr.contains(".nil?"),
        "ruby stream_complete must use .nil?, got: {expr}"
    );
    assert!(
        expr.contains(".empty?"),
        "ruby stream_complete must use .empty?, got: {expr}"
    );
    assert!(
        expr.contains("finish_reason"),
        "ruby stream_complete must reference finish_reason, got: {expr}"
    );
    assert!(
        !expr.contains("?.["),
        "ruby stream_complete must not use TS optional chaining `?.[`, got: {expr}"
    );
}

#[test]
fn ruby_tool_calls_uses_ruby_flat_map_block() {
    let expr = StreamingFieldResolver::accessor("tool_calls", "ruby", "chunks").unwrap();
    assert!(
        !expr.contains("=>"),
        "ruby tool_calls must not contain JS arrow `=>`, got: {expr}"
    );
    assert!(
        expr.contains("flat_map"),
        "ruby tool_calls must use flat_map, got: {expr}"
    );
    assert!(
        expr.contains("{ |c|"),
        "ruby tool_calls must use Ruby block `{{ |c|`, got: {expr}"
    );
    assert!(
        expr.contains("tool_calls"),
        "ruby tool_calls must reference tool_calls, got: {expr}"
    );
    assert!(
        !expr.contains("?.["),
        "ruby tool_calls must not use TS optional chaining `?.[`, got: {expr}"
    );
}

#[test]
fn ruby_finish_reason_uses_to_s_not_get_value() {
    let expr = StreamingFieldResolver::accessor("finish_reason", "ruby", "chunks").unwrap();
    assert!(
        !expr.contains("undefined"),
        "ruby finish_reason must not use JS `undefined`, got: {expr}"
    );
    assert!(
        !expr.contains(".length"),
        "ruby finish_reason must not use JS .length, got: {expr}"
    );
    assert!(
        expr.contains(".empty?"),
        "ruby finish_reason must use .empty?, got: {expr}"
    );
    assert!(
        expr.contains("finish_reason"),
        "ruby finish_reason must reference finish_reason, got: {expr}"
    );
    assert!(
        expr.contains(".to_s"),
        "ruby finish_reason must call .to_s on the enum, got: {expr}"
    );
}
