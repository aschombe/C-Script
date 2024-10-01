# C-Script
This branch is WIP, check the `main` branch for the latest stable version (which is still LISP-like, and written in Rust).
## Description
This is an interpreter (soon to have compiler feature) written in C++. Open to feature suggestions. Check [main.md](docs/main.md) for more information.

## Table of Contents
- [C-Script](#c-script)
  - [Table of Contents](#table-of-contents)
  - [Documentation](#documentation)
  - [Usage](#usage)
  - [Features](#features)
  - [TODO](#todo)
  - [Notes](#notes)

## Documentation
- Check out the [documentation](docs/main.md) for more information on the features of the language.

## Usage
Run the command line tool by running the following command:
```bash
./cst ./<path-to-file>.csf <flag(s)>
```
The file should have the extension `.csf`. You can find some example files in the `examples` directory.
More about the `cst` tool can be found in the [cst.md](docs/cst.md) file.

## Features
- [x] CST command line tool
- [x] AST
- [x] Tokenizer

## TODO
- [ ] ^= doesn't parse correctly
- [ ] Find a nice way to implement ++ and -- alongside ! and - in a single node (maybe create new node type, or change how unary works)
- [ ] Scoping/Environments
- [ ] File imports (for functions, new keyword: import <relative filepath>)
    - [ ]  The filepath should be relative to the file using import, not the location of the cst caller
        - [ ] When you pass the file path to the cst tool, use that to extract the absolute path and pass it to the interpreter
- [ ] Structs: rust (implement methods for struct) or c (pass struct to functions, will require a change to types.hpp)?
- [ ] Type Checker (proper type checker)
- [ ] Interpreter:
    - [ ] Allow for loop init to be a let statement (grab the name before interp_let)
- [ ] Maybe function overloading (based on: number of arguments and types of arguments)
- [ ] Add snippets to token struct (populated during tokenizing):
    - [ ] Starting point: start of line (after previous newline)
    - [ ] Ending point: new line
- [ ] Documentation (maybe doxygen)
- [ ] Compiler
 
## Notes

What should I do if a func param has the same name as a global variable?
- [ ] Local variable takes precedence
- [ ] Global variable takes precedence
- [ ] Error
