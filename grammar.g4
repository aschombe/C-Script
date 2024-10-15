grammar Gram;

program: statement* EOF;

statement: let_stmt
          | set_stmt
          | del_stmt
          | if_stmt
          | for_stmt
          | while_stmt
          | break_stmt
          | continue_stmt
          | return_stmt
          | exit_stmt
          | switch_stmt
          ;

var_type: 'int'
        | 'float'
        | 'bool'
        | 'string'
        | 'struct'
        ;

func_type: 'void'
         | var_type
         ;

set_symbols: '='
           | '+='
           | '-='
           | '*='
           | '/='
           | '%='
           | '^='
           ;

colon: ':';
semi_colon: ';';
equal: '=';

let_stmt: 'let' ID colon var_type equal expr semi_colon;
set_stmt: ID set_symbols expr semi_colon;
del_stmt: 'del' ID semi_colon;

if_stmt: 'if' '(' expr ')' '{' statement* '}' ('elif' '(' expr ')' '{' statement* '}')* ('else' '{' statement* '}')?;

for_stmt: 'for' '(' (let_stmt | set_stmt | ID) ';' expr ';' (set_stmt | ID) ')' '{' statement* '}';
while_stmt: 'while' '(' expr ')' '{' statement* '}';
break_stmt: 'break' ';';
continue_stmt: 'continue' ';';
return_stmt: 'return' expr? ';';
exit_stmt: 'exit' expr? ';';

switch_stmt: 'switch' '(' expr ')' '{' ('case' '(' expr ')' '{' statement* '}')+ ('default' '{' statement* '}')?;

/*
The precedence of operators in C-Script is as follows:
(1 is the highest precedence, 10 is the lowest)
1. Literals, function calls, scope (()), struct member access, struct initialization
2. Unary (-, !) and postfix operators (++, --)
3. Exponentiation (^)
4. Multiplication and division (*, /, %)
5. Addition and subtraction (+, -)
6. Comparison (<, <=, >, >=)
7. Equality (==, !=)
8. Logical AND (&&)
9. Logical OR (||)
10. Assignment (=, +=, -=, *=, /=, %=, ^=)
*/

expr: set_expr;

set_expr: logical_or_expr (set_symbols logical_or_expr)*;
logical_or_expr: logical_and_expr ( '||' logical_and_expr)*;
logical_and_expr: equality_expr ( '&&' equality_expr)*;
equality_expr: comparison_expr ( ('==' | '!=') comparison_expr)*;
comparison_expr: addition_expr ( ('<' | '<=' | '>' | '>=') addition_expr)*;
addition_expr: multiplication_expr ( ('+' | '-') multiplication_expr)*;
multiplication_expr: exponentiation_expr ( ('*' | '/' | '%') exponentiation_expr)*;
exponentiation_expr: unary_expr ( '^' unary_expr)*;
unary_expr: ('-' | '!') unary_expr
          | postfix_expr;
postfix_expr: primary_expr ('++' | '--')?;
primary_expr: ID
            | INT
            | DOUBLE
            | STRING
            | '(' expr ')'
            | func_call
            | struct_member_access
            | struct_init
            ;

func_call: ID '(' (expr (',' expr)*)? ')';
struct_member_access: ID '.' ID;
struct_init: ID '{' (ID ':' expr (',' ID ':' expr)*)? '}';
ID: [a-zA-Z_][a-zA-Z0-9_]*;
INT: [0-9]+;
DOUBLE: [0-9]*'.'[0-9]+;
STRING: '"' .*? '"';
WS: [ \t\r\n]+ -> skip;

