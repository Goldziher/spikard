```zig
const spikard = @import("spikard");

fn retry_task_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("{\"task_id\":\"job-001\",\"retry\":true,\"status\":\"queued\"}", 56);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.post(&retry_task_handler, null, "/tasks/:task_id/retry");
    _ = app.run();
}
```
