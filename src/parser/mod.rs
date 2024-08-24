use crate::{
    ast::Expr, 
    checker::check_type, 
    error_handler::ErrorHandler, 
    var_func::{Function, VariableInfo}
};

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
        Self { 
            tokens, 
            current: 0, 
            global_variables: HashMap::new(), 
            functions: HashMap::new(), 
            in_function: false 
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
                    if !self.in_function {
                        // return Err("Return statement outside of function".to_string());
                        return Err(ErrorHandler::ReturnOutsideFunction.to_string());
                    }
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
                    if self.global_variables.remove(&name).is_some() {
                        return Ok(Some(Expr::Delete(name)));
                    } else {
                        return Err(ErrorHandler::VariableNotFound(name).to_string());
                    }
                },
                "func" => {
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
                    let args2: Vec<(String, String)> = args.clone();
                    let return_type2: String = return_type.clone();
                    let body2: Vec<Expr> = body.clone();
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
                    // for loop AST:
                    // var, condition, increment, body
                    //For(String, Box<Expr>, Box<Expr>, Vec<Expr>),
                    self.advance(1);
                    self.expect("(")?;
                    let var: String = self.current_token().ok_or("Expected variable name")?.clone();
                    self.advance(1);
                    self.expect(";")?;

                    // THIS ISNT WORKING
                    // let condition: Expr = self.parse_expr()?;

                    // collect everything until the next semicolon as the condition
                    let mut condition: Vec<String> = Vec::new();
                    while self.current_token() != Some(&";".to_string()) {
                        condition.push(self.current_token().ok_or("Expected condition")?.clone());
                        self.advance(1);
                    }
                    
                    let condition: String = condition.join(" ");

                    self.expect(";")?;

                    // do the same thing as before and collect everything until the ) to collect as the increment
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
                // while(condition) { body }
                // While(Box<Expr>, Vec<Expr>),
                // condition will be a comparison
                "while" => {
                    self.advance(1);
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
                        self.advance(1);
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
                        self.advance(1);
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
        let mut left: Expr = self.parse_term()?;
        
        while let Some(op) = self.current_token().map(|t: &String| t.clone()) {
            match op.as_str() {
                "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                    self.advance(1);
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
                    self.advance(1);
                    let right: Expr = self.parse_term()?;
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
        let mut left: Expr = self.parse_factor()?;
        
        // Parse arithmetic operators
        while let Some(op) = self.current_token().map(|t: &String| t.clone()) {
            match op.as_str() {
                "*" | "/" | "%" => {
                    self.advance(1);
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
        let token: String = self.current_token().ok_or("Unexpected end of input")?.clone();
        
        let float_pattern: Regex = Regex::new(r"^\d*\.\d+$").unwrap();
        let int_pattern: Regex = Regex::new(r"^\d+$").unwrap();
        let string_pattern: Regex = Regex::new(r#"^".*"$"#).unwrap();
        
        let mut left: Expr = match token.as_str() {
            "(" => {
                self.advance(1);
                let expr: Expr = self.parse_expr()?;
                self.expect(")")?;
                expr
            }
            _ if float_pattern.is_match(&token) => {
                self.advance(1);
                Expr::Float(token.parse().unwrap())
            }
            _ if int_pattern.is_match(&token) => {
                self.advance(1);
                Expr::Int(token.parse().unwrap())
            }
            _ if string_pattern.is_match(&token) => {
                self.advance(1);
                Expr::String(token.trim_matches('"').to_string())
            }
            "true" => {
                self.advance(1);
                Expr::Bool(true)
            }
            "false" => {
                self.advance(1);
                Expr::Bool(false)
            }
            _ if self.global_variables.contains_key(&token) => {
                self.advance(1);
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
                self.advance(1);
                let mut args: Vec<Expr> = Vec::new();
                self.expect("(")?;
                while self.current_token().map(|t: &String| t.clone()) != Some(")".to_string()) {
                    args.push(self.parse_expr()?);
                    if self.current_token().map(|t: &String| t.clone()) == Some(",".to_string()) {
                        self.advance(1);
                    }
                }
                self.expect(")")?;
                Expr::FuncApp(token, args)
            }
            _ => return Err(format!("Unexpected token: {}", token)),
        };
        
        while let Some(op) = self.current_token().map(|t: &String| t.clone()) {
            match op.as_str() {
                "^" => {
                    self.advance(1);
                    let right: Expr = self.parse_factor()?;
                    left = Expr::Pow(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut expressions: Vec<Expr> = Vec::new();
        
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let input: Vec<String> = vec!["1".to_string(), "+".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_subtraction() {
        let input: Vec<String> = vec!["1".to_string(), "-".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::Sub(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_multiplication() {
        let input: Vec<String> = vec!["1".to_string(), "*".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::Mul(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_division() {
        let input: Vec<String> = vec!["1".to_string(), "/".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::Div(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_modulus() {
        let input: Vec<String> = vec!["1".to_string(), "%".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::Mod(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_pow() {
        let input: Vec<String> = vec!["2".to_string(), "^".to_string(), "3".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::Pow(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))));
    }

    #[test]
    fn test_simple_parentheses() {
        let input: Vec<String> = vec!["(".to_string(), "1".to_string(), "+".to_string(), "2".to_string(), ")".to_string(), "*".to_string(), "3".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::Mul(Box::new(Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))), Box::new(Expr::Int(3))));
    }

    #[test]
    fn test_simple_lt() {
        let input: Vec<String> = vec!["1".to_string(), "<".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::IsLT(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_gt() {
        let input: Vec<String> = vec!["1".to_string(), ">".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::IsGT(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_lte() {
        let input: Vec<String> = vec!["1".to_string(), "<=".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::IsLTE(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_gte() {
        let input: Vec<String> = vec!["1".to_string(), ">=".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::IsGTE(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_eq() {
        let input: Vec<String> = vec!["1".to_string(), "==".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::IsEqual(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_simple_ne() {
        let input: Vec<String> = vec!["1".to_string(), "!=".to_string(), "2".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Expr = parser.parse_expr().unwrap();
        assert_eq!(result, Expr::IsNE(Box::new(Expr::Int(1)), Box::new(Expr::Int(2))));
    }

    #[test]
    fn test_variable_assignment() {
        let input: Vec<String> = vec!["let".to_string(), "x".to_string(), ":".to_string(), "int".to_string(), "=".to_string(), "1".to_string(), ";".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Vec<Expr> = parser.parse().unwrap();
        assert_eq!(result, vec![Expr::Let("x".to_string(), Box::new(Expr::Type("int".to_string())), Box::new(Expr::Int(1)))]);
    }

    #[test]
    fn test_variable_assignment_with_expr() {
        let input: Vec<String> = vec!["let".to_string(), "x".to_string(), ":".to_string(), "int".to_string(), "=".to_string(), "1".to_string(), "+".to_string(), "2".to_string(), ";".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Vec<Expr> = parser.parse().unwrap();
        assert_eq!(result, vec![Expr::Let("x".to_string(), Box::new(Expr::Type("int".to_string())), Box::new(Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))))]);
    }

    #[test]
    fn test_variable_assignment_with_expr_and_parentheses() {
        let input: Vec<String> = vec!["let".to_string(), "x".to_string(), ":".to_string(), "int".to_string(), "=".to_string(), "(".to_string(), "1".to_string(), "+".to_string(), "2".to_string(), ")".to_string(), ";".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Vec<Expr> = parser.parse().unwrap();
        assert_eq!(result, vec![Expr::Let("x".to_string(), Box::new(Expr::Type("int".to_string())), Box::new(Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))))]);
    }

    #[test]
    fn test_variable_reassignment() {
        let input: Vec<String> = vec!["let".to_string(), "x".to_string(), ":".to_string(), "int".to_string(), "=".to_string(), "1".to_string(), ";".to_string(), "x".to_string(), "=".to_string(), "2".to_string(), ";".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Vec<Expr> = parser.parse().unwrap();
        assert_eq!(result, vec![Expr::Let("x".to_string(), Box::new(Expr::Type("int".to_string())), Box::new(Expr::Int(1))), Expr::Set("x".to_string(), Box::new(Expr::Int(2)))]);
    }

    #[test]
    fn test_variable_reassignment_with_expr() {
        let input: Vec<String> = vec!["let".to_string(), "x".to_string(), ":".to_string(), "int".to_string(), "=".to_string(), "1".to_string(), ";".to_string(), "x".to_string(), "=".to_string(), "1".to_string(), "+".to_string(), "2".to_string(), ";".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Vec<Expr> = parser.parse().unwrap();
        assert_eq!(result, vec![Expr::Let("x".to_string(), Box::new(Expr::Type("int".to_string())), Box::new(Expr::Int(1))), Expr::Set("x".to_string(), Box::new(Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))))]); 
    }

    #[test]
    fn test_variable_reassignment_with_expr_and_parentheses() {
        let input: Vec<String> = vec!["let".to_string(), "x".to_string(), ":".to_string(), "int".to_string(), "=".to_string(), "1".to_string(), ";".to_string(), "x".to_string(), "=".to_string(), "(".to_string(), "1".to_string(), "+".to_string(), "2".to_string(), ")".to_string(), ";".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Vec<Expr> = parser.parse().unwrap();
        assert_eq!(result, vec![Expr::Let("x".to_string(), Box::new(Expr::Type("int".to_string())), Box::new(Expr::Int(1))), Expr::Set("x".to_string(), Box::new(Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(2)))))]);
    }

    #[test]
    fn test_variable_deletion() {
        let input: Vec<String> = vec!["let".to_string(), "x".to_string(), ":".to_string(), "int".to_string(), "=".to_string(), "1".to_string(), ";".to_string(), "del".to_string(), "x".to_string(), ";".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Vec<Expr> = parser.parse().unwrap();
        assert_eq!(result, vec![Expr::Let("x".to_string(), Box::new(Expr::Type("int".to_string())), Box::new(Expr::Int(1))), Expr::Delete("x".to_string())]);
    }

    #[test]
    fn test_simple_control_flow() {
        // let input: Vec<String> = vec!["if".to_string(), "1".to_string(), "==".to_string(), "1".to_string(), "{".to_string(), "let".to_string(), "x".to_string(), ":".to_string(), "int".to_string(), "=".to_string(), "1".to_string(), ";".to_string(), "}".to_string()];
        let input: Vec<String> = vec!["if".to_string(), "(" .to_string(), "1".to_string(), "==".to_string(), "1".to_string(), ")".to_string(), "{".to_string(), "let".to_string(), "x".to_string(), ":".to_string(), "int".to_string(), "=".to_string(), "1".to_string(), ";".to_string(), "}".to_string()];
        let mut parser: Parser = Parser::new(&input);
        let result: Vec<Expr> = parser.parse().unwrap();
        assert_eq!(result, vec![Expr::IEE(Box::new(Expr::IsEqual(Box::new(Expr::Int(1)), Box::new(Expr::Int(1)))), vec![Expr::Let("x".to_string(), Box::new(Expr::Type("int".to_string())), Box::new(Expr::Int(1)))], None, None)]);
    }
    // control flow syntax:
    /*
    if (cond) {
        expr;
    } elif (cond) {
        expr;
    } else {
        expr;
    }
    */
}