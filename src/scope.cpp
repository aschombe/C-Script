#include "../include/scope.hpp"

/*
typedef std::variant<int, float, bool, std::string> Value;
typedef std::pair<std::string, Value> Variable;
typedef std::pair<std::string, Var_Types> Argument;
typedef std::pair<std::string, std::vector<Argument>> Function;
typedef std::variant<Variable, Function> Scope_Item;
typedef std::unordered_map<std::string, Scope_Item> Scope;

class Scope {
    public:
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

    void print_scopes();
    void print_scope();
    void print_scope_item();

    private:
    std::vector<Scope> scopes;
};
*/

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