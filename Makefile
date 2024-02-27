NAME	= jl

CC		= gcc -g3
INCLUDE	= -I./inc
CFLAGS	= -W -Wall -Wextra -Wno-missing-braces $(INCLUDE)

SRC	= src/main.c						\
	src/files/read.c					\
	src/files/file.c					\
	src/types/list.c					\
	src/parser/cst/delete.c				\
	src/parser/cst/create.c				\
	src/parser/cst/create/operation.c	\
	src/parser/cst/create/number.c		\
	src/parser/cst/create/spaces.c		\
	src/parser/cst/create/atom.c		\
	src/parser/ast/create.c				\
	src/parser/ast/operation.c			\
	src/parser/ast/delete.c				\
	src/eval/eval.c						\
	src/tty/tty.c
OBJ	= $(SRC:.c=.o)

all: $(NAME)

$(NAME): $(OBJ)
	$(CC) -o $(NAME) $(OBJ)

clean:
	$(RM) $(OBJ)

fclean: clean
	$(RM) $(NAME)

re: fclean all

run_tests:
	python3 tests/e2e/launch.py

%.o: %.c
	$(CC) -c $(CFLAGS) -o $@ $<
