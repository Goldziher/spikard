```zig
const spikard = @import("spikard");

fn get_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // Response is validated against schema before sending to client
    // Invalid responses are logged and replaced with 500 error
    return spikard.c.spikard_alloc_string("{\"status\":\"active\",\"data\":\"value\"}", 38);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&get_handler, null, "/resource");
    _ = app.run();
}
```
