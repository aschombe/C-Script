# RS Script
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
TODO

Run the command line tool by running the following command:
```bash
./rss ./<path-to-file> <flag>
```
The file should have the extension `.rss` or `.ll`. You can find some example files in the `examples` directory.
More about the rss tool can be found in the [rss.md](docs/rss.md) file.

## Features
- [x] AST


## Todo
- [ ] Tokenizer/Lexer
- [ ] Parser
- [ ] Interpreter
- [ ] Parsing for if-then-else and switch (and some other things, commented out in parser.mly)
- [ ] Function scoping
 
## Notes

What should I do if a func param has the same name as a global variable?
Ideas:
- [ ] Local variable takes precedence
- [ ] Global variable takes precedence
- [ ] Error