/*
Keywords and symbols in my language:
- keyworks: let, set, del, if, elif, else, for, while, break, continue, return, exit, func, int, float, bool, string, switch, case, default, true, false
- symbols: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^=, (, ), {, }, [, ], ,, :, ;
- comments: same as c++
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
}

// destructor
Lexer::~Lexer() {
  // nothing to do here yet
}

// get current token
std::string Lexer::get_token() {
  return this->token;
}

// get current line
int Lexer::get_line() {
  return this->line;
}

// get current column
int Lexer::get_column() {
  return this->column;
}

// get current position
int Lexer::get_pos() {
  return this->pos;
}

// get current snippet
// std::string Lexer::get_snippet() {
//   return this->snippet;
// }

// get the next token
std::string Lexer::next_token() {
  // skipe whitespace (space, tab, newline)
  while (this->pos < this->code.size() && std::isspace(this->code[this->pos])) {
    if (this->code[this->pos] == '\n') {
      this->line++;
      this->column = 1;
    } else {
      this->column++;
    }
    this->pos++;
  }

  // check if we reached the end of the code
  if (this->pos >= this->code.size()) {
    this->token = "";
    return this->token;
  }

  // skip single-line comments (//)
  if (this->code.substr(this->pos, 2) == "//") {
    this->pos += 2;
    while (this->pos < this->code.size() && this->code[this->pos] != '\n') {
      this->pos++;
    }
    return next_token();
  }

  // skip multi-line comments (/* ... */)
  if (this->code.substr(this->pos, 2) == "/*") {
    this->pos += 2;
    while (this->pos < this->code.size() && this->code.substr(this->pos, 2) != "*/") {
      if (this->code[this->pos] == '\n') {
        this->line++;
        this->column = 1;
      } else {
        this->column++;
      }
      this->pos++;
    }
    this->pos += 2;
    return next_token();
  }

  // check for keywords
  // - keyworks: let, set, del, if, elif, else, for, while, break, continue, return, exit, func, int, float, bool, string, switch, case, default, true, false
  // keywords has a map of keywords in lexer.hpp
  for (auto keyword : keywords) {
    if (this->code.substr(this->pos, keyword.first.size()) == keyword.first) {
      this->token = keyword.first;
      this->pos += keyword.first.size();
      this->column += keyword.first.size();
      return this->token;
    }
  }

  // identifiers: [a-zA-Z_][a-zA-Z0-9_]*
  if (std::isalpha(this->code[this->pos]) || this->code[this->pos] == '_') {
    this->token = "";
    while (this->pos < this->code.size() && (std::isalnum(this->code[this->pos]) || this->code[this->pos] == '_')) {
      this->token += this->code[this->pos];
      this->pos++;
      this->column++;
    }
    return this->token;
  }

  // symbols
  // - symbols: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^, (, ), {, }, [, ], ,, :, ;
  // symbol map too
  for (auto symbol : symbols) {
    if (this->code.substr(this->pos, symbol.first.size()) == symbol.first) {
      this->token = symbol.first;
      this->pos += symbol.first.size();
      this->column += symbol.first.size();
      return this->token;
    }
  }

  // int: [0-9]+
  if (std::isdigit(this->code[this->pos])) {
    this->token = "";
    while (this->pos < this->code.size() && std::isdigit(this->code[this->pos])) {
        this->token += this->code[this->pos];
        this->pos++;
        this->column++;
    }
    
    // Check for float: [0-9]+\.[0-9]+
    if (this->pos < this->code.size() && this->code[this->pos] == '.') {
        this->token += '.';
        this->pos++;
        this->column++;

        // Ensure there's at least one digit after the decimal
        if (this->pos < this->code.size() && std::isdigit(this->code[this->pos])) {
            while (this->pos < this->code.size() && std::isdigit(this->code[this->pos])) {
                this->token += this->code[this->pos];
                this->pos++;
                this->column++;
            }
            return this->token; // Return the float token
        } else {
            throw std::runtime_error("Invalid float literal at line " + std::to_string(this->line) + ", column " + std::to_string(this->column));
        }
    }
    return this->token; // Return the int token if no decimal point follows
  }

  // string: "[^"]*"
  if (this->code[this->pos] == '"') {
    this->token = "";
    this->pos++;
    this->column++;
    while (this->pos < this->code.size() && this->code[this->pos] != '"') {
      this->token += this->code[this->pos];
      this->pos++;
      this->column++;
    }
    this->pos++;
    this->column++;
    return this->token;
  }

  // bool: true, false
  if (this->code.substr(this->pos, 4) == "true") {
    this->token = "true";
    this->pos += 4;
    this->column += 4;
    return this->token;
  }

  if (this->code.substr(this->pos, 5) == "false") {
    this->token = "false";
    this->pos += 5;
    this->column += 5;
    return this->token;
  }

  // unknown token, throw an error with the current line and column, and token
  // build a snippet of the code around the current position
  std::string snippet = this->code.substr(this->pos, 10);
  throw std::runtime_error("Unknown token '" + snippet + "' at line " + std::to_string(this->line) + ", column " + std::to_string(this->column));
}


// peek the next token (don't consume it)
std::string Lexer::peek_token() {
  int saved_pos = this->pos;
  int saved_line = this->line;
  int saved_column = this->column;
  std::string saved_token = this->token;

  std::string token = next_token();

  this->pos = saved_pos;
  this->line = saved_line;
  this->column = saved_column;
  this->token = saved_token;

  return token;
}

#include <iostream>

// tokenize the code
std::vector<std::string> Lexer::tokenize() {
  std::cout << "Begin tokenizing..." << std::endl;
  std::vector<std::string> tokens;
  std::string token = next_token();
  while (token != "") {
    tokens.push_back(token);
    token = next_token();
  }
  std::cout << "Done tokenizing." << std::endl;
  return tokens;
}
