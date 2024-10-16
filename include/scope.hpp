#pragma once

#include "types.hpp"
#include "ast.hpp"
#include <vector>
#include <string>
#include <variant>
#include <unordered_map>
#include <memory>

typedef std::pair<std::string, Value> Variable;
typedef std::pair<std::string, Var_Types> Argument;

struct Function {
  std::string name;
  std::vector<Argument> args;
  // std::vector<std::shared_ptr<ASTNode>> body;
  std::vector<ASTNode*> body;

  // Function(const std::string& name,
  //          const std::vector<Argument>& args,
  //          const std::vector<std::shared_ptr<ASTNode>>& body)       
  //   : name(name), args(args), body(body) {}

  Function(const std::string& name,
           const std::vector<Argument>& args,
           const std::vector<ASTNode*>& body)
    : name(name), args(args), body(body) {}
};

struct Struct {
  std::string name;
  std::unordered_map<std::string, Var_Types> members;

  Struct(const std::string& name, const std::unordered_map<std::string, Var_Types>& members)
    : name(name), members(members) {}

  bool member_exists(const std::string& name) {
    return members.find(name) != members.end();
  }
};

typedef std::variant<Variable, Function, Struct> Scope_Item;
typedef std::unordered_map<std::string, Scope_Item> ScopeMap;

class Scope {
  public:
  Scope();
  ~Scope();
  void push_scope();
  void pop_scope();
  void add_variable(const std::string& name, const Value& value);
  // void add_function(const std::string& name, const std::vector<Argument>& args, const std::vector<std::shared_ptr<ASTNode>>& body);
  void add_function(const std::string& name, const std::vector<Argument>& args, const std::vector<ASTNode*>& body);
  void add_struct(const std::string& name, const std::unordered_map<std::string, Var_Types>& members);

  Value get_variable(const std::string& name);
  Function get_function(const std::string& name);
  /* Struct get_struct(const std::string& name); */

  bool variable_exists(const std::string& name);
  bool function_exists(const std::string& name);
  bool struct_exists(const std::string& name);

  void set_variable(const std::string& name, const Value& value);
  void delete_variable(const std::string& name);

  unsigned int get_scope_level();

  void print_scope();
  
  private:
  unsigned int scope_level = 0;
  std::vector<ScopeMap> scopes;
};
