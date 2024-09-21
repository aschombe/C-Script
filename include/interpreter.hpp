#pragma once
#include "ast.hpp"
#include "scope.hpp"

class Interpreter {
  public:
  Interpreter(const std::vector<std::unique_ptr<ASTNode>>& ast);

  // the main function to run the interpreter
  void run();

  private:
  const std::vector<std::unique_ptr<ASTNode>>& ast;
  std::vector<Scope> scopes;

  Value interp(const std::unique_ptr<ASTNode>& node); 
};
