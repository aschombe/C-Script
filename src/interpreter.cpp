/*
Keywords and symbols in my language:
- keyworks: let, set, del, if, elif, else, for, while, break, continue, return, exit, func, int, float, bool, string, switch, case, default, true, false
- symbols: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^=, (, ), {, }, [, ], ,, :, ;
- comments: same as c++
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

#include "../include/interpreter.hpp"
#include "../include/checker.hpp"
#include <iostream>
#include <cmath>

Interpreter::Interpreter(const std::vector<std::unique_ptr<ASTNode>>& ast) : ast(ast) {
  // create the global scope
  scopes.push_back(Scope());
}

void Interpreter::run() {
  // iterate over the AST nodes
  for (auto& node : ast) {
    interp(node);
  }
}

Value Interpreter::interp(const std::unique_ptr<ASTNode>& node) {
  switch(node->node_type()) {
    case 0: // IntNode
      std::cout << "TODO: IntNode" << std::endl;
      return Value();
    case 1: // FloatNode
      std::cout << "TODO: FloatNode" << std::endl;
      return Value();
    case 3: // BoolNode
      std::cout << "TODO: BoolNode" << std::endl;
      return Value();
    case 4: // VariableNode
      std::cout << "TODO: VariableNode" << std::endl;
      return Value();
    case 5: // BinOp
      std::cout << "TODO: BinOp" << std::endl;
      return Value();
    case 6: // UnaryOp
      std::cout << "TODO: UnaryOp" << std::endl;
      return Value();
    case 7: // Let
      std::cout << "TODO: Let" << std::endl;
      return Value();
    case 8: // Set
      std::cout << "TODO: Set" << std::endl;
      return Value();
    case 9: // Del
      std::cout << "TODO: Del" << std::endl;
      return Value();
    case 10: // IEEE
      std::cout << "TODO: IEEE" << std::endl;
      return Value();
    case 11: // For
      std::cout << "TODO: For" << std::endl;
      return Value();
    case 12: // While
      std::cout << "TODO: While" << std::endl;
      return Value();
    case 13: // Break
      std::cout << "TODO: Break" << std::endl;
      return Value();
    case 14: // Continue
      std::cout << "TODO: Continue" << std::endl;
      return Value();
    case 15: // Func
      std::cout << "TODO: Func" << std::endl;
      return Value();
    case 16: // Call
      std::cout << "TODO: Call" << std::endl;
      return Value();
    case 17: // Return
      std::cout << "TODO: Return" << std::endl;
      return Value();
    case 18: // Exit
      std::cout << "TODO: Exit" << std::endl;
      return Value();
    case 19: // SCD
      std::cout << "TODO: SCD" << std::endl;
      return Value();
    default:
      throw std::runtime_error("Invalid node type");
  }
}

/* Value Interpreter::interp_variable(const std::shared_ptr<VariableNode>& node) { */
/*   std::string var_name = node->name; */

/*   if(!scopes.back().variable_exists(var_name)) { */
/*     throw std::runtime_error("Variable " + var_name + " not found"); */
/*   } else { */
/*     return scopes.back().get_variable(var_name); */
/*   } */
/* } */

/* Value Interpreter::interp_binop(const std::shared_ptr<BinOp>& node) { */
/*   Value left = interp(node->left); */
/*   Value right = interp(node->right); */
  
/*   // grab the operator */
/*   std::string op_string = node->op; */
  
/*   if (op_string == "&&" || op_string == "||") { */
/*     // type check, should both be bool */
/*   } else { */
/*     // type check, should both be int or float */
/*   } */

/*   int op; */
/*   if (op_string == "+") { */
/*     op = 0; */
/*   } else if (op_string == "-") { */
/*     op = 1; */
/*   } else if (op_string == "*") { */
/*     op = 2; */
/*   } else if (op_string == "/") { */
/*     op = 3; */
/*   } else if (op_string == "%") { */
/*     op = 4; */
/*   } else if (op_string == "^") { */
/*     op = 5; */
/*   } else if (op_string == "==") { */
/*     op = 6; */
/*   } else if (op_string == "!=") { */
/*     op = 7; */
/*   } else if (op_string == "<") { */
/*     op = 8; */
/*   } else if (op_string == "<=") { */
/*     op = 9; */
/*   } else if (op_string == ">") { */
/*     op = 10; */
/*   } else if (op_string == ">=") { */
/*     op = 11; */
/*   } else if (op_string == "&&") { */
/*     op = 12; */
/*   } else if (op_string == "||") { */
/*     op = 13; */
/*   } */

