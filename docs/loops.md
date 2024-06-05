# Loops

# Note: output looks a little different than true output because print is a little buggy

## Table of Contents
- [Loops](#loops)
  - [Table of Contents](#table-of-contents)
  - [For Loops](#for-loops)
  <!-- - [While Loops](#while-loops) -->

## For Loops
For loops are done using the 'for' keyword. The syntax is as follows:
```lisp
(for var start end (body))
```

## Examples:
```lisp
(for i 0 10 (print i))
(for i 0 10 (print (add i 1)))
(for i 0 10 (print (mul i 2)))
```

## Output:
```
(for i 0 10 (print i)): 0 1 2 3 4 5 6 7 8 9
(for i 0 10 (print (add i 1))): 1 2 3 4 5 6 7 8 9 10
(for i 0 10 (print (mul i 2))): 0 2 4 6 8 10 12 14 16 18
```