
use std::fmt;

use crate::interp::error_handler::ErrorHandler;

#[derive(Debug, Clone)]
pub enum VariableValue {
    Number(f64),
    Text(String),
}

impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariableValue::Number(num) => write!(f, "{}", num),
            VariableValue::Text(text) => write!(f, "{}", text),
        }
    }
}

impl VariableValue {
    pub fn as_number(&self) -> Result<f64, ErrorHandler> {
        if let VariableValue::Number(num) = self {
            Ok(*num)
        } else {
            Err(ErrorHandler::ParseError("Expected a number".to_string()))
        }
    }

    fn _as_text(&self) -> Result<&str, ErrorHandler> {
        if let VariableValue::Text(text) = self {
            Ok(text)
        } else {
            Err(ErrorHandler::ParseError("Expected a text".to_string()))
        }
    }
}