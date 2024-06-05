mod interp;
use interp::error_handler::ErrorHandler;

use std::{env::args, path::PathBuf};

// if someone passes a file with a .rss extension, interpret it
// if someone passes a file with a .rssc extension, run it
// if someone passes a file with a .rss extension and a -c flag, compile it
// so this will have a single flag for compiling, and the rest will be inferred from the file extension

//let mut interpreter: interp::Interpreter = interp::Interpreter::new();
//let _res: Result<(), ErrorHandler> = interpreter.interp(PathBuf::from(path));

fn main() {
    let mut args = args();
    args.next(); // skip the first argument, which is the name of the program

    let mut path: Option<PathBuf> = None;
    let mut compile: bool = false;

    for arg in args {
        if arg == "-c" {
            compile = true;
        } else {
            path = Some(PathBuf::from(arg));
        }
    }

    if let Some(path) = path {
        if let Some(extension) = path.extension() {
            if extension == "rss" {
                if compile {
                    println!("Compiling is not yet implemented");
                } else {
                    let mut interpreter: interp::Interpreter = interp::Interpreter::new();
                    let _res: Result<(), ErrorHandler> = interpreter.interp(path);
                }
            } else if extension == "rssc" {
                println!("Running is not yet implemented");
            } else {
                println!("Invalid file extension");
            }
        } else {
            println!("No file extension");
        }
    } else {
        println!("Usage: ./rss <file> [-c]");
    }
}
