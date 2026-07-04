```zig
const spikard = @import("spikard");

fn list_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("[{\"id\":1,\"name\":\"Alice\"}]", 28);
}

fn create_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("{\"id\":1,\"name\":\"Alice\"}", 24);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&list_handler, null, "/users");
    _ = app.post(&create_handler, null, "/users");
    _ = app.run();
}
```
