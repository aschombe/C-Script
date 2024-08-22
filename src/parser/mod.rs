// use crate::error_handler::ErrorHandler;
// use crate::keywords::{get_ast_node, get_keyword, Keywords};
// use crate::types::{Type, TypeTag};
// use crate::ast::{ASTNode, ASTNodeTypes};

// use std::iter::Peekable;
// use std::slice::Iter;

// pub fn parse(tokens: Vec<String>) -> Result<Vec<ASTNode>, ErrorHandler> {
//     if tokens.is_empty() {
//         return Err(ErrorHandler::NoProgram);
//     }

//     let mut tokens_iter: Peekable<Iter<String>> = tokens.iter().peekable();
//     let mut ast: Vec<ASTNode> = Vec::new();

//     while let Some(token) = tokens_iter.next() {
//         match token.as_str() {
//             "let" => {
//                 let name: &String = tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected variable name".to_string()))?;
//                 tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected ':'".to_string()))?;
//                 let typ: &String = tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected type".to_string()))?;
                
//                 let type_tag: TypeTag = match typ.as_str() {
//                     "int" => TypeTag::Int,
//                     "float" => TypeTag::Float,
//                     "string" => TypeTag::String,
//                     "bool" => TypeTag::Boolean,
//                     "void" => TypeTag::Void,
//                     _ => return Err(ErrorHandler::TypeError(format!("Invalid type: {}", typ))),
//                 };
                
//                 tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected '='".to_string()))?;
//                 let value: ASTNode = parse_expression(&mut tokens_iter)?;

//                 ast.push(ASTNode::Let(name.clone(), type_tag, Box::new(value)));

//                 tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected ';'".to_string()))?;
//             }
//             "del" => {
//                 let name: &String = tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected variable name".to_string()))?;
//                 ast.push(ASTNode::Del(name.clone()));
//                 tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected ';'".to_string()))?;
//             }
//             "set" => {
//                 let name: &String = tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected variable name".to_string()))?;
//                 tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected '='".to_string()))?;
//                 let value: ASTNode = parse_expression(&mut tokens_iter)?;
//                 ast.push(ASTNode::Set(name.clone(), Box::new(value)));
//                 tokens_iter.next().ok_or(ErrorHandler::SyntaxError("Expected ';'".to_string()))?;
//             }
            
//             _ => {

//             }
//         }
//     }

//     Ok(ast)
// }

// fn parse_expression(tokens_iter: &mut Peekable<Iter<String>>) -> Result<ASTNode, ErrorHandler> {
//     let mut expr_ast: ASTNode = parse_term(tokens_iter)?;

//     while let Some(&token) = tokens_iter.peek() {
//         match token.as_str() {
//             "+" | "-" => {
//                 tokens_iter.next(); // Consume '+' or '-'
//                 let right: ASTNode = parse_term(tokens_iter)?;
//                 expr_ast = ASTNode::NArg(get_keyword(token), vec![expr_ast, right]);
//             }
//             "==" | "!=" | "<" | "<=" | ">" | ">=" => {
//                 tokens_iter.next(); // Consume '==' or '!=' or '<' or '<=' or '>' or '>='
//                 let right: ASTNode = parse_term(tokens_iter)?;
//                 expr_ast = ASTNode::NArg(get_keyword(token), vec![expr_ast, right]);
//             }
//             _ => break,
//         }
//     }

//     Ok(expr_ast)
// }

// fn parse_term(tokens_iter: &mut Peekable<Iter<String>>) -> Result<ASTNode, ErrorHandler> {
//     let mut term_ast: ASTNode = parse_factor(tokens_iter)?;

//     while let Some(&token) = tokens_iter.peek() {
//         match token.as_str() {
//             "*" | "/" | "%" => {
//                 tokens_iter.next(); // Consume '*', '/' or '%'
//                 let right: ASTNode = parse_factor(tokens_iter)?;
//                 term_ast = ASTNode::NArg(get_keyword(token), vec![term_ast, right]);
//             }
//             _ => break,
//         }
//     }

//     Ok(term_ast)
// }

