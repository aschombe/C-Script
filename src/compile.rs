use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::interp::parser::*;

use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::types::{FloatType, FunctionType, IntType, StructType, VoidType};
use inkwell::values::FunctionValue;

use crate::compile::ir_builder::IrBuilder;
pub(crate) mod ir_builder;

pub struct Compiler {
    pub target_name: String,
    pub output_dir: String,
    pub output_name: String,
    pub declared_variables: Vec<String>,
    pub declared_functions: Vec<String>,
    pub used_variables: Vec<String>,
    pub used_functions: Vec<String>,
}

impl Compiler {
    pub fn new(target_path: PathBuf) -> Compiler {
        let target_name: String = target_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let output_dir: String = target_path.parent().unwrap().to_str().unwrap().to_string();

        let output_name: String = target_name.replace(".rss", ".ll");

        Compiler {
            target_name,
            output_dir,
            output_name,
            declared_variables: Vec::new(),
            declared_functions: Vec::new(),
            used_variables: Vec::new(),
            used_functions: Vec::new(),
        }
    }

    pub fn compile(&self) {
        let target_path: String = format!("{}/{}", self.output_dir, self.target_name);
        let output_path: String = format!("{}/{}", self.output_dir, self.output_name);

        let contents: String = fs::read_to_string(target_path).expect("Could not read file");

        // remove all comments
        let mut contents: String = contents
            .lines()
            .map(|line| {
                let mut line = line.to_string();
                if let Some(index) = line.find("//") {
                    line = line.split_at(index).0.to_string();
                }
                line
            })
            .collect::<Vec<String>>()
            .join("\n");

        // remove all empty lines
        contents = contents
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n");

        // ast vec to be translated to LLVM IR
        let mut ast_vec: Vec<ASTNode> = Vec::new();
        let mut expressions: String = String::new();
        let mut open_parentheses: i32 = 0;
        // parse through the contents and extract the AST
        for line in contents.lines() {
            let line: &str = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            for char in line.chars() {
                if char == '(' {
                    open_parentheses += 1;
                } else if char == ')' {
                    open_parentheses -= 1;
                }
            }

            expressions.push_str(line);
            expressions.push_str(" ");

            if open_parentheses == 0 {
                let tokens: Vec<String> = tokenize(&expressions);
                let (ast, _) = parse(&tokens).expect("Could not parse tokens");

                ast_vec.push(ast);
                expressions.clear();
            }
        }

        // prepare context and types
        let context: Context = Context::create();
        let module = context.create_module("main");
        let builder = context.create_builder();

        let void_type: VoidType = context.void_type();
        let int_type: IntType = context.i32_type();
        let func_type: FunctionType = void_type.fn_type(&[], false);
        let f64_type: FloatType = context.f64_type();
        let bool_type: FloatType = context.f64_type();
        let string_type: StructType =
            context.struct_type(&[context.i8_type().array_type(255).into()], false);
        let function: FunctionValue = module.add_function("main", func_type, None);
        let basic_block: BasicBlock = context.append_basic_block(function, "entry");

        builder.position_at_end(basic_block);

        // translate AST to LLVM IR
        // let ir_builder: IrBuilder = IrBuilder::new(
        //     // ast_vec,
        //     int_type,
        //     f64_type,
        //     bool_type,
        //     string_type,
        //     void_type,
        //     builder,
        // );

        for node in &ast_vec {
            let r_builder: IrBuilder = ir_builder::IrBuilder::new(
                int_type,
                f64_type,
                bool_type,
                string_type,
                void_type,
                builder,
            );
            let return_value: Result<String, crate::interp::error_handler::ErrorHandler> =
                r_builder.build_ir(node);
            let _ = builder.build_return(return_value);
        }

        println!("Generated LLVM IR: {}", function);

        // for now just push the AST to the bytecode
        // let mut bytecode: String = String::new();
        // for node in ast_vec {
        //    bytecode.push_str(&format!("{:?}\n", node));
        //}

        // let mut output_file: fs::File =
        //     fs::File::create(output_path).expect("Could not create file");

        // output_file
        //     .write_all(bytecode.as_bytes())
        //     .expect("Could not write to file");
    }
}
