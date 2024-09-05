# Rust Explorer
This branch is WIP, check the `main` branch for the latest stable version (which is still LISP-like).
## Description
This is an interpreter (soon to have compiler feature) written in rust. Open to feature suggestions. Check [main.md](docs/main.md) for more information.

## Table of Contents
- [Rust Explorer](#rust-explorer)
  - [Table of Contents](#table-of-contents)
  - [Documentation](#documentation)
  - [Usage](#usage)
  - [Features](#features)
  - [Todo](#todo)
  - [Notes](#notes)

## Documentation
- Check out the [documentation](docs/main.md) for more information on the features of the language.
- Compilation is not supported on Windows.
- To compile on *nix systems, you need to have LLVM (17) installed and Clang.

## Usage
To run the project, you need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install). After installing Rust, you can generate the `rss` tool by running the following in the project directory:
```bash
cargo run
```

Run the command line tool by running the following command:
```bash
./rss ./<path-to-file> <flag>
```
The file should have the extension `.rss` or `.ll`. You can find some example files in the `examples` directory.
More about the rss tool can be found in the [rss.md](docs/rss.md) file.

## Features
- [x] AST (Arithmetic, Variable, Control Flow, Loops, Functions)


## Todo
- [ ] Return statement AST
- [ ] For loop AST needs fixing (condition and increment have strings instead of AST)
- [ ] While loop AST needs fixing (condition has string instead of AST, body can't parse)
- [ ] Type system is all screwed up
- [ ] Maybe enhance error handling (line numbers)
- [ ] Standard library (very basic stdlib) AST
- [ ] Interpreter (scoping is important)
- [ ] Compiler
 
## Notes

What should I do if a func param has the same name as a global variable?
Ideas:
- [ ] Local variable takes precedence
- [ ] Global variable takes precedence
- [ ] Error