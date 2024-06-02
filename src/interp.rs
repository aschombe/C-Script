use std::{fs::read_to_string, path::PathBuf, fmt};

// Structure for error types
#[derive(Debug)]
pub enum ErrorHandler {
    DivisionByZero,
    UnknownArithmetic(String),
    UnknownOperator(String),
    ParseError(String),
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorHandler::DivisionByZero => write!(f, "Error: Division by zero"),
            ErrorHandler::UnknownArithmetic(op) => write!(f, "Error: Unknown arithmetic operation '{}'", op),
            ErrorHandler::UnknownOperator(op) => write!(f, "Error: Unknown operator '{}'", op),
            ErrorHandler::ParseError(err) => write!(f, "Error: Parse error - {}", err),
        }
    }
}

pub fn eval(expr: &str) -> Result<i32, ErrorHandler> {
    let args: Vec<&str> = expr.trim_matches(|p: char| p == '(' || p == ')')
                              .split_whitespace()
                              .collect();
    match &args[..] {
        // Empty expression e.g. ()
        &[] => Ok(0),
        // Arithmetic expression with multiple arguments e.g. (+ 1 2 3)
        &[op, ref rest @ ..] if !rest.is_empty() => {
            let numbers: Result<Vec<i32>, _> = rest.iter().map(|&arg| arg.parse::<i32>()).collect();
            let numbers = match numbers {
                Ok(nums) => nums,
                Err(_) => return Err(ErrorHandler::ParseError(format!("Failed to parse numbers in '{}'", expr))),
            };

            let result: i32 = match op {
                "+" => numbers.iter().sum(),
                "-" => numbers.iter().skip(1).fold(numbers[0], |acc, &num| acc - num),
                "*" => numbers.iter().product(),
                "/" => {
                    if numbers.iter().skip(1).any(|&num| num == 0) {
                        return Err(ErrorHandler::DivisionByZero);
                    }
                    numbers.iter().skip(1).fold(numbers[0], |acc, &num| acc / num)
                },
                "%" => {
                    if numbers.iter().skip(1).any(|&num| num == 0) {
                        return Err(ErrorHandler::DivisionByZero);
                    }
                    numbers.iter().skip(1).fold(numbers[0], |acc, &num| acc % num)
                },
                "max" => *numbers.iter().max().unwrap(),
                "min" => *numbers.iter().min().unwrap(),
                "pow" => numbers[0].pow(numbers[1] as u32),
                "sqrt" => (numbers[0] as f64).sqrt() as i32,
                "sin" => (numbers[0] as f64).sin() as i32,
                "cos" => (numbers[0] as f64).cos() as i32,
                "tan" => (numbers[0] as f64).tan() as i32,
                "abs" => (numbers[0] as f64).abs() as i32,
                "zero?" => (numbers[0] == 0) as i32,
                "even?" => (numbers[0] % 2 == 0) as i32,
                "odd?" => (numbers[0] % 2 != 0) as i32,
                "pos?" => (numbers[0] > 0) as i32,
                "neg?" => (numbers[0] < 0) as i32,
                "eq?" => (numbers[0] == numbers[1]) as i32,
                "neq?" => (numbers[0] != numbers[1]) as i32,
                "lt?" => (numbers[0] < numbers[1]) as i32,
                "gt?" => (numbers[0] > numbers[1]) as i32,
                "lte?" => (numbers[0] <= numbers[1]) as i32,
                "gte?" => (numbers[0] >= numbers[1]) as i32,
                _ => return Err(ErrorHandler::UnknownArithmetic(op.to_string())),
            };
            Ok(result)
        }

        // Unknown operator
        _ => Err(ErrorHandler::UnknownOperator(args.join(" "))),
    }
}

pub fn interp(path: PathBuf) -> Result<(), ErrorHandler> {
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
            match eval(line) {
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