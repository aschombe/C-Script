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

    // check if the file extension is .rss
    if !filename.ends_with(".rss") {
        println!("Not an rss file: {}", filename);
        return;
    }

    // pass path to the interpreter
    let _res: Result<(), interp::ErrorHandler> = interp::interp(PathBuf::from(filename));
}
