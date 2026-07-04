```zig
const spikard = @import("spikard");

fn events_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // SSE streams are sent as JSON array of SseEvent structs
    return spikard.c.spikard_alloc_string("[{\"event_type\":\"update\",\"data\":\"{\\\"status\\\":\\\"processing\\\"}\"}]", 70);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&events_handler, null, "/events");
    _ = app.run();
}
```
