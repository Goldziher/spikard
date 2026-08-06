```zig
const std = @import("std");
const spikard = @import("spikard");

fn create_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    const request = std.mem.sliceTo(json_req, 0);

    if (std.mem.indexOf(u8, request, "\"name\"")) |_| {
        const ok_body = "{\"id\":1,\"name\":\"Alice\"}";
        return spikard.c.spikard_alloc_string(ok_body, ok_body.len);
    }

    const error_body = "{\"type_uri\":\"https://spikard.dev/errors/validation-error\",\"title\":\"Request Validation Failed\",\"status\":422,\"detail\":\"name is required\",\"errors\":[{\"field\":\"name\",\"message\":\"Name is required\"}]}";
    return spikard.c.spikard_alloc_string(error_body, error_body.len);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    _ = app.post(&create_handler, null, "/items");
    _ = app.run();
}
```
