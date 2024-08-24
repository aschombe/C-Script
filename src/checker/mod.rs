use crate::{ast::Expr, error_handler::ErrorHandler};

pub fn check_type(typ: &str, expr: Expr) -> Result<Expr, String> {
    match typ {
        "int" => match expr {
            Expr::Float(_) => Err(ErrorHandler::TypeError("int".to_string(), "float".to_string()).to_string()),
            Expr::String(_) => Err(ErrorHandler::TypeError("int".to_string(), "string".to_string()).to_string()),
            Expr::Bool(_) => Err(ErrorHandler::TypeError("int".to_string(), "bool".to_string()).to_string()),
            // Expr::List(_) => Err(ErrorHandler::TypeError("int".to_string(), "list".to_string()).to_string()),
            // Expr::Void => Err(ErrorHandler::TypeError("int".to_string(), "void".to_string()).to_string()),
            _ => Ok(expr),
        },
        "float" => match expr {
            Expr::Int(i) => Ok(Expr::Float(i as f64)),
            Expr::String(_) => Err(ErrorHandler::TypeError("float".to_string(), "string".to_string()).to_string()),
            Expr::Bool(_) => Err(ErrorHandler::TypeError("float".to_string(), "bool".to_string()).to_string()),
            // Expr::List(_) => Err(ErrorHandler::TypeError("float".to_string(), "list".to_string()).to_string()),
            // Expr::Void => Err(ErrorHandler::TypeError("float".to_string(), "void".to_string()).to_string()),
            _ => Ok(expr),
        },
        "string" => match expr {
            Expr::Int(_) => Err(ErrorHandler::TypeError("string".to_string(), "int".to_string()).to_string()),
            Expr::Float(_) => Err(ErrorHandler::TypeError("string".to_string(), "float".to_string()).to_string()),
            Expr::Bool(_) => Err(ErrorHandler::TypeError("string".to_string(), "bool".to_string()).to_string()),
            // Expr::List(_) => Err(ErrorHandler::TypeError("string".to_string(), "list".to_string()).to_string()),
            // Expr::Void => Err(ErrorHandler::TypeError("string".to_string(), "void".to_string()).to_string()),
            _ => Ok(expr),
        },
        "bool" => match expr {
            Expr::Int(_) => Err(ErrorHandler::TypeError("bool".to_string(), "int".to_string()).to_string()),
            Expr::Float(_) => Err(ErrorHandler::TypeError("bool".to_string(), "float".to_string()).to_string()),
            Expr::String(_) => Err(ErrorHandler::TypeError("bool".to_string(), "string".to_string()).to_string()),
            // Expr::List(_) => Err(ErrorHandler::TypeError("bool".to_string(), "list".to_string()).to_string()),
            // Expr::Void => Err(ErrorHandler::TypeError("bool".to_string(), "void".to_string()).to_string()),
            _ => Ok(expr),
        },
        // "list" => match expr {
        //     Expr::Int(_) => Err(ErrorHandler::TypeError("list".to_string(), "int".to_string()).to_string()),
        //     Expr::Float(_) => Err(ErrorHandler::TypeError("list".to_string(), "float".to_string()).to_string()),
        //     Expr::String(_) => Err(ErrorHandler::TypeError("list".to_string(), "string".to_string()).to_string()),
        //     Expr::Bool(_) => Err(ErrorHandler::TypeError("list".to_string(), "bool".to_string()).to_string()),
        //     // Expr::Void => Err(ErrorHandler::TypeError("list".to_string(), "void".to_string()).to_string()),
        //     _ => Ok(expr),
        // },
        _ => Err(format!("Unknown type: {}", typ)),
    }
}