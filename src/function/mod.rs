use crate::{ast::ASTNode, types::Type};

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<(String, Type)>,
    pub body: Vec<ASTNode>,
    pub return_type: Type,
}