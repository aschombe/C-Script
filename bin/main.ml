
open Printf
module Interp = Rs_script_lib.Interp
open Interp
(* module Compiler = Rs_script_lib.Compiler *)
(* open Compiler *)

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

let get_extension filename =
  try
    let dot_index = String.rindex filename '.' in
    String.sub filename (dot_index + 1) (String.length filename - dot_index - 1)
  with Not_found -> ""

let handle_file path compile = 
  let extension = get_extension (Filename.basename path) in
  match extension with
  | "rss" -> (
    if compile then
      if Sys.os_type = "Unix" then
        printf "TODO: compile on Unix\n"
        (* spawn a compiler from the Compiler module *)
        (* let compiler = Compiler.new path in
        let result = compiler.compile () in
        if result then
          ()
        else
          printf "Compilation failed\n" *)
      else
        printf "Cannot compile on non-Unix systems\n"
    else
      let result = interp path in
      match result with
      | _ -> ()
  )
      | _ -> printf "Unknown file extension: %s\n" extension

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
