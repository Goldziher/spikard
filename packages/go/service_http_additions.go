package spikard

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
