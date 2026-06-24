use super::model::StreamingFieldResolver;

impl StreamingFieldResolver {
    /// Returns the language-specific stream-collect-into-list snippet that
    /// produces `chunks_var` from `stream_var`.
    ///
    /// Returns `None` when the language has no streaming collect support or
    /// when the collect snippet cannot be expressed generically.
    #[must_use]
    pub fn collect_snippet(lang: &str, stream_var: &str, chunks_var: &str) -> Option<String> {
        Self::collect_snippet_typed(lang, stream_var, chunks_var, None)
    }

    /// Collect stream into a list, with optional `item_type` for languages that need the concrete type.
    ///
    /// When `item_type` is `None`, returns `None` for languages that require an
    /// explicit stream item type. Callers should derive the item type from
    /// streaming adapter metadata or an explicit call override; otherwise they
    /// should emit a diagnostic skip instead of guessing.
    #[must_use]
    pub fn collect_snippet_typed(
        lang: &str,
        stream_var: &str,
        chunks_var: &str,
        item_type: Option<&str>,
    ) -> Option<String> {
        let item_type = item_type.filter(|value| !value.is_empty());
        match lang {
            "rust" => Some(format!(
                "let {chunks_var}: Vec<_> = tokio_stream::StreamExt::collect::<Vec<_>>({stream_var}).await\n        .into_iter()\n        .map(|r| r.expect(\"stream item failed\"))\n        .collect();"
            )),
            "go" => Some(format!(
                "var {chunks_var} []pkg.{}\n\tfor chunk := range {stream_var} {{\n\t\t{chunks_var} = append({chunks_var}, chunk)\n\t}}",
                item_type?
            )),
            "java" => Some(format!(
                "var {chunks_var} = new java.util.ArrayList<{}>();\n        var _it = {stream_var}.iterator();\n        while (_it.hasNext()) {{ {chunks_var}.add(_it.next()); }}",
                item_type?
            )),
            // PHP binding's chat_stream returns Vec<String> (each element is a
            // JSON-serialized chunk) because ext-php-rs can't expose Rust
            // iterators directly. Decode each element and recursively
            // camelCase the keys so accessor chains like
            // `$c->choices[0]->delta->finishReason` resolve against what the
            // non-streaming PHP binding returns (camelCase getters). Three
            // input shapes are tolerated: (a) array of JSON strings — the
            // current binding; (b) single concatenated JSON — older binding
            // output; (c) a real iterator — future binding upgrade.
            "php" => Some(format!(
                "$__camel = function ($v) use (&$__camel) {{ \
                    if (is_array($v)) {{ \
                        $out = []; \
                        foreach ($v as $k => $vv) {{ \
                            $key = is_string($k) ? lcfirst(str_replace(' ', '', ucwords(str_replace('_', ' ', $k)))) : $k; \
                            $out[$key] = $__camel($vv); \
                        }} \
                        return (array_keys($out) === range(0, count($out) - 1)) ? $out : (object) $out; \
                    }} \
                    if (is_object($v)) {{ \
                        $out = new \\stdClass(); \
                        foreach (get_object_vars($v) as $k => $vv) {{ \
                            $key = lcfirst(str_replace(' ', '', ucwords(str_replace('_', ' ', $k)))); \
                            $out->{{$key}} = $__camel($vv); \
                        }} \
                        return $out; \
                    }} \
                    return $v; \
                }};\n        \
                $__decode_chunk = fn($c) => $__camel(is_string($c) ? json_decode($c, true) : (is_array($c) || is_object($c) ? json_decode(json_encode($c), true) : $c));\n        \
                ${chunks_var} = is_string(${stream_var}) \
                    ? array_map($__decode_chunk, (array)(json_decode(${stream_var}, true) ?: [])) \
                    : (is_array(${stream_var}) \
                        ? array_map($__decode_chunk, ${stream_var}) \
                        : array_map($__decode_chunk, iterator_to_array(${stream_var})));"
            )),
            "python" => Some(format!(
                "{chunks_var} = []\n    async for chunk in {stream_var}:\n        {chunks_var}.append(chunk)"
            )),
            "kotlin" => {
                // Kotlin: streaming adapters return Iterator<item type> (from Java bridge).
                // Drain into a Kotlin List using asSequence().toList().
                Some(format!("val {chunks_var} = {stream_var}.asSequence().toList()"))
            }
            "kotlin_android" => {
                // kotlin-android: streaming adapters return Flow<item type> (kotlinx.coroutines).
                // Collect inside a runBlocking coroutine scope using Flow.toList().
                Some(format!("val {chunks_var} = {stream_var}.toList()"))
            }
            "elixir" => Some(format!("{chunks_var} = Enum.to_list({stream_var})")),
            // WASM's chatStream returns a hand-rolled `ChatStreamIterator`
            // struct that exposes `next()` returning `Promise<chunk | null>`,
            // not a JS async iterable. wasm-bindgen does not auto-emit the
            // `Symbol.asyncIterator` protocol, so `for await` on this object
            // throws `TypeError: stream is not async iterable`. Drain via an
            // explicit while/next() loop instead.
            "wasm" => Some(format!(
                "const {chunks_var}: any[] = [];\n    while (true) {{ const _chunk = await {stream_var}.next(); if (_chunk == null) break; {chunks_var}.push(_chunk); }}"
            )),
            "node" | "typescript" => Some(format!(
                "const {chunks_var}: any[] = [];\n    for await (const _chunk of {stream_var}) {{ {chunks_var}.push(_chunk); }}"
            )),
            "swift" => {
                // Swift's chat-stream wrapper returns AsyncThrowingStream<ChunkType, Error>,
                // so consumers drain it with `for try await chunk in stream { ... }`. The
                // chunk type is decoded from the bridge-boundary JSON inside the wrapper —
                // here we just collect the typed Swift values.
                // The item type must come from adapter metadata or an explicit override.
                let item_type = item_type?;
                Some(format!(
                    "var {chunks_var}: [{item_type}] = []\n        for try await _chunk in {stream_var} {{ {chunks_var}.append(_chunk) }}"
                ))
            }
            "zig" => None,
            _ => None,
        }
    }

