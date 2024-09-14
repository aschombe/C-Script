/*
Precedence (highest (0) to lowest (9)):
0: Function Calls, Parentheses, List Indexing (not implemented), Member Access (not implemented)
1: Unary Operators (Negation, Not)
2: Exponentiation
3: Multiplication, Division, Modulus
4: Addition, Subtraction
5: Comparison (<, <=, >, >=)
6: Equality (==, !=)
7: Logical And
8: Logical Or
9: Assignment (=, +=, -=, *=, /=, %=, ^=)
Heres my pest file:
// Tokens
Integer = @{ ASCII_DIGIT+ }
Float = @{ Integer ~ ("." ~ ASCII_DIGIT*)? }
String = { "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
Bool = { "true" | "false" }
ident = @{ ASCII_ALPHA ~ (ASCII_ALPHA | ASCII_DIGIT)* }
type = { "int" | "Float" | "string" | "bool" }

bin_ops = { add | sub | mul | div | modulus | pow | and | or | eq | neq | gt | lt | gte | lte }
add     =  { "+" }
sub     =  { "-" }
mul     =  { "*" }
div     =  { "/" }
modulus =  { "%" }
pow     =  { "^" }
and     =  { "&&" }
or      =  { "||" }
eq      =  { "==" }
neq     =  { "!=" }
gt      =  { ">" }
gte     =  { ">=" }
lte     =  { "<=" }
lt      =  { "<" }

un_ops = { neg | not }
neg = { "-" }
not = { "!" }

assignment = { eq_assign | add_eq | sub_eq | mul_eq | div_eq | mod_eq | pow_eq }
eq_assign = { "=" }
add_eq = { "+=" }
sub_eq = { "-=" }
mul_eq = { "*=" }
div_eq = { "/=" }
mod_eq = { "%=" }
pow_eq = { "^=" }

// Statements
let_stmt = { "let" ~ ident ~ ":" ~ type ~ "=" ~ expr ~ ";" }
set_stmt = { ident ~ "=" ~ expr ~ ";" }
del_stmt = { "del" ~ ident ~ ";" }
var_ref = { ident }

// Literals
int = { Integer }
float = { Float }
string = { String }
bool = { Bool }

// Expressions

// Base expressions
factor = _{ int | float | string | bool | "(" ~ expr ~ ")" | var_ref | func_call }

// Unary Operators (Precedence 1)
unary_expr = _{ (neg | not) ~ factor }

// Exponentiation (Precedence 2)
exponent_expr = _{ unary_expr ~ (pow ~ unary_expr)* }

// Multiplication, Division, Modulus (Precedence 3)
term = _{ exponent_expr ~ (mul | div | modulus) ~ exponent_expr }

// Addition, Subtraction (Precedence 4)
expression = _{ term ~ (add | sub) ~ term }

// Comparison (Precedence 5)
comparison_expr = _{ expression ~ (lt | lte | gt | gte) ~ expression }

// Equality (Precedence 6)
equality_expr = _{ comparison_expr ~ (eq | neq) ~ comparison_expr }

// Logical And (Precedence 7)
and_expr = _{ equality_expr ~ (and ~ equality_expr)* }

// Logical Or (Precedence 8)
or_expr = _{ and_expr ~ (or ~ and_expr)* }

// Assignment (Precedence 9)
assignment_expr = _{ or_expr ~ (eq_assign | add_eq | sub_eq | mul_eq | div_eq | mod_eq | pow_eq) ~ or_expr }

// Top-Level Expression
expr = _{ assignment_expr | func_call | "(" ~ expr ~ ")" }

// Control Flow
iee = { "if" ~ "(" ~ expr ~ ")" ~ "{" ~ stmt* ~ "}" ~ elif* ~ else_? }
elif = { "elif" ~ "(" ~ expr ~ ")" ~ "{" ~ stmt* ~ "}" }
else_ = { "else" ~ "{" ~ stmt* ~ "}" }

switch = { "switch" ~ "(" ~ expr ~ ")" ~ "{" ~ case* ~ default? ~ "}" }
case = { "case" ~ expr ~ ":" ~ "{" ~ stmt* ~ "}" }
default = { "default" ~ ":" ~ "{" ~ stmt* ~ "}" }

// Loops
for_loop = { "for" ~ "(" ~ stmt ~ ";" ~ expr ~ ";" ~ stmt ~ ")" ~ "{" ~ stmt* ~ "}" }
while_loop = { "while" ~ "(" ~ expr ~ ")" ~ "{" ~ stmt* ~ "}" }
continue_stmt = { "continue" ~ ";" }
break_stmt = { "break" ~ ";" }

// Functions
func_def = { "fn" ~ ident ~ "(" ~ params ~ ")" ~ ":" ~ type ~ "{" ~ stmt* ~ "}" }
params = { (ident ~ ":" ~ type) ~ ("," ~ ident ~ ":" ~ type)* }
func_call = { ident ~ "(" ~ exprs ~ ")" }
exprs = { expr ~ ("," ~ expr)* }
return_stmt = { "return" ~ expr ~ ";" }

// Statements
stmt = _{ let_stmt | set_stmt | del_stmt | expr | iee | switch | for_loop | while_loop | continue_stmt | break_stmt | func_def | func_call | return_stmt }
program = _{ stmt* }

// Whitespace
WHITESPACE = _{ " " | "\n" | "\t" | ("//" ~ (!"\n" ~ ANY)*) | ("/*" ~ (!"*/" ~ ANY)* ~ "*\/") }
*/

