```zig
const spikard = @import("spikard");

fn handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = ctx;
    _ = json_req;
    return spikard.c.spikard_alloc_string("{\"data\":\"value\"}", 17);
}
```
