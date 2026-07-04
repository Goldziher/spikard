```zig
const spikard = @import("spikard");

fn secure_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("{\"authorized\":true}", 20);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    // Configure API key auth via ServerConfig
    _ = app.get(&secure_handler, null, "/secure");
    _ = app.run();
}
```
