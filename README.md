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
  <!-- - [License](#license) -->

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
- [x] Print Statements (print x y z) (need to be reworked) (printing variables doesn't work)
- [x] Comments (single line comments)
- [x] Arithmetic Operations (addition, subtraction, multiplication, division, modulo)
- [x] Math Functions (min, max, pow, sqrt, sin, cos, tan, abs, zero?, even?, odd?, pos?, neg?)
- [x] Comparison Operations (eq?, neq?, gt?, lt?, gte?, lte?)
- [x] And, Or, Not (and, or, not) (might need to be reworked)
- [x] Nested Expressions (except for print statements)

## Todo
- [ ] Variables (let, set, get) (broken)
- [ ] Printing Variables and nested expressions
- [ ] Control flow and loops (if, elif, else, while, for)
- [ ] Maybe String Operations
- [ ] Maybe functions

## Notes
- Nothing to see here

<!-- ## License -->
