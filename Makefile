NAME	= jl

CC		= gcc -g3
INCLUDE	= -I./inc
CFLAGS	= -W -Wall -Wextra -Werror $(INCLUDE)

SRC	= src/main.c		\
	src/files/read.c	\
	src/files/file.c
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
