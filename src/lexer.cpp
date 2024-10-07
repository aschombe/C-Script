#include "../include/lexer.hpp"
#include <stdexcept>

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
Token Lexer::get_token() {
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
size_t Lexer::get_pos() {
  return this->pos;
}

std::string Lexer::extract_snippet(size_t pos) {
  // backtrack to the previous newline and advance to the next newline, thats our entire snippet
  size_t start = pos;
  while (start > 0 && this->code[start] != '\n') {
    start--;
  }
  if (this->code[start] == '\n') {
    start++;
  }

  size_t end = pos;
  while (end < this->code.size() && this->code[end] != '\n') {
    end++;
  }

  return this->code.substr(start, end - start);
}

// get the next token
Token Lexer::next_token() {
  // skip whitespace (space, tab, newline)
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
    this->token = Token{END_OF_FILE, this->line, this->column, "", ""};
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

  // check for types (int, float, string, bool)
  for (auto type : types) {
    if (this->code.substr(this->pos, type.first.size()) == type.first) {
      /* this->token = Token{type.second, this->line, this->column, type.first}; */
      std::string snippet = extract_snippet(this->pos);
      this->token = Token{type.second, this->line, this->column, type.first, snippet};
      this->pos += type.first.size();
      this->column += type.first.size();
      return this->token;
    }
  }
  
  // check for keywords
  for (auto keyword : keywords) {
    if (this->code.substr(this->pos, keyword.first.size()) == keyword.first) {
      std::string snippet = extract_snippet(this->pos);
      this->token = Token{keyword.second, this->line, this->column, keyword.first, snippet};
      this->pos += keyword.first.size();
      this->column += keyword.first.size();
      return this->token;
    }
  }


  // identifiers: [a-zA-Z_][a-zA-Z0-9_]*
  if (std::isalpha(this->code[this->pos]) || this->code[this->pos] == '_') {
    std::string snippet = extract_snippet(this->pos);
    this->token = Token{IDENTIFIER, this->line, this->column, "", snippet};
    while (this->pos < this->code.size() && (std::isalnum(this->code[this->pos]) || this->code[this->pos] == '_')) {
      this->token.value += this->code[this->pos];
      this->pos++;
      this->column++;
    }
    return this->token;
  }

  // multi-character symbols
  for (auto symbol : multi_symbols) {
    if (this->code.substr(this->pos, symbol.first.size()) == symbol.first) {
      std::string snippet = extract_snippet(this->pos);
      this->token = Token{symbol.second, this->line, this->column, symbol.first, snippet};
      this->pos += symbol.first.size();
      this->column += symbol.first.size();
      return this->token;
    }
  }

  // single-character symbols
  if (single_symbols.find(std::string(1, this->code[this->pos])) != single_symbols.end()) {
    std::string snippet = extract_snippet(this->pos);
    this->token = Token{single_symbols[std::string(1, this->code[this->pos])], this->line, this->column, std::string(1, this->code[this->pos]), snippet};
    this->pos++;
    this->column++;
    return this->token;
  }

  // int: [0-9]+
  if (std::isdigit(this->code[this->pos])) {
    std::string snippet = extract_snippet(this->pos);
    this->token = Token{INT, this->line, this->column, "", snippet};
    while (this->pos < this->code.size() && std::isdigit(this->code[this->pos])) {
      this->token.value += this->code[this->pos];
      this->pos++;
      this->column++;
    }

    // Check for float: [0-9]+\.[0-9]+
    if (this->pos < this->code.size() && this->code[this->pos] == '.') {
      this->token.value += '.';
      this->pos++;
      this->column++;

      // Ensure there's at least one digit after the decimal
      if (this->pos < this->code.size() && std::isdigit(this->code[this->pos])) {
        while (this->pos < this->code.size() && std::isdigit(this->code[this->pos])) {
          this->token.value += this->code[this->pos];
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
    std::string snippet = extract_snippet(this->pos);
    this->token = Token{STRING, this->line, this->column, "", snippet};
    this->token.value += this->code[this->pos];
    this->pos++;
    this->column++;
    while (this->pos < this->code.size() && this->code[this->pos] != '"') {
      this->token.value += this->code[this->pos];
      this->pos++;
      this->column++;
    }
    this->token.value += this->code[this->pos];
    this->pos++;
    this->column++;
    return this->token;
  }

  // bool: true, false
  if (this->code.substr(this->pos, 4) == "true") {
    std::string snippet = extract_snippet(this->pos);
    this->token = Token{BOOL, this->line, this->column, "true", snippet};
    this->pos += 4;
    this->column += 4;
    return this->token;
  }

  if (this->code.substr(this->pos, 5) == "false") {
    std::string snippet = extract_snippet(this->pos);
    this->token = Token{BOOL, this->line, this->column, "false", snippet};
    this->pos += 5;
    this->column += 5;
    return this->token;
  }

  // unknown token, throw an error with the current line and column, and token
  std::string snippet = extract_snippet(this->pos);
  std::string error_msg = "Unknown token at line " + std::to_string(this->line) + ", column " + std::to_string(this->column) + "\n";
  error_msg += snippet + "\n";
  error_msg += std::string(this->column - 1, ' ') + "^";
  throw std::runtime_error(error_msg);
}

// peek the next token (don't consume it)
Token Lexer::peek_token() {
  size_t saved_pos = this->pos;
  int saved_line = this->line;
  int saved_column = this->column;
  Token saved_token = this->token;

  Token token = next_token();

  this->pos = saved_pos;
  this->line = saved_line;
  this->column = saved_column;
  this->token = saved_token;

  return token;
}

// tokenize the code
std::vector<Token> Lexer::tokenize() {
  std::vector<Token> tokens;
  Token token = next_token();
  while (token.type != END_OF_FILE) {
    tokens.push_back(token);
    token = next_token();
  }
  return tokens;
}
