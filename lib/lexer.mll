{
    open Parser
}

rule tokenizer = parse
    | [' ' '\t' '\n'] { tokenizer lexbuf }
    | ['0'-'9']+ as num { INT(int_of_string num) }
    | ['0'-'9']+ "." ['0'-'9']+ as num { FLOAT(float_of_string num) }
    | "true" { BOOL(true) }
    | "false" { BOOL(false) }
    | ['a'-'z' 'A'-'Z']['a'-'z' 'A'-'Z' '0'-'9']* as id { ID id }
    | "+" { ADD }
    | "-" { SUB }
    | "*" { MUL }
    | "/" { DIV }
    | "%" { MOD }
    | "^" { POW }
    | "==" { EQQ }
    | "!=" { NEQ }
    | "<" { LT }
    | "<=" { LTE }
    | ">" { GT }
    | ">=" { GTE }
    | "&&" { AND }
    | "||" { OR }
    | "=" { EQ }
    | "+=" { ADD_EQ }
    | "-=" { SUB_EQ }
    | "*=" { MUL_EQ }
    | "/=" { DIV_EQ }
    | "%=" { MOD_EQ }
    | "^=" { POW_EQ }
    | "!" { NOT }
    | "~" { NEG }
    | "let" { LET }
    | "del" { DEL }
    | "if" { IF }
    | "elif" { ELIF }
    | "else" { ELSE }
    | "fn" { FN }
    | "switch" { SWITCH }
    | "case" { CASE }
    | "default" { DEFAULT }
    | "for" { FOR }
    | "while" { WHILE }
    | "break" { BREAK }
    | "continue" { CONTINUE }
    | "return" { RETURN }
    | "exit" { EXIT }
    | "(" { LPAREN }
    | ")" { RPAREN }
    | "{" { LBRACE }
    | "}" { RBRACE }
    | ":" { COLON }
    | "," { COMMA }
    | ";" { SEMICOLON }
    | eof { EOF }
    | _ { raise (Failure "invalid character") }
