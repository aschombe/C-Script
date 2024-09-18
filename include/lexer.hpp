#pragma once

#include <iostream>
#include <string>
#include <vector>
#include <map>

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
  // std::string get_snippet();

  // get the current position
  int get_pos();

  // get the current token
  std::string get_token();

  // tokenize the code
  std::vector<std::string> tokenize();


  private:
  // current line and column
  int line;
  int column;

  // snippet for tokenizer error messages
  // std::string snippet;

  // code to tokenize
  std::string code;

  // current position in the code
  int pos;

  // current token
  std::string token;

  // keywords, symbols, and literals
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
    {"(", "("},
    {")", ")"},
    {"{", "{"},
    {"}", "}"},
    {"[", "["},
    {"]", "]"},
    {",", ","},
    {":", ":"},
    {";", ";"}
  };

  std::map<std::string, std::string> literals = {
    {"int", "[0-9]+"},
    {"float", "[0-9]+\\.[0-9]+"},
    {"string", "\"[^\"]*\""},
    {"bool", "(true|false)"}
  };
};