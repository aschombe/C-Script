#pragma once

#include <string>
#include <vector>
#include <memory>
#include <variant>
#include "types.hpp"

typedef std::variant<int, double, bool, std::string> Value;

class ASTNode {
  public:
  Value value;
  int line;
  int col;
  virtual ~ASTNode() = default;
  virtual int node_type() const = 0;

  virtual std::string to_string() const = 0;
};

class IntNode : public ASTNode {
  public:
  int value;
  // line and col here should be the line and col that IntNode inherits from ASTNode
  IntNode(int value, int line, int col) : value(value) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 0;
  }
  
  std::string to_string() const override {
    return "Int(" + std::to_string(value) + ")";
  }
};

class DoubleNode : public ASTNode {
  public:
  double value;
  DoubleNode(double value, int line, int col) : value(value) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 1;
  }

  std::string to_string() const override {
    return "Double(" + std::to_string(value) + ")";
  }
};

class StringNode : public ASTNode {
  public:
  std::string value;
  StringNode(const std::string& value, int line, int col) : value(value) {
    this->line = line;
    this->col = col;
  }
    
  int node_type() const override {
    return 2;
  }

  std::string to_string() const override {
    return "String(" + value + ")";
  }
};

class BoolNode : public ASTNode {
  public:
  bool value;
  BoolNode(bool value, int line, int col) : value(value) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 3;
  }

  std::string to_string() const override {
    return "Bool(" + std::string(value ? "true" : "false") + ")";
  }
};

class VariableNode : public ASTNode {
  public:
  std::string name;
  VariableNode(const std::string& name, int line, int col) : name(name) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 4;
  }

  std::string to_string() const override {
    return "Var(" + name + ")";
  }
};

class BinOpNode : public ASTNode {
  public:
  std::string op;
  std::unique_ptr<ASTNode> left;
  std::unique_ptr<ASTNode> right;
  BinOpNode(const std::string& op, std::unique_ptr<ASTNode> left, std::unique_ptr<ASTNode> right, int line, int col) : op(op), left(std::move(left)), right(std::move(right)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 5;
  }

  std::string to_string() const override {
    return "BinOp(" + op + ", " + left->to_string() + ", " + right->to_string() + ")";
  }
};

class UnaryOpNode : public ASTNode {
  public:
  std::string op;
  std::unique_ptr<ASTNode> expr;
  /* std::variant<std::unique_ptr<ASTNode>, std::string> expr; */
  UnaryOpNode(const std::string& op, std::unique_ptr<ASTNode> expr, int line, int col) : op(op), expr(std::move(expr)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 6;
  }

  std::string to_string() const override {
    return "UnaryOp(" + op + ", " + expr->to_string() + ")";
  }
};

class LetNode : public ASTNode {
  public:
  std::string name;
  Var_Types type;
  std::unique_ptr<ASTNode> value;
  LetNode(const std::string& name, Var_Types type, std::unique_ptr<ASTNode> value, int line, int col) : name(name), type(type), value(std::move(value)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 7;
  }

  std::string to_string() const override {
    return "Let(" + name + ", " + var_type_to_string(type) + ", " + value->to_string() + ")";
  }
};

class SetNode : public ASTNode {
  public:
  std::string op;
  std::string ident;
  std::unique_ptr<ASTNode> right;
  SetNode(const std::string& op, const std::string& ident, std::unique_ptr<ASTNode> right, int line, int col) : op(op), ident(ident), right(std::move(right)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 8;
  }

  std::string to_string() const override {
    return "Set(" + op + ", " + ident + ", " + right->to_string() + ")";
  }
};

class DelNode : public ASTNode {
  public:
  std::string name;
  DelNode(const std::string& name, int line, int col) : name(name) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 9;
  }

  std::string to_string() const override {
    return "Del(" + name + ")";
  }
};

class IEENode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> if_condition;
  std::vector<std::unique_ptr<ASTNode>> if_body;
  std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> elifs;
  std::vector<std::unique_ptr<ASTNode>> else_body;
  IEENode(std::unique_ptr<ASTNode> if_condition, std::vector<std::unique_ptr<ASTNode>> if_body, std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> elifs, std::vector<std::unique_ptr<ASTNode>> else_body, int line, int col) : if_condition(std::move(if_condition)), if_body(std::move(if_body)), elifs(std::move(elifs)), else_body(std::move(else_body)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 10;
  }

  std::string to_string() const override {
    std::string result = "IEE(" + if_condition->to_string() + ", [";
    for (size_t i = 0; i < if_body.size(); i++) {
      result += if_body[i]->to_string();
      if (i < if_body.size() - 1) {
        result += ", ";
      }
    }
    result += "], [";
    for (size_t i = 0; i < elifs.size(); i++) {
      result += "(" + elifs[i].first->to_string() + ", [";
      for (size_t j = 0; j < elifs[i].second.size(); j++) {
        result += elifs[i].second[j]->to_string();
        if (j < elifs[i].second.size() - 1) {
          result += ", ";
        }
      }
      result += "])";
      if (i < elifs.size() - 1) {
        result += ", ";
      }
    }
    result += "], [";
    for (size_t i = 0; i < else_body.size(); i++) {
      result += else_body[i]->to_string();
      if (i < else_body.size() - 1) {
        result += ", ";
      }
    }
    result += "])";
    return result;
  }
};

