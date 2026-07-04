```zig
const std = @import("std");

test "websocket_upgrade_returns_101" {
    const allocator = std.testing.allocator;
    var http_client = std.http.Client{ .allocator = allocator };
    defer http_client.deinit();

    var response_body = std.Io.Writer.Allocating.init(allocator);
    defer response_body.deinit();

    const response = try http_client.fetch(.{
        .location = .{ .url = "http://localhost:8080/ws" },
        .method = .GET,
        .extra_headers = &.{
            .{ .name = "Connection", .value = "Upgrade" },
            .{ .name = "Upgrade", .value = "websocket" },
        },
        .keep_alive = false,
        .redirect_behavior = .unhandled,
        .response_writer = &response_body.writer,
    });
    try std.testing.expectEqual(@as(u10, 101), @intFromEnum(response.status));
}
```
