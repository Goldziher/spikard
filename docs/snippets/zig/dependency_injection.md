```zig
const spikard = @import("spikard");

const Context = struct {
    db_connection: i32,
};

fn get_user_handler(ctx: *anyopaque, json_req: [*:0]const u8) callconv(.C) [*:0]u8 {
    _ = json_req;
    const context = @as(*Context, @ptrCast(ctx));
    _ = context.db_connection;
    return spikard.c.spikard_alloc_string("{\"id\":1,\"name\":\"Alice\"}", 24);
}

pub fn main() !void {
    var app = spikard.api.App.init();
    defer app.deinit();
    
    var context = Context{ .db_connection = 1 };
    _ = app.get(&get_user_handler, &context, "/users/:id");
    _ = app.run();
}
```
