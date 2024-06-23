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

use rss::error_handler::ErrorHandler;

static CODE: &str = r#"
// let x: int = 5 * 2 - 3 / 2 + 1 % 2;
// let y: float = 5.5;
// let x: int = 5;
// let y: int = x;
// let z: float = sqrt(25.0);
// let fact: int = factorial(5);
// let name: string = "Hello, World!";
// if (x > 5 && x < 10 && 1 == 1) {
//     print("Hello, World!");
// } elif (x > 10) {
//     print("Goodbye, World!");
// } else {
//     print("Hello, World Again!");
// }
// let p1:string = "Hello,";
// let p2:string = " World!";
// let p3:string = p1 <> p2;
// print(p3); // Hello, World!
// let y:float = 5.5;
// print(~y); // -5.5
// let z:float = sqrt(25.0);
// print(z); // 5.0
// let a:float = sin(90.0);
// print(a); // 1.0
// let b:float = cos(0.0);
// print(b); // 1.0
// let c:float = tan(45.0);
// print(c); // 1.0
// let d:float = abs(-5.0);
// print(d); // 5.0
// let e:float = floor(5.5);
// print(e); // 5.0
// let f:float = ceil(5.5);
// print(f); // 6.0    
// let g:bool = true;
// let h:bool = !g;
// print(h); // false
// let i:int = len("Hello, World!");
// print(i); // 13
// let j:string = upper("hello, world!");
// print(j); // HELLO, WORLD!
// let k:string = lower("HELLO, WORLD!");
// print(k); // hello, world!
// let l:int = rand(1, 10);
// print(l); // random number between 1 and 10
// let m:int = 5 ^ 2;
// print(m); // 25
// let n:string = "Hello, World!";
// let o:string = substring(n, 0, 5);
// print(o); // Hello
// func factorial ( n : int ) : int {
//     if ( n == 0 ) {
//         return 1;
//     } else {
//         return n * factorial ( n - 1 );
//     }
// }
// let p:int = factorial( 5 );
// print( p ); // 120
"#;

fn main() {
    // tokenize the code
    let tokens: Vec<String> = rss::tokenizer::tokenize(CODE);
    println!("{:?}", tokens);
    let ast: Result<Vec<rss::ast::ASTNode>, ErrorHandler> = rss::parser::parse(tokens);
    match ast {
        Ok(ast) => {
            println!("{:?}", ast);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}