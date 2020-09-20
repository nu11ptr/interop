let print_position outp (lexbuf : Lexing.lexbuf) =
  let pos = lexbuf.lex_curr_p in
  Printf.fprintf outp "%s:%d:%d" pos.pos_fname pos.pos_lnum
    (pos.pos_cnum - pos.pos_bol + 1)

let token_to_string tok =
  match tok with
  | Parser.EQUALS -> "EQUALS"
  | Parser.PLUS -> "PLUS"
  | Parser.MINUS -> "MINUS"
  | Parser.MULT -> "MULT"
  | Parser.DIV -> "DIV"
  | Parser.LPAREN -> "LPAREN"
  | Parser.RPAREN -> "RPAREN"
  | Parser.COLON -> "COLON"
  | Parser.SEMI -> "SEMI"
  | Parser.COMMA -> "COMMA"
  | Parser.RARROW -> "RARROW"
  | Parser.FUNC -> "FUNC"
  | Parser.END -> "END"
  | Parser.LET -> "LET"
  | Parser.VAR -> "VAR"
  | Parser.IF -> "IF"
  | Parser.THEN -> "THEN"
  | Parser.ELSE -> "ELSE"
  | Parser.OR -> "OR"
  | Parser.AND -> "AND"
  | Parser.BOOLEAN b -> "BOOLEAN(" ^ string_of_bool b ^ ")"
  | Parser.INTEGER i -> "INTEGER(" ^ string_of_int i ^ ")"
  | Parser.DOUBLE d -> "DOUBLE(" ^ string_of_float d ^ ")"
  | Parser.IDENT id -> "IDENT(" ^ id ^ ")"
  | Parser.EOF -> "EOF"

let insert_semi = ref false

let insert_semicolons (lexbuf : Lexing.lexbuf) =
  let tok = Lexer.read_token !insert_semi lexbuf in
  (* print_string (token_to_string tok);
     print_newline (); *)
  match tok with
  | Parser.RPAREN as t ->
      insert_semi := true;
      t
  | Parser.IDENT _ as t ->
      insert_semi := true;
      t
  | Parser.INTEGER _ as t ->
      insert_semi := true;
      t
  | Parser.DOUBLE _ as t ->
      insert_semi := true;
      t
  | Parser.BOOLEAN _ as t ->
      insert_semi := true;
      t
  (* TODO: This leads to some ugly user code on single line - rethink 'end' and semicolons *)
  | Parser.END as t ->
      insert_semi := true;
      t
  | _ as t ->
      insert_semi := false;
      t

let gen_ast lexbuf =
  try Parser.program insert_semicolons lexbuf with
  | Lexer.SyntaxError msg ->
      Printf.fprintf stderr "%a: %s\n" print_position lexbuf msg;
      exit (-1)
  | Parser.Error ->
      Printf.fprintf stderr "%a: syntax error\n" print_position lexbuf;
      exit (-1)

let lexbuf_from_file filename =
  let inp = open_in filename in
  let lexbuf = Lexing.from_channel inp in
  lexbuf.lex_curr_p <- { lexbuf.lex_curr_p with pos_fname = filename };
  lexbuf

let lexbuf_from_string code =
  let lexbuf = Lexing.from_string code in
  lexbuf.lex_curr_p <- { lexbuf.lex_curr_p with pos_fname = "<string>" };
  lexbuf

let gen_ast_string lexbuf = gen_ast lexbuf |> Print_ast.generate
