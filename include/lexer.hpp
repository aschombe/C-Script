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
    {"let", LET},
    {"set", SET},
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

  std::map<std::string, TokenType> symbols = {
    {"+", ADD},
    {"-", SUB},
    {"*", MUL},
    {"/", DIV},
    {"%", MOD},
    {"^", POW},
    {"==", EQ},
    {"!=", NE},
    {"<=", LTE},
    {"<", LT},
    {">", GT},
    {">=", GTE},
    {"&&", AND},
    {"||", OR},
    {"=", ASSIGN},
    {"+=", ADD_ASSIGN},
    {"-=", SUB_ASSIGN},
    {"*=", MUL_ASSIGN},
    {"/=", DIV_ASSIGN},
    {"%=", MOD_ASSIGN},
    {"^=", POW_ASSIGN},
    {"++", INCREMENT},
    {"--", DECREMENT},
    {"(", LPAREN},
    {")", RPAREN},
    {"{", LBRACE},
    {"}", RBRACE},
    {"[", LBRACKET},
    {"]", RBRACKET},
    {",", COMMA},
    {":", COLON},
    {";", SEMICOLON}
  };
};