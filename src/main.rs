mod interp;

use std::{
    env::args,
    path::{
        Path,
        PathBuf,
    }
};

fn main() {
    if args().len() == 1 {
        println!("Usage: ./rss <filename>");
        return;
    }

    // get filename
    let filename = args().nth(1).unwrap();

    // check if the file exists
    if !Path::new(&filename).exists() {
        println!("File not found: {}", filename);
        return;
    }

    // check if th    // check if the file extension is .rss.ends_with(".rss") {
    if !filename.ends_with(".rss") {
        println!("Not an rss file: {}", filename);
        return;
    }


    let mut interpreter: interp::Interpreter = interp::Interpreter::new();
    let _res: Result<(), interp::ErrorHandler> = interpreter.interp(PathBuf::from(filename));
}
