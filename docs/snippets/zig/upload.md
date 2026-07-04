```zig
const spikard = @import("spikard");

fn upload_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // File upload is handled by multipart/form-data parsing
    // json_req contains file metadata (filename, content_type, content)
    return spikard.c.spikard_alloc_string("{\"filename\":\"file.txt\",\"size\":1024,\"status\":\"uploaded\"}", 58);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.post(&upload_handler, null, "/upload");
    _ = app.run();
}
```
