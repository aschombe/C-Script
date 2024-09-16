%{
    open Ast
%}

// %type <Ast.expr> main
%type <Ast.expr> expr
%type <Ast.expr list> exprs
%type <Ast.var_type> var_type
%type <Ast.func_type> func_type
%type <(string * Ast.var_type) list> args

%token <int> INT
%token <float> FLOAT
%token <bool> BOOL
%token <string> ID
%token ADD SUB MUL DIV MOD POW
%token EQQ NEQ LT LTE GT GTE AND OR
%token EQ ADD_EQ SUB_EQ MUL_EQ DIV_EQ MOD_EQ POW_EQ
%token NOT
// %token NEG
%token LET DEL
// %token IF ELIF ELSE
%token FN
// %token SWITCH CASE DEFAULT
%token FOR WHILE
// %token BREAK
// %token CONTINUE
// %token RETURN
// %token EXIT
%token LPAREN RPAREN
%token LBRACE RBRACE
%token COLON COMMA SEMICOLON
%token INT_TYPE FLOAT_TYPE BOOL_TYPE STRING_TYPE VOID_TYPE
%token EOF

%start <expr> main

%left OR
%left AND
%nonassoc EQQ NEQ
%nonassoc LT LTE GT GTE
%left ADD SUB
%left MUL DIV MOD
%right POW
%right NOT
// %nonassoc EQ ADD_EQ SUB_EQ MUL_EQ DIV_EQ MOD_EQ POW_EQ

%%

main:
| expr { $1 }
| LET ID COLON var_type EQ expr SEMICOLON { Let($2, $4, $6) }
| ID EQ expr SEMICOLON { Set($1, Assign, $3) }
| ID ADD_EQ expr SEMICOLON { Set($1, AddEq, $3) }
| ID SUB_EQ expr SEMICOLON { Set($1, SubEq, $3) }
| ID MUL_EQ expr SEMICOLON { Set($1, MulEq, $3) }
| ID DIV_EQ expr SEMICOLON { Set($1, DivEq, $3) }
| ID MOD_EQ expr SEMICOLON { Set($1, ModEq, $3) }
| ID POW_EQ expr SEMICOLON { Set($1, PowEq, $3) }
| DEL ID SEMICOLON { Del($2) }
(* IF ELIF ELSE TODO *)
| FN ID LPAREN args RPAREN COLON func_type LBRACE exprs RBRACE { FuncDef($2, $4, $7, $9) }
| ID LPAREN exprs RPAREN SEMICOLON { Call($1, $3) }
(* SWITCH CASE DEFAULT TODO *)
| FOR LPAREN expr SEMICOLON expr SEMICOLON expr RPAREN LBRACE expr RBRACE { For($3, $5, $7, $10) }
| WHILE LPAREN expr RPAREN LBRACE expr RBRACE { While($3, $6) }
| expr { $1 }
| EOF { raise (Failure "unexpected EOF") }

args:
| { [] }
| ID COLON var_type { [($1, $3)] }
| args COMMA ID COLON var_type { $1 @ [($3, $5)] }

exprs:
| { [] }
| expr SEMICOLON { [$1] }
| exprs expr SEMICOLON { $1 @ [$2] }

expr:
| INT { Int($1) }
| FLOAT { Float($1) }
| BOOL { Bool($1) }
| ID { Var($1) }
| expr ADD expr { BinOp(Add, $1, $3) }
| expr SUB expr { BinOp(Sub, $1, $3) }
| expr MUL expr { BinOp(Mul, $1, $3) }
| expr DIV expr { BinOp(Div, $1, $3) }
| expr MOD expr { BinOp(Mod, $1, $3) }
| expr POW expr { BinOp(Pow, $1, $3) }
| expr EQQ expr { BinOp(Eq, $1, $3) }
| expr NEQ expr { BinOp(Neq, $1, $3) }
| expr LT expr { BinOp(Lt, $1, $3) }
| expr LTE expr { BinOp(Lte, $1, $3) }
| expr GT expr { BinOp(Gt, $1, $3) }
| expr GTE expr { BinOp(Gte, $1, $3) }
| expr AND expr { BinOp(And, $1, $3) }
| expr OR expr { BinOp(Or, $1, $3) }
| NOT expr { UnOp(Not, $2) }
| SUB expr { UnOp(Neg, $2) }
| LPAREN expr RPAREN { $2 }

var_type:
| INT_TYPE { IntType }
| FLOAT_TYPE { FloatType }
| BOOL_TYPE { BoolType }
| STRING_TYPE { StringType }

func_type:
| INT_TYPE { IntType }
| FLOAT_TYPE { FloatType }
| BOOL_TYPE { BoolType }
| STRING_TYPE { StringType }
| VOID_TYPE { VoidType }
