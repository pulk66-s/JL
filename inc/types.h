#ifndef __JL_TYPES_H__
#define __JL_TYPES_H__

/**
 * home made types for JL
*/

#include "status.h"
#include <errno.h>

#define true    1
#define false   0

typedef int bool;

typedef struct {
    char *message;
    jl_status_t code;
    char *file;
    int line;
} Error;

#define Maybe(type) struct { type data; bool nothing; }

typedef Maybe(void *) Maybe;
typedef Maybe(Error) MaybeError;

#define Just(t, x) (t){.data = x, .nothing = false}
#define Nothing(t) (t){.nothing = true}

#define Either(l, r) struct { l left; r right; bool is_left; }
#define Left(t, x) (t){.left = x, .is_left = true}
#define Right(t, x) (t){.right = x, .is_left = false}

static inline Error error(char *message, jl_status_t code)
{
    return (Error) {
        .message = message,
        .code = code,
        .file = __FILE__,
        .line = __LINE__
    };
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

#endif
