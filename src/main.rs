// #[cfg(not(windows))]
// mod compile;
// #[cfg(not(windows))]
// use compile::Compiler;

// mod interp;
// use interp::error_handler::ErrorHandler;

// use std::{env::args, path::PathBuf};

// fn main() {
//     let mut args: std::env::Args = args();
//     let program_name: String = args.next().unwrap();

//     let mut path: Option<PathBuf> = None;
//     let mut compile: bool = false;
//     let mut executable_name: Option<String> = None;
//     let current_dir: Option<PathBuf>;

//     current_dir = Some(std::env::current_dir().unwrap());

//     for arg in args {
//         if arg == "-c" {
//             compile = true;
//             // #[cfg(windows)]
//             // {
//             //     println!("Cannot compile on Windows");
//             //     return;
//             // }
//         } else {
//             path = Some(PathBuf::from(arg.clone()));
//             executable_name = Some(arg.clone());

//             // grab the name of the file
//             executable_name = Some(
//                 executable_name
//                     .unwrap()
//                     .split('/')
//                     .last()
//                     .unwrap()
//                     .to_string(),
//             );

//             // remove the extension
//             executable_name = Some(
//                 executable_name
//                     .unwrap()
//                     .split('.')
//                     .next()
//                     .unwrap()
//                     .to_string(),
//             );
//         }
//     }
//     if let Some(path) = path {
//         if let Some(extension) = path.extension() {
//             if extension == "rss" {
//                 if compile {
//                     // compile the file
//                     #[cfg(not(windows))]
//                     {
//                         let compiler: Compiler = Compiler::new(path);
//                         compiler.compile();
//                         return;
//                     }
//                     println!("Cannot compile on Windows");
//                 } else {
//                     // interpret the file
//                     let mut interpreter: interp::Interpreter = interp::Interpreter::new();
//                     let _res: Result<(), ErrorHandler> = interpreter.interp(path);
//                 }
//             } else if extension == "ll" {
//                 #[cfg(windows)]
//                 {
//                     println!("Cannot compile on Windows");
//                     return;
//                 }
//                 // invoke clang to compile the llvm file
//                 let mut cmd: std::process::Command = std::process::Command::new("clang");
//                 cmd.arg("-o");
//                 cmd.arg(format!(
//                     "{}/{}",
//                     current_dir.as_ref().unwrap().to_str().unwrap(),
//                     executable_name.as_ref().unwrap()
//                 ));
//                 cmd.arg(path);
//                 let res: std::process::Output = cmd.output().unwrap();

//                 // run the newly compiled executable
//                 if res.status.success() {
//                     let mut cmd: std::process::Command = std::process::Command::new(format!(
//                         "{}/{}",
//                         current_dir.unwrap().to_str().unwrap(),
//                         executable_name.unwrap()
//                     ));
//                     let res: std::process::Output = cmd.output().unwrap();
//                     println!("{}", String::from_utf8_lossy(&res.stdout));
//                 } else {
//                     println!("{}", String::from_utf8_lossy(&res.stderr));
//                 }
//             } else {
//                 println!("Invalid file extension");
//             }
//         } else {
//             println!("No file extension");
//         }
//     } else {
//         println!("Usage: {} <file> [-c]", program_name);
//     }
// }

// use rss::error_handler::ErrorHandler;

static CODE: &str = r#"let x:int = 5.0;"#;

fn main() {
    // tokenize the code
    let tokens: Vec<String> = rss::tokenizer::tokenize(CODE);
    println!("{:?}", tokens);

    let mut parser: rss::parser::Parser = rss::parser::Parser::new(&tokens);
    let ast: Result<rss::ast::Expr, String> = parser.parse();
    println!("{:?}", ast);
}