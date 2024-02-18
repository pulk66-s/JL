NAME	= jl

CC		= gcc -g3
INCLUDE	= -I./inc
CFLAGS	= -W -Wall -Wextra $(INCLUDE)

SRC	= src/main.c			\
	src/files/read.c		\
	src/files/file.c		\
	src/parser/ast.c		\
	src/parser/program.c	\
	src/types/list.c
OBJ	= $(SRC:.c=.o)

all: $(NAME)

$(NAME): $(OBJ)
	$(CC) -o $(NAME) $(OBJ)

clean:
	$(RM) $(OBJ)

fclean: clean
	$(RM) $(NAME)

re: fclean all

%.o: %.c
	$(CC) -c $(CFLAGS) -o $@ $<
