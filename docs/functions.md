# Functions

# Note: output looks a little different than true output because print is a little buggy

## Table of Contents
- [Functions](#functions)
  - [Table of Contents](#table-of-contents)
  - [Function Declaration](#function-declaration)
  - [Function Calling](#function-calling)
  - [Function Return](#function-return)
  - [Function Scope](#function-scope)

## Function Declaration
Functions are declared using the 'func' keyword. The syntax is as follows:
```lisp
(func name (placeholder arg1 arg2 ... argN) (body))
```

## Examples:
```lisp
(func square ("" x) (mul x x))
(func hello_world (1) (print 'Hello, World!'))
(square 2)
(hello_world)
```

## Output:
```
(func square ("" x) (mul x x)): OK
(func hello_world (1) (print 'Hello, World!')): OK
(square 2): 4
(hello_world): Hello, World!
```

## Function Calling
Functions are called using the function name and the arguments. The syntax is as follows:
```lisp
(name arg1 arg2 ... argN)
```

## Examples:
```lisp
(func square ("" x) (mul x x))
(square 2)
```

## Output:
```
(func square ("" x) (mul x x)): OK
(square 2): 4
```

## Function Return
Functions simply spit out the result whats in them.

## Function Scope
Functions are globally defined and can be called from anywhere in the script. Their parameters are local to the function body. They can also be called recursively (base case must be prepended by keyword "base"). The syntax is as follows:
```lisp
(func name (placeholder arg1 arg2 ... argN) (body))
```

## Examples:
```lisp
(func fact ("" x) (if (lte? x 1) (base 1) (else (mul x (fact (sub x 1))))))
(fact 5)
```

## Output:
```
(func fact ("" x) (if (lte? x 1) (base 1) (else (mul x (fact (sub x 1)))))): OK
(fact 5): 120
```