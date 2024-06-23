// here are some examples of how the tokenizer works:
/*

let x: int = 5;
becomes
["let", "x", ":", "int", "=", "5;", "\n"]

func add(x: int, y: int): int {
    return x + y;
}
let y: int = add(2, 3);
becomes
["func", "add", "(", "x", ":", "int,", "y", ":", "int", ")", ":", "int", "{", "\n", "return", "x", "+", "y;", "\n", "}", "\n", "let", "y", ":", "int", "=", "add", "(", "2,", "3", ")", ";", "\n"]
*/

// here is my types and ast enums:
/*
pub enum Types {
    NumberInt(i64),
    NumberFloat(f64),
    Text(String),
    Boolean(bool),
    Void,
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Value(Types),    
    NArg(String, Vec<ASTNode>), // add, subtract, multiply, divide, modulo, and, or, concat, print
    OneArg(String, Box<ASTNode>), // neg, sqrt, sin, cos, tan, abs, floor, ceil, not, len, upper, lower, exit
    TwoArg(String, Box<ASTNode>, Box<ASTNode>), // pow, rand
    TwoArgComp(String, Box<ASTNode>, Box<ASTNode>), // eq?, neq?, gt?, lt?, gte?, lte?
    Let(String, Types, Box<ASTNode>), // let <name>:<type> = <value>
    Del(String), // del <name>
    IfElifElse(Box<ASTNode>, Vec<(Box<ASTNode>, Vec<ASTNode>)>, Vec<ASTNode>), // WIP
    Switch(Box<ASTNode>, Vec<(Box<ASTNode>, Vec<ASTNode>)>, Vec<ASTNode>),
    For(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>, Vec<ASTNode>),
    Function(String, Vec<(String, Types)>, Vec<ASTNode>), // function <name>(<args>):<return-type> { <code> }
    Return(Box<ASTNode>),
    Substring(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>), // substring(<string>, <start>, <end>)
    Strip(Box<ASTNode>, Box<ASTNode>), // strip(<string>, <char>)
    Replace(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>), // replace(<string>, <old-char>, <new-char>)
    Break,
    Continue,
}

    tokenizer return type is Vec<String>
*/

// every available keyword and operator:
/*
pub enum Keywords {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Concat,
    Neg,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Abs,
    Floor,
    Ceil,
    Not,
    Len,
    Upper,
    Lower,
    Exit,
    Pow,
    Rand,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    Let,
    Del,
    If,
    Elif,
    Else,
    Switch,
    Case,
    Default,
    For,
    While,
    Function,
    Return,
    Break,
    Continue,
    Substring,
    Strip,
    Replace,
    Print
}

pub fn get_keyword(keyword: &str) -> Keywords {
    match keyword {
        "+" => Keywords::Add,
        "-" => Keywords::Sub,
        "*" => Keywords::Mul,
        "/" => Keywords::Div,
        "%" => Keywords::Mod,
        "&&" => Keywords::And,
        "||" => Keywords::Or,
        "<>" => Keywords::Concat,
        "~" => Keywords::Neg,
        "sqrt" => Keywords::Sqrt,
        "sin" => Keywords::Sin,
        "cos" => Keywords::Cos,
        "tan" => Keywords::Tan,
        "abs" => Keywords::Abs,
        "floor" => Keywords::Floor,
        "ceil" => Keywords::Ceil,
        "!" => Keywords::Not,
        "len" => Keywords::Len,
        "upper" => Keywords::Upper,
        "lower" => Keywords::Lower,
        "exit" => Keywords::Exit,
        "^" => Keywords::Pow,
        "rand" => Keywords::Rand,
        "==" => Keywords::Eq,
        "!=" => Keywords::Neq,
        ">" => Keywords::Gt,
        "<" => Keywords::Lt,
        ">=" => Keywords::Gte,
        "<=" => Keywords::Lte,
        "let" => Keywords::Let,
        "del" => Keywords::Del,
        "if" => Keywords::If,
        "elif" => Keywords::Elif,
        "else" => Keywords::Else,
        "switch" => Keywords::Switch,
        "case" => Keywords::Case,
        "default" => Keywords::Default,
        "for" => Keywords::For,
        "while" => Keywords::While,
        "func" => Keywords::Function,
        "return" => Keywords::Return,
        "break" => Keywords::Break,
        "continue" => Keywords::Continue,
        "substring" => Keywords::Substring,
        "strip" => Keywords::Strip,
        "replace" => Keywords::Replace,
        "print" => Keywords::Print,
        _ => panic!("Unknown keyword: {}", keyword),
    }
}


*/

use crate::{ast::ASTNode, types::Types, error_handler::ErrorHandler, keywords::Keywords};