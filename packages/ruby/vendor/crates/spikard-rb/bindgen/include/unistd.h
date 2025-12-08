#pragma once

/* Minimal stub for Windows builds to satisfy rb-sys bindgen when Ruby headers
 * include <unistd.h>. None of the Unix APIs are used by the bindings, so we
 * only provide the handful of types/macros bindgen expects to resolve.
 */

typedef long ssize_t;

#ifndef _SSIZE_T_DEFINED
#define _SSIZE_T_DEFINED
#endif

#ifndef _PID_T_DEFINED
#define _PID_T_DEFINED
typedef int pid_t;
#endif
