#pragma once

#include "ast.hpp"
#include "token.hpp"

class Parser {
  public:
  Parser(const std::vector<Token>& tokens) : tokens(tokens) {}
  
  ~Parser() {
    for (auto node : ast) {
      delete node;
    }
  }

  // std::vector<std::unique_ptr<ASTNode>> parse();
  std::vector<ASTNode*> parse();

  private:
  std::vector<Token> tokens;
  // std::vector<std::unique_ptr<ASTNode>> ast;
  std::vector<ASTNode*> ast;
  size_t current = 0;

  bool is_keyword(const Token& token);
  bool is_assignment(const Token& token);

  ASTNode* parse_keyword();

  ASTNode* parse_import();
  ASTNode* parse_let();
  ASTNode* parse_set();
  ASTNode* parse_del();
  ASTNode* parse_if();
  ASTNode* parse_for();
  ASTNode* parse_while();
  ASTNode* parse_break();
  ASTNode* parse_continue();
  ASTNode* parse_return();
  ASTNode* parse_exit();
  ASTNode* parse_func();
  ASTNode* parse_switch();
  ASTNode* parse_struct_def();

  ASTNode* parse_expression();
  ASTNode* parse_assignment(); // =, +=, -=, *=, /=, %=, ^=
  ASTNode* parse_logical_or();
  ASTNode* parse_logical_and();
  ASTNode* parse_equality();
  ASTNode* parse_comparison();
  ASTNode* parse_term();
  ASTNode* parse_factor();
  ASTNode* parse_exponentiation();
  ASTNode* parse_unary(); // !, -, ++, --
  ASTNode* parse_primary(); // also includes struct initialization (StructName { <field>: value ... }) and access (struct.field)
};
