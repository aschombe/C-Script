(* WIP COMPILER
This will compile to llvm ir code
*)

open Ast

let compile (path: string) : unit =
  let content = read_file path in
  let lexbuf = Lexing.from_string content in
  let ast = Parser.main Lexer.tokenizer lexbuf in
  compile_ast ast

let rec compile_ast = function
  | _ -> print_endline "Not implemented yet"
