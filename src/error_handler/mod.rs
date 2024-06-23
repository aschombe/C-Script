
use std::fmt;

#[derive(Debug)]
pub enum ErrorHandler {
    DivisionByZero,
    TooManyArguments,
    TooFewArguments,
    SyntaxError,
    ParseError(String),
    TypeError(String),
    VariableNotFound(String),
    FunctionNotFound(String),
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorHandler::DivisionByZero => write!(f, "Division by zero"),
            ErrorHandler::TooManyArguments => write!(f, "Too many arguments"),
            ErrorHandler::TooFewArguments => write!(f, "Too few arguments"),
            ErrorHandler::SyntaxError => write!(f, "Syntax error"),
            ErrorHandler::ParseError(err) => write!(f, "Parse error: {}", err),
            ErrorHandler::TypeError(err) => write!(f, "Type error: {}", err),
            ErrorHandler::VariableNotFound(var) => write!(f, "Variable not found: {}", var),
            ErrorHandler::FunctionNotFound(func) => write!(f, "Function not found: {}", func),
        }
    }
}