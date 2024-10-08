#include "../include/scope.hpp"
#include "../include/error_handler.hpp"
#include <iostream>

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

void Scope::add_struct(const std::string& name, const std::unordered_map<std::string, Var_Types>& members) {
  scopes.back()[name] = Struct(name, members);
}

Value Scope::get_variable(const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; i--) {
    if (scopes[i].find(name) != scopes[i].end()) {
      return std::get<Variable>(scopes[i][name]).second;
    }
  }
  ErrorHandler error{ErrorType::SEMANTIC, "Variable " + name + " not found", 0, 0, ""};
  throw error;
}

Function Scope::get_function(const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; i--) {
    if (scopes[i].find(name) != scopes[i].end()) {
      return std::get<Function>(scopes[i][name]);
    }
  }
  ErrorHandler error{ErrorType::SEMANTIC, "Function " + name + " not found", 0, 0, ""};
  throw error;
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

bool Scope::struct_exists(const std::string& name) {
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
  ErrorHandler error{ErrorType::SEMANTIC, "Variable " + name + " not found", 0, 0, ""};
  throw error;
}

void Scope::delete_variable(const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; i--) {
    if (scopes[i].find(name) != scopes[i].end()) {
      scopes[i].erase(name);
      return;
    }
  }
  ErrorHandler error{ErrorType::SEMANTIC, "Variable " + name + " not found", 0, 0, ""};
  throw error;
}

void Scope::print_scope() {
  for (int i = scopes.size() - 1; i >= 0; i--) {
    std::cout << "Scope " << i << std::endl;
    for (auto& [name, item] : scopes[i]) {
      if (std::holds_alternative<Variable>(item)) {
        std::cout << "Variable: " << std::get<Variable>(item).first << std::endl;
      } else {
        std::cout << "Function: " << std::get<Function>(item).name << std::endl;
      }
    }
  }
}

unsigned int Scope::get_scope_level() {
  return this->scope_level;
}
