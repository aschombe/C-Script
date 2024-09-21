#include "../include/scope.hpp"

Scope::Scope() {
  scopes.push_back(ScopeMap());
}

Scope::~Scope() {
  scopes.clear();
}

void Scope::push_scope() {
  scopes.push_back(ScopeMap());
  this->scope_level++;
}

void Scope::pop_scope() {
  scopes.pop_back();
  this->scope_level--;
}

void Scope::add_variable(const std::string& name, const Value& value) {
  scopes.back()[name] = Variable(name, value);
}

void Scope::add_function(const std::string& name, const std::vector<Argument>& args, const std::vector<std::shared_ptr<ASTNode>>& body) {
  scopes.back()[name] = Function(name, args, body);
}

Value Scope::get_variable(const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; i--) {
    if (scopes[i].find(name) != scopes[i].end()) {
      return std::get<Variable>(scopes[i][name]).second;
    }
  }
  throw std::runtime_error("Variable " + name + " not found");
}

Function Scope::get_function(const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; i--) {
    if (scopes[i].find(name) != scopes[i].end()) {
      return std::get<Function>(scopes[i][name]);
    }
  }
  throw std::runtime_error("Function " + name + " not found");
}

bool Scope::variable_exists(const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; i--) {
    if (scopes[i].find(name) != scopes[i].end()) {
      return true;
    }
  }
  return false;
}

bool Scope::function_exists(const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; i--) {
    if (scopes[i].find(name) != scopes[i].end()) {
      return true;
    }
  }
  return false;
}

void Scope::set_variable(const std::string& name, const Value& value) {
  for (int i = scopes.size() - 1; i >= 0; i--) {
    if (scopes[i].find(name) != scopes[i].end()) {
      scopes[i][name] = Variable(name, value);
      return;
    }
  }
  throw std::runtime_error("Variable " + name + " not found");
}

unsigned int Scope::get_scope_level() {
  return this->scope_level;
}
