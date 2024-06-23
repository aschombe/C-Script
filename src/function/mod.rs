#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<(String, Types)>,
    pub body: Vec<ASTNode>,
    pub return_type: Types,
}