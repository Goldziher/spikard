```zig
const spikard = @import("spikard");

fn download_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // Streaming responses are sent as chunked HTTP transfer encoding
    return spikard.c.spikard_alloc_string("{\"chunks\":[{\"data\":\"chunk1\"},{\"data\":\"chunk2\"}]}", 52);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&download_handler, null, "/download");
    _ = app.run();
}
```
