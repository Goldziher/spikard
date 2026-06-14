package spikard

// Config holds server configuration for the App service.
type Config struct {
	// Host address to bind on (default: "0.0.0.0").
	Host string
	// Port to listen on (default: 8000).
	Port int
	// Number of worker threads (0 = auto).
	Workers int
	// Enable gzip/brotli compression.
	Compression bool
	// CORS configuration.
	Cors *CorsConfig
	// Rate limit configuration.
	RateLimit *RateLimitConfig
	// Request timeout duration.
	RequestTimeout time.Duration
	// Maximum body size in bytes.
	MaxBodySize int64
	// JWT authentication config.
	JwtAuth *JwtAuthConfig
	// API key authentication config.
	ApiKeyAuth *ApiKeyAuthConfig
	// Static files serving configuration.
	StaticFiles *StaticFilesConfig
	// Enable request ID generation.
	EnableRequestId bool
	// Enable HTTP tracing.
	EnableHttpTrace bool
	// Graceful shutdown enabled.
	GracefulShutdown bool
	// Graceful shutdown timeout.
	ShutdownTimeout time.Duration
}

// CorsConfig holds CORS settings.
type CorsConfig struct {
	AllowOrigins []string
	AllowMethods []string
	AllowHeaders []string
	ExposeHeaders []string
	AllowCredentials bool
	MaxAge int
}

// RateLimitConfig holds rate limiting settings.
type RateLimitConfig struct {
	RequestsPerSecond int
	BurstSize int
}

// JwtAuthConfig holds JWT authentication settings.
type JwtAuthConfig struct {
	Secret string
	Audience string
	Issuer string
}

// ApiKeyAuthConfig holds API key authentication settings.
type ApiKeyAuthConfig struct {
	HeaderName string
	ValidKeys []string
}

// StaticFilesConfig holds static file serving settings.
type StaticFilesConfig struct {
	Directory string
	PathPrefix string
}


// Error types for the service.
// Each error type implements the error interface and provides HTTP status codes.

// NotFoundError is raised when Raised when the requested resource does not exist..
type NotFoundError struct {
	Message string
	Code    int
}

// Error implements the error interface.
func (e *NotFoundError) Error() string {
	return e.Message
}

// StatusCode returns the HTTP status code for this error.
func (e *NotFoundError) StatusCode() int {
	return 404
}

// ToProblemDetails converts the error to RFC 9457 ProblemDetails format.
func (e *NotFoundError) ToProblemDetails() map[string]interface{} {
	details := map[string]interface{}{
		"status": e.StatusCode(),
		"title":  "NotFoundError",
	}
	if e.Message != "" {
		details["detail"] = e.Message
	}
	return details
}

// ValidationError is raised when Raised when input validation fails. Carries a list of field errors per RFC 9457..
type ValidationError struct {
	Message string
	Code    int
}

// Error implements the error interface.
func (e *ValidationError) Error() string {
	return e.Message
}

// StatusCode returns the HTTP status code for this error.
func (e *ValidationError) StatusCode() int {
	return 422
}

// ToProblemDetails converts the error to RFC 9457 ProblemDetails format.
func (e *ValidationError) ToProblemDetails() map[string]interface{} {
	details := map[string]interface{}{
		"status": e.StatusCode(),
		"title":  "ValidationError",
	}
	if e.Message != "" {
		details["detail"] = e.Message
	}
	return details
}

// UnauthorizedError is raised when Raised when the request lacks valid authentication credentials..
type UnauthorizedError struct {
	Message string
	Code    int
}

// Error implements the error interface.
func (e *UnauthorizedError) Error() string {
	return e.Message
}

// StatusCode returns the HTTP status code for this error.
func (e *UnauthorizedError) StatusCode() int {
	return 401
}

// ToProblemDetails converts the error to RFC 9457 ProblemDetails format.
func (e *UnauthorizedError) ToProblemDetails() map[string]interface{} {
	details := map[string]interface{}{
		"status": e.StatusCode(),
		"title":  "UnauthorizedError",
	}
	if e.Message != "" {
		details["detail"] = e.Message
	}
	return details
}

// ForbiddenError is raised when Raised when the authenticated user lacks permission for the requested action..
type ForbiddenError struct {
	Message string
	Code    int
}

// Error implements the error interface.
func (e *ForbiddenError) Error() string {
	return e.Message
}

// StatusCode returns the HTTP status code for this error.
func (e *ForbiddenError) StatusCode() int {
	return 403
}

// ToProblemDetails converts the error to RFC 9457 ProblemDetails format.
func (e *ForbiddenError) ToProblemDetails() map[string]interface{} {
	details := map[string]interface{}{
		"status": e.StatusCode(),
		"title":  "ForbiddenError",
	}
	if e.Message != "" {
		details["detail"] = e.Message
	}
	return details
}

// RateLimitedError is raised when Raised when the client exceeds the configured request rate limit..
type RateLimitedError struct {
	Message string
	Code    int
}

// Error implements the error interface.
func (e *RateLimitedError) Error() string {
	return e.Message
}

// StatusCode returns the HTTP status code for this error.
func (e *RateLimitedError) StatusCode() int {
	return 429
}

// ToProblemDetails converts the error to RFC 9457 ProblemDetails format.
func (e *RateLimitedError) ToProblemDetails() map[string]interface{} {
	details := map[string]interface{}{
		"status": e.StatusCode(),
		"title":  "RateLimitedError",
	}
	if e.Message != "" {
		details["detail"] = e.Message
	}
	return details
}

