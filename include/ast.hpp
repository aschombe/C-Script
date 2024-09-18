/*
Keywords and symbols in my language:
- keyworks: let, set, del, if, elif, else, for, while, break, continue, return, exit, func, int, float, bool, string, switch, case, default, true, false
- symbols: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^=, (, ), {, }, [, ], ,, :, ;
- comments: //, \/*, *\/ (same as c++)
- identifiers: [a-zA-Z_][a-zA-Z0-9_]*
- int: [0-9]+
- float: [0-9]+\.[0-9]+
- string: "[^"]*"
- bool: true, false
- operators: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=
- precedence:
    0 (highest): function call, scope (())
    1: unary operators (- (negative), ! (not))
    2: exponentiation (^)
    3: multiplication (*, /, %)
    4: addition (+, -)
    5: Comparison1 (<, <=, >, >=)
    6: Comparison2 (==, !=)
    7: Logical AND (&&)
    8: Logical OR (||)
    9 (lowest): assignment (=, +=, -=, *=, /=, %=)
*/


#pragma once

#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <unordered_map>

#include "token.hpp"

// enum class BinOp {
//   ADD, SUB, MUL, DIV, MOD, POW, EQ, NE, LT, LTE, GT, GTE, AND, OR
// };

// enum class AssignOp {
//   ASSIGN, ADD_ASSIGN, SUB_ASSIGN, MUL_ASSIGN, DIV_ASSIGN, MOD_ASSIGN, POW_ASSIGN
// };

// enum class UnOp {
//   NEG, NOT
// };

// enum class VarType {
//   INT, FLOAT, BOOL, STRING
// };

// enum class FuncType {
//   INT, FLOAT, BOOL, STRING, VOID
// };

// class Expr {
//   public:
//     virtual ~Expr() = default;
//     virtual std::string to_string() = 0;
// };

// class Int : public Expr {
//   public:
//     Int(int value);
//     std::string to_string() override;
//   private:
//     int value;
// };

// class Float : public Expr {
//   public:
//     Float(float value);
//     std::string to_string() override;
//   private:
//     float value;
// };

// class Bool : public Expr {
//   public:
//     Bool(bool value);
//     std::string to_string() override;
//   private:
//     bool value;
// };

// class Var : public Expr {
//   public:
//     Var(std::string name);
//     std::string to_string() override;
//   private:
//     std::string name;
// };

// class BinOp : public Expr {
//   public:
//     BinOp(BinOp op, std::unique_ptr<Expr> lhs, std::unique_ptr<Expr> rhs);
//     std::string to_string() override;
//   private:
//     BinOp op;
//     std::unique_ptr<Expr> lhs;
//     std::unique_ptr<Expr> rhs;
// };

// class UnOp : public Expr {
//   public:
//     UnOp(UnOp op, std::unique_ptr<Expr> expr);
//     std::string to_string() override;
//   private:
//     UnOp op;
//     std::unique_ptr<Expr> expr;
// };

// class Let : public Expr {
//   public:
//     Let(std::string name, VarType type, std::unique_ptr<Expr> expr);
//     std::string to_string() override;
//   private:
//     std::string name;
//     VarType type;
//     std::unique_ptr<Expr> expr;
// };

// class Set : public Expr {
//   public:
//     Set(std::string name, AssignOp op, std::unique_ptr<Expr> expr);
//     std::string to_string() override;
//   private:
//     std::string name;
//     AssignOp op;
//     std::unique_ptr<Expr> expr;
// };

// class Del : public Expr {
//   public:
//     Del(std::string name);
//     std::string to_string() override;
//   private:
//     std::string name;
// };

// class IEE : public Expr {
//   public:
//     IEE(std::vector<std::pair<std::unique_ptr<Expr>, std::vector<std::unique_ptr<Expr>>>> cases, std::vector<std::unique_ptr<Expr>> default_case);
//     std::string to_string() override;
//   private:
//     std::vector<std::pair<std::unique_ptr<Expr>, std::vector<std::unique_ptr<Expr>>>> cases;
//     std::vector<std::unique_ptr<Expr>> default_case;
// };

// class FuncDef : public Expr {
//   public:
//     FuncDef(std::string name, std::vector<std::pair<std::string, VarType>> args, FuncType ret, std::vector<std::unique_ptr<Expr>> body);
//     std::string to_string() override;
//   private:
//     std::string name;
//     std::vector<std::pair<std::string, VarType>> args;
//     FuncType ret;
//     std::vector<std::unique_ptr<Expr>> body;
// };

// class Call : public Expr {
//   public:
//     Call(std::string name, std::vector<std::unique_ptr<Expr>> args);
//     std::string to_string() override;
//   private:
//     std::string name;
//     std::vector<std::unique_ptr<Expr>> args;
// };

// class Switch : public Expr {
//   public:
//     Switch(std::unique_ptr<Expr> expr, std::vector<std::pair<std::unique_ptr<Expr>, std::unique_ptr<Expr>> cases, std::unique_ptr<Expr> default_case);
//     std::string to_string() override;
//   private:
//     std::unique_ptr<Expr> expr;
//     std::vector<std::pair<std::unique_ptr<Expr>, std::unique_ptr<Expr>> cases;
//     std::unique_ptr<Expr> default_case;
// };

// class For : public Expr {
//   public:
//     For(std::unique_ptr<Expr> init, std::unique_ptr<Expr> cond, std::unique_ptr<Expr> update, std::unique_ptr<Expr> body);
//     std::string to_string() override;
//   private:
//     std::unique_ptr<Expr> init;
//     std::unique_ptr<Expr> cond;
//     std::unique_ptr<Expr> update;
//     std::unique_ptr<Expr> body;
// };

// class While : public Expr {
//   public:
//     While(std::unique_ptr<Expr> cond, std::unique_ptr<Expr> body);
//     std::string to_string() override;
//   private:
//     std::unique_ptr<Expr> cond;
//     std::unique_ptr<Expr> body;
// };

// class Break : public Expr {
//   public:
//     Break();
//     std::string to_string() override;
// };

// class Continue : public Expr {
//   public:
//     Continue();
//     std::string to_string() override;
// };

// class Return : public Expr {
//   public:
//     Return(std::unique_ptr<Expr> expr);
//     std::string to_string() override;
//   private:
//     std::unique_ptr<Expr> expr;
// };

// class Exit : public Expr {
//   public:
//     Exit(std::unique_ptr<Expr> expr);
//     std::string to_string() override;
//   private:
//     std::unique_ptr<Expr> expr;
// };

// class AST {
//   public:
//     AST(std::vector<std::unique_ptr<Expr>> exprs);
//     std::string to_string();
//   private:
//     std::vector<std::unique_ptr<Expr>> exprs;
// };