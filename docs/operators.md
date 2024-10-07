# Operators

There are a few types of operators in C-Script:
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `^`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `&&`, `||`
- Assignment: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `^=`
- Unary: `-`, `!`, `++`, `--`

## Arithmetic, Comparison and Logical Operators
Arithmetic, comparison and logical operators are all examples of binary operators, meaning they take two operands.

Syntax:
```c
<operand1> <operator> <operand2>
```

### Arithmetic Operators
- `+`: Addition
- `-`: Subtraction
- `*`: Multiplication
- `/`: Division
- `%`: Modulus
- `^`: Exponentiation

### Comparison Operators
- `==`: Equal to
- `!=`: Not equal to
- `<`: Less than
- `<=`: Less than or equal to
- `>`: Greater than
- `>=`: Greater than or equal to

### Logical Operators
- `&&`: Logical AND
- `||`: Logical OR

## Assignment operators
Assignment operators are used to assign values to variables.  
Despite their syntax, they do not fall under binary operators.
Syntax:
```c
<variable> <operator> <expression>
```

Operators:
- `=`: Assign
- `+=`: Add and assign
- `-=`: Subtract and assign
- `*=`: Multiply and assign
- `/=`: Divide and assign
- `%=`: Modulus and assign
- `^=`: Exponentiation and assign

## Unary Operators
Unary operators are operators that take only one operand.

Syntax:
```c
<operator> <operand>
or
<operand> <operator>
```

Operators:
- `-`: Negation
- `!`: Logical NOT
- `++`: Increment
- `--`: Decrement

## Precedence
The precedence of operators in C-Script is as follows:
1. Literals, function calls, scope (()), struct member access, struct initialization
2. Unary and postfix operators (-, !, ++, --)
3. Exponentiation (^)
4. Multiplication and division (*, /, %)
5. Addition and subtraction (+, -)
6. Comparison (<, <=, >, >=)
7. Equality (==, !=)
8. Logical AND (&&)
9. Logical OR (||)
10. Assignment (=, +=, -=, *=, /=, %=, ^=)