// ConflictError is raised when Raised when the request conflicts with the current state of the resource..
type ConflictError struct {
	Message string
	Code    int
}

// Error implements the error interface.
func (e *ConflictError) Error() string {
	return e.Message
}

// StatusCode returns the HTTP status code for this error.
func (e *ConflictError) StatusCode() int {
	return 409
}

// ToProblemDetails converts the error to RFC 9457 ProblemDetails format.
func (e *ConflictError) ToProblemDetails() map[string]interface{} {
	details := map[string]interface{}{
		"status": e.StatusCode(),
		"title":  "ConflictError",
	}
	if e.Message != "" {
		details["detail"] = e.Message
	}
	return details
}

// InternalError is raised when Raised when the server encounters an unexpected failure..
type InternalError struct {
	Message string
	Code    int
}

// Error implements the error interface.
func (e *InternalError) Error() string {
	return e.Message
}

// StatusCode returns the HTTP status code for this error.
func (e *InternalError) StatusCode() int {
	return 500
}

// ToProblemDetails converts the error to RFC 9457 ProblemDetails format.
func (e *InternalError) ToProblemDetails() map[string]interface{} {
	details := map[string]interface{}{
		"status": e.StatusCode(),
		"title":  "InternalError",
	}
	if e.Message != "" {
		details["detail"] = e.Message
	}
	return details
}


// Lifecycle hook registration methods.

// OnRequest registers a lifecycle hook to be called Called before any other processing for each inbound request..
func (s *App) OnRequest(fn func(interface{}) error) error {
	if s.owner == nil {
		return errors.New("service is closed")
	}
	// Register the hook function in the native layer.
	// The FFI call marshals the Go function pointer and binds it.
	return s.registerOnRequestHook(fn)
}


// PreValidation registers a lifecycle hook to be called Called after parsing but before parameter validation..
func (s *App) PreValidation(fn func(interface{}) error) error {
	if s.owner == nil {
		return errors.New("service is closed")
	}
	// Register the hook function in the native layer.
	// The FFI call marshals the Go function pointer and binds it.
	return s.registerPreValidationHook(fn)
}


// PreHandler registers a lifecycle hook to be called Called after validation but before invoking the route handler..
func (s *App) PreHandler(fn func(interface{}) error) error {
	if s.owner == nil {
		return errors.New("service is closed")
	}
	// Register the hook function in the native layer.
	// The FFI call marshals the Go function pointer and binds it.
	return s.registerPreHandlerHook(fn)
}


// OnResponse registers a lifecycle hook to be called Called after the handler returns but before the response is serialized..
func (s *App) OnResponse(fn func(interface{}) error) error {
	if s.owner == nil {
		return errors.New("service is closed")
	}
	// Register the hook function in the native layer.
	// The FFI call marshals the Go function pointer and binds it.
	return s.registerOnResponseHook(fn)
}


// OnError registers a lifecycle hook to be called Called when a handler returns an error..
func (s *App) OnError(fn func(interface{}) error) error {
	if s.owner == nil {
		return errors.New("service is closed")
	}
	// Register the hook function in the native layer.
	// The FFI call marshals the Go function pointer and binds it.
	return s.registerOnErrorHook(fn)
}


// Run starts the HTTP server with the given configuration.
// This method blocks until the server exits or an error occurs.
func (s *App) Run(config Config) error {
	if s.owner == nil {
		return errors.New("service is closed")
	}

	// Set defaults if not provided.
	if config.Host == "" {
		config.Host = "0.0.0.0"
	}
	if config.Port == 0 {
		config.Port = 8000
	}

	// Marshal config to C.
	configJSON, err := json.Marshal(config)
	if err != nil {
		return fmt.Errorf("marshal config: %w", err)
	}

	// Call the C entrypoint with config.
	addr := fmt.Sprintf("%s:%d", config.Host, config.Port)
	cAddr := C.CString(addr)
	defer C.free(unsafe.Pointer(cAddr))

	cConfig := C.CString(string(configJSON))
	defer C.free(unsafe.Pointer(cConfig))

	ret := C.spikard_app_run(
		(*C.SPIKARDAppOpaque)(s.owner),
		cAddr,
		cConfig,
	)

	if ret != 0 {
		return fmt.Errorf("run failed with code %d", ret)
	}

	return nil
}


// Helper functions for request/response handling in chi-style HTTP handlers.

// PathParam extracts a path parameter from a chi router request.
// It looks up the parameter by name from the request context.
func PathParam(r *http.Request, name string) string {
	// chi stores path parameters in the request context.
	// chi.URLParam reads the value by name.
	return chi.URLParam(r, name)
}

// QueryParam extracts a query parameter from the request.
func QueryParam(r *http.Request, name string) string {
	return r.URL.Query().Get(name)
}

// HeaderParam extracts a header value from the request.
func HeaderParam(r *http.Request, name string) string {
	return r.Header.Get(name)
}

// BindJSON unmarshals the request body into the provided interface.
func BindJSON(r *http.Request, v interface{}) error {
	defer r.Body.Close()
	decoder := json.NewDecoder(r.Body)
	return decoder.Decode(v)
}

// RespondJSON marshals the value to JSON and writes it to the response.
func RespondJSON(w http.ResponseWriter, status int, v interface{}) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	encoder := json.NewEncoder(w)
	return encoder.Encode(v)
}
