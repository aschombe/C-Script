
#include "../include/interpreter.hpp"
/* #include "../include/checker.hpp" */
#include <iostream>
#include <cmath>

Interpreter::Interpreter(const std::vector<std::unique_ptr<ASTNode>>& ast, const std::filesystem::path file) : ast(ast), ran_file(file) {}

void Interpreter::run() {
  // iterate over the AST nodes
  for (auto& node : ast) {
    interp(node);
  }
}

Value Interpreter::interp(const std::unique_ptr<ASTNode>& node) {
  switch(node->node_type()) {
    case 0: // IntNode
      return std::get<int>(node->value);
    case 1: // DoubleNode
      return std::get<double>(node->value);
    case 3: // BoolNode
      return std::get<bool>(node->value);
    case 4: // VariableNode
      return scope.get_variable(std::get<std::string>(node->value));
    case 5: // BinOp
      return interp_binop(dynamic_cast<const BinOpNode*>(node.get()));
    case 6: // UnaryOp
      return interp_unaryop(dynamic_cast<const UnaryOpNode*>(node.get()));
    case 7: // Let
      return interp_let(dynamic_cast<const LetNode*>(node.get()));
    case 8: // Set
      return interp_set(dynamic_cast<const SetNode*>(node.get()));
    case 9: // Del
      return interp_del(dynamic_cast<const DelNode*>(node.get()));
    case 10: // IEEE
      return interp_iee(dynamic_cast<const IEENode*>(node.get()));
    case 11: // For
      return interp_for(dynamic_cast<const ForNode*>(node.get()));
    case 12: // While
      return interp_while(dynamic_cast<const WhileNode*>(node.get()));
    case 13: // Break
      return interp_break(dynamic_cast<const BreakNode*>(node.get()));
    case 14: // Continue
      return interp_continue(dynamic_cast<const ContinueNode*>(node.get()));
    case 15: // Func
      return interp_func(dynamic_cast<const FuncNode*>(node.get()));
    case 16: // Call
      return interp_call(dynamic_cast<const CallNode*>(node.get()));
    case 17: // Return
      return interp_return(dynamic_cast<const ReturnNode*>(node.get()));
    case 18: // Exit
      return interp_exit(dynamic_cast<const ExitNode*>(node.get()));
    case 19: // SCD
      return interp_scd(dynamic_cast<const SCDNode*>(node.get()));
    case 20: // import
      return interp_import(dynamic_cast<const ImportNode*>(node.get()));
    case 21: // postfix
      return interp_postfix(dynamic_cast<const PostFixNode*>(node.get()));
    case 22: // struct definition
      return interp_struct(dynamic_cast<const StructNode*>(node.get()));
    default:
      throw std::runtime_error("Invalid node type: " + std::to_string(node->node_type())); 
  }
}

Value Interpreter::interp_import(const ImportNode* node) {
  std::string rel_fpath = node->value;
  if (rel_fpath.length() >= 2 && rel_fpath.front() == '"' && rel_fpath.back() == '"') {
    rel_fpath = rel_fpath.substr(1, rel_fpath.length()-2);
  }

  const std::filesystem::path import_path = this->ran_file.parent_path() / rel_fpath;
  if (!std::filesystem::exists(import_path)) {
    throw std::runtime_error("File does not exist: " + import_path.generic_string());
  }
  // Todo
  return Value();
}

