# Keywords

```plaintext
`add`: `(add <operand1> <operand2> ...)`
`sub`: `(sub <operand1> <operand2> ...)`
`mul`: `(mul <operand1> <operand2> ...)`
`div`: `(div <operand1> <operand2> ...)`
`mod`: `(mod <operand1> <operand2> ...)`
`neg`: `(neg <operand>)`
`max`: `(max <operand1> <operand2> ...)`
`min`: `(min <operand1> <operand2> ...)`
`pow`: `(pow <base> <exponent>)`
`sqrt`: `(sqrt <value>)`
`sin`: `(sin <angle>)`
`cos`: `(cos <angle>)`  
`tan`: `(tan <angle>)`
`abs`: `(abs <value>)`
`floor`: `(floor <value>)`
`ceil`: `(ceil <value>)`
`rand`: `(rand <min-inclusive> <max-inclusive>)`
`if`: `(if <condition> <body> [else <else-body>])`
`switch`: `(switch <variable> (case <value> <body>) ... [default <default-body>])`
`zero?`: `(zero? <value>)`
`even?`: `(even? <value>)`
`odd?`: `(odd? <value>)`
`pos?`: `(pos? <value>)`
`neg?`: `(neg? <value>)`
`eq?`: `(eq? <value1> <value2>)`
`neq?`: `(neq? <value1> <value2>)`
`lt?`: `(lt? <value1> <value2>)`
`lte?`: `(lte? <value1> <value2>)`
`gt?`: `(gt? <value1> <value2>)`
`gte?`: `(gte? <value1> <value2>)`
`and`: `(and <value1> <value2> ...)`
`or`: `(or <value1> <value2> ...)`
`not`: `(not <value>)`
`let`: `(let <variable> <value>)`
`set`: `(set <variable> <value>)`
`get`: `(get <variable>)`
`del`: `(del <variable>)`
`for`: `(for <variable> <start> <end> <body>)`
`concat`: `(concat <string1> <string2> ...)`
`len`: `(len <string>)`
`substring`: `(substring <string> <start> <end>)`
`strip`: `(strip <string> <chars>)`
`replace`: `(replace <string> <old-char> <new-char>)`
`upper`: `(upper <string>)`
`lower`: `(lower <string>)`
`print`: `(print <value1> <value2> ...)`
`exit`: `(exit <code>)`
`debug`: `(debug)`
`base`: `(base <expression>)`
`func`: `(func <name> (<placeholder> <param1> <param2> ...) <body>)`
`<function>`: `(<function> <arg1> <arg2> ...)`
```

### Notes:
- The `base` operator is used to evaluate an expression and return the result.
- Functions have a placeholder parameter because my parser is buggy.
- Anything postfixed with a `?` returns `1.0` if true and `0.0` if false.
- The `exit` operator exits the program with the given code, the code is irrelevant for now.
