# Debug

The debug function prints the current state of the interpreter. This includes the variables and functions that are currently in scope.

## Syntax
```lisp
(debug)
```

## Example
```lisp
(let x 5)
(func square ("" x) (mul x x))
(debug)
```

## Output
```
(let x 5): OK
(func square ("" x) (mul x x)): OK
Variables:
x: Number(5.0)
Functions:
"square": Function { params: ["x"], body: Operator("mul", [Value("x"), Value("x")]) }
```