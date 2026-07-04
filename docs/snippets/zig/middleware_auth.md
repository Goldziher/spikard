```zig
const spikard = @import("spikard");

fn protected_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("{\"data\":\"secret\"}", 18);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    // Auth is configured via ServerConfig jwt_auth or api_key_auth
    // Failed auth returns 401 before reaching handler
    _ = app.get(&protected_handler, null, "/api/protected");
    _ = app.run();
}
```
