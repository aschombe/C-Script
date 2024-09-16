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

(*
Rust Code for old command line utility:
// fn main() {
//     let mut args: std::env::Args = args();
//     let program_name: String = args.next().unwrap();

//     let mut path: Option<PathBuf> = None;
//     let mut compile: bool = false;
//     let mut executable_name: Option<String> = None;
//     let current_dir: Option<PathBuf>;

//     current_dir = Some(std::env::current_dir().unwrap());

//     for arg in args {
//         if arg == "-c" {
//             compile = true;
//             // #[cfg(windows)]
//             // {
//             //     println!("Cannot compile on Windows");
//             //     return;
//             // }
//         } else {
//             path = Some(PathBuf::from(arg.clone()));
//             executable_name = Some(arg.clone());

//             // grab the name of the file
//             executable_name = Some(
//                 executable_name
//                     .unwrap()
//                     .split('/')
//                     .last()
//                     .unwrap()
//                     .to_string(),
//             );

//             // remove the extension
//             executable_name = Some(
//                 executable_name
//                     .unwrap()
//                     .split('.')
//                     .next()
//                     .unwrap()
//                     .to_string(),
//             );
//         }
//     }
//     if let Some(path) = path {
//         if let Some(extension) = path.extension() {
//             if extension == "rss" {
//                 if compile {
//                     // compile the file
//                     #[cfg(not(windows))]
//                     {
//                         let compiler: Compiler = Compiler::new(path);
//                         compiler.compile();
//                         return;
//                     }
//                     println!("Cannot compile on Windows");
//                 } else {
//                     // interpret the file
//                     let mut interpreter: interp::Interpreter = interp::Interpreter::new();
//                     let _res: Result<(), ErrorHandler> = interpreter.interp(path);
//                 }
//             } else if extension == "ll" {
//                 #[cfg(windows)]
//                 {
//                     println!("Cannot compile on Windows");
//                     return;
//                 }
//                 // invoke clang to compile the llvm file
//                 let mut cmd: std::process::Command = std::process::Command::new("clang");
//                 cmd.arg("-o");
//                 cmd.arg(format!(
//                     "{}/{}",
//                     current_dir.as_ref().unwrap().to_str().unwrap(),
//                     executable_name.as_ref().unwrap()
//                 ));
//                 cmd.arg(path);
//                 let res: std::process::Output = cmd.output().unwrap();

//                 // run the newly compiled executable
//                 if res.status.success() {
//                     let mut cmd: std::process::Command = std::process::Command::new(format!(
//                         "{}/{}",
//                         current_dir.unwrap().to_str().unwrap(),
//                         executable_name.unwrap()
//                     ));
//                     let res: std::process::Output = cmd.output().unwrap();
//                     println!("{}", String::from_utf8_lossy(&res.stdout));
//                 } else {
//                     println!("{}", String::from_utf8_lossy(&res.stderr));
//                 }
//             } else {
//                 println!("Invalid file extension");
//             }
//         } else {
//             println!("No file extension");
//         }
//     } else {
//         println!("Usage: {} <file> [-c]", program_name);
//     }
// }
make something similar in OCaml
*)

open Printf
open Unix
open Sys

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
        (* spawn a compiler from the Compiler module *)
        let compiler = Compiler.new path in
        let result = compiler.compile () in
        if result then
          ()
        else
          printf "Compilation failed\n"
      else
        printf "Cannot compile on Windows\n"
    else
      (* spawn an interpreter from the Interp module *)
      let interpreter = Interp.new () in
      let result = interpreter.interp path in
      (* print error that results *)
      match result with
      | Ok () -> ()
      | Error msg -> printf "Runtime error: %s\n" msg
  | _ -> printf "TODO: handle other file extensions\n"

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

      
