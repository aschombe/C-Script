CC = g++
CFLAGS = -Wall -Wextra -g -std=c++17

# make sure the obj and bin directories exist
$(shell mkdir -p obj bin)

SRC_DIR = src
OBJ_DIR = obj
BIN_DIR = bin

OUTPUT_NAME = cst #stands for c scripting tool

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

# memcheck:
# 	valgrind --leak-check=full --show-leak-kinds=all -s --track-origins=yes $(EXEC)
# 	just run valgrind manually:
# 	valgrind --leak-check=full --log-file="valgrind_output.txt" bin/cst examples/arith.csf

.PHONY: all clean
