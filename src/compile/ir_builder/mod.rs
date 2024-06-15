use std::collections::HashMap;

use crate::interp::error_handler::ErrorHandler;
use crate::interp::function::Function;
use crate::interp::parser::ASTNode;
use crate::interp::variable_value::VariableValue;

use inkwell::builder::Builder;
use inkwell::types::{FloatType, IntType, StructType, VoidType};

pub struct IrBuilder<'a> {
    // input_ast: Vec<ASTNode>,
    int_type: IntType<'a>,
    f64_type: FloatType<'a>,
    bool_type: FloatType<'a>,
    string_type: StructType<'a>,
    void_type: VoidType<'a>,
    builder: &'a Builder<'a>,
    ir: String,

    variables: HashMap<String, VariableValue>,
    functions: HashMap<String, Function>,
}

impl<'a> IrBuilder<'a> {
    pub fn new(
        // input_ast: Vec<ASTNode>,
        int_type: IntType<'a>,
        f64_type: FloatType<'a>,
        bool_type: FloatType<'a>,
        string_type: StructType<'a>,
        void_type: VoidType<'a>,
        builder: &'a Builder<'a>,
    ) -> IrBuilder<'a> {
        IrBuilder {
            // input_ast,
            int_type,
            f64_type,
            bool_type,
            string_type,
            void_type,
            builder,
            ir: String::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn build_ir(&self, ast_node: &ASTNode) -> Result<String, ErrorHandler> {
        // for node in &self.input_ast {
        match ast_node {
            ASTNode::NoOp => {
                // do nothing
            }
            ASTNode::Value(val) => {
                if val == "True" {
                    let _err = self
                        .builder
                        .build_return(Some(&self.f64_type.const_float(1.0)));
                } else if val == "False" {
                    let _err = self
                        .builder
                        .build_return(Some(&self.f64_type.const_float(0.0)));
                } else if let Ok(num) = val.parse::<f64>() {
                    let _err = self
                        .builder
                        .build_return(Some(&self.f64_type.const_float(num)));
                } else if let Some(num) = self.variables.get(val) {
                    match num {
                        VariableValue::Number(num) => {
                            let _err = self
                                .builder
                                .build_return(Some(&self.f64_type.const_float(*num)));
                        }
                        VariableValue::Text(_) => {
                            return Err(ErrorHandler::ParseError("Expected a number".to_string()));
                        }
                    }
                } else {
                    return Err(ErrorHandler::VariableNotFound(val.to_string()));
                }
            }

            ASTNode::StringValue(val) => {
                //ir.push_str(&val);
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
                        // build_float_add<T: FLoatMathValue<'ctx>>(&self, lhs: T, rhs: T,
                        // name: &str) -> Result(T, BuilderError)
                        // for operand in operands {}
                        if operands.is_empty() {
                            return Err(ErrorHandler::ParseError("Empty Addition".to_string()));
                        }

                        // put something in the IR
                    }
                    "sub" => {
                        if operands.is_empty() {
                            return Err(ErrorHandler::ParseError("Empty Subtraction".to_string()));
                        }
                    }
                    "mul" => {
                        if operands.is_empty() {
                            return Err(ErrorHandler::ParseError(
                                "Empty Multiplication".to_string(),
                            ));
                        }
                    }
                    // "div" => String::from("DIV"),
                    // "mod" => String::from("MOD"),
                    // "neg" => String::from("NEG"),
                    // "max" => String::from("MAX"),
                    // "min" => String::from("MIN"),
                    // "pow" => String::from("POW"),
                    // "sqrt" => String::from("SQRT"),
                    // "sin" => String::from("SIN"),
                    // "cos" => String::from("COS"),
                    // "tan" => String::from("TAN"),
                    // "abs" => String::from("ABS"),
                    // "floor" => String::from("FLOOR"),
                    // "ceil" => String::from("CEIL"),
                    // "rand" => String::from("RAND"),
                    // "if" => String::from("IF"),
                    // // similarly to the interpreter, else should fall into the if case, so remove later potentially
                    // // "else" => String::from("ELSE"),
                    // "switch" => String::from("SWITCH"),
                    // "zero?" => String::from("ZERO?"),
                    // "even?" => String::from("EVEN?"),
                    // "odd?" => String::from("ODD?"),
                    // "pos?" => String::from("POS?"),
                    // "neg?" => String::from("NEG?"),
                    // "eq?" => String::from("EQ?"),
                    // "neq?" => String::from("NEQ?"),
                    // "lt?" => String::from("LT?"),
                    // "lte?" => String::from("LTE?"),
                    // "gt?" => String::from("GT?"),
                    // "gte?" => String::from("GTE?"),
                    // "and" => String::from("AND"),
                    // "or" => String::from("OR"),
                    // "not" => String::from("NOT"),
                    // "let" => String::from("LET"),
                    // "set" => String::from("SET"),
                    // "get" => String::from("GET"),
                    // "del" => String::from("DEL"),
                    // "for" => String::from("FOR"),
                    // "concat" => String::from("CONCAT"),
                    // "len" => String::from("LEN"),
                    // "substring" => String::from("SUBSTRING"),
                    // "strip" => String::from("STRIP"),
                    // "replace" => String::from("REPLACE"),
                    // "upper" => String::from("UPPER"),
                    // "lower" => String::from("LOWER"),
                    // "print" => String::from("PRINT"),
                    // "exit" => String::from("EXIT"),
                    // "debug" => String::from("DEBUG"),
                    // "base" => String::from("BASE"),
                    // "func" => String::from("FUNC"),
                    // _ => String::from("UNKNOWN OPERATOR"),
                    _ => {
                        return Err(ErrorHandler::ParseError("Unknown Operator".to_string()));
                    }
                };
            } // }
        }

        Ok(self.ir.clone())
    }
}
