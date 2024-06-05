# Control Flow and Logical Operations

# Note: 0 is False, 1 is True for boolean operations

## Table of Contents
- [Control Flow and Logical Operations](#control-flow-and-logical-operations)
  - [Table of Contents](#table-of-contents)
  - [If Statements](#if-statements)
  - [Else Statements](#else-statements)
  - [Switch Statements](#switch-statements)
  - [And, Or, Not](#and-or-not)
  - [Eq?, Neq?, Zero?, Even?, Odd?, Pos?, Neg?](#eq-neq-zero-even-odd-pos-neg)
  - [Comparison Operations](#comparison-operations)

## If Statements
If statements are done using the 'if' keyword. The syntax is as follows:
```lisp
(if (condition) (body))
```

## Example:
```lisp
(if (eq? 1 1) (print "1 is equal to 1"))
```
Output:
```
(if (eq? 1 1) (print "1 is equal to 1")): 1
```

## Else Statements
Else statements are done using the 'else' keyword. The syntax is as follows:
```lisp
(if (condition) (body) (else (body)))
```

## Examples:
```lisp
(if 0 (print "0 is true") (else (print "0 is false")))

(if 0
    (add 5 5)
    (else 
        (if 1
            (add 10 10)
            (else (add 15 15))
        )
    )
)
```
Output:
```
(if 0 (print "0 is true") (else (print "0 is false"))): 0 is false
(if 0 (add 5 5) (else (if 1 (add 10 10) (else (add 15 15))))): 30
```

## Switch Statements
Switch statements are done using the 'switch' keyword. The syntax is as follows:
```lisp
(switch (value) (case x (body)) (case y (body)) ... (default (body)))
```

## Example:
```lisp
(switch 6
    (case 1 (print "one"))
    (case 2 (print "two"))
    (case 3 (print "three"))
    (default (print "default"))
)
```
Output:
```
(switch 6 (case 1 (print "one")) (case 2 (print "two")) (case 3 (print "three")) (default (print "default"))): default
```

## And, Or, Not
And, Or, and Not are done using the 'and', 'or', and 'not' keywords respectively. The syntax is as follows:
```lisp
(and (condition1) (condition2) ... (conditionN))
(or (condition1) (condition2) ... (conditionN))
(not (condition))
```

## Examples:
```lisp
(and (eq? 1 1) (neq? 1 2))
(or (eq? 1 1) (neq? 1 2))
(not (eq? 1 1))
```
Output:
```
(and (eq? 1 1) (neq? 1 2)): 1
(or (eq? 1 1) (neq? 1 2)): 1
(not (eq? 1 1)): 0
```

## Eq?, Neq?, Zero?, Even?, Odd?, Pos?, Neg?
These are comparison operations. The syntax is as follows:
```lisp
(eq? num1 num2)
(neq? num1 num2)
(zero? num)
(even? num)
(odd? num)
(pos? num)
(neg? num)
```

## Examples:
```lisp
(eq? 1 1)
(neq? 1 2)
(zero? 0)
(even? 2)
(odd? 3)
(pos? 1)
(neg? -1)
```
Output:
```
(eq? 1 1): 1
(neq? 1 2): 1
(zero? 0): 1
(even? 2): 1
(odd? 3): 1
(pos? 1): 1
(neg? -1): 1
```

## Comparison Operations
Comparison operations are done using the 'gt?', 'lt?', 'gte?', and 'lte?' keywords. The syntax is as follows:
```lisp
(gt? num1 num2)
(lt? num1 num2)
(gte? num1 num2)
(lte? num1 num2)
```

## Examples:
```lisp
(gt? 2 1)
(lt? 1 2)
(gte? 2 2)
(lte? 2 2)
```
Output:
```
(gt? 2 1): 1
(lt? 1 2): 1
(gte? 2 2): 1
(lte? 2 2): 1
```
