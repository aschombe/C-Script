#pragma once

#include "types.hpp"
#include "ast.hpp"
#include <vector>
#include <string>
#include <variant>
#include <unordered_map>
#include <memory>

typedef std::variant<int, float, bool, std::string> Value;
typedef std::pair<std::string, Value> Variable;
typedef std::pair<std::string, Var_Types> Argument;

struct Function {
  std::string name;
  std::vector<Argument> args;
  std::vector<std::shared_ptr<ASTNode>> body;

  Function(const std::string& name,
           const std::vector<Argument>& args,
           const std::vector<std::shared_ptr<ASTNode>>& body)       
    : name(name), args(args), body(body) {}
};

typedef std::variant<Variable, Function> Scope_Item;
typedef std::unordered_map<std::string, Scope_Item> ScopeMap;

class Scope {
  public:
  Scope();
  ~Scope();
  void push_scope();
  void pop_scope();
  void add_variable(const std::string& name, const Value& value);
  void add_function(const std::string& name, const std::vector<Argument>& args, const std::vector<std::shared_ptr<ASTNode>>& body);

  Value get_variable(const std::string& name);
  Function get_function(const std::string& name);

  bool variable_exists(const std::string& name);
  bool function_exists(const std::string& name);

  void set_variable(const std::string& name, const Value& value);

  unsigned int get_scope_level();
  
  private:
  unsigned int scope_level = 0;
  std::vector<ScopeMap> scopes;
};