    /// Render Zig's streaming collect snippet using the configured module and FFI prefix.
    #[must_use]
    pub fn collect_snippet_zig(
        stream_var: &str,
        chunks_var: &str,
        module_name: &str,
        ffi_prefix: &str,
        owner_type: &str,
        adapter_name: &str,
        item_type: &str,
    ) -> String {
        use heck::ToSnakeCase;

        let owner_snake = owner_type.to_snake_case();
        let item_snake = item_type.to_snake_case();
        let stream_next = format!("{ffi_prefix}_{owner_snake}_{adapter_name}_next");
        let chunk_to_json = format!("{ffi_prefix}_{item_snake}_to_json");
        let chunk_free = format!("{ffi_prefix}_{item_snake}_free");
        let free_string = format!("{ffi_prefix}_free_string");

        // Zig 0.16: ArrayList is unmanaged — no stored allocator.
        // Use `.empty` to initialize, pass `std.heap.c_allocator` to each mutation.
        // `stream_var` is the opaque stream handle obtained via `_start`.
        // We collect every chunk's JSON string into `chunks_var: ArrayList([]u8)`
        // and concatenate delta content into `{chunks_var}_content: ArrayList(u8)`.
        // Accessors use `.items.len` and `{chunks_var}_content.items` on these lists.
        format!(
            concat!(
                "var {chunks_var}: std.ArrayList([]u8) = .empty;
",
                "    defer {{
",
                "        for ({chunks_var}.items) |_cj| std.heap.c_allocator.free(_cj);
",
                "        {chunks_var}.deinit(std.heap.c_allocator);
",
                "    }}
",
                "    var {chunks_var}_content: std.ArrayList(u8) = .empty;
",
                "    defer {chunks_var}_content.deinit(std.heap.c_allocator);
",
                "    while (true) {{
",
                "        const _nc = {module_name}.c.{stream_next}({stream_var});
",
                "        if (_nc == null) break;
",
                "        const _np = {module_name}.c.{chunk_to_json}(_nc);
",
                "        {module_name}.c.{chunk_free}(_nc);
",
                "        if (_np == null) continue;
",
                "        const _ns = std.mem.span(_np);
",
                "        const _nj = try std.heap.c_allocator.dupe(u8, _ns);
",
                "        {module_name}.c.{free_string}(_np);
",
                "        if (std.json.parseFromSlice(std.json.Value, std.heap.c_allocator, _nj, .{{}})) |_cp| {{
",
                "            defer _cp.deinit();
",
                "            if (_cp.value.object.get(\"choices\")) |_chs|
",
                "                if (_chs.array.items.len > 0)
",
                "                    if (_chs.array.items[0].object.get(\"delta\")) |_dl|
",
                "                        if (_dl.object.get(\"content\")) |_ct|
",
                "                            if (_ct == .string) try {chunks_var}_content.appendSlice(std.heap.c_allocator, _ct.string);
",
                "        }} else |_| {{}}
",
                "        try {chunks_var}.append(std.heap.c_allocator, _nj);
",
                "    }}"
            ),
            chunks_var = chunks_var,
            stream_var = stream_var,
            module_name = module_name,
            stream_next = stream_next,
            chunk_to_json = chunk_to_json,
            chunk_free = chunk_free,
            free_string = free_string,
        )
    }
}
