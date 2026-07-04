package spikard

import (
	"encoding/json"
	"fmt"
	"reflect"
	"strings"
)

// TypedHandler is the handler function signature for ergonomic route registration.
// It receives a hydrated DTO (or nil for routes without a body) and returns the response
// value and an optional error.
type TypedHandler func(dto any) (any, error)

// TypedApp wraps the low-level *App with typed-handler and JSON Schema support.
type TypedApp struct {
	inner *App
}

// NewTypedApp creates a new ergonomic application wrapper.
func NewTypedApp() (*TypedApp, error) {
	inner, err := NewApp()
	if err != nil {
		return nil, err
	}
	return &TypedApp{inner: inner}, nil
}

// Get registers a GET route.
// dto should be nil for routes without a body.
// handler is called with (dto) and returns (response, error).
func (a *TypedApp) Get(path string, dto any, handler TypedHandler) error {
	return a.registerRoute("GET", path, dto, handler)
}

// Post registers a POST route with optional body DTO.
// dto is a prototype instance (or nil) whose type drives schema derivation.
// handler is called with the hydrated DTO (or nil) and returns (response, error).
func (a *TypedApp) Post(path string, dto any, handler TypedHandler) error {
	return a.registerRoute("POST", path, dto, handler)
}

// Put registers a PUT route with optional body DTO.
func (a *TypedApp) Put(path string, dto any, handler TypedHandler) error {
	return a.registerRoute("PUT", path, dto, handler)
}

// Patch registers a PATCH route with optional body DTO.
func (a *TypedApp) Patch(path string, dto any, handler TypedHandler) error {
	return a.registerRoute("PATCH", path, dto, handler)
}

// Delete registers a DELETE route.
func (a *TypedApp) Delete(path string, dto any, handler TypedHandler) error {
	return a.registerRoute("DELETE", path, dto, handler)
}

// Head registers a HEAD route (typically no body).
func (a *TypedApp) Head(path string, dto any, handler TypedHandler) error {
	return a.registerRoute("HEAD", path, dto, handler)
}

// Options registers an OPTIONS route (typically no body).
func (a *TypedApp) Options(path string, dto any, handler TypedHandler) error {
	return a.registerRoute("OPTIONS", path, dto, handler)
}

// Run starts the HTTP server using the configured routes.
func (a *TypedApp) Run() error {
	return a.inner.Run()
}

// StartBackground starts the server in a background goroutine.
func (a *TypedApp) StartBackground(host string, port uint16) (*ServerHandle, error) {
	return a.inner.StartBackground(host, port)
}

// Config sets the server configuration.
func (a *TypedApp) Config(config *ServerConfig) error {
	return a.inner.Config(config)
}

// Close closes the application and releases resources.
func (a *TypedApp) Close() {
	a.inner.Close()
}

// private: registerRoute creates a RouteBuilder with the derived schema and registers
// the handler with an adapter that unmarshals the JSON body into the DTO type.
func (a *TypedApp) registerRoute(method, path string, dtoProto any, handler TypedHandler) error {
	// Map HTTP method string to Method constant
	methodConst, err := stringToMethod(method)
	if err != nil {
		return err
	}

	// Create the RouteBuilder for this route
	builder, err := RouteBuilderNew(methodConst, path)
	if err != nil {
		return fmt.Errorf("failed to create RouteBuilder: %w", err)
	}

	// If dto is provided (not nil), derive JSON Schema from its type and attach
	if dtoProto != nil {
		schema := deriveJSONSchema(reflect.TypeOf(dtoProto))
		schemaJSON, err := json.Marshal(schema)
		if err != nil {
			return fmt.Errorf("failed to marshal schema: %w", err)
		}
		builder = builder.RequestSchemaJSON(schemaJSON)
	}

	// Create an adapter function that unmarshals the request body into the DTO type
	// and calls the user handler
	dtoType := reflect.TypeOf(dtoProto)
	adapter := func(reqBytes []byte) ([]byte, error) {
		var reqData map[string]any
		if err := json.Unmarshal(reqBytes, &reqData); err != nil {
			return nil, fmt.Errorf("failed to unmarshal request: %w", err)
		}

		// Extract body from the request envelope
		bodyRaw := reqData["body"]

		// Hydrate DTO if a type was provided
		var dto any
		if dtoType != nil && bodyRaw != nil {
			dtoVal := reflect.New(dtoType).Interface()
			bodyJSON, _ := json.Marshal(bodyRaw)
			if err := json.Unmarshal(bodyJSON, dtoVal); err != nil {
				return nil, fmt.Errorf("failed to unmarshal DTO: %w", err)
			}
			// Dereference pointer to get the actual value
			dto = reflect.ValueOf(dtoVal).Elem().Interface()
		}

		// Call the user handler
		result, err := handler(dto)
		if err != nil {
			// Return error as ProblemDetails
			errResp := map[string]any{
				"status": 500,
				"title":  "InternalError",
				"detail": err.Error(),
			}
			respJSON, _ := json.Marshal(errResp)
			return respJSON, nil
		}

		// Convert result to response envelope
		resp := toResponseEnvelope(result)
		respJSON, _ := json.Marshal(resp)
		return respJSON, nil
	}

	// Register the adapter with the low-level App
	return a.inner.RegisterRoute(adapter, builder)
}