// fn parse_factor(tokens_iter: &mut Peekable<Iter<String>>) -> Result<ASTNode, ErrorHandler> {
//     if let Some(token) = tokens_iter.next() {
//         if let Ok(num) = token.parse::<f64>() {
//             return Ok(ASTNode::Value(Type::Float(num)));
//         } else if let Ok(num) = token.parse::<i64>() {
//             return Ok(ASTNode::Value(Type::Int(num)));
//         } else if token == "true" {
//             return Ok(ASTNode::Value(Type::Boolean(true)));
//         } else if token == "false" {
//             return Ok(ASTNode::Value(Type::Boolean(false)));
//         } else if token.starts_with('"') && token.ends_with('"') {
//             return Ok(ASTNode::Value(Type::String(token.clone())));
//         } else if token == "(" {
//             let expr_ast: ASTNode = parse_expression(tokens_iter)?;
//             if let Some(next_token) = tokens_iter.next() {
//                 if next_token != ")" {
//                     return Err(ErrorHandler::SyntaxError("Expected ')'".to_string()));
//                 }
//                 return Ok(expr_ast);
//             } else {
//                 return Err(ErrorHandler::SyntaxError("Expected ')'".to_string()));
//             }
//         } else if tokens_iter.peek() == Some(&&String::from("(")) {
//             let func_name = token.clone();
//             tokens_iter.next(); // Consume '('
            
//             let ast_type: ASTNodeTypes = get_ast_node(&func_name);
//             match ast_type {
//                 ASTNodeTypes::NArg => {
//                     let mut args: Vec<ASTNode> = Vec::new();
//                     while let Some(&token) = tokens_iter.peek() {
//                         if token == ")" {
//                             tokens_iter.next(); // Consume ')'
//                             break;
//                         }
//                         let arg: ASTNode = parse_expression(tokens_iter)?;
//                         args.push(arg);
//                         if let Some(&token) = tokens_iter.peek() {
//                             if token == "," {
//                                 tokens_iter.next(); // Consume ','
//                             }
//                         }
//                     }
//                     return Ok(ASTNode::NArg(get_keyword(&func_name), args));
//                 }
//                 ASTNodeTypes::OneArg => {
//                     let arg: ASTNode = parse_expression(tokens_iter)?;
//                     if let Some(&token) = tokens_iter.peek() {
//                         if token == ")" {
//                             tokens_iter.next(); // Consume ')'
//                         }
//                     }
//                     return Ok(ASTNode::OneArg(func_name, Box::new(arg)));
//                 }
//                 ASTNodeTypes::TwoArg => {
//                     let arg1: ASTNode = parse_expression(tokens_iter)?;
//                     if let Some(&token) = tokens_iter.peek() {
//                         if token == "," {
//                             tokens_iter.next(); // Consume ','
//                         }
//                     }
//                     let arg2: ASTNode = parse_expression(tokens_iter)?;
//                     if let Some(&token) = tokens_iter.peek() {
//                         if token == ")" {
//                             tokens_iter.next(); // Consume ')'
//                         }
//                     }
//                     return Ok(ASTNode::TwoArg(func_name, Box::new(arg1), Box::new(arg2)));
//                 }
//                 ASTNodeTypes::TwoArgComp => {
//                     let arg1: ASTNode = parse_expression(tokens_iter)?;
//                     if let Some(&token) = tokens_iter.peek() {
//                         if token == "," {
//                             tokens_iter.next(); // Consume ','
//                         }
//                     }
//                     let arg2: ASTNode = parse_expression(tokens_iter)?;
//                     if let Some(&token) = tokens_iter.peek() {
//                         if token == ")" {
//                             tokens_iter.next(); // Consume ')'
//                         }
//                     }
//                     return Ok(ASTNode::TwoArgComp(func_name, Box::new(arg1), Box::new(arg2)));
//                 }
//                 _ => {
//                     let mut args: Vec<ASTNode> = Vec::new();
//                     while let Some(&token) = tokens_iter.peek() {
//                         if token == ")" {
//                             tokens_iter.next(); // Consume ')'
//                             break;
//                         }
//                         let arg: ASTNode = parse_expression(tokens_iter)?;
//                         args.push(arg);
//                         if let Some(&token) = tokens_iter.peek() {
//                             if token == "," {
//                                 tokens_iter.next(); // Consume ','
//                             }
//                         }
//                     }
//                     return Ok(ASTNode::FunctionCall(func_name, args));
//                 }
//             }
//         } else {
//             return Ok(ASTNode::VariableRef(token.clone()));
//         }
//     }

