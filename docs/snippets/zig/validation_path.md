```zig
const spikard = @import("spikard");

fn get_order_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // Path validation: order_id must match integer constraint
    return spikard.c.spikard_alloc_string("{\"id\":1,\"details\":false}", 26);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&get_order_handler, null, "/orders/:order_id");
    _ = app.run();
}
```
