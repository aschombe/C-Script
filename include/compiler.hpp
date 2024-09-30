// compiler.hpp
// does nothing for now, but it will contain the Compiler class in the future.

#pragma once
#include "ast.hpp"
/* #include "token.hpp" */

class Compiler {
  public:
  Compiler(const std::vector<std::unique_ptr<ASTNode>>& ast);

  // main func to run compiler
  void run();

  private:
  const std::vector<std::unique_ptr<ASTNode>>& ast;
};