Value Interpreter::interp_binop(const BinOpNode* node) {
  std::string op = node->op;
  Value left = interp(node->left);
  Value right = interp(node->right);

  if (op == "+") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) + std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) + std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) + std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) + std::get<double>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "-") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) - std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) - std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) - std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) - std::get<double>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "*") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) * std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) * std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) * std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) * std::get<double>(right));
    } else {
        throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index())); 
        }
  } else if (op == "/") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) / std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) / std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) / std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) / std::get<double>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "%") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) % std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) % static_cast<int>(std::get<double>(right)));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(static_cast<int>(std::get<double>(left)) % std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(static_cast<int>(std::get<double>(left)) % static_cast<int>(std::get<double>(right)));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "^") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::pow(std::get<int>(left), std::get<int>(right)));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::pow(std::get<int>(left), std::get<double>(right)));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::pow(std::get<double>(left), std::get<int>(right)));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::pow(std::get<double>(left), std::get<double>(right)));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "==") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) == std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) == std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) == std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) == std::get<double>(right));
    } else if (left.index() == 2 && right.index() == 2) {
      return Value(std::get<std::string>(left) == std::get<std::string>(right));
    } else if (left.index() == 3 && right.index() == 3) {
      return Value(std::get<bool>(left) == std::get<bool>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "!=") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) != std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) != std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) != std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) != std::get<double>(right));
    } else if (left.index() == 2 && right.index() == 2) {
      return Value(std::get<std::string>(left) != std::get<std::string>(right));
    } else if (left.index() == 3 && right.index() == 3) {
      return Value(std::get<bool>(left) != std::get<bool>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "<") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) < std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) < std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) < std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) < std::get<double>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "<=") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) <= std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) <= std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) <= std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) <= std::get<double>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == ">") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) > std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) > std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) > std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) > std::get<double>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == ">=") {
    if (left.index() == 0 && right.index() == 0) {
      return Value(std::get<int>(left) >= std::get<int>(right));
    } else if (left.index() == 0 && right.index() == 1) {
      return Value(std::get<int>(left) >= std::get<double>(right));
    } else if (left.index() == 1 && right.index() == 0) {
      return Value(std::get<double>(left) >= std::get<int>(right));
    } else if (left.index() == 1 && right.index() == 1) {
      return Value(std::get<double>(left) >= std::get<double>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "&&") {
    if (left.index() == 3 && right.index() == 3) {
      return Value(std::get<bool>(left) && std::get<bool>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "||") {
    if (left.index() == 3 && right.index() == 3) {
      return Value(std::get<bool>(left) || std::get<bool>(right));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else {
    throw std::runtime_error("Invalid operation: " + op);
  }
}

Value Interpreter::interp_unaryop(const UnaryOpNode* node) {
  std::string op = node->op;
  Value val = interp(node->expr);

  if (op == "-") {
    if (val.index() == 0) {
      return Value(-std::get<int>(val));
    } else if (val.index() == 1) {
      return Value(-std::get<double>(val));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(val.index()));
    }
  } else if (op == "!") {
    if (val.index() == 3) {
      return Value(!std::get<bool>(val));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(val.index()));
    }
  } else {
    throw std::runtime_error("Invalid operation: " + op);
  }
}

Value Interpreter::interp_postfix(const PostFixNode* node) {
  std::string op = node->op;
  std::string ident = node->ident;
  
  if (op == "++") {
    /* scope.set_variable(ident, Value(scope.get_variable(ident) + 1)); */
    std::cout << "Increment" << std::endl;
  } else if (op == "--") {
    std::cout << "Decrement" << std::endl;
  } else {
    throw std::runtime_error("Invalid operation: " + op);
  }

  return Value();
}

Value Interpreter::interp_let(const LetNode* node) {
  std::string name = node->name;
  Value value = interp(node->value);
  scope.add_variable(name, value);
  return Value();
}

Value Interpreter::interp_set(const SetNode* node) {
  std::string op = node->op;
  std::string var = node->ident;
  Value right = interp(node->right);
  
  if (op == "=") {
    scope.set_variable(var, right);
  } else if (op == "+=") {
    Value left = scope.get_variable(var);
    if (left.index() == 0 && right.index() == 0) {
      scope.set_variable(var, Value(std::get<int>(left) + std::get<int>(right)));
    } else if (left.index() == 0 && right.index() == 1) {
      scope.set_variable(var, Value(std::get<int>(left) + std::get<double>(right)));
    } else if (left.index() == 1 && right.index() == 0) {
      scope.set_variable(var, Value(std::get<double>(left) + std::get<int>(right)));
    } else if (left.index() == 1 && right.index() == 1) {
      scope.set_variable(var, Value(std::get<double>(left) + std::get<double>(right)));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "-=") {
    Value left = scope.get_variable(var);
    if (left.index() == 0 && right.index() == 0) {
      scope.set_variable(var, Value(std::get<int>(left) - std::get<int>(right)));
    } else if (left.index() == 0 && right.index() == 1) {
      scope.set_variable(var, Value(std::get<int>(left) - std::get<double>(right)));
    } else if (left.index() == 1 && right.index() == 0) {
      scope.set_variable(var, Value(std::get<double>(left) - std::get<int>(right)));
    } else if (left.index() == 1 && right.index() == 1) {
      scope.set_variable(var, Value(std::get<double>(left) - std::get<double>(right)));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "*=") {
    Value left = scope.get_variable(var);
    if (left.index() == 0 && right.index() == 0) {
      scope.set_variable(var, Value(std::get<int>(left) * std::get<int>(right)));
    } else if (left.index() == 0 && right.index() == 1) {
      scope.set_variable(var, Value(std::get<int>(left) * std::get<double>(right)));
    } else if (left.index() == 1 && right.index() == 0) {
      scope.set_variable(var, Value(std::get<double>(left) * std::get<int>(right)));
    } else if (left.index() == 1 && right.index() == 1) {
      scope.set_variable(var, Value(std::get<double>(left) * std::get<double>(right)));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "/=") {
    Value left = scope.get_variable(var);
    if (left.index() == 0 && right.index() == 0) {
      scope.set_variable(var, Value(std::get<int>(left) / std::get<int>(right)));
    } else if (left.index() == 0 && right.index() == 1) {
      scope.set_variable(var, Value(std::get<int>(left) / std::get<double>(right)));
    } else if (left.index() == 1 && right.index() == 0) {
      scope.set_variable(var, Value(std::get<double>(left) / std::get<int>(right)));
    } else if (left.index() == 1 && right.index() == 1) {
      scope.set_variable(var, Value(std::get<double>(left) / std::get<double>(right)));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "%=") {
    Value left = scope.get_variable(var);
    if (left.index() == 0 && right.index() == 0) {
      scope.set_variable(var, Value(std::get<int>(left) % std::get<int>(right)));
    } else if (left.index() == 0 && right.index() == 1) {
      scope.set_variable(var, Value(std::get<int>(left) % static_cast<int>(std::get<double>(right))));
    } else if (left.index() == 1 && right.index() == 0) {
      scope.set_variable(var, Value(static_cast<int>(std::get<double>(left)) % std::get<int>(right)));
    } else if (left.index() == 1 && right.index() == 1) {
      scope.set_variable(var, Value(static_cast<int>(std::get<double>(left)) % static_cast<int>(std::get<double>(right))));
    } else {
      throw std::runtime_error("Invalid operation: " + std::to_string(left.index()) + std::to_string(right.index()));
    }
  } else if (op == "^=") {
    std::cout << "Todo: ^=" << std::endl;
  } else {
    throw std::runtime_error("Invalid operation: " + op);
  }

  return Value();
}

Value Interpreter::interp_del(const DelNode* node) {
  scope.delete_variable(node->name);
  return Value();
}

Value Interpreter::interp_iee(const IEENode* node) {
  Value if_condition = interp(node->if_condition);
  if (std::get<bool>(if_condition)) {
    for (auto& n : node->if_body) {
      interp(n);
    }
  } else {
    for (auto& elif : node->elifs) {
      Value elif_condition = interp(elif.first);
      if (std::get<bool>(elif_condition)) {
        for (auto& n : elif.second) {
          interp(n);
        }
        return Value();
      }
    }
    for (auto& n : node->else_body) {
      interp(n);
    }
  }

  return Value();
}

Value Interpreter::interp_for(const ForNode* node) {
  // init is already declared
  while (std::get<bool>(interp(node->condition))) {
    for (auto& n : node->body) {
      interp(n);
    }
    interp(node->increment);
  }

  return Value();
}

Value Interpreter::interp_while(const WhileNode* node) {
  while (std::get<bool>(interp(node->condition))) {
    for (auto& n : node->body) {
      interp(n);
    }
  }

  return Value();
}

Value Interpreter::interp_break(const BreakNode* node) {
  std::cout << "TODO: Implement break" << std::endl;
  (void)node;
  return Value();
}

Value Interpreter::interp_continue(const ContinueNode* node) {
  std::cout << "TODO: Implement continue" << std::endl;
  (void)node;
  return Value();
}

Value Interpreter::interp_func(const FuncNode* node) {
  std::cout << "TODO: Implement function" << std::endl;
  (void)node;
  return Value();
}

Value Interpreter::interp_call(const CallNode* node) {
  std::cout << "TODO: Implement call" << std::endl;
  (void)node;
  return Value();
}

Value Interpreter::interp_return(const ReturnNode* node) {
  std::cout << "TODO: Implement return" << std::endl;
  (void)node;
  return Value();
}

Value Interpreter::interp_exit(const ExitNode* node) {
  std::cout << "TODO: Implement exit" << std::endl;
  (void)node;
  return Value();
}

Value Interpreter::interp_scd(const SCDNode* node) {
  std::cout << "TODO: Implement switch case default" << std::endl;
  (void)node;
  return Value();
}
