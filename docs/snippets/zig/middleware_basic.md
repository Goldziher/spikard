```zig
const spikard = @import("spikard");

fn handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("{\"data\":\"value\"}", 17);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    // Middleware is configured via ServerConfig before running
    _ = app.get(&handler, null, "/api/data");
    _ = app.run();
}
```
