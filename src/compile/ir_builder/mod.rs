use crate::interp::parser::ASTNode;

pub struct IrBuilder {
    pub input_ast: Vec<ASTNode>,
}

impl IrBuilder {
    pub fn new(input_ast: Vec<ASTNode>) -> IrBuilder {
        IrBuilder { input_ast }
    }

    pub fn build_ir(self) -> String {
        // currently just prints the AST
        let mut ir: String = String::new();
        for node in self.input_ast {
            ir.push_str(&format!("{:?}\n", node));
        }
        ir
    }
}