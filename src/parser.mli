
(* The type of tokens. *)

type token = 
  | WHILE
  | VOID_TYPE
  | SUB_EQ
  | SUB
  | STRING_TYPE
  | SEMICOLON
  | RPAREN
  | RBRACE
  | POW_EQ
  | POW
  | OR
  | NOT
  | NEQ
  | MUL_EQ
  | MUL
  | MOD_EQ
  | MOD
  | LTE
  | LT
  | LPAREN
  | LET
  | LBRACE
  | INT_TYPE
  | INT of (int)
  | ID of (string)
  | GTE
  | GT
  | FOR
  | FN
  | FLOAT_TYPE
  | FLOAT of (float)
  | EQQ
  | EQ
  | EOF
  | DIV_EQ
  | DIV
  | DEL
  | COMMA
  | COLON
  | BOOL_TYPE
  | BOOL of (bool)
  | AND
  | ADD_EQ
  | ADD

(* This exception is raised by the monolithic API functions. *)

exception Error

(* The monolithic API. *)

val main: (Lexing.lexbuf -> token) -> Lexing.lexbuf -> (expr)
