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

use crate::{ast::ASTNode, types::Types, error_handler::ErrorHandler};

pub fn parse(tokens: Vec<String>) -> Result<Vec<ASTNode>, ErrorHandler> {
    if tokens.len() == 0 {
        return Err(ErrorHandler::ParseError("No tokens to parse".to_string()));
    }

    // this is an infix language that utilizes order of operations
    let mut ast: Vec<ASTNode> = Vec::new();

    let mut i: usize = 0;

    while i < tokens.len() {
        let token = &tokens[i];
        println!("{}", token);
        match token.as_str() {
            "let" => {
                let name = tokens[i + 1].clone();
                let _colon = tokens[i + 2].clone();
                let type_ = tokens[i + 3].clone();
                let _equal = tokens[i + 4].clone();
                let value = tokens[i + 5].clone();

                let value = match type_.as_str() {
                    "int" => Types::NumberInt(value.parse::<i64>().unwrap()),
                    "float" => Types::NumberFloat(value.parse::<f64>().unwrap()),
                    "string" => Types::Text(value),
                    "bool" => Types::Boolean(value.parse::<bool>().unwrap()),
                    _ => return Err(ErrorHandler::ParseError("Unknown type".to_string())),
                };

                ast.push(ASTNode::Let(name, value.clone(), Box::new(ASTNode::Value(value))));
                i += 6;
            }
            "del" => {
                let name = tokens[i + 1].clone();
                ast.push(ASTNode::Del(name));
                i += 2;
            }
            "if" => {
                let condition = Box::new(ASTNode::Value(Types::Boolean(tokens[i + 1].parse::<bool>().unwrap())));
                let mut elifs: Vec<(Box<ASTNode>, Vec<ASTNode>)> = Vec::new();
                let mut el: Vec<ASTNode> = Vec::new();

                i += 2;

                while i < tokens.len() {
                    let token = &tokens[i];

                    if token == "elif" {
                        let condition = Box::new(ASTNode::Value(Types::Boolean(tokens[i + 1].parse::<bool>().unwrap())));
                        let mut code: Vec<ASTNode> = Vec::new();

                        i += 2;

                        while i < tokens.len() {
                            let token = &tokens[i];

                            if token == "elif" || token == "else" {
                                break;
                            }

                            code.push(ASTNode::Value(Types::NumberInt(token.parse::<i64>().unwrap())));
                            i += 1;
                        }

                        elifs.push((condition, code));
                    } else if token == "else" {
                        i += 1;

                        while i < tokens.len() {
                            let token = &tokens[i];

                            if token == "elif" || token == "else" {
                                break;
                            }

                            el.push(ASTNode::Value(Types::NumberInt(token.parse::<i64>().unwrap())));
                            i += 1;
                        }

                        break;
                    }

                    i += 1;
                }

                ast.push(ASTNode::IfElifElse(condition, elifs, el));
            }
            "switch" => {
                let condition = Box::new(ASTNode::Value(Types::NumberInt(tokens[i + 1].parse::<i64>().unwrap())));
                let mut cases: Vec<(Box<ASTNode>, Vec<ASTNode>)> = Vec::new();
                let mut default: Vec<ASTNode> = Vec::new();

                i += 2;

                while i < tokens.len() {
                    let token = &tokens[i];

                    if token == "case" {
                        let value = Box::new(ASTNode::Value(Types::NumberInt(tokens[i + 1].parse::<i64>().unwrap())));
                        let mut code: Vec<ASTNode> = Vec::new();

                        i += 2;

                        while i < tokens.len() {
                            let token = &tokens[i];

                            if token == "case" || token == "default" {
                                break;
                            }

                            code.push(ASTNode::Value(Types::NumberInt(token.parse::<i64>().unwrap())));
                            i += 1;
                        }

                        cases.push((value, code));
                    } else if token == "default" {
                        i += 1;

                        while i < tokens.len() {
                            let token = &tokens[i];

                            if token == "case" || token == "default" {
                                break;
                            }

                            default.push(ASTNode::Value(Types::NumberInt(token.parse::<i64>().unwrap())));
                            i += 1;
                        }

                        break;
                    }

                    i += 1;
                }

                ast.push(ASTNode::Switch(condition, cases, default));
            }
            "for" => {
                let variable = Box::new(ASTNode::Value(Types::NumberInt(tokens[i + 1].parse::<i64>().unwrap())));
                let condition = Box::new(ASTNode::Value(Types::NumberInt(tokens[i + 2].parse::<i64>().unwrap())));
                let do_something = Box::new(ASTNode::Value(Types::NumberInt(tokens[i + 3].parse::<i64>().unwrap())));
                let mut code: Vec<ASTNode> = Vec::new();

                i += 4;

                while i < tokens.len() {
                    let token = &tokens[i];

                    if token == "for" {
                        break;
                    }

                    code.push(ASTNode::Value(Types::NumberInt(token.parse::<i64>().unwrap())));
                    i += 1;
                }

                ast.push(ASTNode::For(variable, condition, do_something, code));
            }
            "while" => {
                let condition = Box::new(ASTNode::Value(Types::Boolean(tokens[i + 1].parse::<bool>().unwrap())));
                let mut code: Vec<ASTNode> = Vec::new();

                i += 2;

                while i < tokens.len() {
                    let token = &tokens[i];

                    if token == "while" {
                        break;
                    }

                    code.push(ASTNode::Value(Types::NumberInt(token.parse::<i64>().unwrap())));
                    i += 1;
                }

                ast.push(ASTNode::While(condition, code));
            }
            "func" => {
                let name = tokens[i + 1].clone();
                let mut args: Vec<(String, Types)> = Vec::new();

                i += 2;

                while i < tokens.len() {
                    let token = &tokens[i];

                    if token == ")" {
                        break;
                    }

                    let arg_name = tokens[i].clone();
                    let _colon = tokens[i + 1].clone();
                    let arg_type = tokens[i + 2].clone();

                    let arg_type = match arg_type.as_str() {
                        "int" => Types::NumberInt(0),
                        "float" => Types::NumberFloat(0.0),
                        "string" => Types::Text("".to_string()),
                        "bool" => Types::Boolean(false),
                        _ => return Err(ErrorHandler::ParseError("Unknown type".to_string())),
                    };

                    args.push((arg_name, arg_type));
                    i += 3;
                }

                let mut code: Vec<ASTNode> = Vec::new();

                i += 1;

                while i < tokens.len() {
                    let token = &tokens[i];

                    if token == "func" {
                        break;
                    }

                    code.push(ASTNode::Value(Types::NumberInt(token.parse::<i64>().unwrap())));
                    i += 1;
                }

                ast.push(ASTNode::Function(name, args, code));
            }
            "return" => {
                let value = Box::new(ASTNode::Value(Types::NumberInt(tokens[i + 1].parse::<i64>().unwrap())));
                ast.push(ASTNode::Return(value));
                i += 2;
            }
            "print" => {
                let value = Box::new(ASTNode::Value(Types::NumberInt(tokens[i + 1].parse::<i64>().unwrap())));
                ast.push(ASTNode::OneArg("print".to_string(), value));
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(ast)
}