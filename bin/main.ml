open Ast
open Interp
open Lexer
open Parser

(* let rec repl () =
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

let () = repl () *)

open Printf
open Unix
open Sys
open Compiler
open Interp

let get_extension filename =
  try
    let dot_index = String.rindex.filename '.' in
    String.sub filename (dot_index + 1) (String.length filename - dot_index - 1)
  with Not_found -> ""

let handle_file path compile = 
  let extension = get_extension (Filename.basename path) in
  match extension with
  | "rss" ->
    if compile then
      if Sys.os_type = "Unix" then
        print_endline "TODO: compile on Unix"
        (* spawn a compiler from the Compiler module *)
        (* let compiler = Compiler.new path in
        let result = compiler.compile () in
        if result then
          ()
        else
          printf "Compilation failed\n" *)
      else
        print_endline "Cannot compile on non-Unix systems"
    else
      print_endline "TODO: interpret"
      (* spawn an interpreter from the Interp module *)
      (* let interpreter = Interp.new () in
      let result = interpreter.interp path in
      (* print error that results *)
      match result with
      | Ok () -> ()
      | Error msg -> printf "Runtime error: %s\n" msg *)
  | _ -> print_endline "TODO: handle other file extensions"

let main () =
  let args = Array.to_list Sys.argv in
  let program_name = List.hd args in
  let args = List.tl args in
  let compile = List.exists (fun arg -> arg = "-c") args in
  let path = List.find_opt (fun arg -> arg <> "-c") args in
  match path with
  | Some path -> handle_file path compile
  | None -> printf "Usage: %s <file> [-c]\n" program_name

let () = main ()

      
