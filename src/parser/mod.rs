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
    While(Box<ASTNode>, Vec<ASTNode>),
    Function(String, Types, Vec<String>, Vec<ASTNode>),
    Return(Box<ASTNode>),
    Substring(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>), // substring(<string>, <start>, <end>)
    Strip(Box<ASTNode>, Box<ASTNode>), // strip(<string>, <char>)
    Replace(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>), // replace(<string>, <old-char>, <new-char>)
    Break,
    Continue,
}

    tokenizer return type is Vec<String>
*/

use crate::{ast::ASTNode, types::Types, error_handler::ErrorHandler};

pub fn parse(tokens: Vec<String>) -> Result<Vec<ASTNode>, ErrorHandler> {
    if tokens.len() == 0 {
        return Err(ErrorHandler::ParseError("No tokens to parse".to_string()));
    }

    Ok(vec![])
}
