// Placeholder Swift source for the RustBridge target.
// Run `cargo build -p spikard-swift` and copy the generated Swift files here
// (with `import RustBridgeC` prepended). See BUILDING.md for instructions.
//
// This file intentionally stubs all types referenced by the generated
// Spikard.swift so that `swift test` compiles before the actual swift-bridge
// crate is built.  Replace with real generated code once spikard-swift exists.

// MARK: - Stub types referenced by Spikard.swift

public class UploadFile {}
public class CorsConfig {}
public class CompressionConfig {}
public class RateLimitConfig {}
public class JsonRpcMethodInfo {}
public class ProblemDetails {}
public class GraphQLRouteConfig {}
public class SchemaConfig {}
public class QueryOnlyConfig {}
public class QueryMutationConfig {}
public class FullSchemaConfig {}
public class BackgroundTaskConfig {}
public class BackgroundJobMetadata {}
public class GrpcConfig {}
public class JsonRpcConfig {}
public class OpenApiConfig {}
public class ContactInfo {}
public class LicenseInfo {}
public class ServerInfo {}
public class Response {}
public class SseEvent {}
public class JwtConfig {}
public class ApiKeyConfig {}
public class StaticFilesConfig {}
public class ServerConfig {}
public enum Method { case get, post, put, patch, delete, head, options, trace }
public enum SecuritySchemeInfo { case http, apiKey }

// MARK: - Stub handle_request used by generated e2e tests

/// Stub HTTP request dispatcher. Returns immediately with an empty response.
/// Replace with a real FFI call once the swift-bridge layer is implemented.
public func handleRequest() async throws -> Response {
  return Response()
}
