use crate::interp::parser::ASTNode;

// use std::collections::HashMap;

pub struct IrBuilder {
    pub input_ast: Vec<ASTNode>,
    // variables: HashMap<String, String>,
    // functions: HashMap<String, String>,
}

impl IrBuilder {
    pub fn new(input_ast: Vec<ASTNode>) -> IrBuilder {
        IrBuilder { input_ast }
    }

    pub fn build_ir(self) -> Result<String, &'static str> {
        let mut ir: String = String::new();
        // iterate through the input AST and build the LLVM IR
        for node in self.input_ast {
            match node {
                ASTNode::Operator(op, operands) => {
                    match op.as_str() {
                        "add" => String::from("ADD"),
                        "sub" => String::from("SUB"),
                        "mul" => String::from("MUL"),
                        "div" => String::from("DIV"),
                        "mod" => String::from("MOD"),
                        "neg" => String::from("NEG"),
                        "max" => String::from("MAX"),
                        "min" => String::from("MIN"),
                        "pow" => String::from("POW"),
                        "sqrt" => String::from("SQRT"),
                        "sin" => String::from("SIN"),
                        "cos" => String::from("COS"),
                        "tan" => String::from("TAN"),
                        "abs" => String::from("ABS"),
                        "floor" => String::from("FLOOR"),
                        "ceil" => String::from("CEIL"),
                        "rand" => String::from("RAND"),
                        "if" => String::from("IF"),
                        // similarly to the interpreter, else should fall into the if case, so remove later potentially
                        "else" => String::from("ELSE"),
                        "switch" => String::from("SWITCH"),
                        "zero?" => String::from("ZERO?"),
                        "even?" => String::from("EVEN?"),
                        "odd?" => String::from("ODD?"),
                        "pos?" => String::from("POS?"),
                        "neg?" => String::from("NEG?"),
                        "eq?" => String::from("EQ?"),
                        "neq?" => String::from("NEQ?"),
                        "lt?" => String::from("LT?"),
                        "lte?" => String::from("LTE?"),
                        "gt?" => String::from("GT?"),
                        "gte?" => String::from("GTE?"),
                        "and" => String::from("AND"),
                        "or" => String::from("OR"),
                        "not" => String::from("NOT"),
                        "let" => String::from("LET"),
                        "set" => String::from("SET"),
                        "get" => String::from("GET"),
                        "del" => String::from("DEL"),
                        "for" => String::from("FOR"),
                        "concat" => String::from("CONCAT"),
                        "len" => String::from("LEN"),
                        "substring" => String::from("SUBSTRING"),
                        "strip" => String::from("STRIP"),
                        "replace" => String::from("REPLACE"),
                        "upper" => String::from("UPPER"),
                        "lower" => String::from("LOWER"),
                        "print" => String::from("PRINT"),
                        "exit" => String::from("EXIT"),
                        "debug" => String::from("DEBUG"),
                        "base" => String::from("BASE"),
                        "func" => String::from("FUNC"),
                        _ => String::from("UNKNOWN OPERATOR"),
                    };
                }

                ASTNode::Value(_) => todo!(),
                ASTNode::StringValue(_) => todo!(),
                ASTNode::NoOp => todo!(),
            }
        }

        Ok(ir)
    }
}