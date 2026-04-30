const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");

    const asyncapi_module = b.createModule(.{
        .root_source_file = b.path("src/asyncapi_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const asyncapi_tests = b.addTest(.{
        .root_module = asyncapi_module,
    });
    const asyncapi_run = b.addRunArtifact(asyncapi_tests);
    test_step.dependOn(&asyncapi_run.step);

    const auth_module = b.createModule(.{
        .root_source_file = b.path("src/auth_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const auth_tests = b.addTest(.{
        .root_module = auth_module,
    });
    const auth_run = b.addRunArtifact(auth_tests);
    test_step.dependOn(&auth_run.step);

    const background_module = b.createModule(.{
        .root_source_file = b.path("src/background_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const background_tests = b.addTest(.{
        .root_module = background_module,
    });
    const background_run = b.addRunArtifact(background_tests);
    test_step.dependOn(&background_run.step);

    const background_tasks_module = b.createModule(.{
        .root_source_file = b.path("src/background_tasks_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const background_tasks_tests = b.addTest(.{
        .root_module = background_tasks_module,
    });
    const background_tasks_run = b.addRunArtifact(background_tasks_tests);
    test_step.dependOn(&background_tasks_run.step);

    const body_limits_module = b.createModule(.{
        .root_source_file = b.path("src/body_limits_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const body_limits_tests = b.addTest(.{
        .root_module = body_limits_module,
    });
    const body_limits_run = b.addRunArtifact(body_limits_tests);
    test_step.dependOn(&body_limits_run.step);

    const compression_module = b.createModule(.{
        .root_source_file = b.path("src/compression_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const compression_tests = b.addTest(.{
        .root_module = compression_module,
    });
    const compression_run = b.addRunArtifact(compression_tests);
    test_step.dependOn(&compression_run.step);

    const content_types_module = b.createModule(.{
        .root_source_file = b.path("src/content_types_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const content_types_tests = b.addTest(.{
        .root_module = content_types_module,
    });
    const content_types_run = b.addRunArtifact(content_types_tests);
    test_step.dependOn(&content_types_run.step);

    const cookies_module = b.createModule(.{
        .root_source_file = b.path("src/cookies_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const cookies_tests = b.addTest(.{
        .root_module = cookies_module,
    });
    const cookies_run = b.addRunArtifact(cookies_tests);
    test_step.dependOn(&cookies_run.step);

    const cors_module = b.createModule(.{
        .root_source_file = b.path("src/cors_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const cors_tests = b.addTest(.{
        .root_module = cors_module,
    });
    const cors_run = b.addRunArtifact(cors_tests);
    test_step.dependOn(&cors_run.step);

    const di_module = b.createModule(.{
        .root_source_file = b.path("src/di_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const di_tests = b.addTest(.{
        .root_module = di_module,
    });
    const di_run = b.addRunArtifact(di_tests);
    test_step.dependOn(&di_run.step);

    const edge_cases_module = b.createModule(.{
        .root_source_file = b.path("src/edge_cases_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const edge_cases_tests = b.addTest(.{
        .root_module = edge_cases_module,
    });
    const edge_cases_run = b.addRunArtifact(edge_cases_tests);
    test_step.dependOn(&edge_cases_run.step);

    const graphql_operations_module = b.createModule(.{
        .root_source_file = b.path("src/graphql_operations_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const graphql_operations_tests = b.addTest(.{
        .root_module = graphql_operations_module,
    });
    const graphql_operations_run = b.addRunArtifact(graphql_operations_tests);
    test_step.dependOn(&graphql_operations_run.step);

    const graphql_schema_module = b.createModule(.{
        .root_source_file = b.path("src/graphql_schema_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const graphql_schema_tests = b.addTest(.{
        .root_module = graphql_schema_module,
    });
    const graphql_schema_run = b.addRunArtifact(graphql_schema_tests);
    test_step.dependOn(&graphql_schema_run.step);

    const grpc_module = b.createModule(.{
        .root_source_file = b.path("src/grpc_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const grpc_tests = b.addTest(.{
        .root_module = grpc_module,
    });
    const grpc_run = b.addRunArtifact(grpc_tests);
    test_step.dependOn(&grpc_run.step);

    const headers_module = b.createModule(.{
        .root_source_file = b.path("src/headers_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const headers_tests = b.addTest(.{
        .root_module = headers_module,
    });
    const headers_run = b.addRunArtifact(headers_tests);
    test_step.dependOn(&headers_run.step);

    const http_methods_module = b.createModule(.{
        .root_source_file = b.path("src/http_methods_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const http_methods_tests = b.addTest(.{
        .root_module = http_methods_module,
    });
    const http_methods_run = b.addRunArtifact(http_methods_tests);
    test_step.dependOn(&http_methods_run.step);

    const json_bodies_module = b.createModule(.{
        .root_source_file = b.path("src/json_bodies_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const json_bodies_tests = b.addTest(.{
        .root_module = json_bodies_module,
    });
    const json_bodies_run = b.addRunArtifact(json_bodies_tests);
    test_step.dependOn(&json_bodies_run.step);

    const jsonrpc_module = b.createModule(.{
        .root_source_file = b.path("src/jsonrpc_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const jsonrpc_tests = b.addTest(.{
        .root_module = jsonrpc_module,
    });
    const jsonrpc_run = b.addRunArtifact(jsonrpc_tests);
    test_step.dependOn(&jsonrpc_run.step);

    const lifecycle_hooks_module = b.createModule(.{
        .root_source_file = b.path("src/lifecycle_hooks_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const lifecycle_hooks_tests = b.addTest(.{
        .root_module = lifecycle_hooks_module,
    });
    const lifecycle_hooks_run = b.addRunArtifact(lifecycle_hooks_tests);
    test_step.dependOn(&lifecycle_hooks_run.step);

    const multipart_module = b.createModule(.{
        .root_source_file = b.path("src/multipart_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const multipart_tests = b.addTest(.{
        .root_module = multipart_module,
    });
    const multipart_run = b.addRunArtifact(multipart_tests);
    test_step.dependOn(&multipart_run.step);

    const openapi_module = b.createModule(.{
        .root_source_file = b.path("src/openapi_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const openapi_tests = b.addTest(.{
        .root_module = openapi_module,
    });
    const openapi_run = b.addRunArtifact(openapi_tests);
    test_step.dependOn(&openapi_run.step);

    const openrpc_module = b.createModule(.{
        .root_source_file = b.path("src/openrpc_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const openrpc_tests = b.addTest(.{
        .root_module = openrpc_module,
    });
    const openrpc_run = b.addRunArtifact(openrpc_tests);
    test_step.dependOn(&openrpc_run.step);

    const path_params_module = b.createModule(.{
        .root_source_file = b.path("src/path_params_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const path_params_tests = b.addTest(.{
        .root_module = path_params_module,
    });
    const path_params_run = b.addRunArtifact(path_params_tests);
    test_step.dependOn(&path_params_run.step);

    const problem_details_module = b.createModule(.{
        .root_source_file = b.path("src/problem_details_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const problem_details_tests = b.addTest(.{
        .root_module = problem_details_module,
    });
    const problem_details_run = b.addRunArtifact(problem_details_tests);
    test_step.dependOn(&problem_details_run.step);

    const query_params_module = b.createModule(.{
        .root_source_file = b.path("src/query_params_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const query_params_tests = b.addTest(.{
        .root_module = query_params_module,
    });
    const query_params_run = b.addRunArtifact(query_params_tests);
    test_step.dependOn(&query_params_run.step);

    const rate_limit_module = b.createModule(.{
        .root_source_file = b.path("src/rate_limit_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const rate_limit_tests = b.addTest(.{
        .root_module = rate_limit_module,
    });
    const rate_limit_run = b.addRunArtifact(rate_limit_tests);
    test_step.dependOn(&rate_limit_run.step);

    const request_id_module = b.createModule(.{
        .root_source_file = b.path("src/request_id_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const request_id_tests = b.addTest(.{
        .root_module = request_id_module,
    });
    const request_id_run = b.addRunArtifact(request_id_tests);
    test_step.dependOn(&request_id_run.step);

    const request_timeout_module = b.createModule(.{
        .root_source_file = b.path("src/request_timeout_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const request_timeout_tests = b.addTest(.{
        .root_module = request_timeout_module,
    });
    const request_timeout_run = b.addRunArtifact(request_timeout_tests);
    test_step.dependOn(&request_timeout_run.step);

    const response_module = b.createModule(.{
        .root_source_file = b.path("src/response_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const response_tests = b.addTest(.{
        .root_module = response_module,
    });
    const response_run = b.addRunArtifact(response_tests);
    test_step.dependOn(&response_run.step);

    const server_config_module = b.createModule(.{
        .root_source_file = b.path("src/server_config_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const server_config_tests = b.addTest(.{
        .root_module = server_config_module,
    });
    const server_config_run = b.addRunArtifact(server_config_tests);
    test_step.dependOn(&server_config_run.step);

    const sse_module = b.createModule(.{
        .root_source_file = b.path("src/sse_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const sse_tests = b.addTest(.{
        .root_module = sse_module,
    });
    const sse_run = b.addRunArtifact(sse_tests);
    test_step.dependOn(&sse_run.step);

    const static_files_module = b.createModule(.{
        .root_source_file = b.path("src/static_files_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const static_files_tests = b.addTest(.{
        .root_module = static_files_module,
    });
    const static_files_run = b.addRunArtifact(static_files_tests);
    test_step.dependOn(&static_files_run.step);

    const status_codes_module = b.createModule(.{
        .root_source_file = b.path("src/status_codes_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const status_codes_tests = b.addTest(.{
        .root_module = status_codes_module,
    });
    const status_codes_run = b.addRunArtifact(status_codes_tests);
    test_step.dependOn(&status_codes_run.step);

    const streaming_module = b.createModule(.{
        .root_source_file = b.path("src/streaming_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const streaming_tests = b.addTest(.{
        .root_module = streaming_module,
    });
    const streaming_run = b.addRunArtifact(streaming_tests);
    test_step.dependOn(&streaming_run.step);

    const upload_module = b.createModule(.{
        .root_source_file = b.path("src/upload_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const upload_tests = b.addTest(.{
        .root_module = upload_module,
    });
    const upload_run = b.addRunArtifact(upload_tests);
    test_step.dependOn(&upload_run.step);

    const url_encoded_module = b.createModule(.{
        .root_source_file = b.path("src/url_encoded_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const url_encoded_tests = b.addTest(.{
        .root_module = url_encoded_module,
    });
    const url_encoded_run = b.addRunArtifact(url_encoded_tests);
    test_step.dependOn(&url_encoded_run.step);

    const validation_errors_module = b.createModule(.{
        .root_source_file = b.path("src/validation_errors_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const validation_errors_tests = b.addTest(.{
        .root_module = validation_errors_module,
    });
    const validation_errors_run = b.addRunArtifact(validation_errors_tests);
    test_step.dependOn(&validation_errors_run.step);

    const websocket_module = b.createModule(.{
        .root_source_file = b.path("src/websocket_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const websocket_tests = b.addTest(.{
        .root_module = websocket_module,
    });
    const websocket_run = b.addRunArtifact(websocket_tests);
    test_step.dependOn(&websocket_run.step);

}
