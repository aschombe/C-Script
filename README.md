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
- [ ] Switch everything from smart pointers to raw pointers
  - [ ] Lots of valgrind errors, debugging logs are in [examples/test.csf](examples/test.csf)
  - [ ] No snippet parsing primaries??
- [ ] Potentially rework scoping (use environments or just make current scoping better)
- [ ] Function definitions and function calls segfault during parsing
- [ ] Everything in the control_flow example segfaults or doesnt work
- [ ] Return statements with more than one expression are parsing incorrectly
  - [ ] return n * factorial(n-1); parses incorrectly (it misses the factorial(n-1) and then segfaults)
  - [ ] Upon further testing factorial(n-1); misses the (n-1) then seg faults
- [ ] Type Checker (proper type checker) (maybe get rid of current scoping and implement proper environments)
- [ ] Interpreter (everything):
  - [ ] IF A NODE IS MARKED AS IGNORE, SKIP IT
  - [ ] Allow for loop init to be a let statement (grab the name before interp_let)
  - [ ] Maybe allow absolute file paths for importing
  - [ ] Maybe function overloading (based on: number of arguments and types of arguments)
- [ ] Documentation
- [ ] Compiler (transpiler to C for now)
 
## Notes

What should I do if a func param has the same name as a global variable?
- [ ] Func param takes precedence
- [ ] Global variable takes precedence
- [ ] Error
