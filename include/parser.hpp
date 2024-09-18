#pragma once

#include "ast.hpp"

class Parser {
  public:
  Parser(const std::vector<std::string>& tokens) : tokens(tokens) {}

  std::unique_ptr<ASTNode> parse();

  private:
  std::vector<std::string> tokens;
  size_t current = 0; 
  
  std::unique_ptr<ASTNode> parse_expression();
  std::unique_ptr<ASTNode> parse_assignment(); // =, +=, -=, *=, /=, %=
  std::unique_ptr<ASTNode> parse_logical_or();
  std::unique_ptr<ASTNode> parse_logical_and();
  std::unique_ptr<ASTNode> parse_equality();
  std::unique_ptr<ASTNode> parse_comparison();
  std::unique_ptr<ASTNode> parse_term();
  std::unique_ptr<ASTNode> parse_factor();
  std::unique_ptr<ASTNode> parse_exponentiation();
  std::unique_ptr<ASTNode> parse_unary();
  std::unique_ptr<ASTNode> parse_primary();
  // put these two into parse_primary
  // std::unique_ptr<ASTNode> parse_function_call();
  // std::unique_ptr<ASTNode> parse_scope();
};