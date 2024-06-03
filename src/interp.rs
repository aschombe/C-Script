use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;
use std::path::PathBuf;

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

#[derive(Debug, Clone)]
struct Function {
    params: Vec<String>,
    body: ASTNode,
}

#[derive(Debug)]
pub struct Interpreter {
    variables: HashMap<String, f64>,
    functions: HashMap<String, Function>,
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Interpreter {
            variables: self.variables.clone(),
            functions: self.functions.clone(),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
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

    fn eval_ast(&mut self, node: &ASTNode) -> Result<f64, ErrorHandler> {
        match node {
            ASTNode::Value(val) => {
                if val == "True" {
                    Ok(1.0)
                } else if val == "False" {
                    Ok(0.0)
                } else if let Ok(num) = val.parse::<f64>() {
                    Ok(num)
                } else if let Some(&num) = self.variables.get(val) {
                    Ok(num)
                } else {
                    Err(ErrorHandler::VariableNotFound(val.clone()))
                }
            }
            ASTNode::Operator(op, operands) => match op.as_str() {
                "func" => {
                    if operands.len() != 3 {
                        return Err(ErrorHandler::ParseError(format!(
                            "Invalid syntax for '{}'",
                            op
                        )));
                    }

                    if let ASTNode::Value(name) = &operands[0] {
                        if let ASTNode::Operator(_, param_nodes) = &operands[1] {
                            let params: Vec<String> = param_nodes
                                .iter()
                                .map(|param| match param {
                                    ASTNode::Value(val) => Ok(val.clone()),
                                    _ => Err(ErrorHandler::ParseError(
                                        "Invalid parameter".to_string(),
                                    )),
                                })
                                .collect::<Result<Vec<_>, _>>()?;

                            println!(
                                "DEBUG: Function '{}' defined with params: {:?}",
                                name, params
                            );

                            let body: ASTNode = operands[2].clone();
                            let func: Function = Function { params, body };

                            self.functions.insert(name.clone(), func);
                            Ok(0.0)
                        } else {
                            Err(ErrorHandler::ParseError(
                                "Invalid function parameters".to_string(),
                            ))
                        }
                    } else {
                        Err(ErrorHandler::ParseError(
                            "Invalid function name".to_string(),
                        ))
                    }
                }
                "call" => {
                    if operands.len() < 1 {
                        return Err(ErrorHandler::ParseError(format!(
                            "Invalid syntax for '{}'",
                            op
                        )));
                    }

                    if let ASTNode::Value(name) = &operands[0] {
                        if let Some(func) = self.functions.get(name) {
                            println!(
                                "DEBUG: Function '{}' called with args: {:?}",
                                name,
                                &operands[1..]
                            );

                            if operands.len() - 1 != func.params.len() {
                                return Err(ErrorHandler::ParseError(format!(
                                    "Invalid number of arguments for function '{}'",
                                    name
                                )));
                            }

                            let mut local_interpreter: Interpreter = Interpreter {
                                variables: self.variables.clone(),
                                functions: self.functions.clone(),
                            };

                            let mut local_vars: HashMap<String, f64> =
                                local_interpreter.variables.clone();
                            let local_funcs: HashMap<String, Function> =
                                local_interpreter.functions.clone();

                            let mut results: Vec<f64> = Vec::new();
                            for arg in &operands[1..] {
                                results.push(local_interpreter.eval_ast(arg)?);
                            }

                            for (param, result) in func.params.iter().zip(results) {
                                local_vars.insert(param.clone(), result);
                            }

                            println!(
                                "DEBUG: Local variables for function '{}': {:?}",
                                name, local_vars
                            );

                            let mut local_interpreter: Interpreter = Interpreter {
                                variables: local_vars,
                                functions: local_funcs,
                            };

                            local_interpreter.eval_ast(&func.body)
                        } else {
                            Err(ErrorHandler::UnknownOperator(name.clone()))
                        }
                    } else {
                        Err(ErrorHandler::ParseError(
                            "Invalid function name".to_string(),
                        ))
                    }
                }
                "let" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(format!(
                            "Invalid syntax for '{}'",
                            op
                        )));
                    }
                    if let ASTNode::Value(var) = &operands[0] {
                        let value: f64 = self.eval_ast(&operands[1])?;
                        if self.variables.contains_key(var) {
                            return Err(ErrorHandler::ParseError(format!(
                                "Variable '{}' already exists",
                                var
                            )));
                        }
                        self.variables.insert(var.clone(), value);
                        Ok(value)
                    } else {
                        Err(ErrorHandler::ParseError("Invalid let syntax".to_string()))
                    }
                }
                "set" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(format!(
                            "Invalid syntax for '{}'",
                            op
                        )));
                    }
                    if let ASTNode::Value(var) = &operands[0] {
                        let value: f64 = self.eval_ast(&operands[1])?;
                        if !self.variables.contains_key(var) {
                            return Err(ErrorHandler::ParseError(format!(
                                "Variable '{}' not found",
                                var
                            )));
                        }
                        self.variables.insert(var.clone(), value);
                        Ok(value)
                    } else {
                        Err(ErrorHandler::ParseError("Invalid set syntax".to_string()))
                    }
                }
                "if" => {
                    if operands.len() < 2 {
                        return Err(ErrorHandler::ParseError(format!(
                            "Invalid syntax for '{}'",
                            op
                        )));
                    }
                    let condition: f64 = self.eval_ast(&operands[0])?;
                    if condition != 0.0 {
                        self.eval_ast(&operands[1])
                    } else {
                        let mut i: usize = 2;
                        while i < operands.len() {
                            if let ASTNode::Operator(ref cond_op, ref cond_operands) = &operands[i]
                            {
                                match cond_op.as_str() {
                                    "else" => {
                                        if cond_operands.len() != 1 {
                                            return Err(ErrorHandler::ParseError(format!(
                                                "Invalid syntax for '{}'",
                                                cond_op
                                            )));
                                        }
                                        return self.eval_ast(&cond_operands[0]);
                                    }
                                    _ => {
                                        return Err(ErrorHandler::ParseError(
                                            "Invalid conditional syntax".to_string(),
                                        ))
                                    }
                                }
                            } else {
                                return Err(ErrorHandler::ParseError(
                                    "Invalid conditional syntax".to_string(),
                                ));
                            }
                            i += 1;
                        }
                        Ok(0.0)
                    }
                }
                "print" => {
                    for operand in operands {
                        let result: f64 = self.eval_ast(operand)?;
                        println!("{}", result);
                    }
                    Ok(0.0)
                }
                "get" => {
                    if let ASTNode::Value(var) = &operands[0] {
                        if let Some(&val) = self.variables.get(var) {
                            Ok(val)
                        } else {
                            Err(ErrorHandler::VariableNotFound(var.clone()))
                        }
                    } else {
                        Err(ErrorHandler::ParseError("Invalid get syntax".to_string()))
                    }
                }
                "+" | "add" => {
                    let mut result: f64 = 0.0;
                    for operand in operands {
                        result += self.eval_ast(operand)?;
                    }
                    Ok(result)
                }
                "-" | "subtract" | "sub" => {
                    if operands.is_empty() {
                        return Err(ErrorHandler::ParseError("Empty subtraction".to_string()));
                    }
                    let mut result: f64 = self.eval_ast(&operands[0])?;
                    for operand in &operands[1..] {
                        result -= self.eval_ast(operand)?;
                    }
                    Ok(result)
                }
                "*" | "multiply" | "mul" => {
                    let mut result: f64 = 1.0;
                    for operand in operands {
                        result *= self.eval_ast(operand)?;
                    }
                    Ok(result)
                }
                "/" | "divide" | "div" => {
                    if operands.is_empty() {
                        return Err(ErrorHandler::ParseError("Empty division".to_string()));
                    }
                    let mut result: f64 = self.eval_ast(&operands[0])?;
                    for operand in &operands[1..] {
                        let divisor: f64 = self.eval_ast(operand)?;
                        if divisor == 0.0 {
                            return Err(ErrorHandler::DivisionByZero);
                        }
                        result /= divisor;
                    }
                    Ok(result)
                }
                "%" | "modulo" | "mod" => {
                    if operands.is_empty() {
                        return Err(ErrorHandler::ParseError("Empty modulo".to_string()));
                    }
                    let mut result: f64 = self.eval_ast(&operands[0])?;
                    for operand in &operands[1..] {
                        let divisor: f64 = self.eval_ast(operand)?;
                        if divisor == 0.0 {
                            return Err(ErrorHandler::DivisionByZero);
                        }
                        result %= divisor;
                    }
                    Ok(result)
                }
                "max" => {
                    let mut max_val: f64 = f64::MIN;
                    for operand in operands {
                        let val: f64 = self.eval_ast(operand)?;
                        if val > max_val {
                            max_val = val;
                        }
                    }
                    Ok(max_val)
                }
                "min" => {
                    let mut min_val: f64 = f64::MAX;
                    for operand in operands {
                        let val: f64 = self.eval_ast(operand)?;
                        if val < min_val {
                            min_val = val;
                        }
                    }
                    Ok(min_val)
                }
                "pow" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'pow'".to_string(),
                        ));
                    }
                    let base: f64 = self.eval_ast(&operands[0])?;
                    let exp: f64 = self.eval_ast(&operands[1])?;
                    Ok(base.powf(exp))
                }
                "sqrt" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'sqrt'".to_string(),
                        ));
                    }
                    Ok(self.eval_ast(&operands[0])?.sqrt())
                }
                "sin" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'sin'".to_string(),
                        ));
                    }
                    Ok(self.eval_ast(&operands[0])?.sin())
                }
                "cos" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'cos'".to_string(),
                        ));
                    }
                    Ok(self.eval_ast(&operands[0])?.cos())
                }
                "tan" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'tan'".to_string(),
                        ));
                    }
                    Ok(self.eval_ast(&operands[0])?.tan())
                }
                "abs" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'abs'".to_string(),
                        ));
                    }
                    Ok(self.eval_ast(&operands[0])?.abs())
                }
                "zero?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'zero?'".to_string(),
                        ));
                    }
                    Ok((self.eval_ast(&operands[0])? == 0.0) as i32 as f64)
                }
                "even?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'even?'".to_string(),
                        ));
                    }
                    Ok((self.eval_ast(&operands[0])? % 2.0 == 0.0) as i32 as f64)
                }
                "odd?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'odd?'".to_string(),
                        ));
                    }
                    Ok((self.eval_ast(&operands[0])? % 2.0 != 0.0) as i32 as f64)
                }
                "pos?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'pos?'".to_string(),
                        ));
                    }
                    Ok((self.eval_ast(&operands[0])? > 0.0) as i32 as f64)
                }
                "neg?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'neg?'".to_string(),
                        ));
                    }
                    Ok((self.eval_ast(&operands[0])? < 0.0) as i32 as f64)
                }
                "eq?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'eq?'".to_string(),
                        ));
                    }
                    Ok(
                        (self.eval_ast(&operands[0])? == self.eval_ast(&operands[1])?) as i32
                            as f64,
                    )
                }
                "lt?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'lt?'".to_string(),
                        ));
                    }
                    Ok((self.eval_ast(&operands[0])? < self.eval_ast(&operands[1])?) as i32 as f64)
                }
                "lte?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'le?'".to_string(),
                        ));
                    }
                    Ok(
                        (self.eval_ast(&operands[0])? <= self.eval_ast(&operands[1])?) as i32
                            as f64,
                    )
                }
                "gt?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'gt?'".to_string(),
                        ));
                    }
                    Ok((self.eval_ast(&operands[0])? > self.eval_ast(&operands[1])?) as i32 as f64)
                }
                "gte?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'ge?'".to_string(),
                        ));
                    }
                    Ok(
                        (self.eval_ast(&operands[0])? >= self.eval_ast(&operands[1])?) as i32
                            as f64,
                    )
                }
                "and" => {
                    if operands.len() < 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'and'".to_string(),
                        ));
                    }
                    let mut result: f64 = 1.0;
                    for operand in operands {
                        let val: f64 = self.eval_ast(operand)?;
                        if val == 0.0 {
                            result = 0.0;
                            break;
                        }
                    }
                    Ok(result)
                }
                "or" => {
                    if operands.len() < 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'or'".to_string(),
                        ));
                    }
                    let mut result: f64 = 0.0;
                    for operand in operands {
                        let val: f64 = self.eval_ast(operand)?;
                        if val != 0.0 {
                            result = 1.0;
                            break;
                        }
                    }
                    Ok(result)
                }
                "not" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'not'".to_string(),
                        ));
                    }
                    Ok((self.eval_ast(&operands[0])? == 0.0) as i32 as f64)
                }
                _ => Err(ErrorHandler::UnknownOperator(op.clone())),
            },
        }
    }

    pub fn interp(&mut self, path: PathBuf) -> Result<(), ErrorHandler> {
        let contents: String =
            read_to_string(&path).map_err(|e| ErrorHandler::ParseError(e.to_string()))?;
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

#[derive(Debug, Clone)]
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
            ' ' | '\n' if !in_string => {
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

