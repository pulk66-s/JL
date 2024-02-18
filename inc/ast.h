#ifndef __JL_AST_H__
#define __JL_AST_H__

/**
 * Abstract Syntax Tree that represents the structure of the JL.
*/

enum ast_type {
    AST_NUMBER,
    AST_ADD,
};

struct ast;

struct ast_add {
    struct ast *left;
    struct ast *right;
};

struct ast {
    enum ast_type type;
    union {
        int number;
        struct ast_add add;
    };
};

#endif
