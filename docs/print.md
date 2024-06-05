# Print

# Note: output looks a little different than true output because print is a little buggy

Simply spits out the result of the expression, or the string if it is a string literal.

## Examples

```lisp
(print 1)
(print 'hello')
(print (add 1 2))
(let x 5)
(print (add x 5))
```

## Output

```lisp
(print 1): 1
(print 'hello'): hello
(print (add 1 2)): 3
(let x 5): OK
(print (add x 5)): 10
```
