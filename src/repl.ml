open Lexing
open Parser
open Lexer
open Interp

let rec repl () =
  print_string ">>> ";
  let input = read_line () in
  try
    let lexbuf = from_string input in
    let ast = Parser.main Lexer.tokenizer lexbuf in
    let result = eval ast in
    Printf.printf "=> %s\n" (string_of_value result);
  with
  | Lexer.Error -> print_endline "Lexer error"
  | Parser.Error -> print_endline "Parser error"
  | Interp.RunTimeError msg -> Printf.printf "Runtime error: %s\n" msg;
  repl ()

let () = repl ()