#ifndef CALLBACK_H
#define CALLBACK_H

#include "spikard.h"
#include <string.h>

extern char *service_handler_callback(void *ctx, char *req);

static inline char *service_handler_wrapper(void *ctx, const char *req) {
  return service_handler_callback(ctx, (char *)req);
}

#endif // CALLBACK_H
