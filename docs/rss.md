# rss command line tool

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
  - [Flags](#flags)
- [Examples](#examples)

## Installation
To compile the project, you need to have the Rust programming language installed. You can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).
```bash
cargo run
```

## Usage
```bash
./rss <flag> <file_path>
```

### Flags
The different flags are:
- '-h': Display the help message
- '-i': Interprets an RSS file
- '-c': Compiles an RSS file to bytecode with the extension '.rssc'
- '-r': Runs a compiled RSS file with the extension '.rssc'

## Examples
```bash
./rss -i funcs.rss
```

# More Examples to come once I have implemented bytecode and compilation
