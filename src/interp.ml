open Ast
open Read_file
open Scope

exception RunTimeError of string

let var_env = Hashtbl.create 10
let func_env = Hashtbl.create 10

(* interp function takes a path that exists, read the *)
let interp (path: string) : int =
  let content = read_file path in
  let lexbuf = Lexing.from_string content in
  let ast = Parser.main Lexer.tokenizer lexbuf in
  interp_ast ast

let rec interp_ast = function
  | Int n -> n
  | Float f -> f
  | Bool b -> b
  | Var v -> (try Hashtbl.find var_env v with Not_found -> raise (RunTimeError ("variable not found: " ^ v)))
  | BinOp (op, e1, e2) -> (
    let v1 = interp_ast e1 in
    let v2 = interp_ast e2 in
    match op with
    | Add -> v1 + v2
    | Sub -> v1 - v2
    | Mul -> v1 * v2
    | Div -> v1 / v2
    | Mod -> v1 mod v2
    | Pow -> int_of_float (float_of_int v1 ** float_of_int v2)
    | Eq -> if v1 = v2 then 1 else 0
    | Neq -> if v1 <> v2 then 1 else 0
    | Lt -> if v1 < v2 then 1 else 0
    | Lte -> if v1 <= v2 then 1 else 0
    | Gt -> if v1 > v2 then 1 else 0
    | Gte -> if v1 >= v2 then 1 else 0
    | And -> if v1 <> 0 && v2 <> 0 then 1 else 0
    | Or -> if v1 <> 0 || v2 <> 0 then 1 else 0
  )
  | UnOp (op, e) -> (
    let v = interp_ast e in
    match op with
    (* | Neg -> -v *)
    | Sub -> -v 
    | Not -> if v = 0 then 1 else 0
  )
  | Let (v, t, e) -> (
    let v' = interp_ast e in
    Hashtbl.add var_env v v';
    v'
  )
  | Set (v, op, e) -> (
    let v' = interp_ast e in
    let v = (try Hashtbl.find var_env v with Not_found -> raise (RunTimeError ("variable not found: " ^ v))) in
    let v'' = match op with
      | Assign -> v'
      | AddEq -> v + v'
      | SubEq -> v - v'
      | MulEq -> v * v'
      | DivEq -> v / v'
      | ModEq -> v mod v'
      | PowEq -> int_of_float (float_of_int v ** float_of_int v')
    in
    Hashtbl.replace var_env v v'';
    v''
  )
  | Del v -> (
    Hashtbl.remove var_env v;
    0
  )
  | FuncDef (f, args, t, e) -> (
    Hashtbl.add func_env f (args, t, e);
    0
  )
  | Call (f, args) -> (
    let (args', t, e) = (try Hashtbl.find func_env f with Not_found -> raise (RunTimeError ("function not found: " ^ f))) in
    let args'' = List.map2 (fun (a, _) e -> (a, interp_ast e)) args' args in
    List.iter (fun (a, v) -> Hashtbl.add var_env a v) args'';
    let v = interp_ast e in
    List.iter (fun (a, _) -> Hashtbl.remove var_env a) args';
    v
  )
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
  | For (init, cond, step, body) -> (
    interp_ast init;
    let rec loop () =
      if interp_ast cond <> 0 then
        let _ = interp_ast body in
        let _ = interp_ast step in
        loop ()
      else
        0
    in
    loop ()
  )
  | While (cond, body) -> (
    let rec loop () =
      if interp_ast cond <> 0 then
        let _ = interp_ast body in
        loop ()
      else
        0
    in
    loop ()
  )
  | Break -> raise (RunTimeError "break")
  | Continue -> raise (RunTimeError "continue")
  | Return e -> (match e with Some e -> interp_ast e | None -> 0)
  | Exit e -> raise (RunTimeError ("exit: " ^ string_of_int (interp_ast e)))