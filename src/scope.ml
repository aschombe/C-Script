open Ast

type value =
| Int of int
| Float of float
| Bool of bool
| Var of string
(* name, args and arg types, return type, body *)
| Function of string * (string * var_types) list * func_types * expr

type scope = {
  vars: (string, value) Hashtbl.t;
  funcs: (string, (string * var_types) list * func_types * expr list) Hashtbl.t;
}

let create_scope () = {
  vars = Hashtbl.create 10;
  funcs = Hashtbl.create 10;
}

let scope_stack = ref [ create_scope () ]

let push_scope () =
  scope_stack := create_scope () :: !scope_stack

let pop_scope () =
  match !scope_stack with
  | _ :: tl -> scope_stack := tl
  | [] -> failwith "pop_scope: empty scope stack"

let set_var name expr =
  match !scope_stack with
  | [] -> failwith "set_var: empty scope stack"
  | env :: _ -> Hashtbl.replace env.vars name expr

let get_var name =
  let rec lookup env_stack =
    match env_stack with
    | [] -> failwith ("get_var: variable not found: " ^ name)
    | env :: rest ->
        try Hashtbl.find env.vars name
        with Not_found -> lookup rest
  in 
  lookup !scope_stack

let remove_var name =
  match !scope_stack with
  | [] -> failwith "remove_var: empty scope stack"
  | env :: _ -> Hashtbl.remove env.vars name

let set_func name args ret_type body =
  match !scope_stack with
  | [] -> failwith "set_func: empty scope stack"
  | env :: _ -> Hashtbl.replace env.funcs name (args, ret_type, body)

let get_func name =
  let rec lookup env_stack =
    match env_stack with
    | [] -> failwith ("get_func: function not found: " ^ name)
    | env :: rest ->
        try Hashtbl.find env.funcs name
        with Not_found -> lookup rest
  in 
  lookup !scope_stack

let remove_func name =
  match !scope_stack with
  | [] -> failwith "remove_func: empty scope stack"
  | env :: _ -> Hashtbl.remove env.funcs name
