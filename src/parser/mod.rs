/*
let x:int = 5 * 2 - 3 / 2 + 1 % 2;
if (x > 5 && x < 10 && 1 == 1) {
    print("Hello, World!");
} elif (x > 10) {
    print("Goodbye, World!");
} else {
    print("Hello, World Again!");
}
let p1:string = "Hello,";
let p2:string = " World!";
let p3:string = p1 <> p2;
print(p3); // Hello, World!
let y:float = 5.5;
print(~y); // -5.5
let z:float = sqrt(25.0);
print(z); // 5.0
let a:float = sin(90.0);
print(a); // 1.0
let b:float = cos(0.0);
print(b); // 1.0
let c:float = tan(45.0);
print(c); // 1.0
let d:float = abs(-5.0);
print(d); // 5.0
let e:float = floor(5.5);
print(e); // 5.0
let f:float = ceil(5.5);
print(f); // 6.0    
let g:bool = true;
let h:bool = !g;
print(h); // false
let i:int = len("Hello, World!");
print(i); // 13
let j:string = upper("hello, world!");
print(j); // HELLO, WORLD!
let k:string = lower("HELLO, WORLD!");
print(k); // hello, world!
let l:int = rand(1, 10);
print(l); // random number between 1 and 10
let m:int = 5 ^ 2;
print(m); // 25
let n:string = "Hello, World!";
let o:string = substring(n, 0, 5);
print(o); // Hello
func factorial(n:int):int {
    if (n == 0) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
let p:int = factorial(5);
print(p); // 120

the tokenized output of this is:
["let", "x", ":", "int", "=", "5", "*", "2", "-", "3", "/", "2", "+", "1", "%", "2", ";", "if", "(", "x", ">", "5", "&&", "x", "<", "10", "&&", "1", "=", "=", "1", ")", "{", "print", "(", "\"Hello,", "World!\"", ")", ";", "}", "elif", "(", "x", ">", "10", ")", "{", "print", "(", "\"Goodbye,", "World!\"", ")", ";", "}", "else", "{", "print", "(", "\"Hello,", "World", "Again!\"", ")", ";", "}", "let", "p1", ":", "string", "=", "\"Hello,\"", ";", "let", "p2", ":", "string", "=", "\"", "World!\"", ";", "let", "p3", ":", "string", "=", "p1", "<>", "p2", ";", "print", "(", "p3", ")", ";", "let", "y", ":", "float", "=", "5.5", ";", "print", "(", "~y", ")", ";", "let", "z", ":", "float", "=", "sqrt", "(", "25.0", ")", ";", "print", "(", "z", ")", ";", "let", "a", ":", "float", "=", "sin", "(", "90.0", ")", ";", "print", "(", "a", ")", ";", "let", "b", ":", "float", "=", "cos", "(", "0.0", ")", ";", "print", "(", "b", ")", ";", "let", "c", ":", "float", "=", "tan", "(", "45.0", ")", ";", "print", "(", "c", ")", ";", "let", "d", ":", "float", "=", "abs", "(", "-5.0", ")", ";", "print", "(", "d", ")", ";", "let", "e", ":", 
"float", "=", "floor", "(", "5.5", ")", ";", "print", "(", "e", ")", ";", "let", "f", ":", "float", "=", "ceil", "(", "5.5", ")", ";", "print", "(", "f", ")", ";", "let", "g", ":", "bool", "=", "true", ";", "let", "h", ":", "bool", "=", "!g", ";", "print", "(", "h", ")", ";", "let", "i", ":", "int", "=", "len", "(", "\"Hello,", "World!\"", ")", ";", "print", "(", "i", ")", ";", "let", "j", ":", "string", "=", "upper", "(", "\"hello,", "world!\"", ")", ";", "print", "(", "j", ")", ";", "let", "k", ":", "string", "=", "lower", "(", "\"HELLO,", "WORLD!\"", ")", ";", "print", "(", "k", ")", ";", "let", "l", ":", "int", "=", "rand", "(", "1,", "10", ")", ";", "print", "(", "l", ")", ";", "let", "m", ":", "int", "=", "5", "^", "2", ";", "print", "(", "m", ")", ";", "let", "n", ":", "string", "=", "\"Hello,", "World!\"", ";", "let", "o", ":", "string", "=", "substring", "(", "n,", "0,", "5", ")", ";", "print", "(", "o", ")", ";", "func", "factorial", "(", "n", ":", "int", ")", ":", "int", "{", "if", "(", "n", "=", "=", "0", ")", "{", "return", "1", ";", "}", "else", "{", "return", "n", "*", "factorial", "(", "n", "-", "1", ")", ";", "}", "}", "let", "p", ":", "int", "=", "factorial", "(", "5", 
")", ";", "print", "(", "p", ")", ";"]
*/

