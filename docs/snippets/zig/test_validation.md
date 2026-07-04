```zig
const std = @import("std");

test "validation_returns_422_for_invalid_input" {
    const allocator = std.testing.allocator;
    var http_client = std.http.Client{ .allocator = allocator };
    defer http_client.deinit();

    var response_body = std.Io.Writer.Allocating.init(allocator);
    defer response_body.deinit();

    const response = try http_client.fetch(.{
        .location = .{ .url = "http://localhost:8080/payments" },
        .method = .POST,
        .extra_headers = &.{},
        .payload = "{\"amount\":\"invalid\"}",
        .keep_alive = false,
        .redirect_behavior = .unhandled,
        .response_writer = &response_body.writer,
    });
    try std.testing.expectEqual(@as(u10, 422), @intFromEnum(response.status));
}
```
