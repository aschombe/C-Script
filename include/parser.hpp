#pragma once

#include "ast.hpp"
#include "token.hpp"

class Parser {
  public:
  Parser(const std::vector<Token>& tokens) : tokens(tokens) {}

  std::vector<std::unique_ptr<ASTNode>> parse();

  private:
  std::vector<Token> tokens;
  std::vector<std::unique_ptr<ASTNode>> ast;
  size_t current = 0;

  bool is_keyword(const Token& token);
  bool is_assignment(const Token& token);

  std::unique_ptr<ASTNode> parse_keyword();

  std::unique_ptr<ASTNode> parse_import();
  std::unique_ptr<ASTNode> parse_let();
  std::unique_ptr<ASTNode> parse_set();
  std::unique_ptr<ASTNode> parse_del();
  std::unique_ptr<ASTNode> parse_if();
  std::unique_ptr<ASTNode> parse_for();
  std::unique_ptr<ASTNode> parse_while();
  std::unique_ptr<ASTNode> parse_break();
  std::unique_ptr<ASTNode> parse_continue();
  std::unique_ptr<ASTNode> parse_return();
  std::unique_ptr<ASTNode> parse_exit();
  std::unique_ptr<ASTNode> parse_func();
  std::unique_ptr<ASTNode> parse_switch();
  std::unique_ptr<ASTNode> parse_struct_def();
  /* std::unique_ptr<ASTNode> parse_structdecl(); */
  /* std::unique_ptr<ASTNode> parse_structaccess(); */

  std::unique_ptr<ASTNode> parse_expression();
  std::unique_ptr<ASTNode> parse_assignment(); // =, +=, -=, *=, /=, %=, ^=
  std::unique_ptr<ASTNode> parse_logical_or();
  std::unique_ptr<ASTNode> parse_logical_and();
  std::unique_ptr<ASTNode> parse_equality();
  std::unique_ptr<ASTNode> parse_comparison();
  std::unique_ptr<ASTNode> parse_term();
  std::unique_ptr<ASTNode> parse_factor();
  std::unique_ptr<ASTNode> parse_exponentiation();
  std::unique_ptr<ASTNode> parse_unary(); // !, -, ++, --
  std::unique_ptr<ASTNode> parse_primary(); // also includes struct access (struct.field)
};
