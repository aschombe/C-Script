# Keywords

## Variable
- `let`: Declare a variable
- `del`: Delete a Variable
Note: Assigning is done using the assignment operators.

Example:
```c
let x:int = 10;
let y:double = 3.14;
del x;
```

## Control
- `if`: Conditional statement
- `elif`: Else if statement
- `else`: Else statement
- `for`: For loop
- `while`: While loop
- `break`: Break loop
- `continue`: Continue loop
- `return`: Return value from function
- `exit`: Exit program
- `switch`: Switch statement
- `case`: Case statement
- `default`: Default statement
- `struct`: Struct data type

Example:
```c
// If-elif-else
let x:int = 10;
if (x > 5) {
  exit(1);
} (elif x < 5) {
  exit(0);
} else {
  exit(-1);
}

// For loop
let i:int = 0;
for (i = 0; i < 10; i++) {
  if (i == 5) {
    break;
  }
}
// Another way:
for (let i:int = 0; i < 10; i++) {
  if (i == 5) {
    continue;
  }
}

// While loop
let i:int = 0;
while (i < 10) {
  i++;
}

// Switch-case-default
let x:int = 10;
switch (x) {
  case (1) {
    exit(1);
  }
  case (2) {
    exit(2);
  }
  default {
    exit(0);
  }
}
```
## Data type Keywords
- `int`: Integer data type
- `double`: Double data type
- `bool`: Boolean data type
- `string`: String data type
- `struct`: Struct data type

Example:
```c
let x:int = 10;
let y:double = 3.14;
let z:bool = true;
let s:string = "Hello, World!";

struct Person {
  age: int;
}

let p:struct = Person { age: 20 };
```

## Function
- `func`: Function declaration

Example:
```c
func add(a:int, b:int):int {
  return a + b;
}

let sum:int = add(10, 20);
```
