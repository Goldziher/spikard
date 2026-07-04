```zig
const std = @import("std");
const spikard = @import("spikard");

fn handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // Return response as null-terminated string allocated by handler
    return spikard.c.spikard_alloc_string("{\"message\":\"Hello, World!}", 24);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&handler, null, "/hello");
    _ = app.run();
}
```
