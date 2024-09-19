#include "../include/scope.hpp"

/*
typedef std::variant<int, float, bool, std::string> Value;
typedef std::pair<std::string, Value> Variable;
typedef std::pair<std::string, Var_Types> Argument;
typedef std::pair<std::string, std::vector<Argument>> Function;
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
  void set_function(const std::string& name, const std::vector<Argument>& args, const std::vector<std::shared_ptr<ASTNode>>& body);

  int get_scope_level();
  

  private:
  int scope_level = 0;
  std::vector<ScopeMap> scopes;
};
*/

Scope::Scope() {
  scopes.push_back(ScopeMap());
}

Scope::~Scope() {
  scopes.clear();
}

void push_scope(std::vector<Scope>& scopes) {
  scopes.push_back(Scope());
  ++scope_level;
}

void pop_scope(std::vector<Scope>& scopes) {
  scopes.pop_back();
  --scope_level;
}

void add_variable(std::vector<Scope>& scopes, const std::string& name, const Value& value) {
  scopes.back()[name] = Variable(name, value);
}

void add_function(std::vector<Scope>& scopes, const std::string& name, const std::vector<Argument>& args, const std::vector<std::shared_ptr<ASTNode>>& body) {
  scopes.back()[name] = Function(name, args, body);
}

Value get_variable(std::vector<Scope>& scopes, const std::string& name) {
  for (auto it = scopes.rbegin(); it != scopes.rend(); ++it) {
    if (it->count(name)) {
      return std::get<Variable>(it->at(name)).second;
    }
  }
  throw std::runtime_error("Variable " + name + " not found");
}

Function get_function(std::vector<Scope>& scopes, const std::string& name) {
  for (auto it = scopes.rbegin(); it != scopes.rend(); ++it) {
    if (it->count(name)) {
      return std::get<Function>(it->at(name));
    }
  }
  throw std::runtime_error("Function " + name + " not found");
}

bool variable_exists(std::vector<Scope>& scopes, const std::string& name) {
  for (auto it = scopes.rbegin(); it != scopes.rend(); ++it) {
    if (it->count(name)) {
      return true;
    }
  }
  return false;
}

bool function_exists(std::vector<Scope>& scopes, const std::string& name) {
  for (auto it = scopes.rbegin(); it != scopes.rend(); ++it) {
    if (it->count(name)) {
      return true;
    }
  }
  return false;
}

void set_variable(std::vector<Scope>& scopes, const std::string& name, const Value& value) {
  for (auto it = scopes.rbegin(); it != scopes.rend(); ++it) {
    if (it->count(name)) {
      std::get<Variable>(it->at(name)).second = value;
      return;
    }
  }
  throw std::runtime_error("Variable " + name + " not found");
}

void set_function(std::vector<Scope>& scopes, const std::string& name, const std::vector<Argument>& args, const std::vector<std::shared_ptr<ASTNode>>& body) {
  for (auto it = scopes.rbegin(); it != scopes.rend(); ++it) {
    if (it->count(name)) {
      std::get<Function>(it->at(name)) = Function(name, args, body);
      return;
    }
  }
  throw std::runtime_error("Function " + name + " not found");
}

int get_scope_level() {
  return scope_level;
}
