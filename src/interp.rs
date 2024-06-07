use std::{
    collections::HashMap, 
    fs::read_to_string, 
    path::PathBuf, 
    process::exit, 
    str::Lines
};

use parser::{
    parse, 
    tokenize, 
    ASTNode
};
use error_handler::ErrorHandler;
use function::Function;
use variable_value::VariableValue;

pub(crate) mod parser;
pub(crate) mod error_handler;
pub(crate) mod function;
pub(crate) mod variable_value;

#[derive(Debug)]
pub struct Interpreter {
    variables: HashMap<String, VariableValue>,
    functions: HashMap<String, Function>,
    output: Vec<String>,
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Interpreter {
            variables: self.variables.clone(),
            functions: self.functions.clone(),
            output: self.output.clone(),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            output: Vec::new(),
        }
    }

    pub fn eval(&mut self, expr: &str) -> Result<String, ErrorHandler> {
        let tokens: Vec<String> = tokenize(expr);
        let (ast, _) = parse(&tokens)?;
        match self.eval_ast(&ast) {
            Ok(result) => {
                let mut output: String = String::new();
                for line in &self.output {
                    output.push_str(&format!("{}", line));
                }
                self.output.clear();
                if output.is_empty() {
                    Ok(result
                        .map(|val| val.to_string())
                        .unwrap_or("OK".to_string()))
                } else {
                    Ok(output)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn eval_ast(&mut self, node: &ASTNode) -> Result<Option<VariableValue>, ErrorHandler> {
        match node {
            ASTNode::NoOp => Ok(None),
            ASTNode::Value(val) => {
                if val == "True" {
                    Ok(Some(VariableValue::Number(1.0)))
                } else if val == "False" {
                    Ok(Some(VariableValue::Number(0.0)))
                } else if let Ok(num) = val.parse::<f64>() {
                    Ok(Some(VariableValue::Number(num)))
                } else if let Some(num) = self.variables.get(val).cloned() {
                    Ok(Some(num))
                } else {
                    Err(ErrorHandler::VariableNotFound(val.clone()))
                }
            }

            ASTNode::StringValue(val) => Ok(Some(VariableValue::Text(val.clone()))),
            ASTNode::Operator(op, operands) => match op.as_str() {
                // Arithmetic operators:
                "add" => {
                    let mut result: f64 = 0.0;
                    for operand in operands {
                        result += self.eval_ast(operand)?.unwrap().as_number().unwrap();
                    }
                    Ok(Some(VariableValue::Number(result)))
                }
                "sub" => {
                    if operands.is_empty() {
                        return Err(ErrorHandler::ParseError("Empty subtraction".to_string()));
                    }
                    let mut result: f64 = match self.eval_ast(&operands[0])? {
                        Some(val) => val.as_number().unwrap(),
                        None => 0.0,
                    };
                    for operand in &operands[1..] {
                        result -= match self.eval_ast(operand)? {
                            Some(val) => val.as_number().unwrap(),
                            None => 0.0,
                        };
                    }
                    Ok(Some(VariableValue::Number(result)))
                }
                "mul" => {
                    let mut result: f64 = 1.0;
                    for operand in operands {
                        result *= self.eval_ast(operand)?.unwrap().as_number().unwrap();
                    }
                    Ok(Some(VariableValue::Number(result)))
                }
                "div" => {
                    if operands.is_empty() {
                        return Err(ErrorHandler::ParseError("Empty division".to_string()));
                    }
                    let mut result: f64 = match self.eval_ast(&operands[0])? {
                        Some(val) => val.as_number().unwrap(),
                        None => 0.0,
                    };
                    for operand in &operands[1..] {
                        let divisor: f64 = match self.eval_ast(operand)? {
                            Some(val) => val.as_number().unwrap(),
                            None => 0.0,
                        };
                        if divisor == 0.0 {
                            return Err(ErrorHandler::DivisionByZero);
                        }
                        result /= divisor;
                    }
                    Ok(Some(VariableValue::Number(result)))
                }
                "mod" => {
                    if operands.is_empty() {
                        return Err(ErrorHandler::ParseError("Empty modulo".to_string()));
                    }
                    let mut result: f64 =
                        self.eval_ast(&operands[0])?.unwrap().as_number().unwrap();
                    for operand in &operands[1..] {
                        let divisor: f64 = self.eval_ast(operand)?.unwrap().as_number().unwrap();
                        if divisor == 0.0 {
                            return Err(ErrorHandler::DivisionByZero);
                        }
                        result %= divisor;
                    }
                    Ok(Some(VariableValue::Number(result)))
                }
                "neg" => {
                    if operands.is_empty() {
                        return Err(ErrorHandler::ParseError("Empty negation".to_string()));
                    }

                    let result: f64 = -self.eval_ast(&operands[0])?.unwrap().as_number().unwrap();
                    Ok(Some(VariableValue::Number(result)))
                }
                // Other math operators:
                "max" => {
                    let mut max_val: f64 = f64::MIN;
                    for operand in operands {
                        let val: f64 = self.eval_ast(operand)?.unwrap().as_number().unwrap();
                        if val > max_val {
                            max_val = val;
                        }
                    }
                    Ok(Some(VariableValue::Number(max_val)))
                }
                "min" => {
                    let mut min_val: f64 = f64::MAX;
                    for operand in operands {
                        let val: f64 = self.eval_ast(operand)?.unwrap().as_number().unwrap();
                        if val < min_val {
                            min_val = val;
                        }
                    }
                    Ok(Some(VariableValue::Number(min_val)))
                }
                "pow" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'pow'".to_string(),
                        ));
                    }
                    let base: f64 = self.eval_ast(&operands[0])?.unwrap().as_number().unwrap();
                    let exp: f64 = self.eval_ast(&operands[1])?.unwrap().as_number().unwrap();
                    Ok(Some(VariableValue::Number(base.powf(exp))))
                }
                "sqrt" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'sqrt'".to_string(),
                        ));
                    }
                    let val: f64 = self.eval_ast(&operands[0])?.unwrap().as_number().unwrap();
                    if val < 0.0 {
                        return Err(ErrorHandler::ParseError(
                            "Square root of negative number".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(val.sqrt())))
                }
                "sin" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'sin'".to_string(),
                        ));
                    }

                    Ok(Some(VariableValue::Number(
                        self.eval_ast(&operands[0])?
                            .unwrap()
                            .as_number()
                            .unwrap()
                            .sin(),
                    )))
                }
                "cos" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'cos'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        self.eval_ast(&operands[0])?
                            .unwrap()
                            .as_number()
                            .unwrap()
                            .cos(),
                    )))
                }
                "tan" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'tan'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        self.eval_ast(&operands[0])?
                            .unwrap()
                            .as_number()
                            .unwrap()
                            .tan(),
                    )))
                }
                "abs" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'abs'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        self.eval_ast(&operands[0])?
                            .unwrap()
                            .as_number()
                            .unwrap()
                            .abs(),
                    )))
                }
                "floor" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'floor'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        self.eval_ast(&operands[0])?
                            .unwrap()
                            .as_number()
                            .unwrap()
                            .floor(),
                    )))
                }
                "ceil" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'ceil'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        self.eval_ast(&operands[0])?
                            .unwrap()
                            .as_number()
                            .unwrap()
                            .ceil(),
                    )))
                }
                "rand" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'rand'".to_string(),
                        ));
                    }
                    let min: f64 = self.eval_ast(&operands[0])?.unwrap().as_number().unwrap();
                    let max: f64 = self.eval_ast(&operands[1])?.unwrap().as_number().unwrap();

                    if min > max {
                        return Err(ErrorHandler::ParseError(
                            "Minimum value is greater than maximum value".to_string(),
                        ));
                    }

                    Ok(Some(VariableValue::Number(
                        min + (max - min) * rand::random::<f64>(),
                    )))
                }
                // Control flow and logic operators:
                "if" => {
                    if operands.len() < 2 {
                        return Err(ErrorHandler::ParseError(format!(
                            "Invalid syntax for '{}'",
                            op
                        )));
                    }
                    let condition: f64 = match self.eval_ast(&operands[0])? {
                        Some(val) => val.as_number().unwrap(),
                        None => {
                            return Err(ErrorHandler::ParseError("Invalid if syntax".to_string()))
                        }
                    };
                    if condition != 0.0 {
                        self.eval_ast(&operands[1])
                    } else {
                        let i: usize = 2;
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
                        }
                        Ok(None)
                    }
                }
                "switch" => {
                    if operands.len() < 2 {
                        return Err(ErrorHandler::ParseError(format!(
                            "Invalid syntax for '{}'",
                            op
                        )));
                    }
                    let variable: f64 = self.eval_ast(&operands[0])?.unwrap().as_number().unwrap();
                    let mut i: usize = 1;
                    let mut default_body: Option<&ASTNode> = None;
                    while i < operands.len() {
                        if let ASTNode::Operator(ref case_op, ref case_operands) = &operands[i] {
                            match case_op.as_str() {
                                "case" => {
                                    if case_operands.len() != 2 {
                                        return Err(ErrorHandler::ParseError(format!(
                                            "Invalid syntax for '{}'",
                                            case_op
                                        )));
                                    }
                                    let case_val: f64 = self
                                        .eval_ast(&case_operands[0])?
                                        .unwrap()
                                        .as_number()
                                        .unwrap();
                                    if case_val == variable {
                                        return self.eval_ast(&case_operands[1]);
                                    }
                                }
                                "default" => {
                                    if case_operands.len() != 1 {
                                        return Err(ErrorHandler::ParseError(format!(
                                            "Invalid syntax for '{}'",
                                            case_op
                                        )));
                                    }
                                    default_body = Some(&case_operands[0]);
                                }
                                _ => {
                                    return Err(ErrorHandler::ParseError(
                                        "Invalid switch syntax".to_string(),
                                    ))
                                }
                            }
                        } else {
                            return Err(ErrorHandler::ParseError(
                                "Invalid switch syntax".to_string(),
                            ));
                        }
                        i += 1;
                    }
                    if let Some(body) = default_body {
                        self.eval_ast(body)
                    } else {
                        Ok(None)
                    }
                }
                "zero?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'zero?'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap() == 0.0) as i32
                            as f64,
                    )))
                }
                "even?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'even?'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap() % 2.0 == 0.0)
                            as i32 as f64,
                    )))
                }
                "odd?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'odd?'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap() % 2.0 != 0.0)
                            as i32 as f64,
                    )))
                }
                "pos?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'pos?'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap() > 0.0) as i32
                            as f64,
                    )))
                }
                "neg?" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'neg?'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap() < 0.0) as i32
                            as f64,
                    )))
                }
                "eq?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'eq?'".to_string(),
                        ));
                    }
                    let val1: Option<VariableValue> = self.eval_ast(&operands[0])?;
                    let val2: Option<VariableValue> = self.eval_ast(&operands[1])?;
                    match (val1, val2) {
                        (Some(VariableValue::Number(num1)), Some(VariableValue::Number(num2))) => {
                            Ok(Some(VariableValue::Number((num1 == num2) as i32 as f64)))
                        }
                        (Some(VariableValue::Text(text1)), Some(VariableValue::Text(text2))) => {
                            Ok(Some(VariableValue::Number((text1 == text2) as i32 as f64)))
                        }
                        _ => {
                            return Err(ErrorHandler::ParseError("Invalid eq? syntax".to_string()))
                        }
                    }
                }
                "neq?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'neq?'".to_string(),
                        ));
                    }
                    let val1: Option<VariableValue> = self.eval_ast(&operands[0])?;
                    let val2: Option<VariableValue> = self.eval_ast(&operands[1])?;
                    match (val1, val2) {
                        (Some(VariableValue::Number(num1)), Some(VariableValue::Number(num2))) => {
                            Ok(Some(VariableValue::Number((num1 != num2) as i32 as f64)))
                        }
                        (Some(VariableValue::Text(text1)), Some(VariableValue::Text(text2))) => {
                            Ok(Some(VariableValue::Number((text1 != text2) as i32 as f64)))
                        }
                        _ => {
                            return Err(ErrorHandler::ParseError("Invalid neq? syntax".to_string()))
                        }
                    }
                }
                "lt?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'lt?'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap()
                            < self.eval_ast(&operands[1])?.unwrap().as_number().unwrap())
                            as i32 as f64,
                    )))
                }
                "lte?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'le?'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap()
                            <= self.eval_ast(&operands[1])?.unwrap().as_number().unwrap())
                            as i32 as f64,
                    )))
                }
                "gt?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'gt?'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap()
                            > self.eval_ast(&operands[1])?.unwrap().as_number().unwrap())
                            as i32 as f64,
                    )))
                }
                "gte?" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'ge?'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap()
                            >= self.eval_ast(&operands[1])?.unwrap().as_number().unwrap())
                            as i32 as f64,
                    )))
                }
                "and" => {
                    if operands.len() < 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'and'".to_string(),
                        ));
                    }
                    let mut result: f64 = 1.0;
                    for operand in operands {
                        let val: f64 = self.eval_ast(operand)?.unwrap().as_number().unwrap();
                        if val == 0.0 {
                            result = 0.0;
                            break;
                        }
                    }
                    Ok(Some(VariableValue::Number(result)))
                }
                "or" => {
                    if operands.len() < 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'or'".to_string(),
                        ));
                    }
                    let mut result: f64 = 0.0;
                    for operand in operands {
                        let val: f64 = self.eval_ast(operand)?.unwrap().as_number().unwrap();
                        if val != 0.0 {
                            result = 1.0;
                            break;
                        }
                    }
                    Ok(Some(VariableValue::Number(result)))
                }
                "not" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'not'".to_string(),
                        ));
                    }
                    Ok(Some(VariableValue::Number(
                        (self.eval_ast(&operands[0])?.unwrap().as_number().unwrap() == 0.0) as i32
                            as f64,
                    )))
                }
                // Variable operators:
                "let" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(format!(
                            "Invalid syntax for '{}'",
                            op
                        )));
                    }
                    if let ASTNode::Value(var) = &operands[0] {
                        let value: Option<VariableValue> = self.eval_ast(&operands[1])?;
                        if self.variables.contains_key(var) {
                            return Err(ErrorHandler::ParseError(format!(
                                "Variable '{}' already exists",
                                var
                            )));
                        }
                        self.variables.insert(var.clone(), value.unwrap());
                        Ok(None)
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
                        let value: Option<VariableValue> = self.eval_ast(&operands[1])?;
                        if !self.variables.contains_key(var) {
                            return Err(ErrorHandler::ParseError(format!(
                                "Variable '{}' not found",
                                var
                            )));
                        }
                        self.variables.insert(var.clone(), value.unwrap());
                        Ok(None)
                    } else {
                        Err(ErrorHandler::ParseError("Invalid set syntax".to_string()))
                    }
                }
                "get" => {
                    if let ASTNode::Value(var) = &operands[0] {
                        if let Some(val) = self.variables.get(var).cloned() {
                            Ok(Some(val))
                        } else {
                            Err(ErrorHandler::VariableNotFound(var.clone()))
                        }
                    } else {
                        Err(ErrorHandler::ParseError("Invalid get syntax".to_string()))
                    }
                }
                "del" => {
                    if let ASTNode::Value(var) = &operands[0] {
                        if self.variables.contains_key(var) {
                            self.variables.remove(var);
                            Ok(None)
                        } else {
                            Err(ErrorHandler::VariableNotFound(var.clone()))
                        }
                    } else {
                        Err(ErrorHandler::ParseError("Invalid del syntax".to_string()))
                    }
                }
                // Loop operators:
                "for" => {
                    if operands.len() != 4 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'for'".to_string(),
                        ));
                    }

                    if let ASTNode::Value(var) = &operands[0] {
                        let start: f64 = match self.eval_ast(&operands[1])? {
                            Some(VariableValue::Number(val)) => val,
                            _ => {
                                return Err(ErrorHandler::ParseError(
                                    "Invalid for syntax".to_string(),
                                ))
                            }
                        };
                        let end: f64 = match self.eval_ast(&operands[2])? {
                            Some(VariableValue::Number(val)) => val,
                            _ => {
                                return Err(ErrorHandler::ParseError(
                                    "Invalid for syntax".to_string(),
                                ))
                            }
                        };
                        let body: &ASTNode = &operands[3];

                        let mut _result: Option<f64> = None;
                        for i in (start as i32)..(end as i32) {
                            self.variables
                                .insert(var.clone(), VariableValue::Number(i as f64));
                            _result = self.eval_ast(body)?.map(|val| val.as_number().unwrap());
                        }

                        Ok(None)
                    } else {
                        Err(ErrorHandler::ParseError("Invalid for syntax".to_string()))
                    }
                }
                // String operators:
                "concat" => {
                    let mut result: String = String::new();
                    for operand in operands {
                        match self.eval_ast(operand)? {
                            Some(VariableValue::Text(val)) => result.push_str(&val),
                            Some(VariableValue::Number(val)) => result.push_str(&val.to_string()),
                            _ => {
                                return Err(ErrorHandler::ParseError(
                                    "Invalid concat syntax".to_string(),
                                ))
                            }
                        };
                    }
                    Ok(Some(VariableValue::Text(result)))
                }
                "len" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'len'".to_string(),
                        ));
                    }
                    match self.eval_ast(&operands[0])? {
                        Some(VariableValue::Text(val)) => {
                            Ok(Some(VariableValue::Number(val.len() as f64)))
                        }
                        _ => Err(ErrorHandler::ParseError("Invalid len syntax".to_string())),
                    }
                }
                "substring" => {
                    if operands.len() != 3 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'substring'".to_string(),
                        ));
                    }
                    let text: String = match self.eval_ast(&operands[0])? {
                        Some(VariableValue::Text(val)) => val,
                        _ => {
                            return Err(ErrorHandler::ParseError(
                                "Invalid substring syntax".to_string(),
                            ))
                        }
                    };
                    let start: f64 = match self.eval_ast(&operands[1])? {
                        Some(VariableValue::Number(val)) => val,
                        _ => {
                            return Err(ErrorHandler::ParseError(
                                "Invalid substring syntax".to_string(),
                            ))
                        }
                    };
                    let end: f64 = match self.eval_ast(&operands[2])? {
                        Some(VariableValue::Number(val)) => val,
                        _ => {
                            return Err(ErrorHandler::ParseError(
                                "Invalid substring syntax".to_string(),
                            ))
                        }
                    };
                    Ok(Some(VariableValue::Text(
                        text.chars()
                            .skip(start as usize)
                            .take((end - start + 1.0) as usize)
                            .collect(),
                    )))
                }
                "strip" => {
                    if operands.len() != 2 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'strip'".to_string(),
                        ));
                    }
                    let text: String = match self.eval_ast(&operands[0])? {
                        Some(VariableValue::Text(val)) => val,
                        _ => {
                            return Err(ErrorHandler::ParseError(
                                "Invalid strip syntax".to_string(),
                            ))
                        }
                    };
                    let chars: String = match self.eval_ast(&operands[1])? {
                        Some(VariableValue::Text(val)) => val,
                        _ => {
                            return Err(ErrorHandler::ParseError(
                                "Invalid strip syntax".to_string(),
                            ))
                        }
                    };
                    Ok(Some(VariableValue::Text(
                        text.chars().filter(|c| !chars.contains(*c)).collect(),
                    )))
                }
                "replace" => {
                    if operands.len() != 3 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'replace'".to_string(),
                        ));
                    }
                    let text: String = match self.eval_ast(&operands[0])? {
                        Some(VariableValue::Text(val)) => val,
                        _ => {
                            return Err(ErrorHandler::ParseError(
                                "Invalid replace syntax".to_string(),
                            ))
                        }
                    };
                    let old_char: String = match self.eval_ast(&operands[1])? {
                        Some(VariableValue::Text(val)) => val,
                        _ => {
                            return Err(ErrorHandler::ParseError(
                                "Invalid replace syntax".to_string(),
                            ))
                        }
                    };
                    let new_char: String = match self.eval_ast(&operands[2])? {
                        Some(VariableValue::Text(val)) => val,
                        _ => {
                            return Err(ErrorHandler::ParseError(
                                "Invalid replace syntax".to_string(),
                            ))
                        }
                    };

                    Ok(Some(VariableValue::Text(
                        text.chars()
                            .map(|c| {
                                if c == old_char.chars().next().unwrap() {
                                    new_char.chars().next().unwrap()
                                } else {
                                    c
                                }
                            })
                            .collect(),
                    )))
                }
                "upper" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'upper'".to_string(),
                        ));
                    }
                    match self.eval_ast(&operands[0])? {
                        Some(VariableValue::Text(val)) => {
                            Ok(Some(VariableValue::Text(val.to_uppercase())))
                        }
                        _ => Err(ErrorHandler::ParseError("Invalid upper syntax".to_string())),
                    }
                }
                "lower" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'lower'".to_string(),
                        ));
                    }
                    match self.eval_ast(&operands[0])? {
                        Some(VariableValue::Text(val)) => {
                            Ok(Some(VariableValue::Text(val.to_lowercase())))
                        }
                        _ => Err(ErrorHandler::ParseError("Invalid lower syntax".to_string())),
                    }
                }
                // Extraneous operators:
                "print" => {
                    let mut output: String = String::new();
                    for operand in operands {
                        match self.eval_ast(operand)? {
                            Some(VariableValue::Number(val)) => {
                                output.push_str(&format!("{} ", val))
                            }
                            Some(VariableValue::Text(val)) => output.push_str(&format!("{} ", val)),
                            None => {
                                return Err(ErrorHandler::ParseError(
                                    "Invalid print syntax".to_string(),
                                ))
                            }
                        };
                    }
                    self.output.push(output.trim_end().to_string());
                    Ok(None)
                }
                "exit" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(
                            "Invalid number of operands for 'exit'".to_string(),
                        ));
                    }
                    let code: f64 = match self.eval_ast(&operands[0])? {
                        Some(VariableValue::Number(val)) => val,
                        _ => {
                            return Err(ErrorHandler::ParseError("Invalid exit syntax".to_string()))
                        }
                    };
                    println!("Exiting with code {}", code as i32);
                    exit(code as i32)
                }
                "debug" => {
                    if !self.variables.is_empty() {
                        println!("Variables:");
                        for (var, val) in &self.variables {
                            println!("{:?}: {:?}", var, val);
                        }
                    }

                    if !self.functions.is_empty() {
                        println!("Functions:");
                        for (func, f) in &self.functions {
                            println!("{:?}: {:?}", func, f);
                        }
                    }

                    Ok(None)
                }
                // Functions
                "base" => {
                    if operands.len() != 1 {
                        return Err(ErrorHandler::ParseError(format!(
                            "Invalid syntax for '{}'",
                            op
                        )));
                    }
                    self.eval_ast(&operands[0])
                }
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

                            self.functions.insert(
                                name.clone(),
                                Function {
                                    params,
                                    body: operands[2].clone(),
                                },
                            );
                            Ok(None)
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
                // This falls under function operators (also handles unknown keywords)
                _ => {
                    if let Some(func) = self.functions.get(op) {
                        if operands.len() != func.params.len() {
                            return Err(ErrorHandler::ParseError(format!(
                                "Invalid number of arguments for function '{}'",
                                op
                            )));
                        }

                        let mut local_interpreter: Interpreter = self.clone();
                        let mut results: Vec<f64> = Vec::new();

                        for arg in operands {
                            if let Some(val) = local_interpreter.eval_ast(arg)? {
                                if let VariableValue::Number(num) = val {
                                    results.push(num);
                                }
                            }
                        }

                        for (param, result) in func.params.iter().zip(results) {
                            local_interpreter
                                .variables
                                .insert(param.clone(), VariableValue::Number(result));
                        }

                        let result: Option<VariableValue> =
                            local_interpreter.eval_ast(&func.body)?;

                        self.output.extend(local_interpreter.output);

                        Ok(result)
                    } else {
                        Err(ErrorHandler::FunctionOrOperatorNotFound(op.clone()))
                    }
                }
            },
        }
    }

    pub fn interp(&mut self, path: PathBuf) -> Result<(), ErrorHandler> {
        let contents: String =
            read_to_string(&path).map_err(|e| ErrorHandler::ParseError(e.to_string()))?;
        let lines: Lines = contents.lines();

        let mut line_num: i32 = 1;
        let mut expression: String = String::new();
        let mut open_parens: i32 = 0;

        for line in lines {
            let line: &str = line.trim();

            if line.is_empty() || line.starts_with("//") {
                line_num += 1;
                continue;
            }

            for char in line.chars() {
                if char == '(' {
                    open_parens += 1;
                } else if char == ')' {
                    open_parens -= 1;
                }
            }

            expression.push_str(line);
            expression.push(' ');

            if open_parens == 0 && !expression.is_empty() {
                match self.eval(&expression) {
                    Ok(result) => {
                        println!("{}. {}: {}", line_num, expression.trim(), result);
                    }
                    Err(e) => {
                        println!("{}", e);
                        return Err(e);
                    }
                }

                expression.clear();
            }

            line_num += 1;
        }

        if !expression.is_empty() {
            return Err(ErrorHandler::ParseError(
                "Unclosed expression at the end of the file".to_string(),
            ));
        }

        Ok(())
    }
}
