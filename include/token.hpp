#pragma once

#include <string>
#include <stdexcept>

enum TokenType {
  // Literals
  INT,
  DOUBLE,
  BOOL,
  STRING,
  IDENTIFIER,
  TYPE,

  // Keywords
  IMPORT,
  LET,
  DEL,
  IF,
  ELIF,
  ELSE,
  FOR,
  WHILE,
  BREAK,
  CONTINUE,
  RETURN,
  EXIT,
  FUNC,
  SWITCH,
  CASE,
  DEFAULT,

  // Symbols
  ADD,
  SUB,
  MUL,
  DIV,
  MOD,
  POW,
  EQ,
  NE,
  LT,
  LTE,
  GT,
  GTE,
  AND,
  OR,
  NOT,
  ASSIGN,
  ADD_ASSIGN,
  SUB_ASSIGN,
  MUL_ASSIGN,
  DIV_ASSIGN,
  MOD_ASSIGN,
  POW_ASSIGN,
  INCREMENT,
  DECREMENT,
  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,
  LBRACKET,
  RBRACKET,
  COMMA,
  COLON,
  SEMICOLON,

  // End of
  END_OF_FILE
};

struct Token {
  TokenType type;
  int line;
  int col;
  std::string value;
  /* std::string snippet; */
};

// token_to_string
// Given a token type, return a string representation of the token.
inline std::string token_type_to_string(TokenType type) {
  switch (type) {
    case INT: return "int";
    case DOUBLE: return "double";
    case BOOL: return "bool";
    case STRING: return "string";
    case IDENTIFIER: return "identifier";
    case TYPE: return "type";
    case IMPORT: return "import";
    case LET: return "let";
    case DEL: return "del";
    case IF: return "if";
    case ELIF: return "elif";
    case ELSE: return "else";
    case FOR: return "for";
    case WHILE: return "while";
    case BREAK: return "break";
    case CONTINUE: return "continue";
    case RETURN: return "return";
    case EXIT: return "exit";
    case FUNC: return "func";
    case SWITCH: return "switch";
    case CASE: return "case";
    case DEFAULT: return "default";
    case ADD: return "+";
    case SUB: return "-";
    case MUL: return "*";
    case DIV: return "/";
    case MOD: return "%";
    case POW: return "^";
    case EQ: return "==";
    case NE: return "!=";
    case LT: return "<";
    case LTE: return "<=";
    case GT: return ">";
    case GTE: return ">=";
    case AND: return "&&";
    case OR: return "||";
    case NOT: return "!";
    case ASSIGN: return "=";
    case ADD_ASSIGN: return "+=";
    case SUB_ASSIGN: return "-=";
    case MUL_ASSIGN: return "*=";
    case DIV_ASSIGN: return "/=";
    case MOD_ASSIGN: return "%=";
    case POW_ASSIGN: return "^=";
    case INCREMENT: return "++";
    case DECREMENT: return "--";
    case LPAREN: return "(";
    case RPAREN: return ")";
    case LBRACE: return "{";
    case RBRACE: return "}";
    case LBRACKET: return "[";
    case RBRACKET: return "]";
    case COMMA: return ",";
    case COLON: return ":";
    case SEMICOLON: return ";";
    case END_OF_FILE: return "end of file";
    default: throw std::runtime_error("Unknown token type");
  }

  return "";
}

inline std::string token_to_string(const Token& token) {
  // print all of the token information
  std::string result = "\
  Token: " + token_type_to_string(token.type) + "\
  Value: " + token.value + "\
  Line: " + std::to_string(token.line) + "\
  Column: " + std::to_string(token.col);
  return result;
}
