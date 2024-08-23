use crate::{ast::Expr, types::Type};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub recursive: bool,
    pub params: Vec<(String, String)>,
    pub body: Vec<Expr>,
    pub return_type: String,
}

#[derive(Clone)]
pub struct VariableInfo {
    pub name: String,
    pub typ: String,
    pub value: Option<Expr>,
}