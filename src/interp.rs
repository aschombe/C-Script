use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::fmt;

// Structure for error types
#[derive(Debug)]
pub enum ErrorHandler {
    DivisionByZero,
    UnknownOperator(String),
    ParseError(String),
    VariableNotFound(String),
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorHandler::DivisionByZero => write!(f, "Error: Division by zero"),
            ErrorHandler::UnknownOperator(op) => write!(f, "Error: Unknown operator '{}'", op),
            ErrorHandler::ParseError(err) => write!(f, "Error: Parse error - {}", err),
            ErrorHandler::VariableNotFound(var) => write!(f, "Error: Variable '{}' not found", var),
        }
    }
}

pub struct Interpreter {
    variables: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self, expr: &str) -> Result<String, ErrorHandler> {
        let tokens: Vec<String> = tokenize(expr);
        let (ast, _) = parse(&tokens)?;

        match self.eval_ast(&ast) {
            Ok(result) => Ok(result.to_string()),
            Err(e) => Err(e),
        }
    }

    fn eval_ast(&mut self, node: &ASTNode) -> Result<i32, ErrorHandler> {
        match node {
            ASTNode::Value(val) => {
                if let Ok(num) = val.parse::<i32>() {
                    Ok(num)
                } else if let Some(&num) = self.variables.get(val) {
                    Ok(num)
                } else {
                    Err(ErrorHandler::VariableNotFound(val.clone()))
                }
            }
            ASTNode::Operator(op, operands) => {
                // Special handling for let, set, and get
                if op == "let" || op == "set" || op == "get" {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(format!("Invalid syntax for '{}'", op)));
                    }
                }

                let args: Result<Vec<i32>, _> = operands.iter().map(|operand| self.eval_ast(operand)).collect();
                let args: Vec<i32> = args?;

                match op.as_str() {
                    "print" => {
                        for arg in &args {
                            println!("{}", arg);
                        }
                        return Ok(0);
                    }
                    "let" => {
                        if let ASTNode::Value(var) = &operands[0] {
                            let value: i32 = self.eval_ast(&operands[1])?;
                            self.variables.insert(var.clone(), value);
                            return Ok(value);
                        } else {
                            return Err(ErrorHandler::ParseError("Invalid let syntax".to_string()));
                        }
                    }
                    "set" => {
                        if let ASTNode::Value(var) = &operands[0] {
                            if !self.variables.contains_key(var) {
                                return Err(ErrorHandler::VariableNotFound(var.clone()));
                            }
                            let value: i32 = self.eval_ast(&operands[1])?;
                            self.variables.insert(var.clone(), value);
                            return Ok(value);
                        } else {
                            return Err(ErrorHandler::ParseError("Invalid set syntax".to_string()));
                        }
                    }
                    "get" => {
                        if let ASTNode::Value(var) = &operands[0] {
                            if let Some(&val) = self.variables.get(var) {
                                return Ok(val);
                            } else {
                                return Err(ErrorHandler::VariableNotFound(var.clone()));
                            }
                        } else {
                            return Err(ErrorHandler::ParseError("Invalid get syntax".to_string()));
                        }
                    }
                    "+" | "add" => Ok(args.iter().sum()),
                    "-" | "subtract" | "sub" => Ok(args.iter().skip(1).fold(args[0], |acc, &num| acc - num)),
                    "*" | "multiply" | "mul" => Ok(args.iter().product()),
                    "/" | "divide" | "div" => {
                        if args.iter().skip(1).any(|&num| num == 0) {
                            return Err(ErrorHandler::DivisionByZero);
                        }
                        Ok(args.iter().skip(1).fold(args[0], |acc, &num| acc / num))
                    }
                    "%" | "modulo" | "mod" => {
                        if args.iter().skip(1).any(|&num| num == 0) {
                            return Err(ErrorHandler::DivisionByZero);
                        }
                        Ok(args.iter().skip(1).fold(args[0], |acc, &num| acc % num))
                    }
                    "max" => Ok(*args.iter().max().unwrap()),
                    "min" => Ok(*args.iter().min().unwrap()),
                    "pow" => Ok(args[0].pow(args[1] as u32)),
                    "sqrt" => Ok((args[0] as f64).sqrt() as i32),
                    "sin" => Ok((args[0] as f64).sin() as i32),
                    "cos" => Ok((args[0] as f64).cos() as i32),
                    "tan" => Ok((args[0] as f64).tan() as i32),
                    "abs" => Ok((args[0] as f64).abs() as i32),
                    "zero?" => Ok((args[0] == 0) as i32),
                    "even?" => Ok((args[0] % 2 == 0) as i32),
                    "odd?" => Ok((args[0] % 2 != 0) as i32),
                    "pos?" => Ok((args[0] > 0) as i32),
                    "neg?" => Ok((args[0] < 0) as i32),
                    "eq?" => Ok((args[0] == args[1]) as i32),
                    "neq?" => Ok((args[0] != args[1]) as i32),
                    "lt?" => Ok((args[0] < args[1]) as i32),
                    "gt?" => Ok((args[0] > args[1]) as i32),
                    "lte?" => Ok((args[0] <= args[1]) as i32),
                    "gte?" => Ok((args[0] >= args[1]) as i32),
                    "and" => Ok(args.iter().all(|&num| num != 0) as i32),
                    "or" => Ok(args.iter().any(|&num| num != 0) as i32),
                    "not" => Ok((args[0] == 0) as i32),
                    _ => Err(ErrorHandler::UnknownOperator(op.clone())),
                }
            }
        }
    }

    pub fn interp(&mut self, path: PathBuf) -> Result<(), ErrorHandler> {
        let contents: String = read_to_string(&path).map_err(|e| ErrorHandler::ParseError(e.to_string()))?;
        let lines: std::str::Lines = contents.lines();

        let mut line_num: i32 = 1;

        for line in lines {
            let line: &str = line.trim();

            if line.is_empty() || line.starts_with("//") {
                line_num += 1;
                continue;
            }

            if line.starts_with("(") {
                match self.eval(line) {
                    Ok(result) => {
                        println!("{}. {}: {}", line_num, line, result);
                    }
                    Err(e) => {
                        println!("{}", e);
                        return Err(e);
                    }
                }
            }

            line_num += 1;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum ASTNode {
    Operator(String, Vec<ASTNode>),
    Value(String),
}

fn tokenize(expr: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token: String = String::new();
    let mut in_string: bool = false;

    for c in expr.chars() {
        match c {
            '(' | ')' if !in_string => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(c.to_string());
            }
            '"' => {
                token.push(c);
                in_string = !in_string;
                if !in_string {
                    tokens.push(token.clone());
                    token.clear();
                }
            }
            ' ' if !in_string => {
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
            }
            _ => {
                token.push(c);
            }
        }
    }

    if !token.is_empty() {
        tokens.push(token);
    }

    tokens
}

fn parse(tokens: &[String]) -> Result<(ASTNode, usize), ErrorHandler> {
    if tokens.is_empty() {
        return Err(ErrorHandler::ParseError("Empty expression".to_string()));
    }

    let mut index: usize = 0;

    if tokens[index] != "(" {
        return Err(ErrorHandler::ParseError("Expected '('".to_string()));
    }

    index += 1;
    let operator: String = tokens[index].clone();
    index += 1;

    let mut operands: Vec<ASTNode> = Vec::new();

    while index < tokens.len() && tokens[index] != ")" {
        if tokens[index] == "(" {
            let (node, consumed) = parse(&tokens[index..])?;
            operands.push(node);
            index += consumed;
        } else {
            operands.push(ASTNode::Value(tokens[index].clone()));
            index += 1;
        }
    }

    if index >= tokens.len() || tokens[index] != ")" {
        return Err(ErrorHandler::ParseError("Expected ')'".to_string()));
    }

    Ok((ASTNode::Operator(operator, operands), index + 1))
}