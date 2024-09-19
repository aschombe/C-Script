CC = g++
CFLAGS = -Wall -Wextra -g

# make sure the obj and bin directories exist
$(shell mkdir -p obj bin)

SRC_DIR = src
OBJ_DIR = obj
BIN_DIR = bin

OUTPUT_NAME = rss

SRCS = $(wildcard $(SRC_DIR)/*.cpp)
OBJS = $(SRCS:$(SRC_DIR)/%.cpp=$(OBJ_DIR)/%.o)

EXEC = $(BIN_DIR)/$(OUTPUT_NAME)

all: $(EXEC)

$(EXEC): $(OBJS)
	$(CC) -o $@ $^

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ_DIR)/*.o $(EXEC)

memcheck:
	valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes $(EXEC)

.PHONY: all clean
