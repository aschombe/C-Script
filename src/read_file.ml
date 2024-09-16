let read_file (path: string) : string =
  let file = open_in path in
  let rec read_lines () =
    try let line = input_line file in line ^ "\n" ^ read_lines ()
    with End_of_file -> ""
  in
  let content = read_lines () in
  close_in file;
  content