use pest_derive::Parser;
use pest::Parser;
use pest::iterators::Pairs;
use pest::error::Error;

use crate::ast::ASTNode;
use crate::types::Type;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RssParser;

pub fn parse(expression: &str) -> Vec<ASTNode> {

    let mut ast: Vec<ASTNode> = Vec::new();
    let pairs: Result<Pairs<'_, Rule>, Error<Rule>> = RssParser::parse(Rule::stmt, expression);
    // match pairs {
    //     Ok(pairs) => {
    //         for pair in pairs {
    //             println!("{:?}", pair);
    //         }
    //     },
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // }
    // lets actually generate AST now based on the pair rules
    match pairs {
        Ok(pairs) => {
            for pair in pairs {
                match pair.as_rule() {
                    Rule::WHITESPACE => {},
                    Rule::let_stmt => {
                        let mut children: Pairs<'_, Rule> = pair.into_inner();
                        let ident: &str = children.next().unwrap().as_str();
                        let type_: &str = children.next().unwrap().as_str();
                        let expr: &str = children.next().unwrap().as_str();
                        ast.push(ASTNode::Let(ident.to_string(), *Box::new(Type::to_type(type_)), Box::new(ASTNode::Int(expr.parse().unwrap()))));
                    },
                    Rule::set_stmt => {
                        let mut children: Pairs<'_, Rule> = pair.into_inner();
                        let ident: &str = children.next().unwrap().as_str();
                        let expr: &str = children.next().unwrap().as_str();
                        ast.push(ASTNode::Set(ident.to_string(), Box::new(ASTNode::Int(expr.parse::<i64>().unwrap()))));
                    },
                    Rule::del_stmt => {
                        let ident: &str = pair.into_inner().next().unwrap().as_str();
                        ast.push(ASTNode::Del(ident.to_string()));
                    },
                    // Rule::var_ref => {
                    //     let ident: &str = pair.as_str();
                    //     ast.push(ASTNode::VarRef(ident.to_string()));
                    // },
                    // Rule::int => {
                    //     let int: i64 = pair.as_str().parse::<i64>().unwrap();
                    //     ast.push(ASTNode::Int(int));
                    // },
                    // Rule::float => {
                    //     let float: f64 = pair.as_str().parse::<f64>().unwrap();
                    //     ast.push(ASTNode::Float(float));
                    // },
                    // Rule::string => {
                    //     let string: &str = pair.as_str();
                    //     ast.push(ASTNode::String(string.to_string()));
                    // },
                    // Rule::Bool => {
                    //     let bool_: bool = pair.as_str().parse::<bool>().unwrap();
                    //     ast.push(ASTNode::Boolean(bool_));
                    // },
                    Rule::add | Rule::sub | Rule::mul | Rule::div | Rule::modulus | Rule::pow | Rule::and | Rule::or | Rule::eq | Rule::neq | Rule::gt | Rule::lt => {
                        let mut children: Pairs<'_, Rule> = pair.clone().into_inner();
                        let left: ASTNode = parse(children.next().unwrap().as_str())[0].clone();
                        let right: ASTNode = parse(children.next().unwrap().as_str())[0].clone();
                        match pair.as_rule() {
                            Rule::add => {
                                ast.push(ASTNode::Add(Box::new(left), Box::new(right)));
                            },
                            Rule::sub => {
                                ast.push(ASTNode::Sub(Box::new(left), Box::new(right)));
                            },
                            Rule::mul => {
                                ast.push(ASTNode::Mul(Box::new(left), Box::new(right)));
                            },
                            Rule::div => {
                                ast.push(ASTNode::Div(Box::new(left), Box::new(right)));
                            },
                            Rule::modulus => {
                                ast.push(ASTNode::Mod(Box::new(left), Box::new(right)));
                            },
                            Rule::pow => {
                                ast.push(ASTNode::Pow(Box::new(left), Box::new(right)));
                            },
                            Rule::and => {
                                ast.push(ASTNode::And(Box::new(left), Box::new(right)));
                            },
                            Rule::or => {
                                ast.push(ASTNode::Or(Box::new(left), Box::new(right)));
                            },
                            Rule::eq => {
                                ast.push(ASTNode::Eq(Box::new(left), Box::new(right)));
                            },
                            Rule::neq => {
                                ast.push(ASTNode::Ne(Box::new(left), Box::new(right)));
                            },
                            Rule::gt => {
                                ast.push(ASTNode::Gt(Box::new(left), Box::new(right)));
                            },
                            Rule::lt => {
                                ast.push(ASTNode::Lt(Box::new(left), Box::new(right)));
                            },
                            _ => {
                                println!("Rule not implemented yet");
                            }
                        }
                    },
                    _ => {
                        println!("Rule not implemented yet");
                    }
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
    ast
}