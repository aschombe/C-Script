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

    bool count(const std::string& name) const;

    private:
    std::vector<Scope> scopes;
};