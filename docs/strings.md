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
- eq?: Compares two strings. Syntax: `(eq? str1 str2)`
- neq?: Compares two strings. Syntax: `(neq? str1 str2)`

## Examples:
```lisp
(let name 'John')
(print (concat name ' Doe'))
(print (len name))
(print (substring name 1 2))
(eq? name 'John')
(neq? name 'Doe')
(strip 'john' 'n')
(replace 'john' 'n' 'm')
(upper 'john')
(lower 'JOHN')
```

## Output:
```
(let name 'John'): OK
(print (concat name ' Doe')): John Doe
(print (len name)): 4
(print (substring name 1 2)): oh
(eq? name 'John'): 1
(neq? name 'Doe'): 1
(strip 'john' 'n'): joh
(replace 'john' 'n' 'm'): johm
(upper 'john'): JOHN
(lower 'JOHN'): john
```

## Notes
- Strings are 0-indexed
