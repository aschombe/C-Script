use crate::{ast::Expr, checker::check_type, error_handler::ErrorHandler, var_func::{Function, VariableInfo}};
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Parser<'a> {
    tokens: &'a [String],
    current: usize,
    in_function: bool,
    global_variables: HashMap<String, VariableInfo>,
    functions: HashMap<String, Function>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [String]) -> Self {
        Self { tokens, current: 0, global_variables: HashMap::new(), functions: HashMap::new(), in_function: false }
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
                self.advance();
                Ok(())
            } else {
                Err(ErrorHandler::UnexpectedToken(expected.to_string(), token.clone()).to_string())
            }
        } else {
            Err(ErrorHandler::UnexpectedToken(expected.to_string(), "end of input".to_string()).to_string())
        }
    }

    fn parse_keyword(&mut self) -> Result<Option<Expr>, String> {
        if let Some(token) = self.current_token() {
            match token.as_str() {
                "let" => {
                    self.advance();
                    let name: String = self.current_token().ok_or("Expected variable name")?.clone();
                    self.advance();
                    self.expect(":")?;
                    let typ: String = self.current_token().ok_or("Expected type")?.clone();
                    self.advance();
                    self.expect("=")?;
                    let expr: Expr = self.parse_expr()?;
                    let expr: Expr = check_type(&typ, expr)?;
                    self.expect(";")?;
                    let typ2: String = typ.clone();
                    self.global_variables.insert(
                        name.clone(), 
                        VariableInfo { 
                            name: name.clone(), 
                            typ, 
                            value: Some(expr.clone()) 
                        }
                    );
                    return Ok(Some(Expr::Let(name, Box::new(Expr::Type(typ2)), Box::new(expr))));
                },
                "if" => {
                    // If-Condition, If-Body, List of (condition, list of expr) pairs, else body
                    // IEE(Box<Expr>, Vec<Box<Expr>>, Option<Vec<(Expr, Vec<Expr>)>>, Option<Vec<Expr>>),
                    self.advance();
                    let if_condition: Expr = self.parse_expr()?;
                    self.expect("{")?;
                    let mut if_body: Vec<Expr> = Vec::new();
                    while self.current_token() != Some(&"}".to_string()) {
                        if let Some(expr) = self.parse_keyword()? {
                            if_body.push(expr);
                        } else {
                            if_body.push(self.parse_expr()?);
                            self.expect(";")?;
                        }
                    }
                    self.expect("}")?;
                    let mut elifs: Vec<(Expr, Vec<Expr>)> = Vec::new();
                    while self.current_token() == Some(&"elif".to_string()) {
                        self.advance();
                        let elif_condition: Expr = self.parse_expr()?;
                        self.expect("{")?;
                        let mut elif_body: Vec<Expr> = Vec::new();
                        while self.current_token() != Some(&"}".to_string()) {
                            if let Some(expr) = self.parse_keyword()? {
                                elif_body.push(expr);
                            } else {
                                elif_body.push(self.parse_expr()?);
                                self.expect(";")?;
                            }
                        }
                        self.expect("}")?;
                        elifs.push((elif_condition, elif_body));
                    }
                    let mut else_body: Vec<Expr> = Vec::new();
                    if self.current_token() == Some(&"else".to_string()) {
                        self.advance();
                        self.expect("{")?;
                        while self.current_token() != Some(&"}".to_string()) {
                            if let Some(expr) = self.parse_keyword()? {
                                else_body.push(expr);
                            } else {
                                else_body.push(self.parse_expr()?);
                                self.expect(";")?;
                            }
                        }
                        self.expect("}")?;
                    }
                    return Ok(Some(Expr::IEE(Box::new(if_condition), if_body, Some(elifs), Some(else_body))));
                },
                "return" => {
                    if !self.in_function {
                        // return Err("Return statement outside of function".to_string());
                        return Err(ErrorHandler::ReturnOutsideFunction.to_string());
                    }
                    self.advance();
                    let expr: Expr = self.parse_expr()?;
                    self.expect(";")?;
                    return Ok(Some(Expr::Return(Box::new(expr))));
                },
                "del" => {
                    self.advance();
                    let name: String = self.current_token().ok_or("Expected variable name")?.clone();
                    self.advance();
                    self.expect(";")?;
                    if self.global_variables.remove(&name).is_some() {
                        return Ok(Some(Expr::Delete(name)));
                    } else {
                        return Err(ErrorHandler::VariableNotFound(name).to_string());
                    }
                },
                "func" => {
                    self.advance();
                    let name: String = self.current_token().ok_or("Expected function name")?.clone();
                    self.advance();
                    self.expect("(")?;
                    let mut args: Vec<(String, String)> = Vec::new();
                    while self.current_token() != Some(&")".to_string()) {
                        let arg_name: String = self.current_token().ok_or("Expected argument name")?.clone();
                        self.advance();
                        self.expect(":")?;
                        let arg_type: String = self.current_token().ok_or("Expected argument type")?.clone();
                        self.advance();
                        args.push((arg_name, arg_type));
                        if self.current_token() == Some(&",".to_string()) {
                            self.advance();
                        }
                    }
                    self.expect(")")?;
                    self.expect(":")?;
                    let return_type: String = self.current_token().ok_or("Expected return type")?.clone();
                    self.advance();
                    self.expect("{")?;
                    let mut body: Vec<Expr> = Vec::new();
                    self.in_function = true;
                    while self.current_token() != Some(&"}".to_string()) {
                        if let Some(expr) = self.parse_keyword()? {
                            body.push(expr);
                        } else {
                            body.push(self.parse_expr()?);
                            self.expect(";")?;
                        }
                    }
                    self.in_function = false;
                    self.expect("}")?;
                    // func (recursive - optional) name(arg1: type, arg2: type, ...): type { ... }
                    // Func(String, bool, Vec<(String, String)>, String, Vec<Expr>),
                    let args2 = args.clone();
                    let return_type2 = return_type.clone();
                    let body2 = body.clone();
                    self.functions.insert(
                        name.clone(),
                        Function { 
                            name: name.clone(), 
                            recursive: false, 
                            params: args, 
                            return_type, 
                            body 
                        }
                    );
                    return Ok(Some(Expr::Func(name, false, args2, return_type2, body2))); 
                },
                /*
                Supported syntax for for loops:
                let i:int = 0
                for (i; i < 10; i = i + 1) {
                    1 + 1
                }
                */
                "for" => {
                    self.advance(); // Advance past 'for'
                    self.expect("(")?;

                    // Parse initialization
                    let init_var_name = self.current_token().ok_or("Expected variable name")?.clone();
                    self.advance();
                    self.expect(";")?;
                    let init_expr = self.parse_expr()?;

                    // Parse condition
                    let condition = self.parse_expr()?;
                    self.expect(";")?;

                    // Parse increment
                    let increment = self.parse_expr()?;
                    self.expect(")")?;

                    // Parse body
                    self.expect("{")?;
                    let mut body = Vec::new();
                    while self.current_token() != Some(&"}".to_string()) {
                        if let Some(expr) = self.parse_keyword()? {
                            body.push(expr);
                        } else {
                            body.push(self.parse_expr()?);
                            self.expect(";")?;
                        }
                    }
                    self.expect("}")?;

                    // Return the parsed for loop expression
                    return Ok(Some(Expr::For((init_var_name, Box::new(init_expr)), Box::new(condition), Box::new(increment), body)));
                },
                // while(condition) { body }
                // While(Box<Expr>, Vec<Expr>),
                // condition will be a comparison
                "while" => {
                    self.advance();
                    self.expect("(")?;
                    let condition = self.parse_expr()?;
                    self.expect(")")?;
                    self.expect("{")?;
                    let mut body = Vec::new();
                    while self.current_token() != Some(&"}".to_string()) {
                        if let Some(expr) = self.parse_keyword()? {
                            body.push(expr);
                        } else {
                            body.push(self.parse_expr()?);
                            self.expect(";")?;
                        }
                    }
                    self.expect("}")?;
                    return Ok(Some(Expr::While(Box::new(condition), body)));
                }
                // see if its a variable assignment
                _ => {
                    // Check if it's an assignment
                    if self.current_token().map(|t| t.clone()) == Some("=".to_string()) {
                        let name: String = token.clone();
                        self.advance();
                        self.expect("=")?;
                        let expr: Expr = self.parse_expr()?;
                        if let Some(var_info) = self.global_variables.get(&name) {
                            let expr: Expr = check_type(&var_info.typ, expr.clone())?;
                            self.global_variables.insert(
                                name.clone(), 
                                VariableInfo { 
                                    name: name.clone(), 
                                    typ: var_info.typ.clone(), 
                                    value: Some(expr.clone()) 
                                }
                            );
                            self.expect(";")?;
                            return Ok(Some(Expr::Set(name, Box::new(expr))));
                        } else {
                            return Err(ErrorHandler::VariableNotFound(name).to_string());
                        }
                    }
                    
                    // Handle variable assignment
                    if self.global_variables.contains_key(token) {
                        let name: String = token.clone();
                        self.advance();
                        self.expect("=")?;
                        let expr: Expr = self.parse_expr()?;
                        
                        if let Some(var_info) = self.global_variables.get(&name) {
                            let expr: Expr = check_type(&var_info.typ, expr.clone())?;
                            self.global_variables.insert(
                                name.clone(), 
                                VariableInfo { 
                                    name: name.clone(), 
                                    typ: var_info.typ.clone(), 
                                    value: Some(expr.clone()) 
                                }
                            );
                            self.expect(";")?;
                            return Ok(Some(Expr::Set(name, Box::new(expr))));
                        } else {
                            return Err(ErrorHandler::VariableNotFound(name).to_string());
                        }
                    }
                }
            }

            Ok(None)
        } else {
            // Err("Unexpected end of input".to_string())
            Err(ErrorHandler::UnexpectedEndOfProgram.to_string())
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        
        while let Some(op) = self.current_token().map(|t| t.clone()) {
            match op.as_str() {
                "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                    self.advance();
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
                "+" | "-" => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = match op.as_str() {
                        "+" => Expr::Add(Box::new(left), Box::new(right)),
                        "-" => Expr::Sub(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_term(&mut self) -> Result<Expr, String> {
        // Start with parsing factors
        let mut left = self.parse_factor()?;
        
        // Parse arithmetic operators
        while let Some(op) = self.current_token().map(|t| t.clone()) {
            match op.as_str() {
                "*" | "/" | "%" => {
                    self.advance();
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
        
        let float_pattern = Regex::new(r"^\d*\.\d+$").unwrap();
        let int_pattern = Regex::new(r"^\d+$").unwrap();
        let string_pattern = Regex::new(r#"^".*"$"#).unwrap();
        
        let mut left = match token.as_str() {
            "(" => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(")")?;
                expr
            }
            _ if float_pattern.is_match(&token) => {
                self.advance();
                Expr::Float(token.parse().unwrap())
            }
            _ if int_pattern.is_match(&token) => {
                self.advance();
                Expr::Int(token.parse().unwrap())
            }
            _ if string_pattern.is_match(&token) => {
                self.advance();
                Expr::String(token.trim_matches('"').to_string())
            }
            "true" => {
                self.advance();
                Expr::Bool(true)
            }
            "false" => {
                self.advance();
                Expr::Bool(false)
            }
            _ if self.global_variables.contains_key(&token) => {
                self.advance();
                if let Some(var_info) = self.global_variables.get(&token) {
                    if let Some(value) = &var_info.value {
                        value.clone()
                    } else {
                        return Err(ErrorHandler::VariableNotFound(token).to_string());
                    }
                } else {
                    return Err(ErrorHandler::VariableNotFound(token).to_string());
                }
            }

            _ if self.functions.contains_key(&token) => {
                self.advance();
                let mut args = Vec::new();
                self.expect("(")?;
                while self.current_token().map(|t| t.clone()) != Some(")".to_string()) {
                    args.push(self.parse_expr()?);
                    if self.current_token().map(|t| t.clone()) == Some(",".to_string()) {
                        self.advance();
                    }
                }
                self.expect(")")?;
                Expr::FuncApp(token, args)
            }
            _ => return Err(format!("Unexpected token: {}", token)),
        };
        
        while let Some(op) = self.current_token().map(|t| t.clone()) {
            match op.as_str() {
                "^" => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expr::Pow(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut expressions = Vec::new();
        
        while self.current < self.tokens.len() {
            match self.parse_keyword() {
                Ok(Some(expr)) => expressions.push(expr),
                Ok(None) => (),
                Err(e) => return Err(e),
            }
        }
        
        Ok(expressions)
    }
}
