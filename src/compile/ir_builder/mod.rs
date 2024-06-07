use crate::interp::parser::ASTNode;
use crate::interp::error_handler::ErrorHandler;
use std::collections::HashMap;

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

    pub fn build_ir(self) -> Result<String, ErrorHandler> {
        let mut ir: String = String::new();
        // iterate through the input AST and build the LLVM IR
        println!("{:?}", self.input_ast);
        for node in self.input_ast {
            match node {
                // ASTNode::NoOp => continue,
                // ASTNode::Value(val) => {
                //     if val == "True" {
                //         ir.push_str("1");
                //     } else if val == "False" {
                //         ir.push_str("0");
                //     } else if let Ok(num) = val.parse::<f64>() {
                //         ir.push_str(&num.to_string());
                //     } else if let Some(num) = self.variables.get(&val) {
                //         ir.push_str(&num.to_string());
                //     } else {
                //         return Err(ErrorHandler::VariableNotFound(val));
                //     }
                // }
                
                // ASTNode::StringValue(val) => {
                //     ir.push_str(&val);
                // }

                ASTNode::Operator(op, operands) => {
                    match op.as_str() {
                        "add" => {
                            // if operands.is_empty() {
                                // return Err(ErrorHandler::ParseError(("Empty addition".to_string())));
                            // }
                            String::from("ADD")
                        }
                        "sub" => {
                            // if operands.is_empty() {
                                // return Err(ErrorHandler::ParseError(("Empty subtraction".to_string())));
                            // // }
                            String::from("SUB")
                        },
                        "mul" => {
                            // if operands.is_empty() {
                                // return Err(ErrorHandler::ParseError(("Empty multiplication".to_string())));
                            // }
                            String::from("MUL")
                        },
                        "div" => {
                            // if operands.is_empty() {
                                // return Err(ErrorHandler::ParseError(("Empty division".to_string())));
                            // }
                            String::from("DIV")
                        },
                        "mod" => {
                            // if operands.is_empty() {
                            //     return Err(ErrorHandler::ParseError(("Empty modulo".to_string())));
                            // }
                            String::from("MOD")
                        },
                        "neg" => {
                            // if operands.is_empty() {
                            //     return Err(ErrorHandler::ParseError(("Empty negation".to_string())));
                            // }
                            String::from("NEG")
                        },
                        "max" => {
                            // if operands.is_empty() {
                            //     return Err(ErrorHandler::ParseError(("Empty max".to_string())));
                            // }
                            String::from("MAX")
                        },
                        "min" => {
                            // if operands.is_empty() {
                            //     return Err(ErrorHandler::ParseError(("Empty min".to_string())));
                            // }
                            String::from("MIN")
                        },
                        "pow" => {
                            // if operands.len() != 2 {
                            //     return Err(ErrorHandler::ParseError(("Invalid number of operands for pow".to_string())));
                            // }
                            String::from("POW")
                        }
                        "sqrt" => {
                            // if operands.len() != 1 {
                            //     return Err(ErrorHandler::ParseError(("Invalid number of operands for sqrt".to_string())));
                            // }
                            String::from("SQRT")
                        }
                        "sin" => {
                            // if operands.len() != 1 {
                            //     return Err(ErrorHandler::ParseError(("Invalid number of operands for sin".to_string())));
                            // }
                            String::from("SIN")
                        }
                        "cos" => {
                            // if operands.len() != 1 {
                            //     return Err(ErrorHandler::ParseError(("Invalid number of operands for cos".to_string())));
                            // }
                            String::from("COS")
                        }
                        "tan" => {
                            // if operands.len() != 1 {
                            //     return Err(ErrorHandler::ParseError(("Invalid number of operands for tan".to_string())));
                            // }
                            String::from("TAN")
                        }
                        "abs" => {
                            // if operands.len() != 1 {
                            //     return Err(ErrorHandler::ParseError(("Invalid number of operands for cos".to_string())));
                            // }
                            String::from("COS")
                        }
                        "floor" => {
                            // if operands.len() != 1 {
                            //     return Err(ErrorHandler::ParseError(("Invalid number of operands for floor".to_string())));
                            // }
                            String::from("FLOOR")
                        }
                        "ceil" => {
                            // if operands.len() != 1 {
                            //     return Err(ErrorHandler::ParseError(("Invalid number of operands for ceil".to_string())));
                            // }
                            String::from("CEIL")
                        }
                        "rand" => {
                            // if operands.len() != 2 {
                            //     return Err(ErrorHandler::ParseError(("Invalid number of operands for rand".to_string())));
                            // }
                            String::from("RAND")
                        }
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