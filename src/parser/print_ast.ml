open Ast
open Buffer

let generate_prefix buf tabs =
  add_char buf '\n';
  add_string buf (String.make (tabs * 3) ' ');
  add_string buf "└──"

let rec generate_expr buf tabs expr =
  generate_prefix buf tabs;

  match expr with
  | Integer (_, i) ->
      add_string buf "Integer: ";
      add_string buf (string_of_int i)
  | Double (_, d) ->
      add_string buf "Double: ";
      add_string buf (Printf.sprintf "%f" d)
  | Boolean (_, b) ->
      add_string buf "Boolean: ";
      add_string buf (string_of_bool b)
  | IfThenElse (_, _, ie, te, ee) ->
      add_string buf "IfThenElse: ";
      generate_expr buf (tabs + 1) ie;
      generate_expr buf (tabs + 1) te;
      generate_expr buf (tabs + 1) ee
  | IfThenBlock (_, _, ie, tbl, ebl) ->
      add_string buf "IfThenBlock: ";
      generate_expr buf (tabs + 1) ie;
      generate_block buf (tabs + 1) tbl;
      generate_block buf (tabs + 1) ebl
  | Func (_, _, args, ret_type, stmts) ->
      add_string buf "Func: ";
      generate_func_args buf (tabs + 1) args;
      generate_type buf (tabs + 1) ret_type "Return ";
      generate_block buf (tabs + 1) stmts
  | FuncCall (_, _, name, args) ->
      add_string buf "Call: ";
      add_string buf name;
      generate_call_args buf (tabs + 1) args
  | BinOp (_, _, lhs, op, rhs) ->
      add_string buf "Binop: ";
      generate_bin_op buf op;
      generate_expr buf (tabs + 1) lhs;
      generate_expr buf (tabs + 1) rhs
  | Ident (_, _, i) ->
      add_string buf "Ident: ";
      add_string buf i

and generate_bin_op buf op =
  let op_str =
    match op with
    | Plus -> "+"
    | Minus -> "-"
    | Mult -> "*"
    | Div -> "/"
    | And -> "and"
    | Or -> "or"
  in
  add_string buf op_str

and generate_call_arg buf tabs arg = generate_expr buf tabs arg

and generate_call_args buf tabs = function
  | x :: xs ->
      generate_call_arg buf tabs x;
      generate_call_args buf tabs xs
  | [] -> ()

and generate_type buf tabs type_ prefix =
  generate_prefix buf tabs;
  add_string buf prefix;
  add_string buf "Type: ";
  add_string buf (Types.to_string type_)

and generate_stmt buf tabs = function
  | Expr expr -> generate_expr buf tabs expr
  | LetBind (_, mut, name, type_, expr) ->
      generate_prefix buf tabs;
      add_string buf (if mut then "Var: " else "Let: ");
      add_string buf name;
      generate_type buf (tabs + 1) type_ "";
      generate_expr buf (tabs + 1) expr
  | FuncBind (_, name, expr) ->
      generate_prefix buf tabs;
      add_string buf "FuncBind: ";
      add_string buf name;
      generate_expr buf (tabs + 1) expr

and generate_stmts buf tabs = function
  | x :: xs ->
      generate_stmt buf (tabs + 1) x;
      generate_stmts buf tabs xs
  | [] -> ()

and generate_block buf tabs stmts =
  generate_prefix buf tabs;
  add_string buf "Block: ";
  generate_stmts buf tabs stmts

and generate_func_arg buf tabs var type_ =
  generate_prefix buf tabs;
  add_string buf "Arg: ";
  add_string buf var;
  generate_type buf (tabs + 1) type_ ""

and generate_func_args' buf tabs type_ = function
  | x :: xs ->
      generate_func_arg buf tabs x type_;
      generate_func_args' buf tabs type_ xs
  | [] -> ()

and generate_func_args buf tabs = function
  | x :: xs ->
      generate_func_args' buf tabs x.arg_type x.arg_vars;
      generate_func_args buf tabs xs
  | [] -> ()

let rec generate_top_level buf = function
  | x :: xs ->
      generate_stmt buf 0 x;
      generate_top_level buf xs
  | [] ->
      add_char buf '\n';
      contents buf

let generate stmts =
  (* Adjust this number - initial capacity *)
  let buf = create 1024 in
  add_string buf "Program";
  generate_top_level buf stmts
