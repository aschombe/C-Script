use crate::{ast::{self, Expr}, checker::check_type, error_handler::ErrorHandler, var_func::{Function, VariableInfo}};
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Parser<'a> {
    tokens: &'a [String],
    current: usize,
    global_variables: HashMap<String, VariableInfo>,
    functions: HashMap<String, Function>,
    scopes: Vec<HashMap<String, VariableInfo>>, // Stack of scopes for local variables
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [String]) -> Self {
        Self { tokens, current: 0, global_variables: HashMap::new(), functions: HashMap::new(), scopes: vec![HashMap::new()] }
    }

    // Function to push a new scope (e.g., when entering a function)
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // Function to pop the current scope (e.g., when exiting a function)
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    // Function to get a variable from the current scope
    fn get_variable(&self, name: &str) -> Option<&VariableInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var);
            }
        }
        None
    }

    // Function to set a variable in the current scope
    fn set_variable(&mut self, name: String, var_info: VariableInfo) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, var_info);
        }
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
                Err(format!("Expected '{}', found '{}'", expected, token))
            }
        } else {
            Err(format!("Expected '{}', found end of input", expected))
        }
    }

    fn parse_keyword(&mut self) -> Result<Option<Expr>, String> {
        // Handle variable declaration
        if let Some(token) = self.current_token() {
            if token == "let" {
                self.advance();
                let name: String = self.current_token().ok_or("Expected variable name")?.clone();

                self.advance();
                let _ = self.expect(":");
                let typ: String = self.current_token().ok_or("Expected type")?.clone();
        
                self.advance();
                let _ = self.expect("=");
                let expr: Expr = self.parse_expr()?;
                let expr: Expr = check_type(&typ, expr)?;
        
                let _ = self.expect(";");
                
                let typ2: String = typ.clone();
                self.global_variables.insert(name.clone(), VariableInfo { name: name.clone(), typ, value: Some(expr.clone()) });

                return Ok(Some(Expr::Let(name.clone(), Box::new(Expr::Type(typ2)), Box::new(expr))));
            }
    
            // Handle variable assignment
            if self.global_variables.contains_key(token) {
                let name: String = token.clone();
                self.advance();
                self.expect("=")?;
                let expr: Expr = self.parse_expr()?;
                
                if let Some(var_info) = self.global_variables.get(&name) {
                    let expr: Expr = check_type(&var_info.typ, expr.clone())?;
        
                    let name2: String = name.clone();
                    self.global_variables.insert(name.clone(), VariableInfo { name, typ: var_info.typ.clone(), value: Some(expr.clone()) });
        
                    self.expect(";")?;
                    return Ok(Some(Expr::Set(name2, Box::new(expr))));
                } else {
                    return Err(ErrorHandler::VariableNotFound(name).to_string());
                }
            }
    
            // Handle deletion
            if token == "del" {
                self.advance();
                let name: String = self.current_token().ok_or("Expected variable name")?.clone();
    
                self.advance();
                self.expect(";")?;
    
                if self.global_variables.remove(&name).is_some() {
                    return Ok(Some(Expr::Delete(name)));
                } else {
                    return Err(ErrorHandler::VariableNotFound(name).to_string());
                }
            }

            // Handle return statement
            if token == "return" {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(";")?;
                return Ok(Some(Expr::Return(Box::new(expr))));
            }

            // Handle function declaration
            if token == "func" {
                self.advance();
    
                let recursive = if self.current_token() == Some(&"rec".to_string()) {
                    self.advance();
                    true
                } else {
                    false
                };
    
                let name = self.current_token().ok_or("Expected function name")?.clone();
                self.advance();
    
                self.push_scope(); // Start a new local scope for the function
    
                let mut args = Vec::new();
                self.expect("(")?;
                while self.current_token() != Some(&")".to_string()) {
                    let arg_name = self.current_token().ok_or("Expected argument name")?.clone();
                    self.advance();
                    self.expect(":")?;
                    let arg_type = self.current_token().ok_or("Expected argument type")?.clone();
                    args.push((arg_name.clone(), arg_type.clone()));
                    self.set_variable(arg_name.clone(), VariableInfo { name: arg_name.clone(), typ: arg_type.clone(), value: None });
                    self.advance();
                    if self.current_token() == Some(&",".to_string()) {
                        self.advance();
                    }
                }
    
                self.expect(")")?;
    
                self.expect(":")?;
                let return_type = self.current_token().ok_or("Expected return type")?.clone();
                self.advance();
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
    
                self.pop_scope(); // Exit the function scope
    
                // Create function expression
                let func_expr = Expr::Func(
                    name.clone(),
                    recursive,
                    args.clone(),
                    return_type.clone(),
                    body.clone(),
                );
    
                self.functions.insert(
                    name.clone(),
                    Function {
                        name: name.clone(),
                        recursive,
                        params: args.clone(),
                        body: vec![func_expr.clone()],
                        return_type: return_type.clone(),
                    },
                );
    
                return Ok(Some(func_expr));
            }

            // // if (condition) { body } elif (condition) { body } elif (condition) { body } ... else { body }
            // IEE(Box<Expr>, Vec<(Box<Expr>, Box<Expr>)>, Box<Expr>),
            if token == "if" {
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
                let mut elifs = Vec::new();
                while self.current_token() == Some(&"elif".to_string()) {
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
                    elifs.push((Box::new(condition), Box::new(Expr::List(body))));
                }
                let mut else_body = Vec::new();
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
                return Ok(Some(Expr::IEE(Box::new(condition), elifs, Box::new(Expr::List(else_body)))));
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        
        while let Some(op) = self.current_token().map(|t| t.clone()) {
            match op.as_str() {
                "+" | "-" => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = match op.as_str() {
                        "+" => Expr::Add(Box::new(left), Box::new(right)),
                        "-" => Expr::Sub(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }
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
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        
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
            _ if self.get_variable(&token).is_some() => {
                self.advance();
                Expr::VarRef(token)
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