class ForNode : public ASTNode {
  public:
  std::string init;
  std::unique_ptr<ASTNode> condition;
  std::unique_ptr<ASTNode> increment;
  std::vector<std::unique_ptr<ASTNode>> body;
  ForNode(const std::string& init, std::unique_ptr<ASTNode> condition, std::unique_ptr<ASTNode> increment, std::vector<std::unique_ptr<ASTNode>> body, int line, int col) : init(init), condition(std::move(condition)), increment(std::move(increment)), body(std::move(body)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 11;
  }

  std::string to_string() const override {
    std::string result = "For(" + init + ", " + condition->to_string() + ", " + increment->to_string() + ", [";
    for (size_t i = 0; i < body.size(); i++) {
      result += body[i]->to_string();
      if (i < body.size() - 1) {
        result += ", ";
      }
    }
    result += "])";
    return result;
  }
};

class WhileNode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> condition;
  std::vector<std::unique_ptr<ASTNode>> body;
  WhileNode(std::unique_ptr<ASTNode> condition, std::vector<std::unique_ptr<ASTNode>> body, int line, int col) : condition(std::move(condition)), body(std::move(body)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 12;
  }

  std::string to_string() const override {
    std::string result = "While(" + condition->to_string() + ", [";
    for (size_t i = 0; i < body.size(); i++) {
      result += body[i]->to_string();
      if (i < body.size() - 1) {
        result += ", ";
      }
    }
    result += "])";
    return result;
  }
};

class BreakNode : public ASTNode {
  public:
  BreakNode(int line, int col) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 13;
  }

  std::string to_string() const override {
    return "Break()";
  }
};

class ContinueNode : public ASTNode {
  public:
  ContinueNode(int line, int col) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 14;
  }

  std::string to_string() const override {
    return "Continue()";
  }
};

class FuncNode : public ASTNode {
  public:
  std::string name;
  Func_Types type;
  // vector of (arg, type) pairs
  std::vector<std::pair<std::string, Var_Types>> args;
  std::vector<std::unique_ptr<ASTNode>> body;
  FuncNode(const std::string& name, Func_Types type, std::vector<std::pair<std::string, Var_Types>> args, std::vector<std::unique_ptr<ASTNode>> body, int line, int col) : name(name), type(type), args(args), body(std::move(body)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 15;
  }

  std::string to_string() const override {
    std::string result = "Func(" + name + ", " + func_type_to_string(type) + ", [";
    for (size_t i = 0; i < args.size(); i++) {
      result += "(" + args[i].first + ", " + var_type_to_string(args[i].second) + ")";
      if (i < args.size() - 1) {
        result += ", ";
      }
    }
    result += "], [";
    for (size_t i = 0; i < body.size(); i++) {
      result += body[i]->to_string();
      if (i < body.size() - 1) {
        result += ", ";
      }
    }
    result += "])";
    return result;
  }
};

class CallNode : public ASTNode {
  public:
  std::string name;
  std::vector<std::unique_ptr<ASTNode>> args;
  CallNode(const std::string& name, std::vector<std::unique_ptr<ASTNode>> args, int line, int col) : name(name), args(std::move(args)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 16;
  }

  std::string to_string() const override {
    std::string result = "Call(" + name + ", [";
    for (size_t i = 0; i < args.size(); i++) {
      result += args[i]->to_string();
      if (i < args.size() - 1) {
        result += ", ";
      }
    }
    result += "])";
    return result;
  }
};

class ReturnNode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> value;
  ReturnNode(std::unique_ptr<ASTNode> value, int line, int col) : value(std::move(value)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 17;
  }

  std::string to_string() const override {
    return "Return(" + value->to_string() + ")";
  }
};

class ExitNode : public ASTNode {
  public:
  std::unique_ptr<ASTNode> value;
  ExitNode(std::unique_ptr<ASTNode> value, int line, int col) : value(std::move(value)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 18;
  }

  std::string to_string() const override {
    return "Exit(" + value->to_string() + ")";
  }
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
  SCDNode(std::unique_ptr<ASTNode> value, std::vector<std::pair<std::unique_ptr<ASTNode>, std::vector<std::unique_ptr<ASTNode>>>> cases, std::vector<std::unique_ptr<ASTNode>> default_body, int line, int col) : value(std::move(value)), cases(std::move(cases)), default_body(std::move(default_body)) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 19;
  }

  std::string to_string() const override {
    std::string result = "SCD(" + value->to_string() + ", [";
    for (size_t i = 0; i < cases.size(); i++) {
      result += "(" + cases[i].first->to_string() + ", [";
      for (size_t j = 0; j < cases[i].second.size(); j++) {
        result += cases[i].second[j]->to_string();
        if (j < cases[i].second.size() - 1) {
          result += ", ";
        }
      }
      result += "])";
      if (i < cases.size() - 1) {
        result += ", ";
      }
    }
    result += "], [";
    for (size_t i = 0; i < default_body.size(); i++) {
      result += default_body[i]->to_string();
      if (i < default_body.size() - 1) {
        result += ", ";
      }
    }
    result += "])";
    return result;
  }
};

class ImportNode : public ASTNode {
  public:
  std::string value;
  ImportNode(std::string rel_fpath, int line, int col) : value(rel_fpath) {
    this->line = line;
    this->col = col;
  }

  int node_type() const override {
    return 20;
  }

  std::string to_string() const override {
    return "Import(" + value + ")";
  }
};

void print_ast(const std::vector<std::unique_ptr<ASTNode>>& ast);
