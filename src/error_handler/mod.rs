
use std::fmt;

#[derive(Debug)]
pub enum ErrorHandler {
    DivisionByZero,
    TooManyArguments(String),
    TooFewArguments(String),
    SyntaxError(String),
    ParseError(String),
    TypeError(String),
    VariableNotFound(String),
    FunctionNotFound(String),
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorHandler::DivisionByZero => write!(f, "Division by zero"),
            ErrorHandler::TooManyArguments(func) => write!(f, "Too many arguments for function: {}", func),
            ErrorHandler::TooFewArguments(func) => write!(f, "Too few arguments for function: {}", func),
            ErrorHandler::SyntaxError(err) => write!(f, "Syntax error: {}", err),
            ErrorHandler::ParseError(err) => write!(f, "Parse error: {}", err),
            ErrorHandler::TypeError(err) => write!(f, "Type error: {}", err),
            ErrorHandler::VariableNotFound(var) => write!(f, "Variable not found: {}", var),
            ErrorHandler::FunctionNotFound(func) => write!(f, "Function not found: {}", func),
        }
    }
}