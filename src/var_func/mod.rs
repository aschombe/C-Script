use crate::{ast::Expr, types::Type};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub body: Vec<Expr>,
    pub return_type: String,
}

impl Function {
    pub fn new(name: String, params: Vec<(String, String)>, body: Vec<Expr>, return_type: String) -> Self {
        Self { name, params, body, return_type }
    }
}

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub typ: String,
    pub value: Option<Expr>,
}

impl VariableInfo {
    pub fn new(name: String, typ: String, value: Option<Expr>) -> Self {
        Self { name, typ, value }
    }
}