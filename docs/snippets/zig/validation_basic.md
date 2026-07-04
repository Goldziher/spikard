```zig
const spikard = @import("spikard");

fn create_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // Validation happens at the spikard core level via schemas
    // Valid requests reach this handler; invalid requests return 400/422
    return spikard.c.spikard_alloc_string("{\"id\":1,\"amount\":99.99}", 25);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.post(&create_handler, null, "/payments");
    _ = app.run();
}
```
