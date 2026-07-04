```zig
const spikard = @import("spikard");

fn task_status_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("{\"task_id\":\"job-001\",\"status\":\"running\",\"progress\":50}", 58);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.get(&task_status_handler, null, "/tasks/:task_id");
    _ = app.run();
}
```
