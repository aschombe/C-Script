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

#ifndef TOKEN_HPP
#define TOKEN_HPP

#include <iostream>

enum class Token {
  // Whitespace
  WHITESPACE,
  // Error
  ERROR,
  // Comments
  SINGLE_LINE_COMMENT, MULTI_LINE_COMMENT,
  // Keywords
  LET, SET, DEL, IF, ELIF, ELSE, FOR, WHILE, BREAK, CONTINUE, RETURN, EXIT, FUNC, INT, FLOAT, BOOL, STRING, SWITCH, CASE, DEFAULT, TRUE, FALSE,
  // symbols
  ADD, SUB, MUL, DIV, MOD, POW, EQ, NE, LT, LTE, GT, GTE, AND, OR, ASSIGN, ADD_ASSIGN, SUB_ASSIGN, MUL_ASSIGN, DIV_ASSIGN, MOD_ASSIGN, POW_ASSIGN,
  // Syntactic sugar
  LPAREN, RPAREN, LBRACE, RBRACE, LBRACKET, RBRACKET, COMMA, COLON, SEMICOLON,
  // Literals
  IDENT, INT, FLOAT, STRING, BOOL,
  // End of file
  EOF_
}

// std::cout << Token::ADD;
std::ostream& operator<<(std::ostream& os, Token token);

#endif // TOKEN_HPP
