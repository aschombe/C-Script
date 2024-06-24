use crate::error_handler::ErrorHandler;
use crate::keywords::{get_ast_node, get_keyword, Keywords};
use crate::types::{Type, TypeTag};
use crate::ast::{ASTNode, ASTNodeTypes};

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
            "del" => {
                let name: &String = tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected variable name".to_string()))?;
                ast.push(ASTNode::Del(name.clone()));
                tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected ';'".to_string()))?;
            }
            "set" => {
                let name: &String = tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected variable name".to_string()))?;
                tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected '='".to_string()))?;
                let value: ASTNode = parse_expression(&mut tokens_iter)?;
                ast.push(ASTNode::Set(name.clone(), Box::new(value)));
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
            "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                tokens_iter.next(); // Consume '==' or '!=' or '<' or '<=' or '>' or '>='
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
            
            let ast_type: ASTNodeTypes = get_ast_node(&func_name);
            match ast_type {
                ASTNodeTypes::NArg => {
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
                    return Ok(ASTNode::NArg(get_keyword(&func_name), args));
                }
                ASTNodeTypes::OneArg => {
                    let arg: ASTNode = parse_expression(tokens_iter)?;
                    if let Some(&token) = tokens_iter.peek() {
                        if token == ")" {
                            tokens_iter.next(); // Consume ')'
                        }
                    }
                    return Ok(ASTNode::OneArg(func_name, Box::new(arg)));
                }
                ASTNodeTypes::TwoArg => {
                    let arg1: ASTNode = parse_expression(tokens_iter)?;
                    if let Some(&token) = tokens_iter.peek() {
                        if token == "," {
                            tokens_iter.next(); // Consume ','
                        }
                    }
                    let arg2: ASTNode = parse_expression(tokens_iter)?;
                    if let Some(&token) = tokens_iter.peek() {
                        if token == ")" {
                            tokens_iter.next(); // Consume ')'
                        }
                    }
                    return Ok(ASTNode::TwoArg(func_name, Box::new(arg1), Box::new(arg2)));
                }
                ASTNodeTypes::TwoArgComp => {
                    let arg1: ASTNode = parse_expression(tokens_iter)?;
                    if let Some(&token) = tokens_iter.peek() {
                        if token == "," {
                            tokens_iter.next(); // Consume ','
                        }
                    }
                    let arg2: ASTNode = parse_expression(tokens_iter)?;
                    if let Some(&token) = tokens_iter.peek() {
                        if token == ")" {
                            tokens_iter.next(); // Consume ')'
                        }
                    }
                    return Ok(ASTNode::TwoArgComp(func_name, Box::new(arg1), Box::new(arg2)));
                }
                _ => {
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
                }
            }
        } else {
            return Ok(ASTNode::VariableRef(token.clone()));
        }
    }

    Err(ErrorHandler::SyntaxError("Expected expression".to_string()))
}