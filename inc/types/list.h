#ifndef __JL_TYPES_LIST_H__
#define __JL_TYPES_LIST_H__

#include "types/func.h"
#include "types/error.h"

struct list {
    void *data;
    struct list *next;
};

typedef Either(struct list, Error) EitherListOrError;

EitherListOrError list_new(void *data);
EitherListOrError list_append(struct list *list, void *data);
EitherListOrError list_prepend(struct list **list, void *data);
struct list list_empty(void);
void list_free(struct list **list);
size_t list_length(struct list *list);

#endif
