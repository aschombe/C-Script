#include "../include/scope.hpp"

void push_scope(std::vector<Scope>& scopes) {
    scopes.push_back(Scope());
}

void pop_scope(std::vector<Scope>& scopes) {
    scopes.pop_back();
}

void add_variable(std::vector<Scope>& scopes, const std::string& name, const Value& value) {
    if (scopes.empty()) {
        std::cerr << "Error: no scope to add variable to" << std::endl;
        return;
    }

    scopes.back()[name] = Variable(name, value);
}

void add_function(std::vector<Scope>& scopes, const std::string& name, const std::vector<Argument>& args, const std::vector<std::shared_ptr<ASTNode>>& body) {
    if (scopes.empty()) {
        std::cerr << "Error: no scope to add function to" << std::endl;
        return;
    }

    scopes.back()[name] = Function(name, args, body);
}

Value get_variable(const std::vector<Scope>& scopes, const std::string& name) {
    for (auto it = scopes.rbegin(); it != scopes.rend(); it++) {
        auto scope = *it;
        auto item = scope.find(name);
        if (item != scope.end()) {
            return std::get<Variable>(item->second).second;
        }
    }

    std::cerr << "Error: variable " << name << " not found" << std::endl;
    return Value();
}

Function get_function(const std::vector<Scope>& scopes, const std::string& name) {
    for (auto it = scopes.rbegin(); it != scopes.rend(); it++) {
        auto scope = *it;
        auto item = scope.find(name);
        if (item != scope.end()) {
            return std::get<Function>(item->second);
        }
    }

    std::cerr << "Error: function " << name << " not found" << std::endl;
    return Function();
}

bool variable_exists(const std::vector<Scope>& scopes, const std::string& name) {
    for (auto it = scopes.rbegin(); it != scopes.rend(); it++) {
        auto scope = *it;
        auto item = scope.find(name);
        if (item != scope.end()) {
            return true;
        }
    }

    return false;
}

bool function_exists(const std::vector<Scope>& scopes, const std::string& name) {
    for (auto it = scopes.rbegin(); it != scopes.rend(); it++) {
        auto scope = *it;
        auto item = scope.find(name);
        if (item != scope.end()) {
            return true;
        }
    }

    return false;
}

void set_variable(std::vector<Scope>& scopes, const std::string& name, const Value& value) {
    for (auto it = scopes.rbegin(); it != scopes.rend(); it++) {
        auto scope = *it;
        auto item = scope.find(name);
        if (item != scope.end()) {
            std::get<Variable>(item->second).second = value;
            return;
        }
    }

    std::cerr << "Error: variable " << name << " not found" << std::endl;
}