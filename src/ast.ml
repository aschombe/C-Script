(* ast.ml *)

type bin_op = 
| Add 
| Sub 
| Mul 
| Div 
| Mod 
| Pow 
| Eq 
| Neq 
| Lt 
| Lte 
| Gt 
| Gte 
| And 
| Or

type assign_op = 
| Assign 
| AddEq 
| SubEq 
| MulEq 
| DivEq 
| ModEq 
| PowEq

type un_op = 
| Neg 
| Not

type var_types = 
| IntType
| FloatType
| BoolType
| StringType

type func_types = 
| IntType
| FloatType
| BoolType
| StringType
| VoidType

type
  prog = AProg of expr list
and expr =
| Int of int
| Float of float
| Bool of bool
| Var of string
| BinOp of bin_op * expr * expr
| UnOp of un_op * expr
| Let of string * var_types * expr
| Set of string * assign_op * expr
| Del of string
  (* 
    if (cond) {
      expr1
    } elif (cond) {
      expr2
    } elif (cond) {
      expr3
    } ...
    else {
      expr
    }
  *)
  | IEE of (expr * expr list) list * expr list option
  (*
    fn ident(arg1: type, arg2: type, ...):type {
      body
    }
  *)
  (* fn ident(arg1: type, arg2: type, ...):type { body } *)
  | FuncDef of string * (string * var_types) list * func_types * expr list
  | Call of string * expr list
  (*
    switch (expr) {
      case expr {
        expr1
      }
      case expr {
        expr2
      }
      ...
      default {
        expr
      }
    }
  *)
  | Switch of expr * (expr * expr) list * expr option
  (*
    for (init; cond; update) {
      expr
    }
  *)
  | For of expr * expr * expr * expr
  (*
    while (cond) {
      expr
    }
  *)
  | While of expr * expr
  | Break
  | Continue
  | Return of expr option
  | Exit of expr

let rec string_of_expr e =
  match e with
  | Int i -> string_of_int i
  | Float f -> string_of_float f
  | Bool b -> string_of_bool b
  | Var i -> i
  | BinOp (op, e1, e2) -> "BinOp " ^ (match op with
    | Add -> "Add"
    | Sub -> "Sub"
    | Mul -> "Mul"
    | Div -> "Div"
    | Mod -> "Mod"
    | Pow -> "Pow"
    | Eq -> "Eq"
    | Neq -> "Neq"
    | Lt -> "Lt"
    | Lte -> "Lte"
    | Gt -> "Gt"
    | Gte -> "Gte"
    | And -> "And"
    | Or -> "Or")
    ^ " (" ^ (string_of_expr e1) ^ ", " ^ (string_of_expr e2) ^ ")"
  | UnOp (op, e) -> "UnOp " ^ (match op with
    | Neg -> "Neg"
    | Not -> "Not")
    ^ " (" ^ (string_of_expr e) ^ ")"
  | Let (i, t, e) -> "Let(" ^ i ^ "," ^ (string_of_type t) ^ ", " ^ (string_of_expr e) ^ ")"
  | Set (i, op, e) -> "Set(" ^ i ^ ", " ^ (match op with
    | Assign -> "Assign"
    | AddEq -> "AddEq"
    | SubEq -> "SubEq"
    | MulEq -> "MulEq"
    | DivEq -> "DivEq"
    | ModEq -> "ModEq"
    | PowEq -> "PowEq")
    ^ ", " ^ (string_of_expr e) ^ ")"
  | Del i -> "Del(" ^ i ^ ")"
  | IEE (cases, default) -> "IEE(" ^ (String.concat ", " (List.map (fun (cond, exprs) -> "(" ^ (string_of_expr cond) ^ ", [" ^ (String.concat ", " (List.map string_of_expr exprs)) ^ "])") cases))
    ^ ", " ^ (match default with
    | Some exprs -> "[" ^ (String.concat ", " (List.map string_of_expr exprs)) ^ "]"
    | None -> "None") ^ ")"
    | FuncDef (i, args, ret, body) -> "FuncDef(" ^ i ^ ", [" ^ (String.concat ", " (List.map (fun (arg, t) -> "(" ^ arg ^ ", " ^ (string_of_type t) ^ ")") args)) ^ "], " ^ (string_of_func_type ret) ^ ", [" ^ (String.concat ", " (List.map string_of_expr body)) ^ "])"  | Call (i, args) -> "Call(" ^ i ^ ", [" ^ (String.concat ", " (List.map string_of_expr args)) ^ "])"
  | Switch (e, cases, default) -> "Switch(" ^ (string_of_expr e) ^ ", [" ^ (String.concat ", " (List.map (fun (cond, expr) -> "(" ^ (string_of_expr cond) ^ ", " ^ (string_of_expr expr) ^ ")") cases)) ^ "], " ^ (match default with
    | Some expr -> "(" ^ (string_of_expr expr) ^ ")"
    | None -> "None") ^ ")"
  | For (init, cond, update, body) -> "For(" ^ (string_of_expr init) ^ ", " ^ (string_of_expr cond) ^ ", " ^ (string_of_expr update) ^ ", " ^ (string_of_expr body) ^ ")"
  | While (cond, body) -> "While(" ^ (string_of_expr cond) ^ ", " ^ (string_of_expr body) ^ ")"
  | Break -> "Break"
  | Continue -> "Continue"
  | Return e -> "Return(" ^ (match e with
    | Some e -> "(" ^ (string_of_expr e) ^ ")"
    | None -> "None") ^ ")"
  | Exit e -> "Exit(" ^ (string_of_expr e) ^ ")"
and string_of_type = function
  | IntType -> "int"
  | FloatType -> "float"
  | BoolType -> "bool"
  | StringType -> "string"

  
  (* | FuncType (args, ret) -> "func(" ^ (String.concat ", " (List.map string_of_type args)) ^ ") -> " ^ (string_of_type ret) *)
and string_of_func_type = function
  | IntType -> "int"
  | FloatType -> "float"
  | BoolType -> "bool"
  | StringType -> "string"
  | VoidType -> "void"
