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
    LabelNotFound(String),
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorHandler::DivisionByZero => write!(f, "Error: Division by zero"),
            ErrorHandler::UnknownOperator(op) => write!(f, "Error: Unknown operator '{}'", op),
            ErrorHandler::ParseError(err) => write!(f, "Error: Parse error - {}", err),
            ErrorHandler::VariableNotFound(var) => write!(f, "Error: Variable '{}' not found", var),
            ErrorHandler::LabelNotFound(label) => write!(f, "Error: Label '{}' not found", label),
        }
    }
}

#[derive(Debug)]
pub struct Tape {
    nodes: Vec<ASTNode>,
    pc: usize, // program counter
    labels: HashMap<String, usize>,
    pc_stack: Vec<usize>,
}

impl Tape {
    pub fn new() -> Self {
        Tape {
            nodes: Vec::new(),
            pc: 0,
            labels: HashMap::new(),
            pc_stack: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: ASTNode) {
        if let ASTNode::Label(label) = &node {
            self.labels.insert(label.clone(), self.nodes.len());
        } else {
            self.nodes.push(node);
        }
    }

    pub fn current_node(&self) -> Option<&ASTNode> {
        self.nodes.get(self.pc)
    }

    pub fn advance(&mut self) {
        self.pc += 1;
    }

    pub fn jump_to_label(&mut self, label: &str) -> Result<(), ErrorHandler> {
        if let Some(&pos) = self.labels.get(label) {
            self.pc = pos;
            Ok(())
        } else {
            Err(ErrorHandler::LabelNotFound(label.to_string()))
        }
    }
}

#[derive(Debug)]
pub struct Interpreter {
    variables: HashMap<String, f64>,
    tape: Tape,
}

impl Clone for Tape {
    fn clone(&self) -> Self {
        Tape {
            nodes: self.nodes.clone(),
            pc: self.pc,
            labels: self.labels.clone(),
            pc_stack: self.pc_stack.clone(),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            tape: Tape::new(),
        }
    }

    pub fn collect_labels(&mut self, lines: std::str::Lines) -> Result<(), ErrorHandler> {
        let mut line_num = 1;

        for line in lines {
            let line = line.trim();

            if line.is_empty() || line.starts_with("//") {
                line_num += 1;
                continue;
            }

            if line.starts_with("(") {
                let tokens: Vec<String> = tokenize(line);
                if let Ok((ast, _)) = parse(&tokens) {
                    if let ASTNode::Label(label) = &ast {
                        self.tape.add_node(ast.clone());
                    }
                }
            }

            line_num += 1;
        }

        Ok(())
    }

