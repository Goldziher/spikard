```zig
const spikard = @import("spikard");

fn handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("{\"status\":\"ok\"}", 16);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    // Enable HTTP tracing via ServerConfig.enable_http_trace
    _ = app.get(&handler, null, "/health");
    _ = app.run();
}
```
