use super::ASTNode;

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: ASTNode,
}