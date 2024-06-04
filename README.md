# Rust Explorer

## Description
This is an interpreter written in rust.

## Table of Contents
- [Rust Explorer](#rust-explorer)
  - [Table of Contents](#table-of-contents)
  - [Usage](#usage)
  - [Features](#features)
  - [Todo](#todo)
  - [Notes](#notes)

## Usage
To run the project, you need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install). After installing Rust, you can generate the interpreter by running the following in the project directory:
```bash
cargo run
```

Run the interpreter by running the following command:
```bash
./rss <path-to-file>
```
The file should have the extension `.rss`. You can find some example files in the `examples` directory.

## Features
- [x] Tape based interpreter
- [x] Print Statements
- [x] Comments (single line comments)
- [x] Arithmetic Operations (addition, subtraction, multiplication, division, modulo)
- [x] Math Functions (min, max, pow, sqrt, sin, cos, tan, abs, zero?, even?, odd?, pos?, neg?)
- [x] Comparison Operations (eq?, neq?, gt?, lt?, gte?, lte?)
- [x] And, Or, Not
- [x] Nested Expressions
- [x] Variables (let, set, get, del)
- [x] Floating point numbers
- [x] Control Flow (if, else)
- [x] Add boolean keywords (0 is False, 1 is True)
- [x] Functions
- [x] Recursion
- [x] Expansive Examples in the examples directory
- [x] Debug function (prints the current state of the interpreter (variables and functions)) syntax: (debug)
- [x] For loops
- [x] Exit function (exits the interpreter) syntax: (exit num)

## Todo
- [ ] Stack overflow errors for recursion and variable stack
- [ ] Put print statement output after the output for the line being interpreted, and dont print a 0 for every successful line
- [ ] Support for code spanning multiple lines
- [ ] Loops (while (giving me a headache))
- [ ] String Literals (for printing) (will probably need single quotes, to differentiate in the parser)
- [ ] Maybe Type checking

## Notes
- Empty parenthesis '()' are evaluated to 0.0
- Functions: (func "name" (placeholder arg1 arg2 ... argn) (body)), the placeholder can be anything but (), but it must be there (or everything breaks)
- Recursive Functions: Base case must be prepended by "base" keyword. Look in the examples directory for an example of a recursive function