/*   switch(op) { */
/*     case 0: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) + std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) + std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) + std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) + std::get<float>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 1: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) - std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) - std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) - std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) - std::get<float>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 2: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) * std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) * std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) * std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) * std::get<float>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 3: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) / std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) / std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) / std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) / std::get<float>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 4: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) % std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) % static_cast<int>(std::get<float>(right))); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(static_cast<int>(std::get<float>(left)) % std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(static_cast<int>(std::get<float>(left)) % static_cast<int>(std::get<float>(right))); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 5: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::pow(std::get<int>(left), std::get<int>(right))); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::pow(std::get<int>(left), std::get<float>(right))); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::pow(std::get<float>(left), std::get<int>(right))); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::pow(std::get<float>(left), std::get<float>(right))); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 6: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) == std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) == std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) == std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) == std::get<float>(right)); */
/*       } else if (left.index() == 2 && right.index() == 2) { */
/*         return Value(std::get<std::string>(left) == std::get<std::string>(right)); */
/*       } else if (left.index() == 3 && right.index() == 3) { */
/*         return Value(std::get<bool>(left) == std::get<bool>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 7: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) != std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) != std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) != std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) != std::get<float>(right)); */
/*       } else if (left.index() == 2 && right.index() == 2) { */
/*         return Value(std::get<std::string>(left) != std::get<std::string>(right)); */
/*       } else if (left.index() == 3 && right.index() == 3) { */
/*         return Value(std::get<bool>(left) != std::get<bool>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 8: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) < std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) < std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) < std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) < std::get<float>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 9: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) <= std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) <= std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) <= std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) <= std::get<float>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 10: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) > std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) > std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) > std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) > std::get<float>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 11: */
/*       if (left.index() == 0 && right.index() == 0) { */
/*         return Value(std::get<int>(left) >= std::get<int>(right)); */
/*       } else if (left.index() == 0 && right.index() == 1) { */
/*         return Value(std::get<int>(left) >= std::get<float>(right)); */
/*       } else if (left.index() == 1 && right.index() == 0) { */
/*         return Value(std::get<float>(left) >= std::get<int>(right)); */
/*       } else if (left.index() == 1 && right.index() == 1) { */
/*         return Value(std::get<float>(left) >= std::get<float>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 12: */
/*       if (left.index() == 3 && right.index() == 3) { */
/*         return Value(std::get<bool>(left) && std::get<bool>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 13: */
/*       if (left.index() == 3 && right.index() == 3) { */
/*         return Value(std::get<bool>(left) || std::get<bool>(right)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     default: */
/*       throw std::runtime_error("Invalid operation"); */
/*   } */
/* } */

/* Value Interpreter::interp_unaryop(const std::shared_ptr<UnaryOp>& node) { */
/*   Value val = interp(node->expr); */
  
/*   // grab the operator */
/*   std::string op_string = node->op; */
  
/*   if (op_string == "-") { */
/*     // type check, should be int or float */
/*   } else if (op_string == "!") { */
/*     // type check, should be bool */
/*   } */

/*   int op; */
/*   if (op_string == "-") { */
/*     op = 0; */
/*   } else if (op_string == "!") { */
/*     op = 1; */
/*   } */

/*   switch(op) { */
/*     case 0: */
/*       if (val.index() == 0) { */
/*         return Value(-std::get<int>(val)); */
/*       } else if (val.index() == 1) { */
/*         return Value(-std::get<float>(val)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*     case 1: */
/*       if (val.index() == 3) { */
/*         return Value(!std::get<bool>(val)); */
/*       } else { */
/*         throw std::runtime_error("Invalid operation"); */
/*       } */
/*   } */
/* } */

/* Value Interpreter::interp_let(const std::shared_ptr<Let>& node) { */
/*   std::cout << "TODO: interp_let" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_set(const std::shared_ptr<Set>& node) { */
/*   std::cout << "TODO: interp_set" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_del(const std::shared_ptr<Del>& node) { */
/*   std::cout << "TODO: interp_del" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_iee(const std::shared_ptr<IEE>& node) { */
/*   std::cout << "TODO: interp_iee" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_for(const std::shared_ptr<For>& node) { */
/*   std::cout << "TODO: interp_for" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_while(const std::shared_ptr<While>& node) { */
/*   std::cout << "TODO: interp_while" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_break(const std::shared_ptr<Break>& node) { */
/*   std::cout << "TODO: interp_break" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_continue(const std::shared_ptr<Continue>& node) { */
/*   std::cout << "TODO: interp_continue" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_func(const std::shared_ptr<Func>& node) { */
/*   std::cout << "TODO: interp_func" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_call(const std::shared_ptr<Call>& node) { */
/*   std::cout << "TODO: interp_call" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_return(const std::shared_ptr<Return>& node) { */
/*   std::cout << "TODO: interp_return" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_exit(const std::shared_ptr<Exit>& node) { */
/*   std::cout << "TODO: interp_exit" << std::endl; */
/*   return Value(); */
/* } */

/* Value Interpreter::interp_scd(const std::shared_ptr<SCD>& node) { */
/*   std::cout << "TODO: interp_scd" << std::endl; */
/*   return Value(); */
/* } */
