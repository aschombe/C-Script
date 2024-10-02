#pragma once
#include "token.hpp"
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
  Token next_token();

  // peek the next token (don't consume it)
  Token peek_token();

  // get the current line
  int get_line();

  // get the current column
  int get_column();

  // get the current position
  size_t get_pos();

  // get the current token
  Token get_token();

  // tokenize the code
  std::vector<Token> tokenize();

  private:
  // current line and column
  int line;
  int column;

  // code to tokenize
  std::string code;

  // current position in the code
  size_t pos;

  // current token
  Token token;

  // keywords, symbols, and literals
  std::map<std::string, TokenType> keywords = {
    {"import", IMPORT},
    {"struct", STRUCT},
    {"let", LET},
    {"del", DEL},
    {"if", IF},
    {"elif", ELIF},
    {"else", ELSE},
    {"for", FOR},
    {"while", WHILE},
    {"break", BREAK},
    {"continue", CONTINUE},
    {"return", RETURN},
    {"exit", EXIT},
    {"func", FUNC},
    {"switch", SWITCH},
    {"case", CASE},
    {"default", DEFAULT}
  };

  // map types
  std::map<std::string, TokenType> types = {
    {"int", TYPE},
    {"double", TYPE},
    {"bool", TYPE},
    {"string", TYPE}
  };

  // map single character symbols to their token type
  std::map<std::string, TokenType> single_symbols = {
  {"+", ADD},
  {"-", SUB},
  {"*", MUL},
  {"/", DIV},
  {"%", MOD},
  {"^", POW},
  {"=", ASSIGN},
  {"!", NOT},
  {"<", LT},
  {">", GT},
  {"(", LPAREN},
  {")", RPAREN},
  {"{", LBRACE},
  {"}", RBRACE},
  {"[", LBRACKET},
  {"]", RBRACKET},
  {",", COMMA},
  {":", COLON},
  {";", SEMICOLON},
  {".", PERIOD},
  };

  // map multi-character symbols to their token type
  std::map<std::string, TokenType> multi_symbols = {
    {"++", INCREMENT},
    {"--", DECREMENT},
    {"==", EQ},
    {"!=", NE},
    {"<=", LTE},
    {">=", GTE},
    {"&&", AND},
    {"||", OR},
    {"+=", ADD_ASSIGN},
    {"-=", SUB_ASSIGN},
    {"*=", MUL_ASSIGN},
    {"/=", DIV_ASSIGN},
    {"%=", MOD_ASSIGN},
    {"^=", POW_ASSIGN},
  };
};
