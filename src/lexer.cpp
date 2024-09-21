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
size_t Lexer::get_pos() {
  return this->pos;
}

// get current snippet
// std::string Lexer::get_snippet() {
//   return this->snippet;
// }

// get the next token
std::string Lexer::next_token() {
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

  // multi-character symbols: ==, !=, <=, >=, +=, -=, *=, /=, %=, ^=
  std::vector<std::string> multi_char_symbols = {
    "==", "!=", "<=", ">=", "+=", "-=", "*=", "/=", "%=", "^="
  };

  for (const auto& symbol : multi_char_symbols) {
    if (this->code.substr(this->pos, symbol.size()) == symbol) {
      this->token = symbol;
      this->pos += symbol.size();
      this->column += symbol.size();
      return this->token;
    }
  }

  // single-character symbols: +, -, *, /, %, ^, =, !, <, >, (, ), {, }, [, ], ,, :, ;
  std::vector<char> single_char_symbols = {
    '+', '-', '*', '/', '%', '^', '=', '!', '<', '>', '(', ')', '{', '}', '[', ']', ',', ':', ';'
  };

  for (const auto& symbol : single_char_symbols) {
    if (this->code[this->pos] == symbol) {
      this->token = this->code[this->pos];
      this->pos++;
      this->column++;
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
    this->token += this->code[this->pos];
    this->pos++;
    this->column++;
    while (this->pos < this->code.size() && this->code[this->pos] != '"') {
      this->token += this->code[this->pos];
      this->pos++;
      this->column++;
    }
    this->token += this->code[this->pos];
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
  std::string snippet = this->code.substr(this->pos, 10);
  throw std::runtime_error("Unknown token '" + snippet + "' at line " + std::to_string(this->line) + ", column " + std::to_string(this->column));
}
/* std::string Lexer::next_token() { */
/*   // skipe whitespace (space, tab, newline) */
/*   while (this->pos < this->code.size() && std::isspace(this->code[this->pos])) { */
/*     if (this->code[this->pos] == '\n') { */
/*       this->line++; */
/*       this->column = 1; */
/*     } else { */
/*       this->column++; */
/*     } */
/*     this->pos++; */
/*   } */

/*   // check if we reached the end of the code */
/*   if (this->pos >= this->code.size()) { */
/*     this->token = ""; */
/*     return this->token; */
/*   } */

/*   // skip single-line comments (//) */
/*   if (this->code.substr(this->pos, 2) == "//") { */
/*     this->pos += 2; */
/*     while (this->pos < this->code.size() && this->code[this->pos] != '\n') { */
/*       this->pos++; */
/*     } */
/*     return next_token(); */
/*   } */

/*   // skip multi-line comments (/1* ... *1/) */
//   if (this->code.substr(this->pos, 2) == "/*") {
/*     this->pos += 2; */
//     while (this->pos < this->code.size() && this->code.substr(this->pos, 2) != "*/") { */
/*       if (this->code[this->pos] == '\n') { */
/*         this->line++; */
/*         this->column = 1; */
/*       } else { */
/*         this->column++; */
/*       } */
/*       this->pos++; */
/*     } */
/*     this->pos += 2; */
/*     return next_token(); */
/*   } */

/*   // check for keywords */
/*   // - keyworks: let, set, del, if, elif, else, for, while, break, continue, return, exit, func, int, float, bool, string, switch, case, default, true, false */
/*   // keywords has a map of keywords in lexer.hpp */
/*   for (auto keyword : keywords) { */
/*     if (this->code.substr(this->pos, keyword.first.size()) == keyword.first) { */
/*       this->token = keyword.first; */
/*       this->pos += keyword.first.size(); */
/*       this->column += keyword.first.size(); */
/*       return this->token; */
/*     } */
/*   } */

/*   // identifiers: [a-zA-Z_][a-zA-Z0-9_]* */
/*   if (std::isalpha(this->code[this->pos]) || this->code[this->pos] == '_') { */
/*     this->token = ""; */
/*     while (this->pos < this->code.size() && (std::isalnum(this->code[this->pos]) || this->code[this->pos] == '_')) { */
/*       this->token += this->code[this->pos]; */
/*       this->pos++; */
/*       this->column++; */
/*     } */
/*     return this->token; */
/*   } */

/*   // symbols */
/*   // - symbols: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^=, ^, (, ), {, }, [, ], ,, :, ; */
/*   for (auto symbol : symbols) { */
/*     // if (this->code.substr(this->pos, symbol.first.size()) == symbol.first) { */
/*     //   this->token = symbol.first; */
/*     //   this->pos += symbol.first.size(); */
/*     //   this->column += symbol.first.size(); */
/*     //   return this->token; */
/*     // } */
/*     // if the first symbol is =, !, <, >, +, -, *, /, %, ^ */
/*     // then check the second symbol and if its =, then return the two symbols */
/*     if (this->code[this->pos] == symbol.first[0]) { */
/*       if (this->code[this->pos + 1] == symbol.first[1]) { */
/*         this->token = symbol.first; */
/*         this->pos += 2; */
/*         this->column += 2; */
/*         return this->token; */
/*       } else { */
/*         this->token = symbol.first[0]; */
/*         this->pos++; */
/*         this->column++; */
/*         return this->token; */
/*       } */
/*     } */
/*   } */

/*   // int: [0-9]+ */
/*   if (std::isdigit(this->code[this->pos])) { */
/*     this->token = ""; */
/*     while (this->pos < this->code.size() && std::isdigit(this->code[this->pos])) { */
/*         this->token += this->code[this->pos]; */
/*         this->pos++; */
/*         this->column++; */
/*     } */
    
/*     // Check for float: [0-9]+\.[0-9]+ */
/*     if (this->pos < this->code.size() && this->code[this->pos] == '.') { */
/*         this->token += '.'; */
/*         this->pos++; */
/*         this->column++; */

/*         // Ensure there's at least one digit after the decimal */
/*         if (this->pos < this->code.size() && std::isdigit(this->code[this->pos])) { */
/*             while (this->pos < this->code.size() && std::isdigit(this->code[this->pos])) { */
/*                 this->token += this->code[this->pos]; */
/*                 this->pos++; */
/*                 this->column++; */
/*             } */
/*             return this->token; // Return the float token */
/*         } else { */
/*             throw std::runtime_error("Invalid float literal at line " + std::to_string(this->line) + ", column " + std::to_string(this->column)); */
/*         } */
/*     } */
/*     return this->token; // Return the int token if no decimal point follows */
/*   } */

/*   // string: "[^"]*" */
/*   if (this->code[this->pos] == '"') { */
/*     this->token = ""; */
/*     this->pos++; */
/*     this->column++; */
/*     while (this->pos < this->code.size() && this->code[this->pos] != '"') { */
/*       this->token += this->code[this->pos]; */
/*       this->pos++; */
/*       this->column++; */
/*     } */
/*     this->pos++; */
/*     this->column++; */
/*     return this->token; */
/*   } */

/*   // bool: true, false */
/*   if (this->code.substr(this->pos, 4) == "true") { */
/*     this->token = "true"; */
/*     this->pos += 4; */
/*     this->column += 4; */
/*     return this->token; */
/*   } */

/*   if (this->code.substr(this->pos, 5) == "false") { */
/*     this->token = "false"; */
/*     this->pos += 5; */
/*     this->column += 5; */
/*     return this->token; */
/*   } */

/*   // unknown token, throw an error with the current line and column, and token */
/*   // build a snippet of the code around the current position */
/*   std::string snippet = this->code.substr(this->pos, 10); */
/*   throw std::runtime_error("Unknown token '" + snippet + "' at line " + std::to_string(this->line) + ", column " + std::to_string(this->column)); */
/* } */


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

// tokenize the code
std::vector<std::string> Lexer::tokenize() {
  std::vector<std::string> tokens;
  std::string token = next_token();
  while (token != "") {
    tokens.push_back(token);
    token = next_token();
  }
  return tokens;
}
