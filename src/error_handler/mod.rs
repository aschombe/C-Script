
use std::fmt;

#[derive(Debug)]
pub enum ErrorHandler {
    NoProgram,
    DivisionByZero,
    TooManyArguments(String),
    TooFewArguments(String),
    // SyntaxError(String),
    // ParseError(String),
    TypeError(String, String),
    VariableNotFound(String),
    FunctionNotFound(String),
    FunctionOrVariableNotFound(String),
    UnknownToken(String),
    UnexpectedToken(String, String),
    ReturnOutsideFunction,
    UnexpectedEndOfProgram,
    VariableAlreadyExists(String),
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorHandler::NoProgram => write!(f, "No program to run!"),
            ErrorHandler::DivisionByZero => write!(f, "Division by zero!"),
            ErrorHandler::TooManyArguments(func) => write!(f, "Too many arguments for function: {}!", func),
            ErrorHandler::TooFewArguments(func) => write!(f, "Too few arguments for function: {}!", func),
            // ErrorHandler::SyntaxError(err) => write!(f, "Syntax error: {}", err),
            // ErrorHandler::ParseError(err) => write!(f, "Parse error: {}", err),
            ErrorHandler::TypeError(expected, found) => write!(f, "Type error: expected {}, found {}", expected, found),
            ErrorHandler::VariableNotFound(var) => write!(f, "Variable not found: {}", var),
            ErrorHandler::FunctionNotFound(func) => write!(f, "Function not found: {}", func),
            ErrorHandler::FunctionOrVariableNotFound(name) => write!(f, "Function or variable not found: {}", name),
            ErrorHandler::UnknownToken(token) => write!(f, "Unknown token: {}", token),
            ErrorHandler::UnexpectedToken(expected, found) => write!(f, "Unexpected token: expected {}, found {}", expected, found),
            ErrorHandler::ReturnOutsideFunction => write!(f, "Return statement outside function!"),
            ErrorHandler::UnexpectedEndOfProgram => write!(f, "Unexpected end of program!"),
            ErrorHandler::VariableAlreadyExists(var) => write!(f, "Variable already exists: {}", var),
        }
    }
}