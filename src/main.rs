mod interp;
use interp::error_handler::ErrorHandler;

use std::{env::args, path::PathBuf};

fn main() {
    let mut args = args();
    let program_name = args.next().unwrap();

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
                    println!("Compiling is not yet implemented, but it will probably invoke a compiler or something");
                } else {
                    let mut interpreter: interp::Interpreter = interp::Interpreter::new();
                    let _res: Result<(), ErrorHandler> = interpreter.interp(path);
                }
            } else if extension == "rssc" {
                println!(
                    "Running is not yet implemented, but it will probably invoke a VM or something"
                );
            } else {
                println!("Invalid file extension");
            }
        } else {
            println!("No file extension");
        }
    } else {
        println!("Usage: {} <file> [-c]", program_name);
    }
}
