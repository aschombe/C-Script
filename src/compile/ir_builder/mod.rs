use crate::interp::parser::ASTNode;
use crate::interp::error_handler::ErrorHandler;
use std::collections::HashMap;
use llvm_ir::*;

pub struct IrBuilder {
    pub input_ast: Vec<ASTNode>,
    pub used_registers: Vec<String>,
    pub variables: HashMap<String, f64>,
    pub functions: HashMap<String, Vec<ASTNode>>,
}

impl IrBuilder {
    pub fn new(input_ast: Vec<ASTNode>) -> IrBuilder {
        IrBuilder {
            input_ast,
            used_registers: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn build_ir(mut self) -> Result<String, ErrorHandler> {
        let mut ir: String = String::new();
        let mut next_register: u32 = 0;
        let mut next_return_register: u32 = 0;

        ir.push_str("%nop = add i1 0, 0\n");
        self.used_registers.push("nop".to_owned());

        for node in self.input_ast {
            match node {
                ASTNode::NoOp => ir.push_str("%nop"),
                ASTNode::Value(val) => {
                    // these should be emitted to IR
                    if val == "True" {
                        
                    } else if val == "False" {
                        
                    } else if let Ok(num) = val.parse::<f64>() {
                        
                    } else if let Some(num) = self.variables.get(&val) {
                        
                    } else {
                        return Err(ErrorHandler::VariableNotFound(val));
                    }
                }
                
                ASTNode::StringValue(val) => {
                    ir.push_str(&val);
                }

                ASTNode::Operator(op, operands) => {
                    // op is the correct keyword
                    // operands is in AST form, have to extract them properly
                    // example of AST form:
                    // Value("fact")Operator("0", [Value("x")])Operator("if", [Operator("lte?", [Value("x"), Value("1")]), Operator("base", [Value("1")]), Operator("else", [Operator("mul", [Value("x"), Operator("fact", [Operator("sub", [Value("x"), Value("1")])])])])])
                    // Value("mult")Operator("0", [Value("x"), Value("y")])Operator("mul", [Value("x"), Value("y")])
                    // this extraction has to be done per operator, (its nested with other operators, so it will be difficult to turn into LLVM IR)
                    match op.as_str() {
                        "add" => {
                            for operand in operands {
                                match operand {
                                    ASTNode::Value(val) => {
                                        
                                    }
                                    ASTNode::Operator(op, operands) => {
                                        
                                    }
                                    ASTNode::StringValue(_) => todo!(),
                                    ASTNode::NoOp => ir.push_str("%nop"),
                                }
                            }

                            String::from("ADD")
                        }
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