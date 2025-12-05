#pragma once

#ifdef _MSC_VER
#include <io.h>
#include <process.h>
#include <direct.h>

typedef long ssize_t;
typedef int pid_t;

#define access _access
#define dup2 _dup2
#define execve _execve
#define ftruncate _chsize
#define unlink _unlink
#define getpid _getpid
#define isatty _isatty
#define lseek _lseek
#define read _read
#define write _write
#define close _close
#define sleep _sleep

#else
#include_next <unistd.h>
#endif
