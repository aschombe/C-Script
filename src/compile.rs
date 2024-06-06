use std::fs;
use std::io::Write;
use std::path::Path;

pub struct Compiler {
    pub target_name: String,
    pub output_dir: String,
    pub output_name: String,
    pub declared_variables: Vec<String>,
    pub declared_functions: Vec<String>,
    pub used_variables: Vec<String>,
    pub used_functions: Vec<String>,
}

// Optimizations to make when compiling the code:
// Remove all comments
// Remove all empty lines
// Remove infinite loops
// If a variable is declared but not used, remove it
// If a function is declared but not used/called, remove it

impl Compiler {
    pub fn new(target_path: &str) -> Compiler {
        // grab the name of the file with the extension
        let target_name = Path::new(target_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // the output directory is the same as the target directory, so we can just grab the parent directory
        // the output_dir is just the target_path with the target_name removed
        let output_dir = Path::new(target_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // the output name is the same as the target name but with the extension changed to rssc
        let output_name = target_name.replace(".rss", ".rssc");

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

    // for now, just write the file as is to the output directory but with extension rssc
    pub fn compile(&self) {
        println!(
            "Compiling {} to {} in {}",
            self.target_name, self.output_name, self.output_dir
        );

        // read the file, and just spit it out with rssc extension
        // let target_path = format!("{}/{}", self.output_dir, self.target_name);
        // let output_path = format!("{}/{}", self.output_dir, self.output_name);

        // let contents = fs::read_to_string(target_path).expect("Could not read file");
        // let mut output_file = fs::File::create(output_path).expect("Could not create file");

        // output_file
        //     .write_all(contents.as_bytes())
        //     .expect("Could not write to file");
    }
}
