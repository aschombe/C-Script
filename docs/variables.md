# Variables

# Note: output looks a little different than true output because print is a little buggy

## Table of Contents
- [Variables](#variables)
  - [Table of Contents](#table-of-contents)
  - [Variable Declaration](#variable-declaration)
  - [Variable Reassignment](#variable-reassignment)
  - [Variable References](#variable-calling)
  - [Variable Scope](#variable-scope)

## Variable Declaration
Variables are declared using the 'let' keyword. The syntax is as follows:
```lisp
(let var value)
```

## Examples:
```lisp
(let x 5)
(let y 10)
(let z (add x y))
(let name 'John')
```

## Output:
```
(let x 5): OK
(let y 10): OK
(let z (add x y)): OK
(let name 'John'): OK
```

## Variable Reassignment
Variables can be reassigned using the 'set' keyword. The syntax is as follows:
```lisp
(set var value)
```

## Examples:
```lisp
(let x 5)
(print x)
(set x 10)
(print x)
(set x (add x 5))
(print x)
```

## Output:
```
(let x 5): OK
(print x): 5
(set x 10): OK
(print x): 10
(set x (add x 5)): OK
(print x): 15
```

## Variable Calling
Variables can be referenced simply by using their name. The syntax is as follows:
```lisp
(let x 5)
(print x)
```

## Output:
```
(let x 5): OK
(print x): 5
```

## Variable Scope
Variables are global to the script.

## Examples:
```lisp
(let x 5)
(print x)
(let y 10)
(print (add x y))
```

## Output:
```
(let x 5): OK
(print x): 5
(let y 10): OK
(print (add x y)): 15
```