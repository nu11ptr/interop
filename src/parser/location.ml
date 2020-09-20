type t =
  (* start line num, start col num, end line num, end col num *)
  | Position of int * int * int * int
  | Nowhere

let create (startpos : Lexing.position) (endpos : Lexing.position) =
  Position
    ( startpos.pos_lnum,
      startpos.pos_cnum - startpos.pos_bol,
      endpos.pos_lnum,
      endpos.pos_cnum - endpos.pos_bol )

let to_string = function
  | Position (sl, sc, el, ec) when sl = el ->
      Printf.sprintf "Line %d, char %d-%d" sl sc ec
  | Position (sl, sc, el, ec) ->
      Printf.sprintf "Line %d:%d to line %d:%d" sl sc el ec
  | Nowhere -> "<Unknown>"
