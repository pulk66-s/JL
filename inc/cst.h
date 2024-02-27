#ifndef __JL_CST_H__
#define __JL_CST_H__

enum cst_type {
    CST_TYPE_INT,
    CST_TYPE_CHAR,
    CST_TYPE_STRING
};

struct cst {
    enum cst_type type;
    union {
        int i;
        char c;
        char *s;
    } value;
};

struct cst *create_cst(char *expr);
void free_cst(struct cst *cst);

#endif
