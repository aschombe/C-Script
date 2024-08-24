use crate::ast::Expr;
use crate::error_handler::ErrorHandler;

use regex::Regex;

#[derive(Clone)]
pub struct Parser<'a> {
    tokens: &'a [String],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [String]) -> Self {
        Self { 
            tokens, 
            current: 0, 
        }
    }

    fn advance(&mut self, amount: usize) {
        self.current += amount;
    }

    fn current_token(&self) -> Option<&String> {
        self.tokens.get(self.current)
    }

    fn expect(&mut self, expected: &str) -> Result<(), String> {
        if let Some(token) = self.current_token() {
            if token == expected {
                self.advance(1);
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
                    self.advance(1);
                    let name: String = self.current_token().ok_or("Expected variable name")?.clone();
                    self.advance(1);
                    self.expect(":")?;
                    let typ: String = self.current_token().ok_or("Expected type")?.clone();
                    self.advance(1);
                    self.expect("=")?;
                    let expr: Expr = self.parse_expr()?;
                    self.expect(";")?;
                    let typ2: String = typ.clone();
                    return Ok(Some(Expr::Let(name, Box::new(Expr::Type(typ2)), Box::new(expr))));
                },
                "if" => {
                    self.advance(1);
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
                        self.advance(1);
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
                        self.advance(1);
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
                    self.advance(1);
                    let expr: Expr = self.parse_expr()?;
                    self.expect(";")?;
                    return Ok(Some(Expr::Return(Box::new(expr))));
                },
                "del" => {
                    self.advance(1);
                    let name: String = self.current_token().ok_or("Expected variable name")?.clone();
                    self.advance(1);
                    self.expect(";")?;
                    return Ok(Some(Expr::Delete(name)));
                },
                "func" => {
                    println!("Current token: {:?}", self.current_token());
                    self.advance(1);

                    let name: String = self.current_token().ok_or("Expected function name")?.clone();
                    self.advance(1);

                    self.expect("(")?;
                    let mut args: Vec<(String, String)> = Vec::new();
                    while self.current_token() != Some(&")".to_string()) {
                        let arg_name: String = self.current_token().ok_or("Expected argument name")?.clone();
                        self.advance(1);
                        self.expect(":")?;
                        let arg_type: String = self.current_token().ok_or("Expected argument type")?.clone();
                        self.advance(1);
                        args.push((arg_name, arg_type));
                        if self.current_token() == Some(&",".to_string()) {
                            self.advance(1);
                        }
                    }
                    self.expect(")")?;
                    self.expect(":")?;
                    let return_type: String = self.current_token().ok_or("Expected return type")?.clone();
                    self.advance(1);
                    self.expect("{")?;
                    let mut body: Vec<Expr> = Vec::new();
                    while self.current_token() != Some(&"}".to_string()) {
                        if let Some(expr) = self.parse_keyword()? {
                            body.push(expr);
                        } else {
                            body.push(self.parse_expr()?);
                            self.expect(";")?;
                        }
                    }
                    
                    self.expect("}")?;
                    return Ok(Some(Expr::Func(name, args, return_type, body))); 
                },
                "for" => {
                    self.advance(1);
                    self.expect("(")?;
                    let var: String = self.current_token().ok_or("Expected variable name")?.clone();
                    self.advance(1);
                    self.expect(";")?;

                    // Collect condition
                    let mut condition: Vec<String> = Vec::new();
                    while self.current_token() != Some(&";".to_string()) {
                        condition.push(self.current_token().ok_or("Expected condition")?.clone());
                        self.advance(1);
                    }
                    let condition: String = condition.join(" ");

                    self.expect(";")?;

                    // Collect increment
                    let mut increment: Vec<String> = Vec::new();
                    while self.current_token() != Some(&")".to_string()) {
                        increment.push(self.current_token().ok_or("Expected increment")?.clone());
                        self.advance(1);
                    }
                    let increment: String = increment.join(" ");

                    self.expect(")")?;
                    self.expect("{")?;

                    let mut body: Vec<Expr> = Vec::new();
                    while self.current_token() != Some(&"}".to_string()) {
                        if let Some(expr) = self.parse_keyword()? {
                            body.push(expr);
                        } else {
                            body.push(self.parse_expr()?);
                            self.expect(";")?;
                        }
                    }
                    self.expect("}")?;
                    return Ok(Some(Expr::For(var, condition, increment, body)));
                },
                "while" => {
                    self.advance(1);
                    self.expect("(")?;
                    let condition: Expr = self.parse_expr()?;
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
                _ => {
                    if let Some(next_token) = self.tokens.get(self.current + 1) {
                        if next_token == "=" {
                            let name: String = token.clone();
                            self.advance(2);
                            let expr: Expr = self.parse_expr()?;
                            self.expect(";")?;
                            return Ok(Some(Expr::Set(name, Box::new(expr))));
                        }
                    }
                }
            }

            Ok(None)
        } else {
            Err(ErrorHandler::UnexpectedEndOfProgram.to_string())
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        // Handle function calls first
        if let Some(token) = self.current_token() {
            if token.chars().all(|c| c.is_alphabetic()) {
                let name: String = token.clone();
                self.advance(1);
                self.expect("(")?;
                let mut args: Vec<Expr> = Vec::new();
                while self.current_token() != Some(&")".to_string()) {
                    args.push(self.parse_expr()?);
                    if self.current_token() == Some(&",".to_string()) {
                        self.advance(1);
                    }
                }
                self.expect(")")?;
                return Ok(Expr::FuncApp(name, args));
            }
        }
    
        // Handle other expressions
        let mut left: Expr = self.parse_term()?;
    
        while let Some(op) = self.current_token().map(|t: &String| t.clone()) {
            match op.as_str() {
                "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                    self.advance(1);
                    let right: Expr = self.parse_term()?;
                    left = match op.as_str() {
                        "==" => Expr::IsEqual(Box::new(left), Box::new(right)),
                        "!=" => Expr::IsNE(Box::new(left), Box::new(right)),
                        "<" => Expr::IsLT(Box::new(left), Box::new(right)),
                        "<=" => Expr::IsLTE(Box::new(left), Box::new(right)),
                        ">" => Expr::IsGT(Box::new(left), Box::new(right)),
                        ">=" => Expr::IsGTE(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                },
                "+" | "-" => {
                    self.advance(1);
                    let right: Expr = self.parse_term()?;
                    left = match op.as_str() {
                        "+" => Expr::Add(Box::new(left), Box::new(right)),
                        "-" => Expr::Sub(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                },
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left: Expr = self.parse_factor()?;
        
        while let Some(op) = self.current_token().map(|t: &String| t.clone()) {
            match op.as_str() {
                "*" | "/" | "^" => {
                    self.advance(1);
                    let right: Expr = self.parse_factor()?;
                    left = match op.as_str() {
                        "*" => Expr::Mul(Box::new(left), Box::new(right)),
                        "/" => Expr::Div(Box::new(left), Box::new(right)),
                        "^" => Expr::Pow(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                },
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        if let Some(token) = self.current_token() {
            match token.as_str() {
                "(" => {
                    self.advance(1);
                    let expr: Expr = self.parse_expr()?;
                    self.expect(")")?;
                    Ok(expr)
                },
                _ if token.chars().all(|c: char| c.is_digit(10) || c == '.') => {
                    // Check if the token contains a dot to differentiate float from int
                    if token.contains('.') {
                        let value: f64 = token.parse::<f64>().map_err(|e| e.to_string())?;
                        self.advance(1);
                        Ok(Expr::Float(value))
                    } else {
                        let value: i64 = token.parse::<i64>().map_err(|e| e.to_string())?;
                        self.advance(1);
                        Ok(Expr::Int(value))
                    }
                },
                _ if token.chars().all(|c: char| c.is_alphabetic()) => {
                    let name: String = token.clone();
                    self.advance(1);
                    Ok(Expr::VarRef(name))
                },
                _ => Err(format!("Unexpected token: {}", token)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut ast: Vec<Expr> = Vec::new();
        while self.current_token().is_some() {
            if let Some(expr) = self.parse_keyword()? {
                ast.push(expr);
            } else {
                let expr: Expr = self.parse_expr()?;
                ast.push(expr);
    
                // Only expect a semicolon if the current token is not a closing brace
                if self.current_token() != Some(&"}".to_string()) {
                    self.expect(";")?;
                }
            }
        }
        Ok(ast)
    }
}