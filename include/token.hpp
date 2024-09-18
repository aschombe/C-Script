#pragma once

#include <iostream>

enum class Token {
  // Keywords
  LET, SET, DEL, IF, ELIF, ELSE, FOR, WHILE, BREAK, CONTINUE, RETURN, EXIT, FUNC, INT, FLOAT, BOOL, STRING, SWITCH, CASE, DEFAULT, TRUE, FALSE,
  // symbols
  ADD, SUB, MUL, DIV, MOD, POW, EQ, NE, LT, LTE, GT, GTE, AND, OR, ASSIGN, ADD_ASSIGN, SUB_ASSIGN, MUL_ASSIGN, DIV_ASSIGN, MOD_ASSIGN, POW_ASSIGN,
  // Syntactic sugar
  LPAREN, RPAREN, LBRACE, RBRACE, LBRACKET, RBRACKET, COMMA, COLON, SEMICOLON,
  // Literals
  IDENT, INT_LIT, FLOAT_LIT, STRING_LIT, BOOL_LIT,
  // End of file
  END
};

// std::cout << Token::<token_name>;
std::ostream& operator<<(std::ostream& os, Token token);