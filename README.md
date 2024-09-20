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
  - [Todo](#todo)
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
- [x] Lexer
- [x] AST
- [x] Parser (a few bugs)

## Todo
- [ ] Parser Bugs:
    - [ ] Can't parse ^=
    - [ ] Can't parse `if` statements (might be equality issues)
    - [ ] Can't parse `switch` statements (might be equality issues)
    - [ ] Can't parse function declarations
    - [ ] Strings parse as Var("string") instead of String("string")
- [ ] Interpreter
- [ ] Compiler
 
## Notes

What should I do if a func param has the same name as a global variable?
Ideas:
- [ ] Local variable takes precedence
- [ ] Global variable takes precedence
- [ ] Error
