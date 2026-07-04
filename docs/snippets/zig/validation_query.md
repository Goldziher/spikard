```zig
const spikard = @import("spikard");

fn list_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // Query params are validated according to schema constraints
    return spikard.c.spikard_alloc_string("[{\"id\":1}]", 10);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&list_handler, null, "/items");
    _ = app.run();
}
```