// stringToMethod converts an HTTP method string to the Method constant
func stringToMethod(method string) (Method, error) {
	switch strings.ToUpper(method) {
	case "GET":
		return MethodGet, nil
	case "POST":
		return MethodPost, nil
	case "PUT":
		return MethodPut, nil
	case "PATCH":
		return MethodPatch, nil
	case "DELETE":
		return MethodDelete, nil
	case "HEAD":
		return MethodHead, nil
	case "OPTIONS":
		return MethodOptions, nil
	case "CONNECT":
		return MethodConnect, nil
	case "TRACE":
		return MethodTrace, nil
	default:
		return "", fmt.Errorf("unknown HTTP method: %s", method)
	}
}

// deriveJSONSchema recursively maps a Go type to a JSON Schema object.
// It handles structs (→ object), basic types (→ primitives), slices (→ array), etc.
func deriveJSONSchema(t reflect.Type) map[string]any {
	if t == nil {
		return map[string]any{"type": "object"}
	}

	// Dereference pointers
	if t.Kind() == reflect.Ptr {
		return deriveJSONSchema(t.Elem())
	}

	schema := map[string]any{}

	switch t.Kind() {
	case reflect.Struct:
		schema["type"] = "object"
		properties := make(map[string]any)
		required := []string{}

		for i := 0; i < t.NumField(); i++ {
			field := t.Field(i)
			// Skip unexported fields
			if field.PkgPath != "" {
				continue
			}

			// Check json tag for field name and skip directive
			fieldName, skip := jsonFieldName(field)
			if skip {
				continue
			}

			// Recursively derive schema for field type
			fieldSchema := deriveJSONSchema(field.Type)
			properties[fieldName] = fieldSchema

			// Non-pointer fields are required
			if field.Type.Kind() != reflect.Ptr {
				required = append(required, fieldName)
			}
		}

		schema["properties"] = properties
		if len(required) > 0 {
			schema["required"] = required
		}

	case reflect.String:
		schema["type"] = "string"
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
		schema["type"] = "integer"
	case reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64:
		schema["type"] = "integer"
	case reflect.Float32, reflect.Float64:
		schema["type"] = "number"
	case reflect.Bool:
		schema["type"] = "boolean"
	case reflect.Slice, reflect.Array:
		schema["type"] = "array"
		schema["items"] = deriveJSONSchema(t.Elem())
	default:
		// Fallback for unknown types
		schema["type"] = "object"
	}

	return schema
}

// jsonFieldName extracts the JSON field name from a struct field's tag.
// Returns (name, skip) where skip is true if the field should be ignored.
func jsonFieldName(field reflect.StructField) (string, bool) {
	tag := field.Tag.Get("json")
	if tag == "" {
		return field.Name, false
	}

	// Parse the tag: "fieldname,omitempty" or "fieldname" or "-"
	parts := strings.Split(tag, ",")
	name := parts[0]

	// "-" means skip this field
	if name == "-" {
		return "", true
	}

	// Use the tag name if provided, otherwise use field name
	if name != "" {
		return name, false
	}

	return field.Name, false
}

// toResponseEnvelope converts a handler result into the wire response envelope.
// If the result is a map with status_code, content, headers keys, use it as-is.
// Otherwise, wrap it as a 200 response with the result as content.
func toResponseEnvelope(result any) map[string]any {
	if result == nil {
		return map[string]any{
			"status_code": 200,
			"content":     nil,
			"headers":     map[string]string{},
		}
	}

	// Check if result is already an envelope (has status_code, content, headers)
	if m, ok := result.(map[string]any); ok {
		if _, hasStatus := m["status_code"]; hasStatus {
			if _, hasContent := m["content"]; hasContent {
				if _, hasHeaders := m["headers"]; hasHeaders {
					return m
				}
			}
		}
	}

	// Otherwise, wrap as a successful 200 response
	return map[string]any{
		"status_code": 200,
		"content":     result,
		"headers":     map[string]string{},
	}
}
