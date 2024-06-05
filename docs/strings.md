# Strings

## Table of Contents
- [Strings](#strings)
  - [Table of Contents](#table-of-contents)
  - [String Type](#string-type)
  - [String Functions](#string-functions)

## String Type
Strings are declared using single quotes. The syntax is as follows:
```lisp
(let name 'John')
```

## Examples:
```lisp
(let name 'John')
(print name)
```

## Output:
```
(let name 'John'): OK
(print name): John
```

## String Functions
Strings have the following functions:
- `concat`: Concatenates two strings. Syntax: `(concat str1 str2)`
- `len`: Returns the length of the string. Syntax: `(len str)`
- `substring`: Returns a substring of the string. Syntax: `(substring str start end)`
    - `start` and `end` are inclusive on both ends

## Examples:
```lisp
(let name 'John')
(print (concat name ' Doe'))
(print (len name))
(print (substring name 1 2))
```

## Output:
```
(let name 'John'): OK
(print (concat name ' Doe')): John Doe
(print (len name)): 4
(print (substring name 1 2)): oh
```

## Notes
- Strings are 0-indexed