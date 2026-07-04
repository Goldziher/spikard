```zig
const spikard = @import("spikard");
const std = @import("std");

fn create_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // json_req contains the parsed request body as a JSON string
    // Parse using standard JSON parsing, or return the same data
    return spikard.c.spikard_alloc_string("{\"id\":1,\"name\":\"Alice\"}", 24);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.post(&create_handler, null, "/users");
    _ = app.run();
}
```
