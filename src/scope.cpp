#include "../include/scope.hpp"

void push_scope(std::vector<Scope>& scopes) {
  scopes.push_back(Scope());
}

void pop_scope(std::vector<Scope>& scopes) {
  scopes.pop_back();
}

void add_variable(std::vector<Scope>& scopes, const std::string& name, const Value& value) {
  scopes.back()[name] = Variable(name, value);
}

void add_function(std::vector<Scope>& scopes, const std::string& name, const std::vector<Argument>& args, const std::vector<std::shared_ptr<ASTNode>>& body) {
  scopes.back()[name] = Function(name, args, body);
}

bool count(const std::vector<Scope>& scopes, const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; --i) {
    if (scopes[i].count(name)) {
      return true;
    }
  }

  return false;
}

Value get_variable(std::vector<Scope>& scopes, const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; --i) {
    if (scopes[i].count(name)) {
      return std::get<Variable>(scopes[i][name]).second;
    }
  }

  throw std::runtime_error("Error: variable " + name + " does not exist");
}

Function get_function(std::vector<Scope>& scopes, const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; --i) {
    if (scopes[i].count(name)) {
      return std::get<Function>(scopes[i][name]);
    }
  }

  throw std::runtime_error("Error: function " + name + " does not exist");
}

bool variable_exists(std::vector<Scope>& scopes, const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; --i) {
    if (scopes[i].count(name)) {
      return true;
    }
  }

  return false;
}

bool function_exists(std::vector<Scope>& scopes, const std::string& name) {
  for (int i = scopes.size() - 1; i >= 0; --i) {
    if (scopes[i].count(name)) {
      return true;
    }
  }

  return false;
}

void set_variable(std::vector<Scope>& scopes, const std::string& name, const Value& value) {
  for (int i = scopes.size() - 1; i >= 0; --i) {
    if (scopes[i].count(name)) {
      std::get<Variable>(scopes[i][name]).second = value;
      return;
    }
  }

  throw std::runtime_error("Error: variable " + name + " does not exist");
}

void set_function(std::vector<Scope>& scopes, const std::string& name, const std::vector<Argument>& args, const std::vector<std::shared_ptr<ASTNode>>& body) {
  for (int i = scopes.size() - 1; i >= 0; --i) {
    if (scopes[i].count(name)) {
      std::get<Function>(scopes[i][name]) = Function(name, args, body);
      return;
    }
  }

  throw std::runtime_error("Error: function " + name + " does not exist");
}
