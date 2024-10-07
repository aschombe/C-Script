<!-- Describe the language and its features -->
# C-Script

C-Script is a language that is WIP, and has C-like syntax. It was called RS-Script because it was originally written in Rust, but I am now rewriting it in C++.

<!--
Keywords and symbols in my language:
- keyworks: let, set, del, if, elif, else, for, while, break, continue, return, exit, func, int, float, bool, string, switch, case, default, true, false
- symbols: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^=, (, ), {, }, [, ], ,, :, ;
- comments: same as c++
- identifiers: [a-zA-Z_][a-zA-Z0-9_]*
- int: [0-9]+
- float: [0-9]+\.[0-9]+
- string: "[^"]*"
- bool: true, false
- operators: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=
- precedence:
    0 (highest): function call, scope (()), member access and struct intialization, literals
    1: unary operators (- (negative), ! (not))
    2: exponentiation (^)
    3: multiplication (*, /, %)
    4: addition (+, -)
    5: Comparison1 (<, <=, >, >=)
    6: Comparison2 (==, !=)
    7: Logical AND (&&)
    8: Logical OR (||)
    9 (lowest): assignment (=, +=, -=, *=, /=, %=)
-->

## Features of C-Script
- Simple syntax and Keywords
- C-like comments
- Simple data types: int, double, bool, string
- Operators: +, -, *, /, %, ^, ==, !=, <, <=, >, >=, &&, ||, =, +=, -=, *=, /=, %=, ^=, .
- Control structures: if-elif-else, for, while, break, continue, return, exit, switch-case-default
- Structs
- Functions
- Import other cst files

## Precedence
- 0 (highest): literals, function calls, scope (()), struct member access, struct initialization
- 1: unary and postfix operators (-, !, ++, --) 
- 2: exponentiation (^)
- 3: multiplication and division (*, /, %)
- 4: addition and subtraction (+, -)
- 5: comparison (<, <=, >, >=)
- 6: equality (==, !=)
- 7: logical AND (&&)
- 8: logical OR (||)
- 9 (lowest): assignment (=, +=, -=, *=, /=, %=, ^=)

## Operators
- Arithmetic: +, -, *, /, %, ^
- Comparison: ==, !=, <, <=, >, >=
- Logical: &&, ||
- Assignment: =, +=, -=, *=, /=, %=, ^=
- Unary: -, !, ++, --
- Struct: .

Read more about the operators here: [Operators](operators.md)

## Keywords
- Variable: let, del (assigning is done using the assignment operator)
- Control: if, elif, else, for, while, break, continue, return, exit, switch, case, default
- Data types: int, double, bool, string, struct
- Functions: func
- Boolean: true, false

Read more about the keywords here: [Keywords](keywords.md)

## Comments
- Single line comments: `//`
- Multi-line comments: `/* */`
