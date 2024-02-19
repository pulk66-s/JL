#ifndef __JL_TYPES_ERROR_H__
#define __JL_TYPES_ERROR_H__

#include "types/func.h"
#include "status.h"
#include <errno.h>
#include <stdio.h>

typedef struct {
    char *message;
    jl_status_t code;
    char *file;
    int line;
} Error;

static inline void print_error(Error error)
{
    fprintf(stderr, "Error in %s:%d: %s\n", error.file, error.line, error.message);
}

#define error(m, c) (Error) { \
    .message = m, \
    .code = c, \
    .file = __FILE__, \
    .line = __LINE__ \
}

static inline char *fopen_errno(int err)
{
    switch (err) {
        case EACCES:
            return "Permission denied";
        case EEXIST:
            return "File exists";
        case EFAULT:
            return "Bad address";
        case EINTR:
            return "Interrupted system call";
        case EINVAL:
            return "Invalid argument";
        case EIO:
            return "I/O error";
        case EISDIR:
            return "Is a directory";
        case EMFILE:
            return "Too many open files";
        case ENAMETOOLONG:
            return "File name too long";
        case ENFILE:
            return "File table overflow";
        case ENODEV:
            return "No such device";
        case ENOENT:
            return "No such file or directory";
        case ENOMEM:
            return "Out of memory";
        case ENOSPC:
            return "No space left on device";
        case ENOTDIR:
            return "Not a directory";
        case ENXIO:
            return "No such device or address";
        case EPERM:
            return "Operation not permitted";
        case EROFS:
            return "Read-only file system";
        case ETXTBSY:
            return "Text file busy";
        default:
            return "Unknown error";
    }
}

typedef Maybe(Error) MaybeError;

#endif