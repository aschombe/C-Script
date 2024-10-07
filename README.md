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
- [x] Error handling

## TODO
- [ ] Allow error handler to also accept AST nodes to grab line number, col number and snippet (also fix this issue with scoping errors)
- [ ] Return AST is parsing incorrectly
- [ ] Type Checker (proper type checker)
- [ ] Interpreter (everything):
    - [ ] Allow for loop init to be a let statement (grab the name before interp_let)
    - [ ] Maybe allow absolute file paths for importing
    - [ ] Maybe function overloading (based on: number of arguments and types of arguments)
- [ ] Documentation
- [ ] Compiler
 
## Notes

What should I do if a func param has the same name as a global variable?
- [ ] Func param takes precedence
- [ ] Global variable takes precedence
- [ ] Error
