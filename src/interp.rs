use std::{collections::HashMap, fs::read_to_string, path::PathBuf, fmt};

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
        let args: Vec<&str> = expr.trim_matches(|p: char| p == '(' || p == ')')
                                  .split_whitespace()
                                  .collect();
        match &args[..] {
            // Empty expression e.g. ()
            &[] => Ok("0".to_string()),
            // Handling expressions with operators
            &[op, ref rest @ ..] if !rest.is_empty() => {
                match op {
                    "print" => {
                        for arg in rest {
                            if let Ok(num) = arg.parse::<i32>() {
                                println!("{}", num);
                            } else if let Some(&val) = self.variables.get(&arg.to_string()) {
                                println!("{}", val);
                            } else {
                                println!("{}", arg.trim_matches('"'));
                            }
                        }
                        return Ok("0".to_string());
                    }
                    "let" => {
                        if rest.len() != 2 {
                            return Err(ErrorHandler::ParseError(format!("Invalid let syntax in '{}'", expr)));
                        }
                        let var = rest[0].to_string();
                        let val = rest[1].parse::<i32>().map_err(|_| ErrorHandler::ParseError(format!("Invalid number '{}'", rest[1])))?;
                        self.variables.insert(var, val);
                        return Ok(val.to_string());
                    }
                    "set" => {
                        if rest.len() != 2 {
                            return Err(ErrorHandler::ParseError(format!("Invalid set syntax in '{}'", expr)));
                        }
                        let var = rest[0].to_string();
                        let val = rest[1].parse::<i32>().map_err(|_| ErrorHandler::ParseError(format!("Invalid number '{}'", rest[1])))?;
                        if !self.variables.contains_key(&var) {
                            return Err(ErrorHandler::VariableNotFound(var));
                        }
                        self.variables.insert(var, val);
                        return Ok(val.to_string());
                    }
                    "get" => {
                        if rest.len() != 1 {
                            return Err(ErrorHandler::ParseError(format!("Invalid get syntax in '{}'", expr)));
                        }
                        let var = rest[0].to_string();
                        match self.variables.get(&var) {
                            Some(&val) => return Ok(val.to_string()),
                            None => return Err(ErrorHandler::VariableNotFound(var)),
                        }
                    }
                    _ => {
                        let args: Result<Vec<i32>, _> = rest.iter().map(|&arg| {
                            if let Ok(num) = arg.parse::<i32>() {
                                Ok(num)
                            } else if let Some(&val) = self.variables.get(arg) {
                                Ok(val)
                            } else {
                                Err(ErrorHandler::ParseError(format!("Failed to parse args in '{}'", expr)))
                            }
                        }).collect();
                        let args = match args {
                            Ok(nums) => nums,
                            Err(_) => return Err(ErrorHandler::ParseError(format!("Failed to parse args in '{}'", expr))),
                        };

                        let result: i32 = match op {
                            "+" | "add" => args.iter().sum(),
                            "-" | "subtract" => args.iter().skip(1).fold(args[0], |acc, &num| acc - num),
                            "*" | "multiply" => args.iter().product(),
                            "/" | "divide" => {
                                if args.iter().skip(1).any(|&num| num == 0) {
                                    return Err(ErrorHandler::DivisionByZero);
                                }
                                args.iter().skip(1).fold(args[0], |acc, &num| acc / num)
                            },
                            "%" => {
                                if args.iter().skip(1).any(|&num| num == 0) {
                                    return Err(ErrorHandler::DivisionByZero);
                                }
                                args.iter().skip(1).fold(args[0], |acc, &num| acc % num)
                            },
                            "max" => *args.iter().max().unwrap(),
                            "min" => *args.iter().min().unwrap(),
                            "pow" => args[0].pow(args[1] as u32),
                            "sqrt" => (args[0] as f64).sqrt() as i32,
                            "sin" => (args[0] as f64).sin() as i32,
                            "cos" => (args[0] as f64).cos() as i32,
                            "tan" => (args[0] as f64).tan() as i32,
                            "abs" => (args[0] as f64).abs() as i32,
                            "zero?" => (args[0] == 0) as i32,
                            "even?" => (args[0] % 2 == 0) as i32,
                            "odd?" => (args[0] % 2 != 0) as i32,
                            "pos?" => (args[0] > 0) as i32,
                            "neg?" => (args[0] < 0) as i32,
                            "eq?" => (args[0] == args[1]) as i32,
                            "neq?" => (args[0] != args[1]) as i32,
                            "lt?" => (args[0] < args[1]) as i32,
                            "gt?" => (args[0] > args[1]) as i32,
                            "lte?" => (args[0] <= args[1]) as i32,
                            "gte?" => (args[0] >= args[1]) as i32,
                            "and" => args.iter().all(|&num| num != 0) as i32,
                            "or" => args.iter().any(|&num| num != 0) as i32,
                            "not" => (args[0] == 0) as i32,
                            // "if" => {
                            //     if args[0] != 0 {
                            //         args[1]
                            //     } else {
                            //         args[2]
                            //     }
                            // }
                            // "elif" => {
                            //     if args[0] != 0 {
                            //         args[1]
                            //     } else if args[2] != 0 {
                            //         args[3]
                            //     } else {
                            //         args[4]
                            //     }
                            // }
                            // "else" => args[0],
                            _ => return Err(ErrorHandler::UnknownOperator(op.to_string())),
                        };
                        Ok(result.to_string())
                    }
                }
            }
            // Unknown operator
            _ => Err(ErrorHandler::UnknownOperator(args.join(" "))),
        }
    }

    pub fn interp(&mut self, path: PathBuf) -> Result<(), ErrorHandler> {
        // Read the contents of the file
        let contents: String = read_to_string(&path).map_err(|e| ErrorHandler::ParseError(e.to_string()))?;

        // Split the contents of the file by newlines
        let lines: std::str::Lines = contents.lines();

        let mut line_num: i32 = 1;

        // Iterate over each line
        for line in lines {
            // Remove leading and trailing whitespace
            let line = line.trim();

            // If the line is empty, skip it
            if line.is_empty() {
                line_num += 1;
                continue;
            }

            // If the line starts with a //, it is a comment
            if line.starts_with("//") {
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