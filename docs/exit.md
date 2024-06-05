# Exit

The exit function kills the interpreter. The syntax is as follows:
```lisp
(exit num)
```
- `num` is the exit code, which as of now is not used for anything.

## Example
```lisp
(print 'Hi')
(exit 0)
(print 'This will not be printed')
```

## Output
```
(print 'Hi'): Hi
Exiting with code 0
```