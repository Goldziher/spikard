// swift-format-ignore-file
import RustBridgeC

public func graphQlRouteConfigPath<GenericIntoRustString: IntoRustString>(_ client: GraphQLRouteConfigRef, _ path: GenericIntoRustString) -> GraphQLRouteConfig {
  GraphQLRouteConfig(ptr: __swift_bridge__$graph_ql_route_config_path(client.ptr, { let rustString = path.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func graphQlRouteConfigMethod<GenericIntoRustString: IntoRustString>(_ client: GraphQLRouteConfigRef, _ method: GenericIntoRustString) -> GraphQLRouteConfig {
  GraphQLRouteConfig(ptr: __swift_bridge__$graph_ql_route_config_method(client.ptr, { let rustString = method.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func graphQlRouteConfigEnablePlayground(_ client: GraphQLRouteConfigRef, _ enable: Bool) -> GraphQLRouteConfig {
  GraphQLRouteConfig(ptr: __swift_bridge__$graph_ql_route_config_enable_playground(client.ptr, enable))
}
public func graphQlRouteConfigDescription<GenericIntoRustString: IntoRustString>(_ client: GraphQLRouteConfigRef, _ description: GenericIntoRustString) -> GraphQLRouteConfig {
  GraphQLRouteConfig(ptr: __swift_bridge__$graph_ql_route_config_description(client.ptr, { let rustString = description.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func graphQlRouteConfigGetPath(_ client: GraphQLRouteConfigRef) -> RustString {
  RustString(ptr: __swift_bridge__$graph_ql_route_config_get_path(client.ptr))
}
public func graphQlRouteConfigGetMethod(_ client: GraphQLRouteConfigRef) -> RustString {
  RustString(ptr: __swift_bridge__$graph_ql_route_config_get_method(client.ptr))
}
public func graphQlRouteConfigIsPlaygroundEnabled(_ client: GraphQLRouteConfigRef) -> Bool {
  __swift_bridge__$graph_ql_route_config_is_playground_enabled(client.ptr)
}
public func graphQlRouteConfigGetDescription(_ client: GraphQLRouteConfigRef) -> RustString {
  RustString(ptr: __swift_bridge__$graph_ql_route_config_get_description(client.ptr))
}
public func routeBuilderHandlerName<GenericIntoRustString: IntoRustString>(_ client: RouteBuilderRef, _ name: GenericIntoRustString) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_handler_name(client.ptr, { let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func routeBuilderRequestSchemaJson<GenericIntoRustString: IntoRustString>(_ client: RouteBuilderRef, _ schema: GenericIntoRustString) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_request_schema_json(client.ptr, { let rustString = schema.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func routeBuilderResponseSchemaJson<GenericIntoRustString: IntoRustString>(_ client: RouteBuilderRef, _ schema: GenericIntoRustString) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_response_schema_json(client.ptr, { let rustString = schema.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func routeBuilderParamsSchemaJson<GenericIntoRustString: IntoRustString>(_ client: RouteBuilderRef, _ schema: GenericIntoRustString) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_params_schema_json(client.ptr, { let rustString = schema.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func routeBuilderFileParamsJson<GenericIntoRustString: IntoRustString>(_ client: RouteBuilderRef, _ schema: GenericIntoRustString) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_file_params_json(client.ptr, { let rustString = schema.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func routeBuilderCors(_ client: RouteBuilderRef, _ cors: CorsConfig) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_cors(client.ptr, {cors.isOwned = false; return cors.ptr;}()))
}
public func routeBuilderCompression(_ client: RouteBuilderRef, _ compression: CompressionConfig) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_compression(client.ptr, {compression.isOwned = false; return compression.ptr;}()))
}
public func routeBuilderSync(_ client: RouteBuilderRef) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_sync(client.ptr))
}
public func routeBuilderHandlerDependencies<GenericIntoRustString: IntoRustString>(_ client: RouteBuilderRef, _ dependencies: RustVec<GenericIntoRustString>) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_handler_dependencies(client.ptr, { let val = dependencies; val.isOwned = false; return val.ptr }()))
}
public func request_noop(_ client: RequestRef) {
  __swift_bridge__$request_noop(client.ptr)
}
public func request_data_noop(_ client: RequestDataRef) {
  __swift_bridge__$request_data_noop(client.ptr)
}
public func handler_result_noop(_ client: HandlerResultRef) {
  __swift_bridge__$handler_result_noop(client.ptr)
}
public func schemaQueryOnly() -> QueryOnlyConfig {
  QueryOnlyConfig(ptr: __swift_bridge__$schema_query_only())
}
public func schemaQueryMutation() -> QueryMutationConfig {
  QueryMutationConfig(ptr: __swift_bridge__$schema_query_mutation())
}
public func schemaFull() -> FullSchemaConfig {
  FullSchemaConfig(ptr: __swift_bridge__$schema_full())
}
public func appRawPtr(_ client: AppRefMut) -> UInt {
  __swift_bridge__$app_raw_ptr(client.ptr)
}
public func config(_ client: AppRefMut) {
  __swift_bridge__$config(client.ptr)
}
public func run(_ client: AppRefMut) -> RustString {
  RustString(ptr: __swift_bridge__$run(client.ptr))
}
public func routeBuilderNew<GenericIntoRustString: IntoRustString>(_ method: MethodRef, _ path: GenericIntoRustString) -> RouteBuilder {
  RouteBuilder(ptr: __swift_bridge__$route_builder_new(method.ptr, { let rustString = path.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func corsConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CorsConfig {
  try { let val = __swift_bridge__$cors_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CorsConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func compressionConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> CompressionConfig {
  try { let val = __swift_bridge__$compression_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return CompressionConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func uploadFileFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> UploadFile {
  try { let val = __swift_bridge__$upload_file_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return UploadFile(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func schemaConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SchemaConfig {
  try { let val = __swift_bridge__$schema_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SchemaConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func queryOnlyConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> QueryOnlyConfig {
  try { let val = __swift_bridge__$query_only_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return QueryOnlyConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func queryMutationConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> QueryMutationConfig {
  try { let val = __swift_bridge__$query_mutation_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return QueryMutationConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func fullSchemaConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> FullSchemaConfig {
  try { let val = __swift_bridge__$full_schema_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return FullSchemaConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func backgroundTaskConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> BackgroundTaskConfig {
  try { let val = __swift_bridge__$background_task_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BackgroundTaskConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func backgroundJobMetadataFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> BackgroundJobMetadata {
  try { let val = __swift_bridge__$background_job_metadata_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return BackgroundJobMetadata(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func rateLimitConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> RateLimitConfig {
  try { let val = __swift_bridge__$rate_limit_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return RateLimitConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func grpcConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> GrpcConfig {
  try { let val = __swift_bridge__$grpc_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return GrpcConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func jsonRpcConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> JsonRpcConfig {
  try { let val = __swift_bridge__$json_rpc_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return JsonRpcConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func openApiConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> OpenApiConfig {
  try { let val = __swift_bridge__$open_api_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return OpenApiConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func responseFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Response {
  try { let val = __swift_bridge__$response_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Response(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func sseEventFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SseEvent {
  try { let val = __swift_bridge__$sse_event_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SseEvent(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func jwtConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> JwtConfig {
  try { let val = __swift_bridge__$jwt_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return JwtConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func apiKeyConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ApiKeyConfig {
  try { let val = __swift_bridge__$api_key_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ApiKeyConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func staticFilesConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> StaticFilesConfig {
  try { let val = __swift_bridge__$static_files_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return StaticFilesConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func serverConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ServerConfig {
  try { let val = __swift_bridge__$server_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ServerConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func jsonRpcMethodInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> JsonRpcMethodInfo {
  try { let val = __swift_bridge__$json_rpc_method_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return JsonRpcMethodInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func problemDetailsFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ProblemDetails {
  try { let val = __swift_bridge__$problem_details_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ProblemDetails(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func asyncApiConfigFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> AsyncApiConfig {
  try { let val = __swift_bridge__$async_api_config_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return AsyncApiConfig(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func parsedChannelFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ParsedChannel {
  try { let val = __swift_bridge__$parsed_channel_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ParsedChannel(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func parsedOperationFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ParsedOperation {
  try { let val = __swift_bridge__$parsed_operation_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ParsedOperation(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func parsedMessageFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ParsedMessage {
  try { let val = __swift_bridge__$parsed_message_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ParsedMessage(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func parseResultFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ParseResult {
  try { let val = __swift_bridge__$parse_result_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ParseResult(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func contactInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ContactInfo {
  try { let val = __swift_bridge__$contact_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ContactInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func licenseInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> LicenseInfo {
  try { let val = __swift_bridge__$license_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return LicenseInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func serverInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> ServerInfo {
  try { let val = __swift_bridge__$server_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return ServerInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func methodFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> Method {
  try { let val = __swift_bridge__$method_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return Method(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func securitySchemeInfoFromJson<GenericIntoRustString: IntoRustString>(_ json: GenericIntoRustString) throws -> SecuritySchemeInfo {
  try { let val = __swift_bridge__$security_scheme_info_from_json({ let rustString = json.intoRustString(); rustString.isOwned = false; return rustString.ptr }()); if val.is_ok { return SecuritySchemeInfo(ptr: val.ok_or_err!) } else { throw RustString(ptr: val.ok_or_err!) } }()
}
public func __alef_phantom_vec_upload_file() -> RustVec<UploadFile> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_upload_file())
}
public func __alef_phantom_vec_graph_ql_route_config() -> RustVec<GraphQLRouteConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_graph_ql_route_config())
}
public func __alef_phantom_vec_schema_config() -> RustVec<SchemaConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_schema_config())
}
public func __alef_phantom_vec_query_only_config() -> RustVec<QueryOnlyConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_query_only_config())
}
public func __alef_phantom_vec_query_mutation_config() -> RustVec<QueryMutationConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_query_mutation_config())
}
public func __alef_phantom_vec_full_schema_config() -> RustVec<FullSchemaConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_full_schema_config())
}
public func __alef_phantom_vec_background_task_config() -> RustVec<BackgroundTaskConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_background_task_config())
}
public func __alef_phantom_vec_background_job_metadata() -> RustVec<BackgroundJobMetadata> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_background_job_metadata())
}
public func __alef_phantom_vec_cors_config() -> RustVec<CorsConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_cors_config())
}
public func __alef_phantom_vec_compression_config() -> RustVec<CompressionConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_compression_config())
}
public func __alef_phantom_vec_rate_limit_config() -> RustVec<RateLimitConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_rate_limit_config())
}
public func __alef_phantom_vec_grpc_config() -> RustVec<GrpcConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_grpc_config())
}
public func __alef_phantom_vec_json_rpc_config() -> RustVec<JsonRpcConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_json_rpc_config())
}
public func __alef_phantom_vec_open_api_config() -> RustVec<OpenApiConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_open_api_config())
}
public func __alef_phantom_vec_response() -> RustVec<Response> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_response())
}
public func __alef_phantom_vec_sse_event() -> RustVec<SseEvent> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_sse_event())
}
public func __alef_phantom_vec_jwt_config() -> RustVec<JwtConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_jwt_config())
}
public func __alef_phantom_vec_api_key_config() -> RustVec<ApiKeyConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_api_key_config())
}
public func __alef_phantom_vec_static_files_config() -> RustVec<StaticFilesConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_static_files_config())
}
public func __alef_phantom_vec_server_config() -> RustVec<ServerConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_server_config())
}
public func __alef_phantom_vec_route_builder() -> RustVec<RouteBuilder> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_route_builder())
}
public func __alef_phantom_vec_json_rpc_method_info() -> RustVec<JsonRpcMethodInfo> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_json_rpc_method_info())
}
public func __alef_phantom_vec_problem_details() -> RustVec<ProblemDetails> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_problem_details())
}
public func __alef_phantom_vec_async_api_config() -> RustVec<AsyncApiConfig> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_async_api_config())
}
public func __alef_phantom_vec_parsed_channel() -> RustVec<ParsedChannel> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_parsed_channel())
}
public func __alef_phantom_vec_parsed_operation() -> RustVec<ParsedOperation> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_parsed_operation())
}
public func __alef_phantom_vec_parsed_message() -> RustVec<ParsedMessage> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_parsed_message())
}
public func __alef_phantom_vec_parse_result() -> RustVec<ParseResult> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_parse_result())
}
public func __alef_phantom_vec_parse_request() -> RustVec<ParseRequest> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_parse_request())
}
public func __alef_phantom_vec_validation_response() -> RustVec<ValidationResponse> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_validation_response())
}
public func __alef_phantom_vec_validate_request() -> RustVec<ValidateRequest> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_validate_request())
}
public func __alef_phantom_vec_contact_info() -> RustVec<ContactInfo> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_contact_info())
}
public func __alef_phantom_vec_license_info() -> RustVec<LicenseInfo> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_license_info())
}
public func __alef_phantom_vec_server_info() -> RustVec<ServerInfo> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_server_info())
}
public func __alef_phantom_vec_handler_result() -> RustVec<HandlerResult> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_handler_result())
}
public func __alef_phantom_vec_request() -> RustVec<Request> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_request())
}
public func __alef_phantom_vec_request_data() -> RustVec<RequestData> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_request_data())
}
public func __alef_phantom_vec_method() -> RustVec<Method> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_method())
}
public func __alef_phantom_vec_security_scheme_info() -> RustVec<SecuritySchemeInfo> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_security_scheme_info())
}
public func __alef_phantom_vec_testing_sse_event() -> RustVec<TestingSseEvent> {
  RustVec(ptr: __swift_bridge__$__alef_phantom_vec_testing_sse_event())
}

public class UploadFile: UploadFileRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$UploadFile$_free(ptr)
    }
  }
}
public class UploadFileRefMut: UploadFileRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class UploadFileRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension UploadFileRef {
  public func filename() -> RustString {
    RustString(ptr: __swift_bridge__$UploadFile$filename(ptr))
  }

  public func contentType() -> Optional<RustString> {
    { let val = __swift_bridge__$UploadFile$content_type(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func size() -> Optional<UInt> {
    __swift_bridge__$UploadFile$size(ptr).intoSwiftRepr()
  }

  public func content() -> RustVec<UInt8> {
    RustVec(ptr: __swift_bridge__$UploadFile$content(ptr))
  }

  public func contentEncoding() -> Optional<RustString> {
    { let val = __swift_bridge__$UploadFile$content_encoding(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }
}
extension UploadFile: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_UploadFile$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_UploadFile$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: UploadFile) {
    __swift_bridge__$Vec_UploadFile$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_UploadFile$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (UploadFile(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<UploadFileRef> {
    let pointer = __swift_bridge__$Vec_UploadFile$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return UploadFileRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<UploadFileRefMut> {
    let pointer = __swift_bridge__$Vec_UploadFile$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return UploadFileRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<UploadFileRef> {
    UnsafePointer<UploadFileRef>(OpaquePointer(__swift_bridge__$Vec_UploadFile$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_UploadFile$len(vecPtr)
  }
}


public class GraphQLRouteConfig: GraphQLRouteConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$GraphQLRouteConfig$_free(ptr)
    }
  }
}
public class GraphQLRouteConfigRefMut: GraphQLRouteConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class GraphQLRouteConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension GraphQLRouteConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_GraphQLRouteConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_GraphQLRouteConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: GraphQLRouteConfig) {
    __swift_bridge__$Vec_GraphQLRouteConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_GraphQLRouteConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (GraphQLRouteConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<GraphQLRouteConfigRef> {
    let pointer = __swift_bridge__$Vec_GraphQLRouteConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return GraphQLRouteConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<GraphQLRouteConfigRefMut> {
    let pointer = __swift_bridge__$Vec_GraphQLRouteConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return GraphQLRouteConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<GraphQLRouteConfigRef> {
    UnsafePointer<GraphQLRouteConfigRef>(OpaquePointer(__swift_bridge__$Vec_GraphQLRouteConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_GraphQLRouteConfig$len(vecPtr)
  }
}


public class SchemaConfig: SchemaConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$SchemaConfig$_free(ptr)
    }
  }
}
extension SchemaConfig {
  public convenience init(_ introspection_enabled: Bool, _ complexity_limit: Optional<UInt>, _ depth_limit: Optional<UInt>) {
    self.init(ptr: __swift_bridge__$SchemaConfig$new(introspection_enabled, complexity_limit.intoFfiRepr(), depth_limit.intoFfiRepr()))
  }
}
public class SchemaConfigRefMut: SchemaConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class SchemaConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension SchemaConfigRef {
  public func introspectionEnabled() -> Bool {
    __swift_bridge__$SchemaConfig$introspection_enabled(ptr)
  }

  public func complexityLimit() -> Optional<UInt> {
    __swift_bridge__$SchemaConfig$complexity_limit(ptr).intoSwiftRepr()
  }

  public func depthLimit() -> Optional<UInt> {
    __swift_bridge__$SchemaConfig$depth_limit(ptr).intoSwiftRepr()
  }
}
extension SchemaConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_SchemaConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_SchemaConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SchemaConfig) {
    __swift_bridge__$Vec_SchemaConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_SchemaConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (SchemaConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SchemaConfigRef> {
    let pointer = __swift_bridge__$Vec_SchemaConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return SchemaConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SchemaConfigRefMut> {
    let pointer = __swift_bridge__$Vec_SchemaConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return SchemaConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SchemaConfigRef> {
    UnsafePointer<SchemaConfigRef>(OpaquePointer(__swift_bridge__$Vec_SchemaConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_SchemaConfig$len(vecPtr)
  }
}


public class QueryOnlyConfig: QueryOnlyConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$QueryOnlyConfig$_free(ptr)
    }
  }
}
extension QueryOnlyConfig {
  public convenience init(_ introspection_enabled: Bool, _ complexity_limit: Optional<UInt>, _ depth_limit: Optional<UInt>) {
    self.init(ptr: __swift_bridge__$QueryOnlyConfig$new(introspection_enabled, complexity_limit.intoFfiRepr(), depth_limit.intoFfiRepr()))
  }
}
public class QueryOnlyConfigRefMut: QueryOnlyConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class QueryOnlyConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension QueryOnlyConfigRef {
  public func introspectionEnabled() -> Bool {
    __swift_bridge__$QueryOnlyConfig$introspection_enabled(ptr)
  }

  public func complexityLimit() -> Optional<UInt> {
    __swift_bridge__$QueryOnlyConfig$complexity_limit(ptr).intoSwiftRepr()
  }

  public func depthLimit() -> Optional<UInt> {
    __swift_bridge__$QueryOnlyConfig$depth_limit(ptr).intoSwiftRepr()
  }
}
extension QueryOnlyConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_QueryOnlyConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_QueryOnlyConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: QueryOnlyConfig) {
    __swift_bridge__$Vec_QueryOnlyConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_QueryOnlyConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (QueryOnlyConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<QueryOnlyConfigRef> {
    let pointer = __swift_bridge__$Vec_QueryOnlyConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return QueryOnlyConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<QueryOnlyConfigRefMut> {
    let pointer = __swift_bridge__$Vec_QueryOnlyConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return QueryOnlyConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<QueryOnlyConfigRef> {
    UnsafePointer<QueryOnlyConfigRef>(OpaquePointer(__swift_bridge__$Vec_QueryOnlyConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_QueryOnlyConfig$len(vecPtr)
  }
}


public class QueryMutationConfig: QueryMutationConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$QueryMutationConfig$_free(ptr)
    }
  }
}
extension QueryMutationConfig {
  public convenience init(_ introspection_enabled: Bool, _ complexity_limit: Optional<UInt>, _ depth_limit: Optional<UInt>) {
    self.init(ptr: __swift_bridge__$QueryMutationConfig$new(introspection_enabled, complexity_limit.intoFfiRepr(), depth_limit.intoFfiRepr()))
  }
}
public class QueryMutationConfigRefMut: QueryMutationConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class QueryMutationConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension QueryMutationConfigRef {
  public func introspectionEnabled() -> Bool {
    __swift_bridge__$QueryMutationConfig$introspection_enabled(ptr)
  }

  public func complexityLimit() -> Optional<UInt> {
    __swift_bridge__$QueryMutationConfig$complexity_limit(ptr).intoSwiftRepr()
  }

  public func depthLimit() -> Optional<UInt> {
    __swift_bridge__$QueryMutationConfig$depth_limit(ptr).intoSwiftRepr()
  }
}
extension QueryMutationConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_QueryMutationConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_QueryMutationConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: QueryMutationConfig) {
    __swift_bridge__$Vec_QueryMutationConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_QueryMutationConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (QueryMutationConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<QueryMutationConfigRef> {
    let pointer = __swift_bridge__$Vec_QueryMutationConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return QueryMutationConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<QueryMutationConfigRefMut> {
    let pointer = __swift_bridge__$Vec_QueryMutationConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return QueryMutationConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<QueryMutationConfigRef> {
    UnsafePointer<QueryMutationConfigRef>(OpaquePointer(__swift_bridge__$Vec_QueryMutationConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_QueryMutationConfig$len(vecPtr)
  }
}


public class FullSchemaConfig: FullSchemaConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$FullSchemaConfig$_free(ptr)
    }
  }
}
extension FullSchemaConfig {
  public convenience init(_ introspection_enabled: Bool, _ complexity_limit: Optional<UInt>, _ depth_limit: Optional<UInt>) {
    self.init(ptr: __swift_bridge__$FullSchemaConfig$new(introspection_enabled, complexity_limit.intoFfiRepr(), depth_limit.intoFfiRepr()))
  }
}
public class FullSchemaConfigRefMut: FullSchemaConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class FullSchemaConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension FullSchemaConfigRef {
  public func introspectionEnabled() -> Bool {
    __swift_bridge__$FullSchemaConfig$introspection_enabled(ptr)
  }

  public func complexityLimit() -> Optional<UInt> {
    __swift_bridge__$FullSchemaConfig$complexity_limit(ptr).intoSwiftRepr()
  }

  public func depthLimit() -> Optional<UInt> {
    __swift_bridge__$FullSchemaConfig$depth_limit(ptr).intoSwiftRepr()
  }
}
extension FullSchemaConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_FullSchemaConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_FullSchemaConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: FullSchemaConfig) {
    __swift_bridge__$Vec_FullSchemaConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_FullSchemaConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (FullSchemaConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FullSchemaConfigRef> {
    let pointer = __swift_bridge__$Vec_FullSchemaConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return FullSchemaConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<FullSchemaConfigRefMut> {
    let pointer = __swift_bridge__$Vec_FullSchemaConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return FullSchemaConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<FullSchemaConfigRef> {
    UnsafePointer<FullSchemaConfigRef>(OpaquePointer(__swift_bridge__$Vec_FullSchemaConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_FullSchemaConfig$len(vecPtr)
  }
}


public class BackgroundTaskConfig: BackgroundTaskConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$BackgroundTaskConfig$_free(ptr)
    }
  }
}
extension BackgroundTaskConfig {
  public convenience init(_ max_queue_size: UInt, _ max_concurrent_tasks: UInt, _ drain_timeout_secs: UInt64) {
    self.init(ptr: __swift_bridge__$BackgroundTaskConfig$new(max_queue_size, max_concurrent_tasks, drain_timeout_secs))
  }
}
public class BackgroundTaskConfigRefMut: BackgroundTaskConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class BackgroundTaskConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension BackgroundTaskConfigRef {
  public func maxQueueSize() -> UInt {
    __swift_bridge__$BackgroundTaskConfig$max_queue_size(ptr)
  }

  public func maxConcurrentTasks() -> UInt {
    __swift_bridge__$BackgroundTaskConfig$max_concurrent_tasks(ptr)
  }

  public func drainTimeoutSecs() -> UInt64 {
    __swift_bridge__$BackgroundTaskConfig$drain_timeout_secs(ptr)
  }
}
extension BackgroundTaskConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_BackgroundTaskConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_BackgroundTaskConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BackgroundTaskConfig) {
    __swift_bridge__$Vec_BackgroundTaskConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_BackgroundTaskConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (BackgroundTaskConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BackgroundTaskConfigRef> {
    let pointer = __swift_bridge__$Vec_BackgroundTaskConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BackgroundTaskConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BackgroundTaskConfigRefMut> {
    let pointer = __swift_bridge__$Vec_BackgroundTaskConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BackgroundTaskConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<BackgroundTaskConfigRef> {
    UnsafePointer<BackgroundTaskConfigRef>(OpaquePointer(__swift_bridge__$Vec_BackgroundTaskConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_BackgroundTaskConfig$len(vecPtr)
  }
}


public class BackgroundJobMetadata: BackgroundJobMetadataRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$BackgroundJobMetadata$_free(ptr)
    }
  }
}
extension BackgroundJobMetadata {
  public convenience init<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString, _ request_id: Optional<GenericIntoRustString>) {
    self.init(ptr: __swift_bridge__$BackgroundJobMetadata$new({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(request_id) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
  }
}
public class BackgroundJobMetadataRefMut: BackgroundJobMetadataRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class BackgroundJobMetadataRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension BackgroundJobMetadataRef {
  public func name() -> RustString {
    RustString(ptr: __swift_bridge__$BackgroundJobMetadata$name(ptr))
  }

  public func requestId() -> Optional<RustString> {
    { let val = __swift_bridge__$BackgroundJobMetadata$request_id(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }
}
extension BackgroundJobMetadata: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_BackgroundJobMetadata$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_BackgroundJobMetadata$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: BackgroundJobMetadata) {
    __swift_bridge__$Vec_BackgroundJobMetadata$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_BackgroundJobMetadata$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (BackgroundJobMetadata(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BackgroundJobMetadataRef> {
    let pointer = __swift_bridge__$Vec_BackgroundJobMetadata$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BackgroundJobMetadataRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<BackgroundJobMetadataRefMut> {
    let pointer = __swift_bridge__$Vec_BackgroundJobMetadata$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return BackgroundJobMetadataRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<BackgroundJobMetadataRef> {
    UnsafePointer<BackgroundJobMetadataRef>(OpaquePointer(__swift_bridge__$Vec_BackgroundJobMetadata$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_BackgroundJobMetadata$len(vecPtr)
  }
}


public class CorsConfig: CorsConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$CorsConfig$_free(ptr)
    }
  }
}
extension CorsConfig {
  public convenience init<GenericIntoRustString: IntoRustString>(_ allowed_origins: RustVec<GenericIntoRustString>, _ allowed_methods: RustVec<GenericIntoRustString>, _ allowed_headers: RustVec<GenericIntoRustString>, _ expose_headers: Optional<RustVec<GenericIntoRustString>>, _ max_age: Optional<UInt32>, _ allow_credentials: Optional<Bool>) {
    self.init(ptr: __swift_bridge__$CorsConfig$new({ let val = allowed_origins; val.isOwned = false; return val.ptr }(), { let val = allowed_methods; val.isOwned = false; return val.ptr }(), { let val = allowed_headers; val.isOwned = false; return val.ptr }(), { if let val = expose_headers { val.isOwned = false; return val.ptr } else { return nil } }(), max_age.intoFfiRepr(), allow_credentials.intoFfiRepr()))
  }
}
public class CorsConfigRefMut: CorsConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class CorsConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension CorsConfigRef {
  public func allowedOrigins() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$CorsConfig$allowed_origins(ptr))
  }

  public func allowedMethods() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$CorsConfig$allowed_methods(ptr))
  }

  public func allowedHeaders() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$CorsConfig$allowed_headers(ptr))
  }

  public func exposeHeaders() -> Optional<RustVec<RustString>> {
    { let val = __swift_bridge__$CorsConfig$expose_headers(ptr); if val != nil { return RustVec(ptr: val!) } else { return nil } }()
  }

  public func maxAge() -> Optional<UInt32> {
    __swift_bridge__$CorsConfig$max_age(ptr).intoSwiftRepr()
  }

  public func allowCredentials() -> Optional<Bool> {
    __swift_bridge__$CorsConfig$allow_credentials(ptr).intoSwiftRepr()
  }
}
extension CorsConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_CorsConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_CorsConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CorsConfig) {
    __swift_bridge__$Vec_CorsConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_CorsConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (CorsConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CorsConfigRef> {
    let pointer = __swift_bridge__$Vec_CorsConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CorsConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CorsConfigRefMut> {
    let pointer = __swift_bridge__$Vec_CorsConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CorsConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CorsConfigRef> {
    UnsafePointer<CorsConfigRef>(OpaquePointer(__swift_bridge__$Vec_CorsConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_CorsConfig$len(vecPtr)
  }
}


public class CompressionConfig: CompressionConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$CompressionConfig$_free(ptr)
    }
  }
}
extension CompressionConfig {
  public convenience init(_ gzip: Bool, _ brotli: Bool, _ min_size: UInt, _ quality: UInt32) {
    self.init(ptr: __swift_bridge__$CompressionConfig$new(gzip, brotli, min_size, quality))
  }
}
public class CompressionConfigRefMut: CompressionConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class CompressionConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension CompressionConfigRef {
  public func gzip() -> Bool {
    __swift_bridge__$CompressionConfig$gzip(ptr)
  }

  public func brotli() -> Bool {
    __swift_bridge__$CompressionConfig$brotli(ptr)
  }

  public func minSize() -> UInt {
    __swift_bridge__$CompressionConfig$min_size(ptr)
  }

  public func quality() -> UInt32 {
    __swift_bridge__$CompressionConfig$quality(ptr)
  }
}
extension CompressionConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_CompressionConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_CompressionConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: CompressionConfig) {
    __swift_bridge__$Vec_CompressionConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_CompressionConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (CompressionConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CompressionConfigRef> {
    let pointer = __swift_bridge__$Vec_CompressionConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CompressionConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<CompressionConfigRefMut> {
    let pointer = __swift_bridge__$Vec_CompressionConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return CompressionConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<CompressionConfigRef> {
    UnsafePointer<CompressionConfigRef>(OpaquePointer(__swift_bridge__$Vec_CompressionConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_CompressionConfig$len(vecPtr)
  }
}


public class RateLimitConfig: RateLimitConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$RateLimitConfig$_free(ptr)
    }
  }
}
extension RateLimitConfig {
  public convenience init(_ per_second: UInt64, _ burst: UInt32, _ ip_based: Bool) {
    self.init(ptr: __swift_bridge__$RateLimitConfig$new(per_second, burst, ip_based))
  }
}
public class RateLimitConfigRefMut: RateLimitConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class RateLimitConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension RateLimitConfigRef {
  public func perSecond() -> UInt64 {
    __swift_bridge__$RateLimitConfig$per_second(ptr)
  }

  public func burst() -> UInt32 {
    __swift_bridge__$RateLimitConfig$burst(ptr)
  }

  public func ipBased() -> Bool {
    __swift_bridge__$RateLimitConfig$ip_based(ptr)
  }
}
extension RateLimitConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_RateLimitConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_RateLimitConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RateLimitConfig) {
    __swift_bridge__$Vec_RateLimitConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_RateLimitConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (RateLimitConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RateLimitConfigRef> {
    let pointer = __swift_bridge__$Vec_RateLimitConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return RateLimitConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RateLimitConfigRefMut> {
    let pointer = __swift_bridge__$Vec_RateLimitConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return RateLimitConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RateLimitConfigRef> {
    UnsafePointer<RateLimitConfigRef>(OpaquePointer(__swift_bridge__$Vec_RateLimitConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_RateLimitConfig$len(vecPtr)
  }
}


public class GrpcConfig: GrpcConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$GrpcConfig$_free(ptr)
    }
  }
}
extension GrpcConfig {
  public convenience init(_ enabled: Bool, _ max_message_size: UInt, _ enable_compression: Bool, _ request_timeout: Optional<UInt64>, _ max_concurrent_streams: UInt32, _ enable_keepalive: Bool, _ keepalive_interval: UInt64, _ keepalive_timeout: UInt64, _ max_stream_response_bytes: Optional<UInt>) {
    self.init(ptr: __swift_bridge__$GrpcConfig$new(enabled, max_message_size, enable_compression, request_timeout.intoFfiRepr(), max_concurrent_streams, enable_keepalive, keepalive_interval, keepalive_timeout, max_stream_response_bytes.intoFfiRepr()))
  }
}
public class GrpcConfigRefMut: GrpcConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class GrpcConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension GrpcConfigRef {
  public func enabled() -> Bool {
    __swift_bridge__$GrpcConfig$enabled(ptr)
  }

  public func maxMessageSize() -> UInt {
    __swift_bridge__$GrpcConfig$max_message_size(ptr)
  }

  public func enableCompression() -> Bool {
    __swift_bridge__$GrpcConfig$enable_compression(ptr)
  }

  public func requestTimeout() -> Optional<UInt64> {
    __swift_bridge__$GrpcConfig$request_timeout(ptr).intoSwiftRepr()
  }

  public func maxConcurrentStreams() -> UInt32 {
    __swift_bridge__$GrpcConfig$max_concurrent_streams(ptr)
  }

  public func enableKeepalive() -> Bool {
    __swift_bridge__$GrpcConfig$enable_keepalive(ptr)
  }

  public func keepaliveInterval() -> UInt64 {
    __swift_bridge__$GrpcConfig$keepalive_interval(ptr)
  }

  public func keepaliveTimeout() -> UInt64 {
    __swift_bridge__$GrpcConfig$keepalive_timeout(ptr)
  }

  public func maxStreamResponseBytes() -> Optional<UInt> {
    __swift_bridge__$GrpcConfig$max_stream_response_bytes(ptr).intoSwiftRepr()
  }
}
extension GrpcConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_GrpcConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_GrpcConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: GrpcConfig) {
    __swift_bridge__$Vec_GrpcConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_GrpcConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (GrpcConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<GrpcConfigRef> {
    let pointer = __swift_bridge__$Vec_GrpcConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return GrpcConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<GrpcConfigRefMut> {
    let pointer = __swift_bridge__$Vec_GrpcConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return GrpcConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<GrpcConfigRef> {
    UnsafePointer<GrpcConfigRef>(OpaquePointer(__swift_bridge__$Vec_GrpcConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_GrpcConfig$len(vecPtr)
  }
}


public class JsonRpcConfig: JsonRpcConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$JsonRpcConfig$_free(ptr)
    }
  }
}
extension JsonRpcConfig {
  public convenience init<GenericIntoRustString: IntoRustString>(_ enabled: Bool, _ endpoint_path: GenericIntoRustString, _ enable_batch: Bool, _ max_batch_size: UInt) {
    self.init(ptr: __swift_bridge__$JsonRpcConfig$new(enabled, { let rustString = endpoint_path.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), enable_batch, max_batch_size))
  }
}
public class JsonRpcConfigRefMut: JsonRpcConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class JsonRpcConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension JsonRpcConfigRef {
  public func enabled() -> Bool {
    __swift_bridge__$JsonRpcConfig$enabled(ptr)
  }

  public func endpointPath() -> RustString {
    RustString(ptr: __swift_bridge__$JsonRpcConfig$endpoint_path(ptr))
  }

  public func enableBatch() -> Bool {
    __swift_bridge__$JsonRpcConfig$enable_batch(ptr)
  }

  public func maxBatchSize() -> UInt {
    __swift_bridge__$JsonRpcConfig$max_batch_size(ptr)
  }
}
extension JsonRpcConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_JsonRpcConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_JsonRpcConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: JsonRpcConfig) {
    __swift_bridge__$Vec_JsonRpcConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_JsonRpcConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (JsonRpcConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<JsonRpcConfigRef> {
    let pointer = __swift_bridge__$Vec_JsonRpcConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return JsonRpcConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<JsonRpcConfigRefMut> {
    let pointer = __swift_bridge__$Vec_JsonRpcConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return JsonRpcConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<JsonRpcConfigRef> {
    UnsafePointer<JsonRpcConfigRef>(OpaquePointer(__swift_bridge__$Vec_JsonRpcConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_JsonRpcConfig$len(vecPtr)
  }
}


public class OpenApiConfig: OpenApiConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$OpenApiConfig$_free(ptr)
    }
  }
}
extension OpenApiConfig {
  public convenience init<GenericIntoRustString: IntoRustString>(_ enabled: Bool, _ title: GenericIntoRustString, _ version: GenericIntoRustString, _ description: Optional<GenericIntoRustString>, _ swagger_ui_path: GenericIntoRustString, _ redoc_path: GenericIntoRustString, _ openapi_json_path: GenericIntoRustString, _ contact: Optional<ContactInfo>, _ license: Optional<LicenseInfo>, _ servers: RustVec<ServerInfo>, _ security_schemes: GenericIntoRustString) {
    self.init(ptr: __swift_bridge__$OpenApiConfig$new(enabled, { let rustString = title.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = version.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let rustString = optionalStringIntoRustString(description) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), { let rustString = swagger_ui_path.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = redoc_path.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = openapi_json_path.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { if let val = contact { val.isOwned = false; return val.ptr } else { return nil } }(), { if let val = license { val.isOwned = false; return val.ptr } else { return nil } }(), { let val = servers; val.isOwned = false; return val.ptr }(), { let rustString = security_schemes.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
  }
}
public class OpenApiConfigRefMut: OpenApiConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class OpenApiConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension OpenApiConfigRef {
  public func enabled() -> Bool {
    __swift_bridge__$OpenApiConfig$enabled(ptr)
  }

  public func title() -> RustString {
    RustString(ptr: __swift_bridge__$OpenApiConfig$title(ptr))
  }

  public func version() -> RustString {
    RustString(ptr: __swift_bridge__$OpenApiConfig$version(ptr))
  }

  public func description() -> Optional<RustString> {
    { let val = __swift_bridge__$OpenApiConfig$description(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func swaggerUiPath() -> RustString {
    RustString(ptr: __swift_bridge__$OpenApiConfig$swagger_ui_path(ptr))
  }

  public func redocPath() -> RustString {
    RustString(ptr: __swift_bridge__$OpenApiConfig$redoc_path(ptr))
  }

  public func openapiJsonPath() -> RustString {
    RustString(ptr: __swift_bridge__$OpenApiConfig$openapi_json_path(ptr))
  }

  public func contact() -> Optional<ContactInfo> {
    { let val = __swift_bridge__$OpenApiConfig$contact(ptr); if val != nil { return ContactInfo(ptr: val!) } else { return nil } }()
  }

  public func license() -> Optional<LicenseInfo> {
    { let val = __swift_bridge__$OpenApiConfig$license(ptr); if val != nil { return LicenseInfo(ptr: val!) } else { return nil } }()
  }

  public func servers() -> RustVec<ServerInfo> {
    RustVec(ptr: __swift_bridge__$OpenApiConfig$servers(ptr))
  }

  public func securitySchemes() -> RustString {
    RustString(ptr: __swift_bridge__$OpenApiConfig$security_schemes(ptr))
  }
}
extension OpenApiConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_OpenApiConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_OpenApiConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: OpenApiConfig) {
    __swift_bridge__$Vec_OpenApiConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_OpenApiConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (OpenApiConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OpenApiConfigRef> {
    let pointer = __swift_bridge__$Vec_OpenApiConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return OpenApiConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<OpenApiConfigRefMut> {
    let pointer = __swift_bridge__$Vec_OpenApiConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return OpenApiConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<OpenApiConfigRef> {
    UnsafePointer<OpenApiConfigRef>(OpaquePointer(__swift_bridge__$Vec_OpenApiConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_OpenApiConfig$len(vecPtr)
  }
}


public class Response: ResponseRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$Response$_free(ptr)
    }
  }
}
extension Response {
  public convenience init<GenericIntoRustString: IntoRustString>(_ content: Optional<GenericIntoRustString>, _ status_code: UInt16, _ headers: GenericIntoRustString) {
    self.init(ptr: __swift_bridge__$Response$new({ if let rustString = optionalStringIntoRustString(content) { rustString.isOwned = false; return rustString.ptr } else { return nil } }(), status_code, { let rustString = headers.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
  }
}
public class ResponseRefMut: ResponseRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ResponseRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ResponseRef {
  public func content() -> Optional<RustString> {
    { let val = __swift_bridge__$Response$content(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func statusCode() -> UInt16 {
    __swift_bridge__$Response$status_code(ptr)
  }

  public func headers() -> RustString {
    RustString(ptr: __swift_bridge__$Response$headers(ptr))
  }
}
extension Response: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_Response$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_Response$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Response) {
    __swift_bridge__$Vec_Response$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_Response$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (Response(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseRef> {
    let pointer = __swift_bridge__$Vec_Response$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ResponseRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ResponseRefMut> {
    let pointer = __swift_bridge__$Vec_Response$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ResponseRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ResponseRef> {
    UnsafePointer<ResponseRef>(OpaquePointer(__swift_bridge__$Vec_Response$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_Response$len(vecPtr)
  }
}


public class SseEvent: SseEventRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$SseEvent$_free(ptr)
    }
  }
}
public class SseEventRefMut: SseEventRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class SseEventRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension SseEventRef {
  public func eventType() -> Optional<RustString> {
    { let val = __swift_bridge__$SseEvent$event_type(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func data() -> RustString {
    RustString(ptr: __swift_bridge__$SseEvent$data(ptr))
  }

  public func id() -> Optional<RustString> {
    { let val = __swift_bridge__$SseEvent$id(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func retry() -> Optional<UInt64> {
    __swift_bridge__$SseEvent$retry(ptr).intoSwiftRepr()
  }
}
extension SseEvent: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_SseEvent$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_SseEvent$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SseEvent) {
    __swift_bridge__$Vec_SseEvent$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_SseEvent$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (SseEvent(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SseEventRef> {
    let pointer = __swift_bridge__$Vec_SseEvent$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return SseEventRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SseEventRefMut> {
    let pointer = __swift_bridge__$Vec_SseEvent$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return SseEventRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SseEventRef> {
    UnsafePointer<SseEventRef>(OpaquePointer(__swift_bridge__$Vec_SseEvent$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_SseEvent$len(vecPtr)
  }
}


public class JwtConfig: JwtConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$JwtConfig$_free(ptr)
    }
  }
}
public class JwtConfigRefMut: JwtConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class JwtConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension JwtConfigRef {
  public func secret() -> RustString {
    RustString(ptr: __swift_bridge__$JwtConfig$secret(ptr))
  }

  public func algorithm() -> RustString {
    RustString(ptr: __swift_bridge__$JwtConfig$algorithm(ptr))
  }

  public func audience() -> Optional<RustVec<RustString>> {
    { let val = __swift_bridge__$JwtConfig$audience(ptr); if val != nil { return RustVec(ptr: val!) } else { return nil } }()
  }

  public func issuer() -> Optional<RustString> {
    { let val = __swift_bridge__$JwtConfig$issuer(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func leeway() -> UInt64 {
    __swift_bridge__$JwtConfig$leeway(ptr)
  }
}
extension JwtConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_JwtConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_JwtConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: JwtConfig) {
    __swift_bridge__$Vec_JwtConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_JwtConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (JwtConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<JwtConfigRef> {
    let pointer = __swift_bridge__$Vec_JwtConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return JwtConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<JwtConfigRefMut> {
    let pointer = __swift_bridge__$Vec_JwtConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return JwtConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<JwtConfigRef> {
    UnsafePointer<JwtConfigRef>(OpaquePointer(__swift_bridge__$Vec_JwtConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_JwtConfig$len(vecPtr)
  }
}


public class ApiKeyConfig: ApiKeyConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ApiKeyConfig$_free(ptr)
    }
  }
}
public class ApiKeyConfigRefMut: ApiKeyConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ApiKeyConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ApiKeyConfigRef {
  public func keys() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$ApiKeyConfig$keys(ptr))
  }

  public func headerName() -> RustString {
    RustString(ptr: __swift_bridge__$ApiKeyConfig$header_name(ptr))
  }
}
extension ApiKeyConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ApiKeyConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ApiKeyConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ApiKeyConfig) {
    __swift_bridge__$Vec_ApiKeyConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ApiKeyConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ApiKeyConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ApiKeyConfigRef> {
    let pointer = __swift_bridge__$Vec_ApiKeyConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ApiKeyConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ApiKeyConfigRefMut> {
    let pointer = __swift_bridge__$Vec_ApiKeyConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ApiKeyConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ApiKeyConfigRef> {
    UnsafePointer<ApiKeyConfigRef>(OpaquePointer(__swift_bridge__$Vec_ApiKeyConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ApiKeyConfig$len(vecPtr)
  }
}


public class StaticFilesConfig: StaticFilesConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$StaticFilesConfig$_free(ptr)
    }
  }
}
public class StaticFilesConfigRefMut: StaticFilesConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class StaticFilesConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension StaticFilesConfigRef {
  public func directory() -> RustString {
    RustString(ptr: __swift_bridge__$StaticFilesConfig$directory(ptr))
  }

  public func routePrefix() -> RustString {
    RustString(ptr: __swift_bridge__$StaticFilesConfig$route_prefix(ptr))
  }

  public func indexFile() -> Bool {
    __swift_bridge__$StaticFilesConfig$index_file(ptr)
  }

  public func cacheControl() -> Optional<RustString> {
    { let val = __swift_bridge__$StaticFilesConfig$cache_control(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }
}
extension StaticFilesConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_StaticFilesConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_StaticFilesConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: StaticFilesConfig) {
    __swift_bridge__$Vec_StaticFilesConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_StaticFilesConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (StaticFilesConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StaticFilesConfigRef> {
    let pointer = __swift_bridge__$Vec_StaticFilesConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return StaticFilesConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<StaticFilesConfigRefMut> {
    let pointer = __swift_bridge__$Vec_StaticFilesConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return StaticFilesConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<StaticFilesConfigRef> {
    UnsafePointer<StaticFilesConfigRef>(OpaquePointer(__swift_bridge__$Vec_StaticFilesConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_StaticFilesConfig$len(vecPtr)
  }
}


public class ServerConfig: ServerConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ServerConfig$_free(ptr)
    }
  }
}
extension ServerConfig {
  public convenience init<GenericIntoRustString: IntoRustString>(_ host: GenericIntoRustString, _ port: UInt16, _ workers: UInt, _ enable_request_id: Bool, _ max_body_size: Optional<UInt>, _ request_timeout: Optional<UInt64>, _ compression: Optional<CompressionConfig>, _ rate_limit: Optional<RateLimitConfig>, _ jwt_auth: Optional<JwtConfig>, _ api_key_auth: Optional<ApiKeyConfig>, _ static_files: RustVec<StaticFilesConfig>, _ graceful_shutdown: Bool, _ shutdown_timeout: UInt64, _ asyncapi: Optional<AsyncApiConfig>, _ openapi: Optional<OpenApiConfig>, _ jsonrpc: Optional<JsonRpcConfig>, _ grpc: Optional<GrpcConfig>, _ background_tasks: BackgroundTaskConfig, _ enable_http_trace: Bool) {
    self.init(ptr: __swift_bridge__$ServerConfig$new({ let rustString = host.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), port, workers, enable_request_id, max_body_size.intoFfiRepr(), request_timeout.intoFfiRepr(), { if let val = compression { val.isOwned = false; return val.ptr } else { return nil } }(), { if let val = rate_limit { val.isOwned = false; return val.ptr } else { return nil } }(), { if let val = jwt_auth { val.isOwned = false; return val.ptr } else { return nil } }(), { if let val = api_key_auth { val.isOwned = false; return val.ptr } else { return nil } }(), { let val = static_files; val.isOwned = false; return val.ptr }(), graceful_shutdown, shutdown_timeout, { if let val = asyncapi { val.isOwned = false; return val.ptr } else { return nil } }(), { if let val = openapi { val.isOwned = false; return val.ptr } else { return nil } }(), { if let val = jsonrpc { val.isOwned = false; return val.ptr } else { return nil } }(), { if let val = grpc { val.isOwned = false; return val.ptr } else { return nil } }(), {background_tasks.isOwned = false; return background_tasks.ptr;}(), enable_http_trace))
  }
}
public class ServerConfigRefMut: ServerConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ServerConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ServerConfigRef {
  public func host() -> RustString {
    RustString(ptr: __swift_bridge__$ServerConfig$host(ptr))
  }

  public func port() -> UInt16 {
    __swift_bridge__$ServerConfig$port(ptr)
  }

  public func workers() -> UInt {
    __swift_bridge__$ServerConfig$workers(ptr)
  }

  public func enableRequestId() -> Bool {
    __swift_bridge__$ServerConfig$enable_request_id(ptr)
  }

  public func maxBodySize() -> Optional<UInt> {
    __swift_bridge__$ServerConfig$max_body_size(ptr).intoSwiftRepr()
  }

  public func requestTimeout() -> Optional<UInt64> {
    __swift_bridge__$ServerConfig$request_timeout(ptr).intoSwiftRepr()
  }

  public func compression() -> Optional<CompressionConfig> {
    { let val = __swift_bridge__$ServerConfig$compression(ptr); if val != nil { return CompressionConfig(ptr: val!) } else { return nil } }()
  }

  public func rateLimit() -> Optional<RateLimitConfig> {
    { let val = __swift_bridge__$ServerConfig$rate_limit(ptr); if val != nil { return RateLimitConfig(ptr: val!) } else { return nil } }()
  }

  public func jwtAuth() -> Optional<JwtConfig> {
    { let val = __swift_bridge__$ServerConfig$jwt_auth(ptr); if val != nil { return JwtConfig(ptr: val!) } else { return nil } }()
  }

  public func apiKeyAuth() -> Optional<ApiKeyConfig> {
    { let val = __swift_bridge__$ServerConfig$api_key_auth(ptr); if val != nil { return ApiKeyConfig(ptr: val!) } else { return nil } }()
  }

  public func staticFiles() -> RustVec<StaticFilesConfig> {
    RustVec(ptr: __swift_bridge__$ServerConfig$static_files(ptr))
  }

  public func gracefulShutdown() -> Bool {
    __swift_bridge__$ServerConfig$graceful_shutdown(ptr)
  }

  public func shutdownTimeout() -> UInt64 {
    __swift_bridge__$ServerConfig$shutdown_timeout(ptr)
  }

  public func asyncapi() -> Optional<AsyncApiConfig> {
    { let val = __swift_bridge__$ServerConfig$asyncapi(ptr); if val != nil { return AsyncApiConfig(ptr: val!) } else { return nil } }()
  }

  public func openapi() -> Optional<OpenApiConfig> {
    { let val = __swift_bridge__$ServerConfig$openapi(ptr); if val != nil { return OpenApiConfig(ptr: val!) } else { return nil } }()
  }

  public func jsonrpc() -> Optional<JsonRpcConfig> {
    { let val = __swift_bridge__$ServerConfig$jsonrpc(ptr); if val != nil { return JsonRpcConfig(ptr: val!) } else { return nil } }()
  }

  public func grpc() -> Optional<GrpcConfig> {
    { let val = __swift_bridge__$ServerConfig$grpc(ptr); if val != nil { return GrpcConfig(ptr: val!) } else { return nil } }()
  }

  public func backgroundTasks() -> BackgroundTaskConfig {
    BackgroundTaskConfig(ptr: __swift_bridge__$ServerConfig$background_tasks(ptr))
  }

  public func enableHttpTrace() -> Bool {
    __swift_bridge__$ServerConfig$enable_http_trace(ptr)
  }
}
extension ServerConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ServerConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ServerConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ServerConfig) {
    __swift_bridge__$Vec_ServerConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ServerConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ServerConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ServerConfigRef> {
    let pointer = __swift_bridge__$Vec_ServerConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ServerConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ServerConfigRefMut> {
    let pointer = __swift_bridge__$Vec_ServerConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ServerConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ServerConfigRef> {
    UnsafePointer<ServerConfigRef>(OpaquePointer(__swift_bridge__$Vec_ServerConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ServerConfig$len(vecPtr)
  }
}


public class RouteBuilder: RouteBuilderRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$RouteBuilder$_free(ptr)
    }
  }
}
public class RouteBuilderRefMut: RouteBuilderRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class RouteBuilderRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension RouteBuilder: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_RouteBuilder$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_RouteBuilder$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RouteBuilder) {
    __swift_bridge__$Vec_RouteBuilder$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_RouteBuilder$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (RouteBuilder(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RouteBuilderRef> {
    let pointer = __swift_bridge__$Vec_RouteBuilder$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return RouteBuilderRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RouteBuilderRefMut> {
    let pointer = __swift_bridge__$Vec_RouteBuilder$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return RouteBuilderRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RouteBuilderRef> {
    UnsafePointer<RouteBuilderRef>(OpaquePointer(__swift_bridge__$Vec_RouteBuilder$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_RouteBuilder$len(vecPtr)
  }
}


public class JsonRpcMethodInfo: JsonRpcMethodInfoRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$JsonRpcMethodInfo$_free(ptr)
    }
  }
}
public class JsonRpcMethodInfoRefMut: JsonRpcMethodInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class JsonRpcMethodInfoRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension JsonRpcMethodInfoRef {
  public func methodName() -> RustString {
    RustString(ptr: __swift_bridge__$JsonRpcMethodInfo$method_name(ptr))
  }

  public func description() -> Optional<RustString> {
    { let val = __swift_bridge__$JsonRpcMethodInfo$description(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func paramsSchema() -> Optional<RustString> {
    { let val = __swift_bridge__$JsonRpcMethodInfo$params_schema(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func resultSchema() -> Optional<RustString> {
    { let val = __swift_bridge__$JsonRpcMethodInfo$result_schema(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func deprecated() -> Bool {
    __swift_bridge__$JsonRpcMethodInfo$deprecated(ptr)
  }

  public func tags() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$JsonRpcMethodInfo$tags(ptr))
  }
}
extension JsonRpcMethodInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_JsonRpcMethodInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_JsonRpcMethodInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: JsonRpcMethodInfo) {
    __swift_bridge__$Vec_JsonRpcMethodInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_JsonRpcMethodInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (JsonRpcMethodInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<JsonRpcMethodInfoRef> {
    let pointer = __swift_bridge__$Vec_JsonRpcMethodInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return JsonRpcMethodInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<JsonRpcMethodInfoRefMut> {
    let pointer = __swift_bridge__$Vec_JsonRpcMethodInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return JsonRpcMethodInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<JsonRpcMethodInfoRef> {
    UnsafePointer<JsonRpcMethodInfoRef>(OpaquePointer(__swift_bridge__$Vec_JsonRpcMethodInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_JsonRpcMethodInfo$len(vecPtr)
  }
}


public class ProblemDetails: ProblemDetailsRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ProblemDetails$_free(ptr)
    }
  }
}
public class ProblemDetailsRefMut: ProblemDetailsRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ProblemDetailsRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ProblemDetailsRef {
  public func typeUri() -> RustString {
    RustString(ptr: __swift_bridge__$ProblemDetails$type_uri(ptr))
  }

  public func title() -> RustString {
    RustString(ptr: __swift_bridge__$ProblemDetails$title(ptr))
  }

  public func status() -> UInt16 {
    __swift_bridge__$ProblemDetails$status(ptr)
  }

  public func detail() -> Optional<RustString> {
    { let val = __swift_bridge__$ProblemDetails$detail(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func instance() -> Optional<RustString> {
    { let val = __swift_bridge__$ProblemDetails$instance(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func extensions() -> RustString {
    RustString(ptr: __swift_bridge__$ProblemDetails$extensions(ptr))
  }
}
extension ProblemDetails: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ProblemDetails$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ProblemDetails$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ProblemDetails) {
    __swift_bridge__$Vec_ProblemDetails$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ProblemDetails$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ProblemDetails(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProblemDetailsRef> {
    let pointer = __swift_bridge__$Vec_ProblemDetails$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ProblemDetailsRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ProblemDetailsRefMut> {
    let pointer = __swift_bridge__$Vec_ProblemDetails$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ProblemDetailsRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ProblemDetailsRef> {
    UnsafePointer<ProblemDetailsRef>(OpaquePointer(__swift_bridge__$Vec_ProblemDetails$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ProblemDetails$len(vecPtr)
  }
}


public class AsyncApiConfig: AsyncApiConfigRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$AsyncApiConfig$_free(ptr)
    }
  }
}
extension AsyncApiConfig {
  public convenience init<GenericIntoRustString: IntoRustString>(_ enabled: Bool, _ spec: Optional<GenericIntoRustString>) {
    self.init(ptr: __swift_bridge__$AsyncApiConfig$new(enabled, { if let rustString = optionalStringIntoRustString(spec) { rustString.isOwned = false; return rustString.ptr } else { return nil } }()))
  }
}
public class AsyncApiConfigRefMut: AsyncApiConfigRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class AsyncApiConfigRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension AsyncApiConfigRef {
  public func enabled() -> Bool {
    __swift_bridge__$AsyncApiConfig$enabled(ptr)
  }

  public func spec() -> Optional<RustString> {
    { let val = __swift_bridge__$AsyncApiConfig$spec(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }
}
extension AsyncApiConfig: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_AsyncApiConfig$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_AsyncApiConfig$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AsyncApiConfig) {
    __swift_bridge__$Vec_AsyncApiConfig$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_AsyncApiConfig$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (AsyncApiConfig(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AsyncApiConfigRef> {
    let pointer = __swift_bridge__$Vec_AsyncApiConfig$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return AsyncApiConfigRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AsyncApiConfigRefMut> {
    let pointer = __swift_bridge__$Vec_AsyncApiConfig$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return AsyncApiConfigRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AsyncApiConfigRef> {
    UnsafePointer<AsyncApiConfigRef>(OpaquePointer(__swift_bridge__$Vec_AsyncApiConfig$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_AsyncApiConfig$len(vecPtr)
  }
}


public class ParsedChannel: ParsedChannelRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ParsedChannel$_free(ptr)
    }
  }
}
public class ParsedChannelRefMut: ParsedChannelRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ParsedChannelRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ParsedChannelRef {
  public func name() -> RustString {
    RustString(ptr: __swift_bridge__$ParsedChannel$name(ptr))
  }

  public func address() -> RustString {
    RustString(ptr: __swift_bridge__$ParsedChannel$address(ptr))
  }

  public func messages() -> RustVec<RustString> {
    RustVec(ptr: __swift_bridge__$ParsedChannel$messages(ptr))
  }

  public func bindings() -> Optional<RustString> {
    { let val = __swift_bridge__$ParsedChannel$bindings(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }
}
extension ParsedChannel: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ParsedChannel$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ParsedChannel$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ParsedChannel) {
    __swift_bridge__$Vec_ParsedChannel$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ParsedChannel$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ParsedChannel(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParsedChannelRef> {
    let pointer = __swift_bridge__$Vec_ParsedChannel$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParsedChannelRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParsedChannelRefMut> {
    let pointer = __swift_bridge__$Vec_ParsedChannel$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParsedChannelRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ParsedChannelRef> {
    UnsafePointer<ParsedChannelRef>(OpaquePointer(__swift_bridge__$Vec_ParsedChannel$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ParsedChannel$len(vecPtr)
  }
}


public class ParsedOperation: ParsedOperationRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ParsedOperation$_free(ptr)
    }
  }
}
public class ParsedOperationRefMut: ParsedOperationRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ParsedOperationRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ParsedOperationRef {
  public func name() -> RustString {
    RustString(ptr: __swift_bridge__$ParsedOperation$name(ptr))
  }

  public func action() -> RustString {
    RustString(ptr: __swift_bridge__$ParsedOperation$action(ptr))
  }

  public func channel() -> RustString {
    RustString(ptr: __swift_bridge__$ParsedOperation$channel(ptr))
  }
}
extension ParsedOperation: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ParsedOperation$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ParsedOperation$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ParsedOperation) {
    __swift_bridge__$Vec_ParsedOperation$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ParsedOperation$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ParsedOperation(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParsedOperationRef> {
    let pointer = __swift_bridge__$Vec_ParsedOperation$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParsedOperationRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParsedOperationRefMut> {
    let pointer = __swift_bridge__$Vec_ParsedOperation$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParsedOperationRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ParsedOperationRef> {
    UnsafePointer<ParsedOperationRef>(OpaquePointer(__swift_bridge__$Vec_ParsedOperation$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ParsedOperation$len(vecPtr)
  }
}


public class ParsedMessage: ParsedMessageRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ParsedMessage$_free(ptr)
    }
  }
}
public class ParsedMessageRefMut: ParsedMessageRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ParsedMessageRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ParsedMessageRef {
  public func name() -> RustString {
    RustString(ptr: __swift_bridge__$ParsedMessage$name(ptr))
  }

  public func schema() -> Optional<RustString> {
    { let val = __swift_bridge__$ParsedMessage$schema(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }
}
extension ParsedMessage: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ParsedMessage$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ParsedMessage$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ParsedMessage) {
    __swift_bridge__$Vec_ParsedMessage$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ParsedMessage$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ParsedMessage(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParsedMessageRef> {
    let pointer = __swift_bridge__$Vec_ParsedMessage$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParsedMessageRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParsedMessageRefMut> {
    let pointer = __swift_bridge__$Vec_ParsedMessage$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParsedMessageRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ParsedMessageRef> {
    UnsafePointer<ParsedMessageRef>(OpaquePointer(__swift_bridge__$Vec_ParsedMessage$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ParsedMessage$len(vecPtr)
  }
}


public class ParseResult: ParseResultRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ParseResult$_free(ptr)
    }
  }
}
public class ParseResultRefMut: ParseResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ParseResultRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ParseResultRef {
  public func specVersion() -> RustString {
    RustString(ptr: __swift_bridge__$ParseResult$spec_version(ptr))
  }

  public func title() -> RustString {
    RustString(ptr: __swift_bridge__$ParseResult$title(ptr))
  }

  public func apiVersion() -> RustString {
    RustString(ptr: __swift_bridge__$ParseResult$api_version(ptr))
  }

  public func channels() -> RustVec<ParsedChannel> {
    RustVec(ptr: __swift_bridge__$ParseResult$channels(ptr))
  }

  public func operations() -> RustVec<ParsedOperation> {
    RustVec(ptr: __swift_bridge__$ParseResult$operations(ptr))
  }

  public func messages() -> RustVec<ParsedMessage> {
    RustVec(ptr: __swift_bridge__$ParseResult$messages(ptr))
  }
}
extension ParseResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ParseResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ParseResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ParseResult) {
    __swift_bridge__$Vec_ParseResult$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ParseResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ParseResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParseResultRef> {
    let pointer = __swift_bridge__$Vec_ParseResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParseResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParseResultRefMut> {
    let pointer = __swift_bridge__$Vec_ParseResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParseResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ParseResultRef> {
    UnsafePointer<ParseResultRef>(OpaquePointer(__swift_bridge__$Vec_ParseResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ParseResult$len(vecPtr)
  }
}


public class ParseRequest: ParseRequestRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ParseRequest$_free(ptr)
    }
  }
}
public class ParseRequestRefMut: ParseRequestRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ParseRequestRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ParseRequestRef {
  public func spec() -> RustString {
    RustString(ptr: __swift_bridge__$ParseRequest$spec(ptr))
  }
}
extension ParseRequest: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ParseRequest$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ParseRequest$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ParseRequest) {
    __swift_bridge__$Vec_ParseRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ParseRequest$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ParseRequest(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParseRequestRef> {
    let pointer = __swift_bridge__$Vec_ParseRequest$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParseRequestRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ParseRequestRefMut> {
    let pointer = __swift_bridge__$Vec_ParseRequest$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ParseRequestRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ParseRequestRef> {
    UnsafePointer<ParseRequestRef>(OpaquePointer(__swift_bridge__$Vec_ParseRequest$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ParseRequest$len(vecPtr)
  }
}


public class ValidationResponse: ValidationResponseRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ValidationResponse$_free(ptr)
    }
  }
}
public class ValidationResponseRefMut: ValidationResponseRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ValidationResponseRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ValidationResponseRef {
  public func valid() -> Bool {
    __swift_bridge__$ValidationResponse$valid(ptr)
  }
}
extension ValidationResponse: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ValidationResponse$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ValidationResponse$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ValidationResponse) {
    __swift_bridge__$Vec_ValidationResponse$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ValidationResponse$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ValidationResponse(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ValidationResponseRef> {
    let pointer = __swift_bridge__$Vec_ValidationResponse$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ValidationResponseRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ValidationResponseRefMut> {
    let pointer = __swift_bridge__$Vec_ValidationResponse$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ValidationResponseRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ValidationResponseRef> {
    UnsafePointer<ValidationResponseRef>(OpaquePointer(__swift_bridge__$Vec_ValidationResponse$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ValidationResponse$len(vecPtr)
  }
}


public class ValidateRequest: ValidateRequestRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ValidateRequest$_free(ptr)
    }
  }
}
public class ValidateRequestRefMut: ValidateRequestRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ValidateRequestRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ValidateRequestRef {
  public func spec() -> RustString {
    RustString(ptr: __swift_bridge__$ValidateRequest$spec(ptr))
  }

  public func channel() -> RustString {
    RustString(ptr: __swift_bridge__$ValidateRequest$channel(ptr))
  }

  public func message() -> RustString {
    RustString(ptr: __swift_bridge__$ValidateRequest$message(ptr))
  }

  public func payload() -> RustString {
    RustString(ptr: __swift_bridge__$ValidateRequest$payload(ptr))
  }
}
extension ValidateRequest: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ValidateRequest$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ValidateRequest$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ValidateRequest) {
    __swift_bridge__$Vec_ValidateRequest$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ValidateRequest$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ValidateRequest(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ValidateRequestRef> {
    let pointer = __swift_bridge__$Vec_ValidateRequest$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ValidateRequestRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ValidateRequestRefMut> {
    let pointer = __swift_bridge__$Vec_ValidateRequest$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ValidateRequestRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ValidateRequestRef> {
    UnsafePointer<ValidateRequestRef>(OpaquePointer(__swift_bridge__$Vec_ValidateRequest$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ValidateRequest$len(vecPtr)
  }
}


public class ContactInfo: ContactInfoRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ContactInfo$_free(ptr)
    }
  }
}
public class ContactInfoRefMut: ContactInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ContactInfoRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ContactInfoRef {
  public func name() -> Optional<RustString> {
    { let val = __swift_bridge__$ContactInfo$name(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func email() -> Optional<RustString> {
    { let val = __swift_bridge__$ContactInfo$email(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }

  public func url() -> Optional<RustString> {
    { let val = __swift_bridge__$ContactInfo$url(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }
}
extension ContactInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ContactInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ContactInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ContactInfo) {
    __swift_bridge__$Vec_ContactInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ContactInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ContactInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ContactInfoRef> {
    let pointer = __swift_bridge__$Vec_ContactInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ContactInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ContactInfoRefMut> {
    let pointer = __swift_bridge__$Vec_ContactInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ContactInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ContactInfoRef> {
    UnsafePointer<ContactInfoRef>(OpaquePointer(__swift_bridge__$Vec_ContactInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ContactInfo$len(vecPtr)
  }
}


public class LicenseInfo: LicenseInfoRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$LicenseInfo$_free(ptr)
    }
  }
}
public class LicenseInfoRefMut: LicenseInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class LicenseInfoRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension LicenseInfoRef {
  public func name() -> RustString {
    RustString(ptr: __swift_bridge__$LicenseInfo$name(ptr))
  }

  public func url() -> Optional<RustString> {
    { let val = __swift_bridge__$LicenseInfo$url(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }
}
extension LicenseInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_LicenseInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_LicenseInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: LicenseInfo) {
    __swift_bridge__$Vec_LicenseInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_LicenseInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (LicenseInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LicenseInfoRef> {
    let pointer = __swift_bridge__$Vec_LicenseInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return LicenseInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<LicenseInfoRefMut> {
    let pointer = __swift_bridge__$Vec_LicenseInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return LicenseInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<LicenseInfoRef> {
    UnsafePointer<LicenseInfoRef>(OpaquePointer(__swift_bridge__$Vec_LicenseInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_LicenseInfo$len(vecPtr)
  }
}


public class ServerInfo: ServerInfoRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$ServerInfo$_free(ptr)
    }
  }
}
public class ServerInfoRefMut: ServerInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class ServerInfoRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension ServerInfoRef {
  public func url() -> RustString {
    RustString(ptr: __swift_bridge__$ServerInfo$url(ptr))
  }

  public func description() -> Optional<RustString> {
    { let val = __swift_bridge__$ServerInfo$description(ptr); if val != nil { return RustString(ptr: val!) } else { return nil } }()
  }
}
extension ServerInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_ServerInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_ServerInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ServerInfo) {
    __swift_bridge__$Vec_ServerInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_ServerInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (ServerInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ServerInfoRef> {
    let pointer = __swift_bridge__$Vec_ServerInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ServerInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ServerInfoRefMut> {
    let pointer = __swift_bridge__$Vec_ServerInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return ServerInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<ServerInfoRef> {
    UnsafePointer<ServerInfoRef>(OpaquePointer(__swift_bridge__$Vec_ServerInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_ServerInfo$len(vecPtr)
  }
}


public class TestingSseEvent: TestingSseEventRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$TestingSseEvent$_free(ptr)
    }
  }
}
public class TestingSseEventRefMut: TestingSseEventRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class TestingSseEventRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension TestingSseEventRef {
  public func data() -> RustString {
    RustString(ptr: __swift_bridge__$TestingSseEvent$data(ptr))
  }
}
extension TestingSseEvent: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_TestingSseEvent$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_TestingSseEvent$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: TestingSseEvent) {
    __swift_bridge__$Vec_TestingSseEvent$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_TestingSseEvent$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (TestingSseEvent(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TestingSseEventRef> {
    let pointer = __swift_bridge__$Vec_TestingSseEvent$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return TestingSseEventRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<TestingSseEventRefMut> {
    let pointer = __swift_bridge__$Vec_TestingSseEvent$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return TestingSseEventRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<TestingSseEventRef> {
    UnsafePointer<TestingSseEventRef>(OpaquePointer(__swift_bridge__$Vec_TestingSseEvent$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_TestingSseEvent$len(vecPtr)
  }
}


public class Request: RequestRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$Request$_free(ptr)
    }
  }
}
public class RequestRefMut: RequestRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class RequestRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension Request: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_Request$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_Request$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Request) {
    __swift_bridge__$Vec_Request$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_Request$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (Request(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RequestRef> {
    let pointer = __swift_bridge__$Vec_Request$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return RequestRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RequestRefMut> {
    let pointer = __swift_bridge__$Vec_Request$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return RequestRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RequestRef> {
    UnsafePointer<RequestRef>(OpaquePointer(__swift_bridge__$Vec_Request$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_Request$len(vecPtr)
  }
}


public class RequestData: RequestDataRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$RequestData$_free(ptr)
    }
  }
}
public class RequestDataRefMut: RequestDataRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class RequestDataRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension RequestData: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_RequestData$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_RequestData$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RequestData) {
    __swift_bridge__$Vec_RequestData$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_RequestData$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (RequestData(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RequestDataRef> {
    let pointer = __swift_bridge__$Vec_RequestData$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return RequestDataRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RequestDataRefMut> {
    let pointer = __swift_bridge__$Vec_RequestData$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return RequestDataRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RequestDataRef> {
    UnsafePointer<RequestDataRef>(OpaquePointer(__swift_bridge__$Vec_RequestData$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_RequestData$len(vecPtr)
  }
}


public class Method: MethodRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$Method$_free(ptr)
    }
  }
}
public class MethodRefMut: MethodRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class MethodRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension MethodRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$Method$to_string(ptr))
  }
}
extension Method: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_Method$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_Method$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Method) {
    __swift_bridge__$Vec_Method$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_Method$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (Method(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<MethodRef> {
    let pointer = __swift_bridge__$Vec_Method$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return MethodRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<MethodRefMut> {
    let pointer = __swift_bridge__$Vec_Method$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return MethodRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<MethodRef> {
    UnsafePointer<MethodRef>(OpaquePointer(__swift_bridge__$Vec_Method$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_Method$len(vecPtr)
  }
}


public class SecuritySchemeInfo: SecuritySchemeInfoRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$SecuritySchemeInfo$_free(ptr)
    }
  }
}
public class SecuritySchemeInfoRefMut: SecuritySchemeInfoRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class SecuritySchemeInfoRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension SecuritySchemeInfoRef {
  public func to_string() -> RustString {
    RustString(ptr: __swift_bridge__$SecuritySchemeInfo$to_string(ptr))
  }
}
extension SecuritySchemeInfo: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_SecuritySchemeInfo$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_SecuritySchemeInfo$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: SecuritySchemeInfo) {
    __swift_bridge__$Vec_SecuritySchemeInfo$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_SecuritySchemeInfo$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (SecuritySchemeInfo(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SecuritySchemeInfoRef> {
    let pointer = __swift_bridge__$Vec_SecuritySchemeInfo$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return SecuritySchemeInfoRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<SecuritySchemeInfoRefMut> {
    let pointer = __swift_bridge__$Vec_SecuritySchemeInfo$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return SecuritySchemeInfoRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<SecuritySchemeInfoRef> {
    UnsafePointer<SecuritySchemeInfoRef>(OpaquePointer(__swift_bridge__$Vec_SecuritySchemeInfo$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_SecuritySchemeInfo$len(vecPtr)
  }
}


public class HandlerResult: HandlerResultRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$HandlerResult$_free(ptr)
    }
  }
}
public class HandlerResultRefMut: HandlerResultRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class HandlerResultRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension HandlerResult: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_HandlerResult$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_HandlerResult$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: HandlerResult) {
    __swift_bridge__$Vec_HandlerResult$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_HandlerResult$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (HandlerResult(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<HandlerResultRef> {
    let pointer = __swift_bridge__$Vec_HandlerResult$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return HandlerResultRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<HandlerResultRefMut> {
    let pointer = __swift_bridge__$Vec_HandlerResult$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return HandlerResultRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<HandlerResultRef> {
    UnsafePointer<HandlerResultRef>(OpaquePointer(__swift_bridge__$Vec_HandlerResult$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_HandlerResult$len(vecPtr)
  }
}


public class App: AppRefMut {
  public var isOwned: Bool = true

  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }

  deinit {
    if isOwned {
      __swift_bridge__$App$_free(ptr)
    }
  }
}
extension App {
  public convenience init() {
    self.init(ptr: __swift_bridge__$App$new())
  }
}
public class AppRefMut: AppRef {
  public override init(ptr: UnsafeMutableRawPointer) {
    super.init(ptr: ptr)
  }
}
public class AppRef {
  public var ptr: UnsafeMutableRawPointer

  public init(ptr: UnsafeMutableRawPointer) {
    self.ptr = ptr
  }
}
extension App: Vectorizable {
  public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
    __swift_bridge__$Vec_App$new()
  }

  public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
    __swift_bridge__$Vec_App$drop(vecPtr)
  }

  public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: App) {
    __swift_bridge__$Vec_App$push(vecPtr, {value.isOwned = false; return value.ptr;}())
  }

  public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
    let pointer = __swift_bridge__$Vec_App$pop(vecPtr)
    if pointer == nil {
      return nil
    } else {
      return (App(ptr: pointer!) as! Self)
    }
  }

  public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AppRef> {
    let pointer = __swift_bridge__$Vec_App$get(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return AppRef(ptr: pointer!)
    }
  }

  public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AppRefMut> {
    let pointer = __swift_bridge__$Vec_App$get_mut(vecPtr, index)
    if pointer == nil {
      return nil
    } else {
      return AppRefMut(ptr: pointer!)
    }
  }

  public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AppRef> {
    UnsafePointer<AppRef>(OpaquePointer(__swift_bridge__$Vec_App$as_ptr(vecPtr)))
  }

  public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
    __swift_bridge__$Vec_App$len(vecPtr)
  }
}
