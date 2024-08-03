use crate::{ast::ASTNode, types::Type};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub body: Vec<ASTNode>,
    pub return_type: Type,
}