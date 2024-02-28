#ifndef __JL_CST_H__
#define __JL_CST_H__

enum cst_type {
    CST_TYPE_INT,
    CST_TYPE_CHAR,
    CST_TYPE_STRING,
    CST_TYPE_OPERATION,
};

struct cst {
    enum cst_type type;
    union {
        int i;
        char c;
        char *s;
        struct cst_operation {
            struct cst *left_expr;
            struct cst *spaces1;
            struct cst *add_atom;
            struct cst *spaces2;
            struct cst *right_expr;
            struct cst *spaces3;
        } operation;
    } value;
};

struct cst *create_cst(char *expr);
void free_cst(struct cst *cst);
struct cst *create_cst_space(char **expr);
struct cst *create_cst_spaces(char **expr);
struct cst *create_cst_add_atom(char **expr);
struct cst *create_cst_operation(char **expr);
struct cst *create_cst_number(char **expr);
struct cst *create_cst_endline(char **expr);

#endif
