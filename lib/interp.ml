open Ast

exception RunTimeError of string

let var_env = Hashtbl.create 10
let func_env = Hashtbl.create 10

let rec eval = function
  | Int n -> n
  | Float f -> f
  | Bool b -> b
  | Var v -> (try Hashtbl.find var_env v with Not_found -> raise (RunTimeError ("variable not found: " ^ v)))
  | BinOp (op, e1, e2) -> (
    let v1 = eval e1 in
    let v2 = eval e2 in
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
    let v = eval e in
    match op with
    (* | Neg -> -v *)
    | Sub -> -v 
    | Not -> if v = 0 then 1 else 0
  )
  | Let (v, t, e) -> (
    let v' = eval e in
    Hashtbl.add var_env v v';
    v'
  )
  | Set (v, op, e) -> (
    let v' = eval e in
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
    let args'' = List.map2 (fun (a, _) e -> (a, eval e)) args' args in
    List.iter (fun (a, v) -> Hashtbl.add var_env a v) args'';
    let v = eval e in
    List.iter (fun (a, _) -> Hashtbl.remove var_env a) args';
    v
  )
  (* | IEE (cases, default) -> (
    let rec eval_cases = function
      | [] -> (match default with Some e -> eval e | None -> 0)
      | (c, es) :: cs -> if eval c <> 0 then List.fold_left (fun _ e -> eval e) 0 es else eval_cases cs
    in
    eval_cases cases
  )
  | Switch (e, cases, default) -> (
    let v = eval e in
    let rec eval_cases = function
      | [] -> (match default with Some e -> eval e | None -> 0)
      | (c, e) :: cs -> if v = eval c then eval e else eval_cases cs
    in
    eval_cases cases
  ) *)
  | For (init, cond, step, body) -> (
    eval init;
    let rec loop () =
      if eval cond <> 0 then
        let _ = eval body in
        let _ = eval step in
        loop ()
      else
        0
    in
    loop ()
  )
  | While (cond, body) -> (
    let rec loop () =
      if eval cond <> 0 then
        let _ = eval body in
        loop ()
      else
        0
    in
    loop ()
  )
  | Break -> raise (RunTimeError "break")
  | Continue -> raise (RunTimeError "continue")
  | Return e -> (match e with Some e -> eval e | None -> 0)
  | Exit e -> raise (RunTimeError ("exit: " ^ string_of_int (eval e)))