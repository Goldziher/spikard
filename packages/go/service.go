package spikard

/*
#include <string.h>
#include "spikard.h"
extern char* service_handler_callback(void* ctx, char* req);
typedef char* (*ServiceHandlerCallbackPtr)(void*, const char*);
static inline ServiceHandlerCallbackPtr get_service_handler_callback(void) {
  return (ServiceHandlerCallbackPtr)service_handler_callback;
}
*/
import "C"

import (
	"encoding/json"
	"errors"
	"fmt"
	"sync"
	"unsafe"
)
// ──────────────────────────────────────────── Service Definitions ──

// Service: App
// extern AppOpaque* spikard_app_new(void);
// extern void spikard_app_free(AppOpaque* ptr);
// extern int spikard_app_register_route(
//     AppOpaque* owner,
//     char* (*callback)(void*, const char*),
//     void* context,
//     void* builder// );
// extern void spikard_app_ep_run(
//     AppOpaque* owner// );
// extern const char* spikard_app_ep_into_router(
//     AppOpaque* owner// );
// ──────────────────────────────────────────── Handler Registry ──

// HandlerFunc is the signature for Go handler functions.
// They receive JSON-serialized request and return JSON response.
type HandlerFunc func([]byte) ([]byte, error)

// handlerRegistry maps opaque context indices to Go handlers.
var (
	handlerRegistryMu sync.Mutex
	handlerRegistry   = make(map[uintptr]HandlerFunc)
	handlerNextID     uintptr = 1
)

// registerHandler stores a Go handler in the registry and returns its opaque context ID.
func registerHandler(fn HandlerFunc) uintptr {
	handlerRegistryMu.Lock()
	defer handlerRegistryMu.Unlock()
	id := handlerNextID
	handlerNextID++
	handlerRegistry[id] = fn
	return id
}

// unregisterHandler removes a handler from the registry.
func unregisterHandler(id uintptr) {
	handlerRegistryMu.Lock()
	defer handlerRegistryMu.Unlock()
	delete(handlerRegistry, id)
}

// invokeHandler looks up a handler by ID and invokes it with the request JSON.
// Returns the response JSON or an error.
func invokeHandler(ctx uintptr, reqJSON []byte) ([]byte, error) {
	handlerRegistryMu.Lock()
	handler, ok := handlerRegistry[ctx]
	handlerRegistryMu.Unlock()
	if !ok {
		return nil, errors.New("handler not found")
	}
	return handler(reqJSON)
}

// cgo trampoline matching the C callback typedef:
// char* (*)(void* context, const char* request_json)
//
// This function is exported with //export so cgo can call it from C.
// It looks up the handler in the registry and invokes it.
//
//export service_handler_callback
func service_handler_callback(ctx unsafe.Pointer, reqCStr *C.char) *C.char {
	ctxID := uintptr(ctx)
	reqJSON := C.GoBytes(unsafe.Pointer(reqCStr), C.int(C.strlen(reqCStr)))

	respJSON, err := invokeHandler(ctxID, reqJSON)
	if err != nil {
		errJSON, _ := json.Marshal(map[string]string{"error": err.Error()})
		respJSON = errJSON
	}

	// Allocate C string from Go heap (caller responsible for freeing).
	cResp := C.CString(string(respJSON))
	return cResp
}
// ──────────────────────────────────────────── Go Service API ──