    pub fn eval(&mut self, expr: &str) -> Result<(), ErrorHandler> {
        let tokens: Vec<String> = tokenize(expr);
        let (ast, _) = parse(&tokens)?;

        self.tape.add_node(ast);

        while self.tape.pc < self.tape.nodes.len() {
            let node = self
                .tape
                .current_node()
                .cloned()
                .ok_or(ErrorHandler::ParseError("No current node".to_string()))?;
            match self.eval_ast(&node, 0) {
                Ok(result) => {
                    if let Some(val) = result {
                        println!("{}", val);
                    }
                    self.tape.advance();
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn eval_ast(&mut self, node: &ASTNode, depth: usize) -> Result<Option<f64>, ErrorHandler> {
        match node {
            ASTNode::NoOp => Ok(None),
            ASTNode::Value(val) => {
                if val == "True" {
                    Ok(Some(1.0))
                } else if val == "False" {
                    Ok(Some(0.0))
                } else if let Ok(num) = val.parse::<f64>() {
                    Ok(Some(num))
                } else if let Some(&num) = self.variables.get(val) {
                    Ok(Some(num))
                } else {
                    Err(ErrorHandler::VariableNotFound(val.clone()))
                }
            }

            ASTNode::Operator(op, operands) => {
                match op.as_str() {
                    /*   
                    Arithmetic operators:
                    */
                    "+" | "add" => {
                        let mut result: f64 = 0.0;
                        for operand in operands {
                            if let Some(value) = self.eval_ast(operand, depth)? {
                                result += value;
                            }
                        }
                        Ok(Some(result))
                    }
                    "-" | "subtract" | "sub" => {
                        if operands.is_empty() {
                            return Err(ErrorHandler::ParseError("Empty subtraction".to_string()));
                        }
                        let mut result: f64 = self.eval_ast(&operands[0], depth)?.unwrap_or(0.0);
                        for operand in &operands[1..] {
                            result -= self.eval_ast(operand, depth)?.unwrap_or(0.0);
                        }
                        Ok(Some(result))
                    }
                    "*" | "multiply" | "mul" => {
                        let mut result: f64 = 1.0;
                        for operand in operands {
                            result *= self.eval_ast(operand, depth)?.unwrap_or(0.0);
                        }
                        Ok(Some(result))
                    }
                    "/" | "divide" | "div" => {
                        if operands.is_empty() {
                            return Err(ErrorHandler::ParseError("Empty division".to_string()));
                        }
                        let mut result: f64 = match self.eval_ast(&operands[0], depth)? {
                            Some(val) => val,
                            None => return Err(ErrorHandler::ParseError("Invalid division syntax".to_string())),
                        };
                        for operand in &operands[1..] {
                            let divisor: f64 = match self.eval_ast(operand, depth)? {
                                Some(val) => val,
                                None => return Err(ErrorHandler::ParseError("Invalid division syntax".to_string())),
                            };
                            if divisor == 0.0 {
                                return Err(ErrorHandler::DivisionByZero);
                            }
                            result /= divisor;
                        }
                        Ok(Some(result))
                    }
                    "%" | "modulo" | "mod" => {
                        if operands.is_empty() {
                            return Err(ErrorHandler::ParseError("Empty modulo".to_string()));
                        }
                        let mut result: f64 = match self.eval_ast(&operands[0], depth)? {
                            Some(val) => val,
                            None => return Err(ErrorHandler::ParseError("Invalid modulo syntax".to_string())),
                        };
                        for operand in &operands[1..] {
                            let divisor: f64 = match self.eval_ast(operand, depth)? {
                                Some(val) => val,
                                None => return Err(ErrorHandler::ParseError("Invalid modulo syntax".to_string())),
                            };
                            if divisor == 0.0 {
                                return Err(ErrorHandler::DivisionByZero);
                            }
                            result %= divisor;
                        }
                        Ok(Some(result))
                    }
                    /*
                    Other mathematical operators:
                    */
                    "max" => {
                        let mut max_val: f64 = f64::MIN;
                        for operand in operands {
                            let val: f64 = match self.eval_ast(operand, depth)? {
                                Some(val) => val,
                                None => return Err(ErrorHandler::ParseError("Invalid max syntax".to_string())),
                            };
                            if val > max_val {
                                max_val = val;
                            }
                        }
                        Ok(Some(max_val))
                    }
                    "min" => {
                        let mut min_val: f64 = f64::MAX;
                        for operand in operands {
                            let val: f64 = match self.eval_ast(operand, depth)? {
                                Some(val) => val,
                                None => return Err(ErrorHandler::ParseError("Invalid min syntax".to_string())),
                            };
                            if val < min_val {
                                min_val = val;
                            }
                        }
                        Ok(Some(min_val))
                    }
                    "pow" => {
                        if operands.len() != 2 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'pow'".to_string(),
                            ));
                        }
                        let base: f64 = match self.eval_ast(&operands[0], depth)? {
                            Some(val) => val,
                            None => return Err(ErrorHandler::ParseError("Invalid pow syntax".to_string())),
                        };
                        let exp: f64 = match self.eval_ast(&operands[1], depth)? {
                            Some(val) => val,
                            None => return Err(ErrorHandler::ParseError("Invalid pow syntax".to_string())),
                        };
                        Ok(Some(base.powf(exp)))
                    }
                    "sqrt" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'sqrt'".to_string(),
                            ));
                        }
                        let val: f64 = match self.eval_ast(&operands[0], depth)? {
                            Some(val) => val,
                            None => return Err(ErrorHandler::ParseError("Invalid sqrt syntax".to_string())),
                        };
                        if val < 0.0 {
                            return Err(ErrorHandler::ParseError("Invalid sqrt syntax".to_string()));
                        }
                        Ok(Some(val.sqrt()))
                    }
                    "sin" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'sin'".to_string(),
                            ));
                        }
                        let val: f64 = self.eval_ast(&operands[0], depth)?.unwrap();
                        Ok(Some(val.sin()))
                    }
                    "cos" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'cos'".to_string(),
                            ));
                        }
                        let val: f64 = self.eval_ast(&operands[0], depth)?.unwrap();
                        Ok(Some(val.cos()))
                    }
                    "tan" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'tan'".to_string(),
                            ));
                        }
                        let val: f64 = self.eval_ast(&operands[0], depth)?.unwrap();
                        Ok(Some(val.tan()))
                    }
                    "abs" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'abs'".to_string(),
                            ));
                        }
                        let val: f64 = self.eval_ast(&operands[0], depth)?.unwrap();
                        Ok(Some(val.abs()))
                    }
                    /*
                    Logical (and boolean related) operators:
                    */
                    "if" => {
                        if operands.len() < 2 {
                            return Err(ErrorHandler::ParseError(format!(
                                "Invalid syntax for '{}'",
                                op
                            )));
                        }
                        let condition: f64 = match self.eval_ast(&operands[0], depth)? {
                            Some(val) => val,
                            None => return Err(ErrorHandler::ParseError("Invalid condition syntax".to_string())),
                        };
                        if condition != 0.0 {
                            self.eval_ast(&operands[1], depth)
                        } else {
                            let i = 2;
                            while i < operands.len() {
                                if let ASTNode::Operator(ref cond_op, ref cond_operands) =
                                    &operands[i]
                                {
                                    match cond_op.as_str() {
                                        "else" => {
                                            if cond_operands.len() != 1 {
                                                return Err(ErrorHandler::ParseError(format!(
                                                    "Invalid syntax for '{}'",
                                                    cond_op
                                                )));
                                            }
                                            return self.eval_ast(&cond_operands[0], depth);
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
                    "zero?" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'zero?'".to_string(),
                            ));
                        }
                        Ok(Some((self.eval_ast(&operands[0], depth)? == Some(0.0)) as i32 as f64))
                    }
                    "even?" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'even?'".to_string(),
                            ));
                        }
                        let val: f64 = self.eval_ast(&operands[0], depth)?.unwrap();
                        Ok(Some((val % 2.0 == 0.0) as i32 as f64))
                    }
                    "odd?" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'odd?'".to_string(),
                            ));
                        }
                        let val: f64 = self.eval_ast(&operands[0], depth)?.unwrap();
                        Ok(Some((val % 2.0 != 0.0) as i32 as f64))
                    }
                    "pos?" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'pos?'".to_string(),
                            ));
                        }
                        let val: f64 = self.eval_ast(&operands[0], depth)?.unwrap();
                        Ok(Some((val > 0.0) as i32 as f64))
                    }
                    "neg?" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'neg?'".to_string(),
                            ));
                        }
                        let val: f64 = self.eval_ast(&operands[0], depth)?.unwrap();
                        Ok(Some((val < 0.0) as i32 as f64))
                    }
                    "eq?" => {
                        if operands.len() != 2 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'eq?'".to_string(),
                            ));
                        }
                        Ok(Some((self.eval_ast(&operands[0], depth)?
                            == self.eval_ast(&operands[1], depth)?)
                            as i32 as f64))
                    }
                    "neq?" => {
                        if operands.len() != 2 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'neq?'".to_string(),
                            ));
                        }
                        Ok(Some((self.eval_ast(&operands[0], depth)?
                            != self.eval_ast(&operands[1], depth)?)
                            as i32 as f64))
                    }
                    "lt?" => {
                        if operands.len() != 2 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'lt?'".to_string(),
                            ));
                        }
                        Ok(Some((self.eval_ast(&operands[0], depth)?
                            < self.eval_ast(&operands[1], depth)?) as i32
                            as f64))
                    }
                    "lte?" => {
                        if operands.len() != 2 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'le?'".to_string(),
                            ));
                        }
                        Ok(Some((self.eval_ast(&operands[0], depth)?
                            <= self.eval_ast(&operands[1], depth)?)
                            as i32 as f64))
                    }
                    "gt?" => {
                        if operands.len() != 2 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'gt?'".to_string(),
                            ));
                        }
                        Ok(Some((self.eval_ast(&operands[0], depth)?
                            > self.eval_ast(&operands[1], depth)?) as i32
                            as f64))
                    }
                    "gte?" => {
                        if operands.len() != 2 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'ge?'".to_string(),
                            ));
                        }
                        Ok(Some((self.eval_ast(&operands[0], depth)?
                            >= self.eval_ast(&operands[1], depth)?)
                            as i32 as f64))
                    }
                    "and" => {
                        if operands.len() < 2 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'and'".to_string(),
                            ));
                        }
                        let mut result: f64 = 1.0;
                        for operand in operands {
                            let val: f64 = match self.eval_ast(operand, depth)? {
                                Some(value) => value,
                                None => return Err(ErrorHandler::ParseError(
                                    "Invalid operand for 'and'".to_string(),
                                )),
                            };
                            if val == 0.0 {
                                result = 0.0;
                                break;
                            }
                        }
                        Ok(Some(result))
                    }
                    "or" => {
                        if operands.len() < 2 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'or'".to_string(),
                            ));
                        }
                        let mut result: f64 = 0.0;
                        for operand in operands {
                            let val: f64 = match self.eval_ast(operand, depth)? {
                                Some(value) => value,
                                None => return Err(ErrorHandler::ParseError(
                                    "Invalid operand for 'or'".to_string(),
                                )),
                            };
                            if val != 0.0 {
                                result = 1.0;
                                break;
                            }
                        }
                        Ok(Some(result))
                    }
                    "not" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'not'".to_string(),
                            ));
                        }
                        Ok(Some((self.eval_ast(&operands[0], depth)? == Some(0.0)) as i32 as f64))
                    }
                    /*
                    Variable related operators:
                    */
                    "let" => {
                        if operands.len() != 2 {
                            return Err(ErrorHandler::ParseError(format!(
                                "Invalid syntax for '{}'",
                                op
                            )));
                        }
                        if let ASTNode::Value(var) = &operands[0] {
                            let value: f64 = match self.eval_ast(&operands[1], depth)? {
                                Some(val) => val,
                                None => return Err(ErrorHandler::ParseError("Invalid let syntax".to_string())),
                            };
                            if self.variables.contains_key(var) {
                                return Err(ErrorHandler::ParseError(format!(
                                    "Variable '{}' already exists",
                                    var
                                )));
                            }
                            self.variables.insert(var.clone(), value);
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
                            let value: f64 = match self.eval_ast(&operands[1], depth)? {
                                Some(val) => val,
                                None => return Err(ErrorHandler::ParseError("Invalid set syntax".to_string())),
                            };
                            if !self.variables.contains_key(var) {
                                return Err(ErrorHandler::ParseError(format!(
                                    "Variable '{}' not found",
                                    var
                                )));
                            }
                            self.variables.insert(var.clone(), value);
                            Ok(Some(value))
                        } else {
                            Err(ErrorHandler::ParseError("Invalid set syntax".to_string()))
                        }
                    }
                    "get" => {
                        if let ASTNode::Value(var) = &operands[0] {
                            if let Some(&val) = self.variables.get(var) {
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
                    /*
                    Function-ish related operators:
                    */
                    "proc" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(format!(
                                "Invalid syntax for '{}'",
                                op
                            )));
                        }
                        if let ASTNode::Value(label) = &operands[0] {
                            self.tape.pc_stack.push(self.tape.pc);
                            self.tape.jump_to_label(label)?;
                            Ok(None)
                        } else {
                            Err(ErrorHandler::ParseError("Invalid call syntax".to_string()))
                        }
                    }
                    "ret" => {
                        if operands.len() != 0 {
                            return Err(ErrorHandler::ParseError(format!(
                                "Invalid syntax for '{}'",
                                op
                            )));
                        }
                        if let Some(pc) = self.tape.pc_stack.pop() {
                            self.tape.pc = pc;
                            Ok(None)
                        } else {
                            Err(ErrorHandler::ParseError("No return address".to_string()))
                        }
                    }
                    "base" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(format!(
                                "Invalid syntax for '{}'",
                                op
                            )));
                        }
                        self.eval_ast(&operands[0], depth)
                    }
                    /*
                    Label and jump operators:
                    */
                    "label" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(format!(
                                "Invalid syntax for '{}'",
                                op
                            )));
                        }
                        if let ASTNode::Value(label) = &operands[0] {
                            self.tape.labels.insert(label.clone(), self.tape.pc);
                            Ok(None)
                        } else {
                            Err(ErrorHandler::ParseError("Invalid label syntax".to_string()))
                        }
                    }
                    "jump" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(format!(
                                "Invalid syntax for '{}'",
                                op
                            )));
                        }
                        if let ASTNode::Value(label) = &operands[0] {
                            self.tape.jump_to_label(label)?;
                            Ok(None)
                        } else {
                            Err(ErrorHandler::ParseError("Invalid jump syntax".to_string()))
                        }
                    }                    
                    /*
                    Other operators:
                    */
                    "exit" => {
                        if operands.len() != 1 {
                            return Err(ErrorHandler::ParseError(
                                "Invalid number of operands for 'exit'".to_string(),
                            ));
                        }
                        let code: f64 = match self.eval_ast(&operands[0], depth)? {
                            Some(value) => value,
                            None => 0.0,
                        };
                        std::process::exit(code as i32);
                    }
                    "print" => {
                        for operand in operands {
                            if let Some(result) = self.eval_ast(operand, depth)? {
                                println!("{}", result);
                            }
                        }
                        Ok(None)
                    }
                    "debug" => {
                        if !self.variables.is_empty() {
                            println!("Variables:");
                            for (var, val) in &self.variables {
                                println!("{}: {}", var, val);
                            }
                        }

                        if !self.tape.labels.is_empty() {
                            println!("Labels:");
                            for (label, pos) in &self.tape.labels {
                                println!("{}: {}", label, pos);
                            }
                        }

                        if !self.tape.pc_stack.is_empty() {
                            println!("PC Stack:");
                            for pc in &self.tape.pc_stack {
                                println!("{}", pc);
                            }
                        }

                        Ok(None)
                    }
                    _ => Err(ErrorHandler::UnknownOperator(op.clone())),
                }
            }
            ASTNode::Label(_) => Ok(None),
            ASTNode::Jump(label) => {
                self.tape.jump_to_label(label)?;
                Ok(None)
            }
        }
    }

    pub fn interp(&mut self, path: PathBuf) -> Result<(), ErrorHandler> {
        let contents = read_to_string(&path).map_err(|e| ErrorHandler::ParseError(e.to_string()))?;
        let lines = contents.lines();

        self.collect_labels(lines.clone())?;

        let mut line_num = 1;

        for line in lines {
            let line = line.trim();

            if line.is_empty() || line.starts_with("//") {
                line_num += 1;
                continue;
            }

            if line.starts_with("(") {
                match self.eval(line) {
                    Ok(_) => {
                        println!("{}. {}: OK", line_num, line);
                    }
                    Err(e) => {
                        println!("{:?}", e);
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
pub enum ASTNode {
    Operator(String, Vec<ASTNode>),
    Value(String),
    NoOp,
    Label(String),
    Jump(String),
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

    let mut index = 0;

    if tokens[index] != "(" {
        return Err(ErrorHandler::ParseError("Expected '('. Good luck!".to_string()));
    }

    index += 1;

    if index < tokens.len() && tokens[index] == ")" {
        return Ok((ASTNode::NoOp, index + 1));
    }

    let operator = tokens[index].clone();
    index += 1;

    if operator == "label" {
        if index >= tokens.len() || tokens[index] == ")" {
            return Err(ErrorHandler::ParseError("Expected label name".to_string()));
        }
        let label = tokens[index].clone();
        index += 1;

        if index >= tokens.len() || tokens[index] != ")" {
            return Err(ErrorHandler::ParseError("Expected ')' after label name".to_string()));
        }
        return Ok((ASTNode::Label(label), index + 1));
    }

    let mut operands = Vec::new();

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