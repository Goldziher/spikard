```zig
const spikard = @import("spikard");

fn get_user_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // Path params are decoded from the request JSON
    return spikard.c.spikard_alloc_string("{\"id\":123,\"name\":\"Alice\"}", 28);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&get_user_handler, null, "/users/:id");
    _ = app.run();
}
```
