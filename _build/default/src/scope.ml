open Ast

type value =
| Int of int
| Float of float
| Bool of bool
| Var of string
(* name, args and arg types, return type, body *)
| Function of string * (string * var_types) list * func_types * expr

type scope = {
  variables: (string, value) Hashtbl.t;
  functions: (string, (string * var_types) list * func_types * expr) Hashtbl.t;
  mutable cache: value option;
  parent: scope option;
}

let create_scope parent_scope = {
  variables = Hashtbl.create 10;
  functions = Hashtbl.create 10;
  cache = None;
  parent = parent_scope;
}

let scope_stack = Stack.create()

let enter_scope () =
  let parent_scope = if Stack.is_empty scope_stack then None else Some (Stack.top scope_stack) in
  let new_scope = create_scope parent_scope in
  Stack.push new_scope scope_stack

let exit_scope () =
  if not (Stack.is_empty scope_stack) then
    ignore (Stack.pop scope_stack)

let lookup_variable name =
  let rec lookup_var_in_scope scope =
    try
      if Hashtbl.mem scope.variables name then
        Some (Hashtbl.find scope.variables name)
      else if Hashtbl.mem scope.variables name then
        let value = Hashtbl.find scope.variables name in
        Hashtbl.add scope.variables name value;
        Some value
      else
        match scope.parent with
        | Some parent -> lookup_var_in_scope parent
        | None -> None
    with Not_found -> None
  in
  if Stack.is_empty scope_stack then None else lookup_var_in_scope (Stack.top scope_stack)

let lookup_function name =
  let rec lookup_func_in_scope scope =
    try
      if Hashtbl.mem scope.functions name then
        Some (Hashtbl.find scope.functions name)
      else
        match scope.parent with
        | Some parent -> lookup_func_in_scope parent
        | None -> None
    with Not_found -> None
  in
  if Stack.is_empty scope_stack then None else lookup_func_in_scope (Stack.top scope_stack)

let add_variable name value =
  if not (Stack.is_empty scope_stack) then
    Hashtbl.add (Stack.top scope_stack).variables name value

let add_function name func =
  if not (Stack.is_empty scope_stack) then
    Hashtbl.add (Stack.top scope_stack).functions name func
