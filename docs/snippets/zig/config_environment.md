```zig
const spikard = @import("spikard");
const std = @import("std");

fn handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("{\"status\":\"ok\"}", 16);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    
    const port = if (std.os.getenv("PORT")) |port_str|
        try std.fmt.parseInt(u16, port_str, 10) catch 8000
    else
        8000;
    
    _ = port;
    _ = app.get(&handler, null, "/health");
    _ = app.run();
}
```