use crate::error_handler::ErrorHandler;
use crate::keywords::{get_ast_node, get_keyword, Keywords};
use crate::types::{Type, TypeTag};
use crate::ast::{ASTNode, ASTNodeTypes};
use crate::function::Function;

use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(tokens: Vec<String>) -> Result<Vec<ASTNode>, ErrorHandler> {
    if tokens.is_empty() {
        return Err(ErrorHandler::NoProgram);
    }

    let mut tokens_iter: Peekable<Iter<String>> = tokens.iter().peekable();
    let mut ast: Vec<ASTNode> = Vec::new();

    while let Some(token) = tokens_iter.next() {
        match token.as_str() {
            "let" => {
                let name: &String = tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected variable name".to_string()))?;
                tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected ':'".to_string()))?;
                let typ: &String = tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected type".to_string()))?;
                
                let type_tag: TypeTag = match typ.as_str() {
                    "int" => TypeTag::Int,
                    "float" => TypeTag::Float,
                    "string" => TypeTag::String,
                    "bool" => TypeTag::Boolean,
                    "void" => TypeTag::Void,
                    _ => return Err(ErrorHandler::TypeError(format!("Invalid type: {}", typ))),
                };
                
                tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected '='".to_string()))?;
                let value: ASTNode = parse_expression(&mut tokens_iter)?;

                ast.push(ASTNode::Let(name.clone(), type_tag, Box::new(value)));

                tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected ';'".to_string()))?;
            }
            _ => {

            }
        }
    }

    Ok(ast)
}

fn parse_expression(tokens_iter: &mut Peekable<Iter<String>>) -> Result<ASTNode, ErrorHandler> {
    let mut expr_ast: ASTNode = parse_term(tokens_iter)?;

    while let Some(&token) = tokens_iter.peek() {
        match token.as_str() {
            "+" | "-" => {
                tokens_iter.next(); // Consume '+' or '-'
                let right: ASTNode = parse_term(tokens_iter)?;
                expr_ast = ASTNode::NArg(get_keyword(token), vec![expr_ast, right]);
            }
            _ => break,
        }
    }

    Ok(expr_ast)
}

fn parse_term(tokens_iter: &mut Peekable<Iter<String>>) -> Result<ASTNode, ErrorHandler> {
    let mut term_ast: ASTNode = parse_factor(tokens_iter)?;

    while let Some(&token) = tokens_iter.peek() {
        match token.as_str() {
            "*" | "/" | "%" => {
                tokens_iter.next(); // Consume '*', '/' or '%'
                let right: ASTNode = parse_factor(tokens_iter)?;
                term_ast = ASTNode::NArg(get_keyword(token), vec![term_ast, right]);
            }
            _ => break,
        }
    }

    Ok(term_ast)
}

fn parse_factor(tokens_iter: &mut Peekable<Iter<String>>) -> Result<ASTNode, ErrorHandler> {
    if let Some(token) = tokens_iter.next() {
        if let Ok(num) = token.parse::<f64>() {
            return Ok(ASTNode::Value(Type::Float(num)));
        } else if let Ok(num) = token.parse::<i64>() {
            return Ok(ASTNode::Value(Type::Int(num)));
        } else if token == "true" {
            return Ok(ASTNode::Value(Type::Boolean(true)));
        } else if token == "false" {
            return Ok(ASTNode::Value(Type::Boolean(false)));
        } else if token.starts_with('"') && token.ends_with('"') {
            return Ok(ASTNode::Value(Type::String(token.clone())));
        } else if token == "(" {
            let expr_ast: ASTNode = parse_expression(tokens_iter)?;
            if let Some(next_token) = tokens_iter.next() {
                if next_token != ")" {
                    return Err(ErrorHandler::SyntaxError("Expected ')'".to_string()));
                }
                return Ok(expr_ast);
            } else {
                return Err(ErrorHandler::SyntaxError("Expected ')'".to_string()));
            }
        } else if tokens_iter.peek() == Some(&&String::from("(")) {
            let func_name = token.clone();
            tokens_iter.next(); // Consume '('
            let mut args: Vec<ASTNode> = Vec::new();
            while let Some(&token) = tokens_iter.peek() {
                if token == ")" {
                    tokens_iter.next(); // Consume ')'
                    break;
                }
                let arg: ASTNode = parse_expression(tokens_iter)?;
                args.push(arg);
                if let Some(&token) = tokens_iter.peek() {
                    if token == "," {
                        tokens_iter.next(); // Consume ','
                    }
                }
            }
            return Ok(ASTNode::FunctionCall(func_name, args));
        } else {
            return Ok(ASTNode::VariableRef(token.clone()));
        }
    }

    Err(ErrorHandler::SyntaxError("Expected expression".to_string()))
}