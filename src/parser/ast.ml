type binop = Plus | Minus | Mult | Div | And | Or

type expr =
  | Integer of Location.t * int
  | Double of Location.t * float
  | Boolean of Location.t * bool
  (* If expr, Then block, Else block *)
  | IfThenBlock of Location.t * Types.t * expr * block * block
  (* If expr, Then expr, Else expr *)
  | IfThenElse of Location.t * Types.t * expr * expr * expr
  (* name, args *)
  | FuncCall of Location.t * Types.t * string * expr list
  (* lhs, op, rhs *)
  | BinOp of Location.t * Types.t * expr * binop * expr
  | Ident of Location.t * Types.t * string

and stmt =
  | Expr of expr
  (* mutable, var, type, expr *)
  | LetBind of Location.t * bool * string * Types.t * expr

and block = stmt list

type func_args = {
  location : Location.t;
  arg_vars : string list;
  arg_type : Types.t;
}

type decl =
  (* name, args, ret_type, block*)
  | FuncDecl of
      Location.t * Types.t * string * func_args list * Types.t * stmt list

type prog = decl list

let arg_types args =
  let l =
    List.map (fun arg -> List.map (fun _ -> arg.arg_type) arg.arg_vars) args
  in
  List.flatten l

let expr_type = function
  | Integer _ -> Types.Integer
  | Double _ -> Types.Double
  | Boolean _ -> Types.Boolean
  | IfThenBlock (_, t, _, _, _) -> t
  | IfThenElse (_, t, _, _, _) -> t
  | FuncCall (_, t, _, _) -> t
  | BinOp (_, t, _, _, _) -> t
  | Ident (_, t, _) -> t

let stmt_type = function
  | Expr e -> expr_type e
  | LetBind (_, _, _, _, _) -> Types.Unit

let rec block_type = function
  | [ x ] -> stmt_type x
  | _ :: xs -> block_type xs
  | [] -> Types.Unit

let expr_loc = function
  | Integer (loc, _) -> loc
  | Double (loc, _) -> loc
  | Boolean (loc, _) -> loc
  | IfThenBlock (loc, _, _, _, _) -> loc
  | IfThenElse (loc, _, _, _, _) -> loc
  | FuncCall (loc, _, _, _) -> loc
  | BinOp (loc, _, _, _, _) -> loc
  | Ident (loc, _, _) -> loc

(* Get location of first statement in block *)
let block_start_loc = function
  | LetBind (loc, _, _, _, _) :: _ -> loc
  | Expr s :: _ -> expr_loc s
  | [] -> Nowhere
