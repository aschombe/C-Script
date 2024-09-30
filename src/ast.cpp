#include "../include/ast.hpp"
#include <iostream>

void print_ast(const std::vector<std::unique_ptr<ASTNode>>& ast) {
  std::cout << "Abstract syntax tree:" << std::endl;
  std::cout << "[";
  for (size_t i = 0; i < ast.size(); i++) {
    std::cout << ast[i]->to_string();
    if (i < ast.size() - 1) {
      std::cout << ", ";
    }
  }
  std::cout << "]" << std::endl;
}
