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
- '-c': Compiles an RSS file to bytecode with the extension '.rssc'

## Examples
```bash
./rss funcs.rss // This interprets the file
./rss funcs.rss -c // This compiles the file to bytecode (funcs.rssc)
./rss func.rssc // This runs the compiled file
```

# More Examples to come once I have implemented bytecode and compilation
