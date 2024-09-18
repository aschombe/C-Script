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

#pragma once

#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <regex>

/* takes a string of code and returns a vector of tokens */
class Lexer {
  public:
    // constructor
    Lexer(std::string code);
    
    // destructor
    ~Lexer();
    
    // get the next token
    std::string next_token();
    
    // peek the next token (don't consume it)
    std::string peek_token();
    
    // get the current line
    int get_line();

    // get the current column
    int get_column();

    // get the current snippet
    std::string get_snippet();

    // get the current position
    int get_pos();

    // get the current token
    std::string get_token();

    // tokenize the code
    std::vector<std::string> tokenize();

  private:
    int line;
    int column;
    
    // snippet for tokenizer error messages
    std::string snippet;
    
    // code to tokenize
    std::string code;
    
    // current position in the code
    int pos;
    
    // current token
    std::string token;
    
    // regex for Keywords
    std::map<std::string, std::string> keywords = {
      {"let", "let"},
      {"set", "set"},
      {"del", "del"},
      {"if", "if"},
      {"elif", "elif"},
      {"else", "else"},
      {"for", "for"},
      {"while", "while"},
      {"break", "break"},
      {"continue", "continue"},
      {"return", "return"},
      {"exit", "exit"},
      {"func", "func"},
      {"int", "int"},
      {"float", "float"},
      {"bool", "bool"},
      {"string", "string"},
      {"switch", "switch"},
      {"case", "case"},
      {"default", "default"},
      {"true", "true"},
      {"false", "false"}
    };

    // regex for operators and symbols
    std::map<std::string, std::string> symbols = {
      {"+", "+"},
      {"-", "-"},
      {"*", "*"},
      {"/", "/"},
      {"%", "%"},
      {"^", "^"},
      {"==", "=="},
      {"!=", "!="},
      {"<", "<"},
      {"<=", "<="},
      {">", ">"},
      {">=", ">="},
      {"&&", "&&"},
      {"||", "||"},
      {"=", "="},
      {"+=", "+="},
      {"-=", "-="},
      {"*=", "*="},
      {"/=", "/="},
      {"%=", "%="},
      {"^=", "^="},
      {"(", ")"},
      {"{", "}"},
      {"[", "]"},
      {",", ","},
      {":", ":"},
      {";", ";"}
    };

    // regex for comments (same as c++, //, /*, */)
    std::regex comment_regex{"(//|/\\*|\\*/)"};

    // regex for identifiers
    std::regex identifier_regex{"[a-zA-Z_][a-zA-Z0-9_]*"};

    // regex for literals
    std::regex int_regex{"[0-9]+"};
    std::regex float_regex{"[0-9]+\\.[0-9]+"};
    std::regex string_regex{"\"[^\"]*\""};
    std::regex bool_regex{"(true|false)"};

    // regex for whitespace (' ', '\t', '\n')
    std::regex whitespace_regex{"[ \t\n]"};

};