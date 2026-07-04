```zig
const spikard = @import("spikard");

fn ws_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // WebSocket upgrade is handled by spikard at protocol level
    // Handler receives text/binary frames, returns response frames
    return spikard.c.spikard_alloc_string("{\"type\":\"message\",\"text\":\"Hello from server\"}", 48);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&ws_handler, null, "/ws");
    _ = app.run();
}
```
