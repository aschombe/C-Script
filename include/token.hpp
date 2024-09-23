#pragma once

#include <string>

enum TokenType {
  // Literals
  INT,
  DOUBLE,
  BOOL,
  STRING,
  IDENTIFIER,
  TYPE,

  // Keywords
  LET,
  SET,
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
  int column;
  std::string value;
};

// token_to_string
// Given a token type, return a string representation of the token.
inline std::string token_to_string(TokenType type) {
  switch (type) {
    case INT: return "int";
    case DOUBLE: return "double";
    case BOOL: return "bool";
    case STRING: return "string";
    case IDENTIFIER: return "identifier";
    case TYPE: return "type";
    case LET: return "let";
    case SET: return "set";
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
  }
}