#ifndef __JL_STATUS_H__
#define __JL_STATUS_H__

/**
 * @brief   Status codes
 * use these code when returning from a function
*/

typedef enum {
    JL_OK = 0,      // No error
    JL_ERROR = 1    // General error
} jl_status_t;

#endif
