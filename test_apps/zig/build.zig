const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");
    const ffi_path = b.option([]const u8, "ffi_path", "Path to directory containing libspikard_ffi") orelse "../../target/release";
    const ffi_include = b.option([]const u8, "ffi_include_path", "Path to directory containing FFI header") orelse "../../crates/spikard-ffi/include";

    const spikard_module = b.addModule("spikard", .{
        .root_source_file = b.path("../../packages/zig/src/spikard.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    spikard_module.addLibraryPath(.{ .cwd_relative = ffi_path });
    spikard_module.addIncludePath(.{ .cwd_relative = ffi_include });
    spikard_module.linkSystemLibrary("spikard_ffi", .{});

    const _alloc = b.allocator;
    var mock_server_url: ?[]const u8 = b.graph.environ_map.get("MOCK_SERVER_URL");
    var mock_servers_json: ?[]const u8 = null;
    var mock_servers_map = std.StringHashMap([]const u8).init(_alloc);
    if (mock_server_url == null) {
        const _bin = b.pathFromRoot("../rust/target/release/mock-server");
        const _fixtures = b.pathFromRoot("../../fixtures");
        var _threaded = std.Io.Threaded.init(_alloc, .{});
        const _io = _threaded.io();
        const _spawned = std.process.spawn(_io, .{
            .argv = &.{ _bin, _fixtures },
            .stdin = .pipe,
            .stdout = .pipe,
            .stderr = .inherit,
        });
        if (_spawned) |_child| {
            // The child is intentionally not awaited: it lives for the duration
            // of the `zig build` process, which spans test execution.
            const _stdout = _child.stdout.?;
            var _buf: [65536]u8 = undefined;
            var _file_reader = _stdout.readerStreaming(_io, &_buf);
            const _r = &_file_reader.interface;
            // Read startup lines: MOCK_SERVER_URL= then MOCK_SERVERS= (always
            // emitted, possibly `{}`). Cap the loop so a misbehaving server
            // cannot block the build indefinitely.
            var _saw_url = false;
            var _i: usize = 0;
            while (_i < 64) : (_i += 1) {
                const _line_raw = _r.takeDelimiterExclusive('\n') catch break;
                const _line = std.mem.trim(u8, _line_raw, " \r\t");
                if (std.mem.startsWith(u8, _line, "MOCK_SERVER_URL=")) {
                    mock_server_url = _alloc.dupe(u8, _line["MOCK_SERVER_URL=".len..]) catch null;
                    _saw_url = true;
                } else if (std.mem.startsWith(u8, _line, "MOCK_SERVERS=")) {
                    const _json = _line["MOCK_SERVERS=".len..];
                    mock_servers_json = _alloc.dupe(u8, _json) catch null;
                    if (std.json.parseFromSlice(std.json.Value, _alloc, _json, .{})) |_parsed| {
                        if (_parsed.value == .object) {
                            var _entries = _parsed.value.object.iterator();
                            while (_entries.next()) |_entry| {
                                if (_entry.value_ptr.* == .string) {
                                    const _key = std.fmt.allocPrint(_alloc, "MOCK_SERVER_{s}", .{_entry.key_ptr.*}) catch continue;
                                    for (_key) |*_c| _c.* = std.ascii.toUpper(_c.*);
                                    const _val = _alloc.dupe(u8, _entry.value_ptr.*.string) catch continue;
                                    mock_servers_map.put(_key, _val) catch {};
                                }
                            }
                        }
                    } else |_| {}
                    break;
                } else if (_saw_url) {
                    break;
                }
            }
        } else |_| {
            // Binary not built — leave mock_server_url null so tests surface a
            // clear connection error rather than a build failure.
        }
    }

    const auth_module = b.createModule(.{
        .root_source_file = b.path("src/auth_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    auth_module.addImport("spikard", spikard_module);
    const auth_tests = b.addTest(.{
        .name = "auth_test",
        .root_module = auth_module,
        .use_llvm = true,
    });
    const auth_run = b.addRunArtifact(auth_tests);
    if (mock_server_url) |_url| {
        auth_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        auth_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            auth_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&auth_run.step);

    const background_module = b.createModule(.{
        .root_source_file = b.path("src/background_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    background_module.addImport("spikard", spikard_module);
    const background_tests = b.addTest(.{
        .name = "background_test",
        .root_module = background_module,
        .use_llvm = true,
    });
    const background_run = b.addRunArtifact(background_tests);
    if (mock_server_url) |_url| {
        background_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        background_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            background_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&background_run.step);

    const background_tasks_module = b.createModule(.{
        .root_source_file = b.path("src/background_tasks_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    background_tasks_module.addImport("spikard", spikard_module);
    const background_tasks_tests = b.addTest(.{
        .name = "background_tasks_test",
        .root_module = background_tasks_module,
        .use_llvm = true,
    });
    const background_tasks_run = b.addRunArtifact(background_tasks_tests);
    if (mock_server_url) |_url| {
        background_tasks_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        background_tasks_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            background_tasks_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&background_tasks_run.step);

    const body_limits_module = b.createModule(.{
        .root_source_file = b.path("src/body_limits_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    body_limits_module.addImport("spikard", spikard_module);
    const body_limits_tests = b.addTest(.{
        .name = "body_limits_test",
        .root_module = body_limits_module,
        .use_llvm = true,
    });
    const body_limits_run = b.addRunArtifact(body_limits_tests);
    if (mock_server_url) |_url| {
        body_limits_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        body_limits_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            body_limits_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&body_limits_run.step);

    const compression_module = b.createModule(.{
        .root_source_file = b.path("src/compression_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    compression_module.addImport("spikard", spikard_module);
    const compression_tests = b.addTest(.{
        .name = "compression_test",
        .root_module = compression_module,
        .use_llvm = true,
    });
    const compression_run = b.addRunArtifact(compression_tests);
    if (mock_server_url) |_url| {
        compression_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        compression_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            compression_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&compression_run.step);

    const content_types_module = b.createModule(.{
        .root_source_file = b.path("src/content_types_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    content_types_module.addImport("spikard", spikard_module);
    const content_types_tests = b.addTest(.{
        .name = "content_types_test",
        .root_module = content_types_module,
        .use_llvm = true,
    });
    const content_types_run = b.addRunArtifact(content_types_tests);
    if (mock_server_url) |_url| {
        content_types_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        content_types_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            content_types_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&content_types_run.step);

    const cookies_module = b.createModule(.{
        .root_source_file = b.path("src/cookies_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    cookies_module.addImport("spikard", spikard_module);
    const cookies_tests = b.addTest(.{
        .name = "cookies_test",
        .root_module = cookies_module,
        .use_llvm = true,
    });
    const cookies_run = b.addRunArtifact(cookies_tests);
    if (mock_server_url) |_url| {
        cookies_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        cookies_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            cookies_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&cookies_run.step);

    const cors_module = b.createModule(.{
        .root_source_file = b.path("src/cors_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    cors_module.addImport("spikard", spikard_module);
    const cors_tests = b.addTest(.{
        .name = "cors_test",
        .root_module = cors_module,
        .use_llvm = true,
    });
    const cors_run = b.addRunArtifact(cors_tests);
    if (mock_server_url) |_url| {
        cors_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        cors_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            cors_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&cors_run.step);

    const di_module = b.createModule(.{
        .root_source_file = b.path("src/di_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    di_module.addImport("spikard", spikard_module);
    const di_tests = b.addTest(.{
        .name = "di_test",
        .root_module = di_module,
        .use_llvm = true,
    });
    const di_run = b.addRunArtifact(di_tests);
    if (mock_server_url) |_url| {
        di_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        di_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            di_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&di_run.step);

    const edge_cases_module = b.createModule(.{
        .root_source_file = b.path("src/edge_cases_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    edge_cases_module.addImport("spikard", spikard_module);
    const edge_cases_tests = b.addTest(.{
        .name = "edge_cases_test",
        .root_module = edge_cases_module,
        .use_llvm = true,
    });
    const edge_cases_run = b.addRunArtifact(edge_cases_tests);
    if (mock_server_url) |_url| {
        edge_cases_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        edge_cases_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            edge_cases_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&edge_cases_run.step);

    const graphql_operations_module = b.createModule(.{
        .root_source_file = b.path("src/graphql_operations_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    graphql_operations_module.addImport("spikard", spikard_module);
    const graphql_operations_tests = b.addTest(.{
        .name = "graphql_operations_test",
        .root_module = graphql_operations_module,
        .use_llvm = true,
    });
    const graphql_operations_run = b.addRunArtifact(graphql_operations_tests);
    if (mock_server_url) |_url| {
        graphql_operations_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        graphql_operations_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            graphql_operations_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&graphql_operations_run.step);

    const graphql_schema_module = b.createModule(.{
        .root_source_file = b.path("src/graphql_schema_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    graphql_schema_module.addImport("spikard", spikard_module);
    const graphql_schema_tests = b.addTest(.{
        .name = "graphql_schema_test",
        .root_module = graphql_schema_module,
        .use_llvm = true,
    });
    const graphql_schema_run = b.addRunArtifact(graphql_schema_tests);
    if (mock_server_url) |_url| {
        graphql_schema_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        graphql_schema_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            graphql_schema_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&graphql_schema_run.step);

    const grpc_module = b.createModule(.{
        .root_source_file = b.path("src/grpc_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    grpc_module.addImport("spikard", spikard_module);
    const grpc_tests = b.addTest(.{
        .name = "grpc_test",
        .root_module = grpc_module,
        .use_llvm = true,
    });
    const grpc_run = b.addRunArtifact(grpc_tests);
    if (mock_server_url) |_url| {
        grpc_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        grpc_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            grpc_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&grpc_run.step);

    const headers_module = b.createModule(.{
        .root_source_file = b.path("src/headers_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    headers_module.addImport("spikard", spikard_module);
    const headers_tests = b.addTest(.{
        .name = "headers_test",
        .root_module = headers_module,
        .use_llvm = true,
    });
    const headers_run = b.addRunArtifact(headers_tests);
    if (mock_server_url) |_url| {
        headers_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        headers_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            headers_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&headers_run.step);

    const http_methods_module = b.createModule(.{
        .root_source_file = b.path("src/http_methods_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    http_methods_module.addImport("spikard", spikard_module);
    const http_methods_tests = b.addTest(.{
        .name = "http_methods_test",
        .root_module = http_methods_module,
        .use_llvm = true,
    });
    const http_methods_run = b.addRunArtifact(http_methods_tests);
    if (mock_server_url) |_url| {
        http_methods_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        http_methods_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            http_methods_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&http_methods_run.step);

    const json_bodies_module = b.createModule(.{
        .root_source_file = b.path("src/json_bodies_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    json_bodies_module.addImport("spikard", spikard_module);
    const json_bodies_tests = b.addTest(.{
        .name = "json_bodies_test",
        .root_module = json_bodies_module,
        .use_llvm = true,
    });
    const json_bodies_run = b.addRunArtifact(json_bodies_tests);
    if (mock_server_url) |_url| {
        json_bodies_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        json_bodies_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            json_bodies_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&json_bodies_run.step);

    const jsonrpc_module = b.createModule(.{
        .root_source_file = b.path("src/jsonrpc_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    jsonrpc_module.addImport("spikard", spikard_module);
    const jsonrpc_tests = b.addTest(.{
        .name = "jsonrpc_test",
        .root_module = jsonrpc_module,
        .use_llvm = true,
    });
    const jsonrpc_run = b.addRunArtifact(jsonrpc_tests);
    if (mock_server_url) |_url| {
        jsonrpc_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        jsonrpc_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            jsonrpc_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&jsonrpc_run.step);

    const lifecycle_hooks_module = b.createModule(.{
        .root_source_file = b.path("src/lifecycle_hooks_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    lifecycle_hooks_module.addImport("spikard", spikard_module);
    const lifecycle_hooks_tests = b.addTest(.{
        .name = "lifecycle_hooks_test",
        .root_module = lifecycle_hooks_module,
        .use_llvm = true,
    });
    const lifecycle_hooks_run = b.addRunArtifact(lifecycle_hooks_tests);
    if (mock_server_url) |_url| {
        lifecycle_hooks_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        lifecycle_hooks_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            lifecycle_hooks_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&lifecycle_hooks_run.step);

    const multipart_module = b.createModule(.{
        .root_source_file = b.path("src/multipart_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    multipart_module.addImport("spikard", spikard_module);
    const multipart_tests = b.addTest(.{
        .name = "multipart_test",
        .root_module = multipart_module,
        .use_llvm = true,
    });
    const multipart_run = b.addRunArtifact(multipart_tests);
    if (mock_server_url) |_url| {
        multipart_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        multipart_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            multipart_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&multipart_run.step);

    const openapi_module = b.createModule(.{
        .root_source_file = b.path("src/openapi_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    openapi_module.addImport("spikard", spikard_module);
    const openapi_tests = b.addTest(.{
        .name = "openapi_test",
        .root_module = openapi_module,
        .use_llvm = true,
    });
    const openapi_run = b.addRunArtifact(openapi_tests);
    if (mock_server_url) |_url| {
        openapi_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        openapi_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            openapi_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&openapi_run.step);

    const openrpc_module = b.createModule(.{
        .root_source_file = b.path("src/openrpc_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    openrpc_module.addImport("spikard", spikard_module);
    const openrpc_tests = b.addTest(.{
        .name = "openrpc_test",
        .root_module = openrpc_module,
        .use_llvm = true,
    });
    const openrpc_run = b.addRunArtifact(openrpc_tests);
    if (mock_server_url) |_url| {
        openrpc_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        openrpc_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            openrpc_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&openrpc_run.step);

    const path_params_module = b.createModule(.{
        .root_source_file = b.path("src/path_params_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    path_params_module.addImport("spikard", spikard_module);
    const path_params_tests = b.addTest(.{
        .name = "path_params_test",
        .root_module = path_params_module,
        .use_llvm = true,
    });
    const path_params_run = b.addRunArtifact(path_params_tests);
    if (mock_server_url) |_url| {
        path_params_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        path_params_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            path_params_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&path_params_run.step);

    const problem_details_module = b.createModule(.{
        .root_source_file = b.path("src/problem_details_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    problem_details_module.addImport("spikard", spikard_module);
    const problem_details_tests = b.addTest(.{
        .name = "problem_details_test",
        .root_module = problem_details_module,
        .use_llvm = true,
    });
    const problem_details_run = b.addRunArtifact(problem_details_tests);
    if (mock_server_url) |_url| {
        problem_details_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        problem_details_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            problem_details_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&problem_details_run.step);

    const query_params_module = b.createModule(.{
        .root_source_file = b.path("src/query_params_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    query_params_module.addImport("spikard", spikard_module);
    const query_params_tests = b.addTest(.{
        .name = "query_params_test",
        .root_module = query_params_module,
        .use_llvm = true,
    });
    const query_params_run = b.addRunArtifact(query_params_tests);
    if (mock_server_url) |_url| {
        query_params_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        query_params_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            query_params_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&query_params_run.step);

    const rate_limit_module = b.createModule(.{
        .root_source_file = b.path("src/rate_limit_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    rate_limit_module.addImport("spikard", spikard_module);
    const rate_limit_tests = b.addTest(.{
        .name = "rate_limit_test",
        .root_module = rate_limit_module,
        .use_llvm = true,
    });
    const rate_limit_run = b.addRunArtifact(rate_limit_tests);
    if (mock_server_url) |_url| {
        rate_limit_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        rate_limit_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            rate_limit_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&rate_limit_run.step);

    const request_id_module = b.createModule(.{
        .root_source_file = b.path("src/request_id_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    request_id_module.addImport("spikard", spikard_module);
    const request_id_tests = b.addTest(.{
        .name = "request_id_test",
        .root_module = request_id_module,
        .use_llvm = true,
    });
    const request_id_run = b.addRunArtifact(request_id_tests);
    if (mock_server_url) |_url| {
        request_id_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        request_id_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            request_id_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&request_id_run.step);

    const request_timeout_module = b.createModule(.{
        .root_source_file = b.path("src/request_timeout_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    request_timeout_module.addImport("spikard", spikard_module);
    const request_timeout_tests = b.addTest(.{
        .name = "request_timeout_test",
        .root_module = request_timeout_module,
        .use_llvm = true,
    });
    const request_timeout_run = b.addRunArtifact(request_timeout_tests);
    if (mock_server_url) |_url| {
        request_timeout_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        request_timeout_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            request_timeout_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&request_timeout_run.step);

    const response_module = b.createModule(.{
        .root_source_file = b.path("src/response_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    response_module.addImport("spikard", spikard_module);
    const response_tests = b.addTest(.{
        .name = "response_test",
        .root_module = response_module,
        .use_llvm = true,
    });
    const response_run = b.addRunArtifact(response_tests);
    if (mock_server_url) |_url| {
        response_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        response_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            response_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&response_run.step);

    const server_config_module = b.createModule(.{
        .root_source_file = b.path("src/server_config_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    server_config_module.addImport("spikard", spikard_module);
    const server_config_tests = b.addTest(.{
        .name = "server_config_test",
        .root_module = server_config_module,
        .use_llvm = true,
    });
    const server_config_run = b.addRunArtifact(server_config_tests);
    if (mock_server_url) |_url| {
        server_config_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        server_config_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            server_config_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&server_config_run.step);

    const sse_module = b.createModule(.{
        .root_source_file = b.path("src/sse_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    sse_module.addImport("spikard", spikard_module);
    const sse_tests = b.addTest(.{
        .name = "sse_test",
        .root_module = sse_module,
        .use_llvm = true,
    });
    const sse_run = b.addRunArtifact(sse_tests);
    if (mock_server_url) |_url| {
        sse_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        sse_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            sse_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&sse_run.step);

    const static_files_module = b.createModule(.{
        .root_source_file = b.path("src/static_files_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    static_files_module.addImport("spikard", spikard_module);
    const static_files_tests = b.addTest(.{
        .name = "static_files_test",
        .root_module = static_files_module,
        .use_llvm = true,
    });
    const static_files_run = b.addRunArtifact(static_files_tests);
    if (mock_server_url) |_url| {
        static_files_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        static_files_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            static_files_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&static_files_run.step);

    const status_codes_module = b.createModule(.{
        .root_source_file = b.path("src/status_codes_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    status_codes_module.addImport("spikard", spikard_module);
    const status_codes_tests = b.addTest(.{
        .name = "status_codes_test",
        .root_module = status_codes_module,
        .use_llvm = true,
    });
    const status_codes_run = b.addRunArtifact(status_codes_tests);
    if (mock_server_url) |_url| {
        status_codes_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        status_codes_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            status_codes_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&status_codes_run.step);

    const streaming_module = b.createModule(.{
        .root_source_file = b.path("src/streaming_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    streaming_module.addImport("spikard", spikard_module);
    const streaming_tests = b.addTest(.{
        .name = "streaming_test",
        .root_module = streaming_module,
        .use_llvm = true,
    });
    const streaming_run = b.addRunArtifact(streaming_tests);
    if (mock_server_url) |_url| {
        streaming_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        streaming_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            streaming_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&streaming_run.step);

    const upload_module = b.createModule(.{
        .root_source_file = b.path("src/upload_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    upload_module.addImport("spikard", spikard_module);
    const upload_tests = b.addTest(.{
        .name = "upload_test",
        .root_module = upload_module,
        .use_llvm = true,
    });
    const upload_run = b.addRunArtifact(upload_tests);
    if (mock_server_url) |_url| {
        upload_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        upload_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            upload_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&upload_run.step);

    const url_encoded_module = b.createModule(.{
        .root_source_file = b.path("src/url_encoded_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    url_encoded_module.addImport("spikard", spikard_module);
    const url_encoded_tests = b.addTest(.{
        .name = "url_encoded_test",
        .root_module = url_encoded_module,
        .use_llvm = true,
    });
    const url_encoded_run = b.addRunArtifact(url_encoded_tests);
    if (mock_server_url) |_url| {
        url_encoded_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        url_encoded_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            url_encoded_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&url_encoded_run.step);

    const validation_errors_module = b.createModule(.{
        .root_source_file = b.path("src/validation_errors_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    validation_errors_module.addImport("spikard", spikard_module);
    const validation_errors_tests = b.addTest(.{
        .name = "validation_errors_test",
        .root_module = validation_errors_module,
        .use_llvm = true,
    });
    const validation_errors_run = b.addRunArtifact(validation_errors_tests);
    if (mock_server_url) |_url| {
        validation_errors_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        validation_errors_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            validation_errors_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&validation_errors_run.step);

    const websocket_module = b.createModule(.{
        .root_source_file = b.path("src/websocket_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    websocket_module.addImport("spikard", spikard_module);
    const websocket_tests = b.addTest(.{
        .name = "websocket_test",
        .root_module = websocket_module,
        .use_llvm = true,
    });
    const websocket_run = b.addRunArtifact(websocket_tests);
    if (mock_server_url) |_url| {
        websocket_run.setEnvironmentVariable("MOCK_SERVER_URL", _url);
    }
    if (mock_servers_json) |_json| {
        websocket_run.setEnvironmentVariable("MOCK_SERVERS", _json);
    }
    {
        var _it = mock_servers_map.iterator();
        while (_it.next()) |_entry| {
            websocket_run.setEnvironmentVariable(_entry.key_ptr.*, _entry.value_ptr.*);
        }
    }
    test_step.dependOn(&websocket_run.step);

}
