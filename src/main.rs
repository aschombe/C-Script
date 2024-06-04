mod interp;
use interp::error_handler::ErrorHandler;

use std::{
    env::args,
    path::{
        Path,
        PathBuf,
    }
};


fn main() {
    if args().len() == 1 {
        println!("Usage: ./rss <filename>.rss");
        return;
    }
    let filename = args().nth(1).unwrap();

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
