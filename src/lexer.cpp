/*
Keywords and symbols in my language:
- keyworks: let, set, del, if, elif, else, for, while, break, continue, return, exit, func, int, float, bool, string, switch, case, default, true, false
- symbols: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^=, (, ), {, }, [, ], ,, :, ;
- comments: //, \/*, *\/ (same as c++)
- identifiers: [a-zA-Z_][a-zA-Z0-9_]*
- int: [0-9]+
- float: [0-9]+\.[0-9]+
- string: "[^"]*"
- bool: true, false
- operators: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=
- precedence:
    0 (highest): function call, scope (())
    1: unary operators (- (negative), ! (not))
    2: exponentiation (^)
    3: multiplication (*, /, %)
    4: addition (+, -)
    5: Comparison1 (<, <=, >, >=)
    6: Comparison2 (==, !=)
    7: Logical AND (&&)
    8: Logical OR (||)
    9 (lowest): assignment (=, +=, -=, *=, /=, %=)
*/

#include "../include/lexer.hpp"

// constructor
Lexer::Lexer(std::string code) {
  this->code = code;
  this->line = 1;
  this->column = 1;
  this->pos = 0;
  this->snippet = "";
  // keywords, symbols, comments, identifier, and literal regex is already defined in lexer.hpp
  // so we dont have to add to the maps here
}

// destructor
Lexer::~Lexer() {
  // nothing to do here yet
}

// get current token
std::string get_token() {
  return this->token;
}

// get current line
int get_line() {
  return this->line;
}

// get current column
int get_column() {
  return this->column;
}

// get current position
int get_pos() {
  return this->pos;
}

// get current snippet
std::string get_snippet() {
  return this->snippet;
}

// get the next token
std::string next_token() {
  // if we are at the end of the code, return an empty string
  if (this->pos >= this->code.size()) {
    return "";
  }
  // if we are at the end of the line, increment the line and reset the column
  if (this->code[this->pos] == '\n') {
    this->line++;
    this->column = 1;
  }
  // if we are at a space, tab, or newline, skip it (regex for it called whitespace_regex)
  if 
    this->pos++;
    this->column++;
    return next_token();
  }
}

// peek the next token
std::string peek_token() {

}

// tokenize the code
std::vector<std::string> tokenize() {

}
