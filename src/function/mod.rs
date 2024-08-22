use crate::{ast::Expr, types::Type};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub body: Vec<Expr>,
    pub return_type: Type,
}