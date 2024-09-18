/*
Keywords and symbols in my language:
- keyworks: let, set, del, if, elif, else, for, while, break, continue, return, exit, func, int, float, bool, string, switch, case, default, true, false
- symbols: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^=, (, ), {, }, [, ], ,, :, ;
- comments: same as c++
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

#pragma once

#include <string>
#include <vector>
#include <memory>
#include "types.hpp"

class ASTNode {
  public:
  virtual ~ASTNode() = default;
};

class IntNode : public ASTNode {
  public:
  int value;
  IntNode(int value) : value(value) {}
};

class FloatNode : public ASTNode {
  public:
  float value;
  FloatNode(float value) : value(value) {}
};

class StringNode : public ASTNode {
  public:
  std::string value;
  StringNode(const std::string& value) : value(value) {}
};

class BoolNode : public ASTNode {
  public:
  bool value;
  BoolNode(bool value) : value(value) {}
};

class VariableNode : public ASTNode {
  public:
  std::string name;
  VariableNode(const std::string& name) : name(name) {}
};

class ParenNode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> expr;
  ParenNode(std::unique_ptr<ASTNode> expr) : expr(std::move(expr)) {}
};

class BinOpNode : public ASTNode {
  public:
  std::string op;
  std::unique_ptr<ASTNode> left;
  std::unique_ptr<ASTNode> right;
  BinOpNode(const std::string& op, std::unique_ptr<ASTNode> left, std::unique_ptr<ASTNode> right) : op(op), left(std::move(left)), right(std::move(right)) {}
};

class UnaryOpNode : public ASTNode {
  public:
  std::string op;
  std::unique_ptr<ASTNode> expr;
  UnaryOpNode(const std::string& op, std::unique_ptr<ASTNode> expr) : op(op), expr(std::move(expr)) {}
};

class LetNode : public ASTNode {
  public:
  std::string name;
  Var_Types type;
  std::unique_ptr<ASTNode> value;
  LetNode(const std::string& name, Var_Types type, std::unique_ptr<ASTNode> value) : name(name), type(type), value(std::move(value)) {}
};

class SetNode : public ASTNode {
  public:
  std::string op;
  std::unique_ptr<ASTNode> left;
  std::unique_ptr<ASTNode> right;
  SetNode(const std::string& op, std::unique_ptr<ASTNode> left, std::unique_ptr<ASTNode> right) : op(op), left(std::move(left)), right(std::move(right)) {}
};

class DelNode : public ASTNode {
  public:
  std::string name;
  DelNode(const std::string& name) : name(name) {}
};

class IEENode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> if_condition;
  std::vector<std::unique_ptr<ASTNode>> if_body;
  std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> elifs;
  std::vector<std::unique_ptr<ASTNode>> else_body;
  IEENode(std::unique_ptr<ASTNode> if_condition, std::vector<std::unique_ptr<ASTNode>> if_body, std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> elifs, std::vector<std::unique_ptr<ASTNode>> else_body) : if_condition(std::move(if_condition)), if_body(std::move(if_body)), elifs(std::move(elifs)), else_body(std::move(else_body)) {}
};

class ForNode : public ASTNode {
  public:
  // std::unique_ptr<ASTNode> init;
  std::string init;
  std::unique_ptr<ASTNode> condition;
  std::unique_ptr<ASTNode> increment;
  std::vector<std::unique_ptr<ASTNode>> body;
  ForNode(const std::string& init, std::unique_ptr<ASTNode> condition, std::unique_ptr<ASTNode> increment, std::vector<std::unique_ptr<ASTNode>> body) : init(init), condition(std::move(condition)), increment(std::move(increment)), body(std::move(body)) {}
};

class WhileNode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> condition;
  std::vector<std::unique_ptr<ASTNode>> body;
  WhileNode(std::unique_ptr<ASTNode> condition, std::vector<std::unique_ptr<ASTNode>> body) : condition(std::move(condition)), body(std::move(body)) {}
};

class BreakNode : public ASTNode {
  public:
  BreakNode() {}
};

class ContinueNode : public ASTNode {
  public:
  ContinueNode() {}
};

class FuncNode : public ASTNode {
  public:
  std::string name;
  Func_Types type;
  // vector of (arg, type) pairs
  std::vector<std::pair<std::string, Var_Types>> args;
  std::vector<std::unique_ptr<ASTNode>> body;
  FuncNode(const std::string& name, Func_Types type, std::vector<std::pair<std::string, Var_Types>> args, std::vector<std::unique_ptr<ASTNode>> body) : name(name), type(type), args(args), body(std::move(body)) {}
};

class CallNode : public ASTNode {
  public:
  std::string name;
  std::vector<std::unique_ptr<ASTNode>> args;
  CallNode(const std::string& name, std::vector<std::unique_ptr<ASTNode>> args) : name(name), args(std::move(args)) {}
};

class ReturnNode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> value;
  ReturnNode(std::unique_ptr<ASTNode> value) : value(std::move(value)) {}
};

class ExitNode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> value;
  ExitNode(std::unique_ptr<ASTNode> value) : value(std::move(value)) {}
};

// Switch-Case-Default Node
/*
switch (expr) {
  case expr {
    body
    body
    ...
  }
  case expr {
    body
    body
    ...
  }
  ...
  default {
    body
    body
    ...
  }
}
*/
class SCDNode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> value;
  std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> cases;
  std::vector<std::unique_ptr<ASTNode>> default_body;
  SCDNode(std::unique_ptr<ASTNode> value, std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> cases, std::vector<std::unique_ptr<ASTNode>> default_body) : value(std::move(value)), cases(std::move(cases)), default_body(std::move(default_body)) {}
};