//     Err(ErrorHandler::SyntaxError("Expected expression".to_string()))
// }

use crate::ast::Expr;

#[derive(Clone)]
pub struct Parser<'a> {
    tokens: &'a [String],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [String]) -> Self {
        Self { tokens, current: 0 }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn current_token(&self) -> Option<&String> {
        self.tokens.get(self.current)
    }

    fn expect(&mut self, expected: &str) -> Result<(), String> {
        if let Some(token) = self.current_token() {
            if token == expected {
                let mut parser = self.clone();
                parser.advance();
                Ok(())
            } else {
                Err(format!("Expected '{}', found '{}'", expected, token))
            }
        } else {
            Err(format!("Expected '{}', found end of input", expected))
        }
    }

    fn parse_var_decl(&mut self) -> Result<Expr, String> {
        // Expect and consume "let"
        self.expect("let")?;
        self.advance(); // Move to the next token
    
        // Extract the variable name
        let name = self.current_token().ok_or("Expected variable name")?.clone();
        if !self.tokens.contains(&name) {
            return Err("Variable name not found".to_string());
        }
        self.advance(); // Move to the next token
    
        // Expect ":"
        self.expect(":")?;
        self.advance(); // Move to the next token
    
        // Extract the type
        let typ = self.current_token().ok_or("Expected type")?.clone();
        if !self.tokens.contains(&typ) {
            return Err("Type not found".to_string());
        }
        self.advance(); // Move to the next token
    
        // Expect "="
        self.expect("=")?;
        self.advance(); // Move to the next token
    
        // Parse the expression
        let expr = self.parse_expr()?;
    
        // Expect ";"
        self.expect(";")?;
    
        Ok(Expr::Let(name, Box::new(Expr::Type(typ)), Box::new(expr)))
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        
        while let Some(op) = self.current_token().map(|t| t.clone()) {
            match op.as_str() {
                "+" | "-" => {
                    self.advance(); // Consume '+' or '-'
                    let right = self.parse_term()?;
                    left = match op.as_str() {
                        "+" => Expr::Add(Box::new(left), Box::new(right)),
                        "-" => Expr::Sub(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }
                "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                    self.advance(); // Consume comparison operators
                    let right = self.parse_term()?;
                    left = match op.as_str() {
                        "==" => Expr::IsEqual(Box::new(left), Box::new(right)),
                        "!=" => Expr::IsNE(Box::new(left), Box::new(right)),
                        "<" => Expr::IsLT(Box::new(left), Box::new(right)),
                        "<=" => Expr::IsLTE(Box::new(left), Box::new(right)),
                        ">" => Expr::IsGT(Box::new(left), Box::new(right)),
                        ">=" => Expr::IsGTE(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
    
        while let Some(op) = self.current_token().map(|t| t.clone()) { // Extract the operator
            match op.as_str() {
                "*" | "/" | "%" => { 
                    self.advance(); // Consume '*', '/' or '%'
                    let right = self.parse_factor()?;
                    left = match op.as_str() {
                        "*" => Expr::Mul(Box::new(left), Box::new(right)),
                        "/" => Expr::Div(Box::new(left), Box::new(right)),
                        "%" => Expr::Mod(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }
    
        Ok(left)
    }
    
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let token = self.current_token().ok_or("Unexpected end of input")?.clone();
    
        let mut left = match token.as_str() {
            "(" => {
                self.advance(); // Consume '('
                let expr = self.parse_expr()?;
                self.expect(")")?; // Expect and consume ')'
                expr
            }
            _ if token.chars().all(|c: char| c.is_digit(10)) => {
                self.advance(); // Consume number
                Expr::Int(token.parse().unwrap())
            }
            _ => return Err(format!("Unexpected token: {}", token)),
        };
        
        while let Some(op) = self.current_token().map(|t| t.clone()) {
            match op.as_str() {
                "^" => {
                    self.advance(); // Consume '^'
                    let right = self.parse_factor()?;
                    left = Expr::Pow(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_var_decl()
    }
}