// App is a wrapper around the native service.
//
// Spikard application builder.
type App struct {
	owner unsafe.Pointer // *SPIKARDAppOpaque from C
	mu    sync.Mutex
}
// NewApp creates a new App instance.
func NewApp() (*App, error) {
	owner := unsafe.Pointer(C.spikard_app_new())
	if owner == nil {
		return nil, errors.New("failed to create App")
	}
	return &App{owner: owner}, nil
}
// Close frees the App instance.
func (s *App) Close() {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner != nil {
		C.spikard_app_free((*C.SPIKARDAppOpaque)(s.owner))
		s.owner = nil
	}
}
// RegisterRoute registers a handler for the route registration.
//
// Register a route using the provided builder and handler function.
//
// # Errors
//
// Returns an error if route construction fails or if the handler registration fails.
func (s *App) RegisterRoute(handler HandlerFunc, builder *RouteBuilder) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_register_route(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		(*C.SPIKARDRouteBuilder)(unsafe.Pointer(builder.ptr)),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Get() registers a handler via the get variant.
//
// Register a GET route at the given path.
func (s *App) Get(handler HandlerFunc, path string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_get(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		C.CString(path),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Post() registers a handler via the post variant.
//
// Register a POST route at the given path.
func (s *App) Post(handler HandlerFunc, path string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_post(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		C.CString(path),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Put() registers a handler via the put variant.
//
// Register a PUT route at the given path.
func (s *App) Put(handler HandlerFunc, path string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_put(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		C.CString(path),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Patch() registers a handler via the patch variant.
//
// Register a PATCH route at the given path.
func (s *App) Patch(handler HandlerFunc, path string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_patch(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		C.CString(path),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Delete() registers a handler via the delete variant.
//
// Register a DELETE route at the given path.
func (s *App) Delete(handler HandlerFunc, path string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_delete(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		C.CString(path),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Head() registers a handler via the head variant.
//
// Register a HEAD route at the given path.
func (s *App) Head(handler HandlerFunc, path string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_head(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		C.CString(path),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Options() registers a handler via the options variant.
//
// Register an OPTIONS route at the given path.
func (s *App) Options(handler HandlerFunc, path string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_options(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		C.CString(path),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Connect() registers a handler via the connect variant.
//
// Register a CONNECT route at the given path.
func (s *App) Connect(handler HandlerFunc, path string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_connect(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		C.CString(path),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Trace() registers a handler via the trace variant.
//
// Register a TRACE route at the given path.
func (s *App) Trace(handler HandlerFunc, path string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ctxID := registerHandler(handler)
	ret := C.spikard_app_trace(
		(*C.SPIKARDAppOpaque)(s.owner),
		C.get_service_handler_callback(),
		unsafe.Pointer(ctxID),
		C.CString(path),
	)

	if ret != 0 {
		unregisterHandler(ctxID)
		return fmt.Errorf("registration failed: error code %d", ret)
	}
	return nil
}
// Config() configures the service via 'config'.
//
// Set the server configuration.
func (s *App) Config(config *ServerConfig) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	c_configJSON, err := json.Marshal(config)
	if err != nil {
		return err
	}
	c_config := C.spikard_server_config_from_json(C.CString(string(c_configJSON)))
	if c_config == nil {
		return errors.New("ServerConfig config failed")
	}
	defer C.spikard_server_config_free(c_config)
	new_owner := C.spikard_app_config(
		(*C.SPIKARDAppOpaque)(s.owner),
		c_config,
	)

	if new_owner == nil {
		return errors.New("configurator failed")
	}
	s.owner = unsafe.Pointer(new_owner)
	return nil
}
// Run() runs the service's run entrypoint.
//
// Run the HTTP server using the configured routes.
//
// # Errors
//
// Returns an error if server construction or execution fails.
func (s *App) Run() error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ret := C.spikard_app_ep_run(
		(*C.SPIKARDAppOpaque)(s.owner),
	)
	if ret != 0 {
		return fmt.Errorf("run failed: error code %d", ret)
	}
	return nil
}

// IntoRouter() runs the service's into_router entrypoint.
//
// Build the underlying Axum router.
//
// # Errors
//
// Returns an error if server or router construction fails.
func (s *App) IntoRouter() error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return errors.New("service is closed")
	}
	ret := C.spikard_app_ep_into_router(
		(*C.SPIKARDAppOpaque)(s.owner),
	)
	if ret != 0 {
		return fmt.Errorf("into_router failed: error code %d", ret)
	}
	return nil
}

// ServerHandle allows stopping a service started via StartBackground.
type ServerHandle struct {
	service *App
}

// Stop gracefully shuts down the server.
func (h *ServerHandle) Stop() error {
	if h.service == nil {
		return errors.New("service already stopped")
	}
	h.service.Close()
	h.service = nil
	return nil
}

// StartBackground starts the service in a background goroutine and returns a handle.
// It blocks until the TCP socket is bound, so the server is guaranteed to be accepting
// connections when this call returns.
func (s *App) StartBackground(host string, port uint16) (*ServerHandle, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.owner == nil {
		return nil, errors.New("service is closed")
	}

	// Spawn Run in a goroutine. The C entrypoint will block there,
	// and we exit this function once the socket is bound.
	go func() {
		_ = s.Run()
	}()

	// Return immediately with a handle for shutdown.
	return &ServerHandle{service: s}, nil
}
