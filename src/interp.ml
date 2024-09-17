open Ast
open Read_file
open Scope

exception RunTimeError of string

let env = create_scope ()

let interp_ast = function
  | AProg(_e) -> 0.0
  (* | Int n   -> float_of_int n *)
  (* | Float f -> f *)
  (* | Bool b  -> if b then 1.0 else 0.0 *)
  (* | Var v   -> get_var v *)
  (* | BinOp (op, e1, e2) -> ( *)
  (*   let v1 = interp_ast e1 in *)
  (*   let v2 = interp_ast e2 in *)
  (*   match op with *)
  (*   | Add -> v1 +. v2 *)
  (*   | Sub -> v1 -. v2 *)
  (*   | Mul -> v1 *. v2 *)
  (*   | Div -> if v2 = 0.0 then raise (RunTimeError "division by zero") else v1 /. v2 *)
  (*   | Mod -> if v2 = 0.0 then raise (RunTimeError "division by zero") else v1 mod_float v2 *)
  (*   | Pow -> int_of_float (float_of_int v1 ** float_of_int v2) *)
  (*   | Eq  -> if v1 = v2 then 1.0 else 0.0 *)
  (*   | Neq -> if v1 <> v2 then 1.0 else 0.0 *)
  (*   | Lt  -> if v1 < v2 then 1.0 else 0.0 *)
  (*   | Gt  -> if v1 > v2 then 1.0 else 0.0 *)
  (*   | Lte -> if v1 <= v2 then 1.0 else 0.0 *)
  (*   | Gte -> if v1 >= v2 then 1.0 else 0.0 *)
  (*   | And -> if v1 <> 0.0 then v2 else 0.0 *)
  (*   | Or  -> if v1 <> 0.0 then 1.0 else v2 *)
  (*   ) *)
  (* | UnOp (op, e) -> ( *)
  (*   let v = interp_ast e in *)
  (*   match op with *)
  (*   | Neg -> -v *) 
  (*   | Not -> if v = 0.0 then 1.0 else 0.0 *)
  (* ) *)
  (* | Let (v, t, e) -> ( *)
  (*   let v' = interp_ast e in *)
  (*   set_var v v'; *)
  (*   v' *)
  (* ) *)
  (* | Set (v, op, e) -> ( *)
  (*   let v' = interp_ast e in *)
  (*   let v = get_var v in *)
  (*   let v'' = match op with *)
  (*     | Assign -> v' *)
  (*     | AddEq  -> v + v' *)
  (*     | SubEq  -> v - v' *)
  (*     | MulEq  -> v * v' *)
  (*     | DivEq  -> v / v' *)
  (*     | ModEq  -> v mod v' *)
  (*     | PowEq  -> int_of_float (float_of_int v ** float_of_int v') *)
  (*   in *)
  (*   set_var v v''; *)
  (*   v'' *)
  (* ) *)
  (* | Del v -> ( *)
  (*   remove_var v; *)
  (*   0.0 *)
  (* ) *)
  (* | FuncDef (f, args, t, e) -> ( *)
  (*   set_func f args e; *)
  (*   0.0 *)
  (* ) *)
  (* | Call (f, args) -> ( *)
  (*   (1* use get_func to get the function *1) *)
  (*   let args' = List.map interp_ast args in *)
  (*   let (arg_names, body) = get_func f in *)
  (*   push_scope (); *)
  (*   List.iter2 set_var arg_names args'; *)
  (*   let ret = interp_ast body in *)
  (*   pop_scope (); *)
  (*   ret *)
  (* ) *)
  (* | IEE (cases, default) -> (
    let rec interp_ast_cases = function
      | [] -> (match default with Some e -> interp_ast e | None -> 0)
      | (c, es) :: cs -> if interp_ast c <> 0 then List.fold_left (fun _ e -> interp_ast e) 0 es else interp_ast_cases cs
    in
    interp_ast_cases cases
  )
  | Switch (e, cases, default) -> (
    let v = interp_ast e in
    let rec interp_ast_cases = function
      | [] -> (match default with Some e -> interp_ast e | None -> 0)
      | (c, e) :: cs -> if v = interp_ast c then interp_ast e else interp_ast_cases cs
    in
    interp_ast_cases cases
  ) *)
  (* | For (init, cond, step, body) -> ( *)
  (*   interp_ast init; *)
  (*   let rec loop () = *)
  (*     if interp_ast cond <> 0.0 then *)
  (*       let _ = interp_ast body in *)
  (*       let _ = interp_ast step in *)
  (*       loop () *)
  (*     else *)
  (*       0.0 *)
  (*   in *)
  (*   loop () *)
  (* ) *)
  (* | While (cond, body) -> ( *)
  (*   let rec loop () = *)
  (*     if interp_ast cond <> 0.0 then *)
  (*       let _ = interp_ast body in *)
  (*       loop () *)
  (*     else *)
  (*       0.0 *)
  (*   in *)
  (*   loop () *)
  (* ) *)
  (* | Break    -> raise (RunTimeError "break") *)
  (* | Continue -> raise (RunTimeError "continue") *)
  (* | Return e -> (match e with Some e -> interp_ast e | None -> 0.0) *)
  (* | Exit e   -> raise (RunTimeError ("exit: " ^ string_of_int (interp_ast e))) *)

(* interp function takes a path that exists, read the *)
let interp (path: string) : float =
  let content = read_file path in
  let lexbuf = Lexing.from_string content in
  let ast = Parser.main Lexer.tokenizer lexbuf in
  interp_ast ast
