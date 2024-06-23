use std::fmt;

use crate::error_handler::ErrorHandler;

#[derive(Debug, Clone)]
pub enum Type {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    FunctionCall(String, Vec<Type>),
    Variable(String, Box<Type>),
    Void,
}

#[derive(Debug, Clone)]
pub enum TypeTag {
    Int,
    Float,
    String,
    Boolean,
    Void,
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeTag::Int => write!(f, "Int"),
            TypeTag::Float => write!(f, "Float"),
            TypeTag::String => write!(f, "String"),
            TypeTag::Boolean => write!(f, "Boolean"),
            TypeTag::Void => write!(f, "Void"),
            _ => panic!("{} is not a valid type", self),
        }
    }
}

impl Type {
    fn get_type_tag(&self) -> TypeTag {
        match self {
            Type::Int(_) => TypeTag::Int,
            Type::Float(_) => TypeTag::Float,
            Type::String(_) => TypeTag::String,
            Type::Boolean(_) => TypeTag::Boolean,
            Type::Variable(_, typ) => typ.get_type_tag(),
            Type::Void => TypeTag::Void,
            _ => panic!("{} is not a valid type", self),
        }
    }

    fn as_int(&self) -> Result<i64, String> {
        if let Type::Int(num) = self {
            Ok(*num)
        } else {
            Err(ErrorHandler::TypeError("Expected an integer".to_string()).to_string())
        }
    }

    fn as_float(&self) -> Result<f64, String> {
        if let Type::Float(num) = self {
            Ok(*num)
        } else {
            Err(ErrorHandler::TypeError("Expected a float".to_string()).to_string())
        }
    }

    fn as_string(&self) -> Result<&str, String> {
        if let Type::String(text) = self {
            Ok(text)
        } else {
            Err(ErrorHandler::TypeError("Expected a string".to_string()).to_string())
        }
    }

    fn as_boolean(&self) -> Result<bool, String> {
        if let Type::Boolean(boolean) = self {
            Ok(*boolean)
        } else {
            Err(ErrorHandler::TypeError("Expected a boolean".to_string()).to_string())
        }
    }

    // fn as_type_tag(&self) -> Result<&str, String> {
    //     if let Type::TypeTag(tag) = self {
    //         Ok(tag)
    //     } else {
    //         Err(ErrorHandler::TypeError("Expected a type tag".to_string()).to_string())
    //     }
    // }

    fn is_int(&self) -> bool {
        if let Type::Int(_) = self {
            true
        } else {
            false
        }
    }

    fn is_float(&self) -> bool {
        if let Type::Float(_) = self {
            true
        } else {
            false
        }
    }

    fn is_string(&self) -> bool {
        if let Type::String(_) = self {
            true
        } else {
            false
        }
    }

    fn is_boolean(&self) -> bool {
        if let Type::Boolean(_) = self {
            true
        } else {
            false
        }
    }

    fn is_void(&self) -> bool {
        if let Type::Void = self {
            true
        } else {
            false
        }
    }

    // fn is_type_tag(&self) -> bool {
    //     if let Type::TypeTag(_) = self {
    //         true
    //     } else {
    //         false
    //     }
    // }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int(num) => write!(f, "{}", num),
            Type::Float(num) => write!(f, "{}", num),
            Type::String(text) => write!(f, "{}", text),
            Type::Boolean(boolean) => write!(f, "{}", boolean),
            Type::Variable(name, _) => write!(f, "{}", name),
            Type::FunctionCall(name, _) => write!(f, "{}", name),
            Type::Void => write!(f, "void"),
            _ => panic!("{} is not a valid type", self),    
        }
    }
}