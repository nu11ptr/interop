type t =
  | Unknown
  | Unit
  | Integer
  | Double
  | Boolean
  | Func of t list * t
  | Raw of string

let basic s =
  match s with
  | "" -> Unknown
  | "unit" -> Unit
  | "int" -> Integer
  | "double" -> Double
  | "bool" -> Boolean
  | _ as raw ->
      (* TODO: process func type defaulting to raw if it doesn't match regex *)
      Raw raw

let basicOpt ?(func_rt = false) s =
  let s' =
    match s with Some s -> s | None when func_rt -> "unit" | None -> ""
  in
  basic s'

let basic_func arg_types rt =
  let ret_type = basicOpt ~func_rt:true rt in
  (Func (arg_types, ret_type), ret_type)

let rec to_string t =
  match t with
  | Unknown -> "<Unknown>"
  | Unit -> "unit"
  | Integer -> "int"
  | Double -> "double"
  | Boolean -> "bool"
  | Func (arg_types, ret_type) ->
      let arg_types' = List.map to_string arg_types in
      "func (" ^ String.concat ", " arg_types' ^ ") -> " ^ to_string ret_type
  | Raw s -> "Raw:" ^ s

let is_known = ( = ) Unknown

let is_complete = function Unknown | Raw _ -> false | _ -> true
