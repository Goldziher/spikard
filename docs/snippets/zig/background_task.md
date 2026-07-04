```zig
const spikard = @import("spikard");

fn submit_task_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    // Submit background task via task queue
    return spikard.c.spikard_alloc_string("{\"task_id\":\"job-001\",\"status\":\"queued\"}", 43);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.post(&submit_task_handler, null, "/tasks");
    _ = app.run();
}
```
