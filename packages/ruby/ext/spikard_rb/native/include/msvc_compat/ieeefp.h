#pragma once

#ifdef _MSC_VER
// Windows MSVC compatibility stub for ieeefp.h
// Ruby's missing.h may include this header on BSD-derived systems
// On Windows MSVC, float.h provides equivalent functionality

#include <float.h>

#else
// On non-MSVC platforms, use system header if available
#if __has_include(<ieeefp.h>)
#include <ieeefp.h>
#endif
#endif
