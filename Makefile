NAME	= jl

CC		= gcc -g3
INCLUDE	= -I./inc
CFLAGS	= -W -Wall -Wextra -Wno-missing-braces $(INCLUDE)

SRC	= src/main.c		\
	src/tty.c			\
	src/cst/create.c	\
	src/cst/delete.c	\
	src/ast/create.c	\
	src/ast/delete.c	\
	src/eval/ast.c

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
