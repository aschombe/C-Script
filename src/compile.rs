use std::fs;
use std::io::Write;
use std::path::PathBuf;

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

        let output_name: String = target_name.replace(".rss", ".rssc");

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

        // generate bytcode for every token possible
        let mut bytecode: String = String::new();
        for line in contents.lines() {
            bytecode.push_str(&format!("{}\n", line));
        }

        let mut output_file: fs::File =
            fs::File::create(output_path).expect("Could not create file");

        output_file
            .write_all(bytecode.as_bytes())
            .expect("Could not write to file");
    }
}
