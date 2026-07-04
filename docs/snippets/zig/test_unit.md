```zig
const std = @import("std");
const spikard = @import("spikard");

test "handler_returns_valid_json" {
    const result = spikard.c.spikard_alloc_string("{\"id\":1}", 7);
    defer spikard.api._free_string(result);
    try std.testing.expect(result != null);
}
```
