#include <string.h>
#include "spikard.h"

// Forward declaration from cgo-generated code
extern char* service_handler_callback(void* ctx, char* req);

// Wrapper to cast const char* to char* for the callback.
// This is used by service.go to bridge the FFI (which expects const char*)
// with the Go callback signature (which takes char*).
char* service_handler_trampoline(void* ctx, const char* req) {
    return service_handler_callback(ctx, (char*)req);
}
