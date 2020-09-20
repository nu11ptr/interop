{
  open Parser

  exception SyntaxError of string

  let next_line (lexbuf : Lexing.lexbuf) =
    let pos = lexbuf.lex_curr_p in
    lexbuf.lex_curr_p <-
      { pos with pos_bol = lexbuf.lex_curr_pos; pos_lnum = pos.pos_lnum + 1 }
}

let ws = [' ' '\t' '\r']
let newline = '\n'
let id = ['a'-'z' 'A'-'Z' '_']['a'-'z' 'A'-'Z' '0'-'9' '_']*
let digit = ['0'-'9']
let int_lit = digit+
let float_lit = digit+'.'digit*

rule read_token insert_semi = parse
  | '='             { EQUALS }
  | '+'             { PLUS }
  | '-'             { MINUS }
  | '*'             { MULT }
  | '/'             { DIV }
  | '('             { LPAREN }
  | ')'             { RPAREN }
  | ':'             { COLON }
  | ';'             { SEMI }
  | ','             { COMMA }
  | '#'             { read_comment insert_semi lexbuf }
  | "->"            { RARROW }
  | "func"          { FUNC }
  | "end"           { END }
  | "let"           { LET }
  | "var"           { VAR }
  | "if"            { IF }
  | "then"          { THEN }
  | "else"          { ELSE }
  | "or"            { OR }
  | "and"           { AND }
  | "true"          { BOOLEAN true }
  | "false"         { BOOLEAN false }
  | int_lit         { INTEGER (int_of_string (Lexing.lexeme lexbuf)) }
  | float_lit       { DOUBLE (float_of_string (Lexing.lexeme lexbuf)) }
  | id              { IDENT (Lexing.lexeme lexbuf) }
  | ws              { read_token insert_semi lexbuf }
  | newline         { next_line lexbuf; if insert_semi then SEMI else (read_token false lexbuf) }
  | eof             { EOF }
  | _ {raise (SyntaxError ("Illegal character: " ^ Lexing.lexeme lexbuf)) }
and read_comment insert_semi = parse
  | newline { next_line lexbuf; if insert_semi then SEMI else (read_token false lexbuf) } 
  | eof { EOF }
  | _ { read_comment insert_semi lexbuf }
