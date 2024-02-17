#ifndef __JL_STATUS_H__
#define __JL_STATUS_H__

/**
 * @brief   Status codes
 * use these code when returning from a function
*/

typedef enum {
    JL_OK = 0,
    JL_ERROR = 1,
    JL_LIBC_ERROR,
    JL_OUT_OF_MEMORY,
} jl_status_t;

#endif
