
use std::fmt;

#[derive(Debug)]
pub enum ErrorHandler {
    DivisionByZero,
    ParseError(String),
    VariableNotFound(String),
    FunctionOrOperatorNotFound(String),
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorHandler::DivisionByZero => write!(f, "Error: Division by zero"),
            ErrorHandler::ParseError(err) => write!(f, "Error: Parse error - {}", err),
            ErrorHandler::VariableNotFound(var) => write!(f, "Error: Variable '{}' not found", var),
            ErrorHandler::FunctionOrOperatorNotFound(name) => {
                write!(f, "Error: Function or operator '{}' not found", name)
            }
        }
    }
}