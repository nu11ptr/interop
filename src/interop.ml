open Parsing

let stdout = ref false

let ast = ref false

let files = ref []

let process_args () =
  let arg_defs =
    [
      ("-stdout", Arg.Set stdout, "Send output to stdout instead of file(s)");
      ( "-ast",
        Arg.Set ast,
        "Generate a tree of the AST instead of generating code)" );
    ]
  in
  let set_file file = files := file :: !files in
  let usage = "Usage: interop [-ast] [-stdout] file..." in
  Arg.parse arg_defs set_file usage;
  files := List.rev !files

let process_file f =
  let code, ext =
    if !ast then (Parse.gen_ast_string (Parse.lexbuf_from_file f), ".ast")
    else ("", "")
  in
  if !stdout then print_endline code
  else
    let filename = Filename.remove_extension f in
    let out = open_out (filename ^ ext) in
    output_string out code;
    close_out out

let () =
  process_args ();
  List.iter process_file !files
