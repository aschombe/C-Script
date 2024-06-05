mod interp;
use interp::error_handler::ErrorHandler;

use std::{
    env::args,
    path::{Path, PathBuf},
};

fn main() {
    let mut args = args().skip(1);
    let flag = args.next().unwrap_or_else(|| {
        println!("Usage: ./rss <flag> <filename>.rss");
        std::process::exit(1);
    });

    match flag.as_str() {
        "-i" => {
            let filename = args.next().unwrap_or_else(|| {
                println!("Usage: ./rss -i <filename>.rss");
                std::process::exit(1);
            });

            if !Path::new(&filename).exists() {
                println!("File not found: {}", filename);
                return;
            }

            if !filename.ends_with(".rss") {
                println!("Not an rss file: {}", filename);
                return;
            }

            let mut interpreter: interp::Interpreter = interp::Interpreter::new();
            let _res: Result<(), ErrorHandler> = interpreter.interp(PathBuf::from(filename));
        }
        "-c" => {
            let filename = args.next().unwrap_or_else(|| {
                println!("Usage: ./rss -c <filename>.rss");
                std::process::exit(1);
            });

            if !Path::new(&filename).exists() {
                println!("File not found: {}", filename);
                return;
            }

            if !filename.ends_with(".rss") {
                println!("Not an rss file: {}", filename);
                return;
            }

            // let mut compiler: interp::Compiler = interp::Compiler::new();
            // let _res: Result<(), ErrorHandler> = compiler.compile(PathBuf::from(filename));
            println!("Not implemented yet");
            std::process::exit(1);
        }
        "-r" => {
            let filename = args.next().unwrap_or_else(|| {
                println!("Usage: ./rss -r <filename>.rssc");
                std::process::exit(1);
            });

            if !Path::new(&filename).exists() {
                println!("File not found: {}", filename);
                return;
            }

            if !filename.ends_with(".rssc") {
                println!("Not an rssc file: {}", filename);
                return;
            }

            // let mut interpreter: interp::Interpreter = interp::Interpreter::new();
            // let _res: Result<(), ErrorHandler> = interpreter.interp(PathBuf::from(filename));
            println!("Not implemented yet");
            std::process::exit(1);
        }
        "-h" => {
            println!("Usage: ./rss <flag> <filename>.rss");
            println!("-i for interpret mode (expects an rss file)");
            println!("-c to compile an rss file to rssc (expects an rss file)");
            println!("-r to run an rssc file (expects an rssc file)");
            println!("-h for help");
        }
        _ => {
            println!("Usage: ./rss <flag> <filename>.rss");
            println!("-i for interpret mode (expects an rss file)");
            println!("-c to compile an rss file to rssc (expects an rss file)");
            println!("-r to run an rssc file (expects an rssc file)");
            println!("-h for help");
        }
    }
}
