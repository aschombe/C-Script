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
// import the pratt parser
// use pest::{iterators::Pairs, pratt_parser::{Assoc, Op, PrattParser}};
// use lazy_static::lazy_static;

use crate::ast::ASTNode;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RssParser;

// lazy_static! {
//     static ref PRATT_PARSER: PrattParser<Rule> = {
//         use Rule::*;
//         use Assoc::*;

//         PrattParser::new()
//             .op(Op::infix(add, Left) | Op::infix(sub, Left))
//             .op(Op::infix(mul, Left) | Op::infix(div, Left))
//             .op(Op::infix(modulus, Left))
//             .op(Op::infix(pow, Right))
//             .op(Op::infix(eq, Left) | Op::infix(neq, Left))
//             .op(Op::infix(gt, Left) | Op::infix(gte, Left) | Op::infix(lt, Left) | Op::infix(lte, Left))
//             .op(Op::infix(and, Left))
//             .op(Op::infix(or, Left))
//             .op(Op::prefix(neg) | Op::prefix(not))
//             .op(Op::infix(eq_assign, Right) | Op::infix(add_eq, Right) | Op::infix(sub_eq, Right) | Op::infix(mul_eq, Right) | Op::infix(div_eq, Right) | Op::infix(mod_eq, Right) | Op::infix(pow_eq, Right))
//     };
// }

// pub fn parse(expression: Pairs<Rule>) -> ASTNode {
//     // this will be for generating the AST
//     PRATT_PARSER
//         .map_primary(|primary| match primary.as_rule() {
//             Rule::int => ASTNode::Int(primary.as_str().parse().unwrap()),
//             Rule::float => ASTNode::Float(primary.as_str().parse().unwrap()),
//             Rule::string => ASTNode::String(primary.as_str().to_string()),
//             Rule::Bool => ASTNode::Boolean(primary.as_str().parse().unwrap()),
//             Rule::var_ref => ASTNode::VarRef(primary.as_str().to_string()),
//             Rule::expr => parse(primary.into_inner()),
//             _ => unreachable!(),
//         })
//         .map_infix(|lhs, op, rhs| match op.as_rule() {
//             Rule::add => ASTNode::Add(Box::new(lhs), Box::new(rhs)),
//             Rule::sub => ASTNode::Sub(Box::new(lhs), Box::new(rhs)),
//             Rule::mul => ASTNode::Mul(Box::new(lhs), Box::new(rhs)),
//             Rule::div => ASTNode::Div(Box::new(lhs), Box::new(rhs)),
//             Rule::modulus => ASTNode::Mod(Box::new(lhs), Box::new(rhs)),
//             Rule::pow => ASTNode::Pow(Box::new(lhs), Box::new(rhs)),
//             Rule::eq => ASTNode::Eq(Box::new(lhs), Box::new(rhs)),
//             Rule::neq => ASTNode::Ne(Box::new(lhs), Box::new(rhs)),
//             Rule::gt => ASTNode::Gt(Box::new(lhs), Box::new(rhs)),
//             Rule::gte => ASTNode::Gte(Box::new(lhs), Box::new(rhs)),
//             Rule::lt => ASTNode::Lt(Box::new(lhs), Box::new(rhs)),
//             Rule::lte => ASTNode::Lte(Box::new(lhs), Box::new(rhs)),
//             Rule::and => ASTNode::And(Box::new(lhs), Box::new(rhs)),
//             Rule::or => ASTNode::Or(Box::new(lhs), Box::new(rhs)),
//             Rule::eq_assign => ASTNode::Set(lhs.to_string(), Box::new(rhs)),
//             Rule::add_eq => ASTNode::SetAdd(lhs.to_string(), Box::new(rhs)),
//             Rule::sub_eq => ASTNode::SetSub(lhs.to_string(), Box::new(rhs)),
//             Rule::mul_eq => ASTNode::SetMul(lhs.to_string(), Box::new(rhs)),
//             Rule::div_eq => ASTNode::SetDiv(lhs.to_string(), Box::new(rhs)),
//             Rule::mod_eq => ASTNode::SetMod(lhs.to_string(), Box::new(rhs)),
//             Rule::pow_eq => ASTNode::SetPow(lhs.to_string(), Box::new(rhs)),
//             _ => unreachable!(),
//         })
//         .map_prefix(|op, expr| match op.as_rule() {
//             Rule::neg => ASTNode::Neg(Box::new(expr)),
//             Rule::not => ASTNode::Not(Box::new(expr)),
//             _ => unreachable!(),
//         })
//         .map_postfix(|expr, op| match op.as_rule() {
//             Rule::func_call => ASTNode::FuncCall(expr.to_string(), vec![]),
//             _ => unreachable!(),
//         })
//         .parse(expression);
    
//     ASTNode::Unknown
// }

// use the regular pest parser not the pratt parser
pub fn parse(expression: &str) {
    
}