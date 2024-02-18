#include "types/list.h"
#include "status.h"
#include <stdlib.h>

EitherListOrError list_new(void *data)
{
    struct list *list = malloc(sizeof(struct list));

    if (list == NULL)
        return Right(EitherListOrError, error("Out of memory", JL_OUT_OF_MEMORY));
    list->data = data;
    list->next = NULL;
    return Left(EitherListOrError, list);
}

EitherListOrError list_append(struct list *list, void *data)
{
    struct list *new_list = malloc(sizeof(struct list));

    if (new_list == NULL)
        return Right(EitherListOrError, error("Out of memory", JL_OUT_OF_MEMORY));
    new_list->data = data;
    new_list->next = NULL;
    while (list->next != NULL)
        list = list->next;
    list->next = new_list;
    return Left(EitherListOrError, new_list);
}

void list_free(struct list **list)
{
    struct list *current = *list;
    struct list *next;

    while (current != NULL) {
        next = current->next;
        free(current);
        current = next;
    }
    *list = NULL;
}

EitherListOrError list_prepend(struct list **list, void *data)
{
    struct list *new_list = malloc(sizeof(struct list));

    if (new_list == NULL)
        return Right(EitherListOrError, error("Out of memory", JL_OUT_OF_MEMORY));
    new_list->data = data;
    new_list->next = *list;
    *list = new_list;
    return Left(EitherListOrError, new_list);
}

struct list list_empty(void)
{
    return (struct list) { .data = NULL, .next = NULL };
}

size_t list_length(struct list *list)
{
    size_t length = 0;

    while (list != NULL) {
        length++;
        list = list->next;
    }
    return length;
}
