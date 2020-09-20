open OUnit2
open Parsing

let code_func_bind =
  {src|
func test(h, w: double):
end

func test2(h: int) -> int:
    let f =
        func (x: int) -> int:
            x
        end
    1
end
|src}

let code_simple =
  {src|
func circle(r: double) -> double:
    3.14 * r * r
end

# A comment
func cyl(r: double, h: double) -> double:
    h * circle(r) # Another comment
end

# NOTE: In real lang main takes list of args and returns int
func main():
    let vol: double = cyl(2.0, 5.0)
    println(vol)
end
|src}

let expect_func_bind =
  {src|Program
└──FuncBind: test
   └──Func: 
      └──Arg: h
         └──Type: double
      └──Arg: w
         └──Type: double
      └──Return Type: unit
      └──Block: 
└──FuncBind: test2
   └──Func: 
      └──Arg: h
         └──Type: int
      └──Return Type: int
      └──Block: 
         └──Let: f
            └──Type: <Unknown>
            └──Func: 
               └──Arg: x
                  └──Type: int
               └──Return Type: int
               └──Block: 
                  └──Ident: x
         └──Integer: 1
|src}

let expect_simple =
  {src|Program
└──FuncBind: circle
   └──Func: 
      └──Arg: r
         └──Type: double
      └──Return Type: double
      └──Block: 
         └──Binop: *
            └──Binop: *
               └──Double: 3.140000
               └──Ident: r
            └──Ident: r
└──FuncBind: cyl
   └──Func: 
      └──Arg: r
         └──Type: double
      └──Arg: h
         └──Type: double
      └──Return Type: double
      └──Block: 
         └──Binop: *
            └──Ident: h
            └──Call: circle
               └──Ident: r
└──FuncBind: main
   └──Func: 
      └──Return Type: unit
      └──Block: 
         └──Let: vol
            └──Type: double
            └──Call: cyl
               └──Double: 2.000000
               └──Double: 5.000000
         └──Call: println
            └──Ident: vol
|src}

let test_ast expected code _ =
  let lexbuf = Parse.lexbuf_from_string code in
  let ast = Parse.gen_ast_string lexbuf in
  let diffs = Odiff.strings_diffs expected ast in
  print_endline (Odiff.string_of_diffs diffs);
  assert_equal ~msg:"Expected ast not equal to actual" expected ast

let tests =
  "ast tests"
  >::: [
         "simple ast" >:: test_ast expect_simple code_simple;
         "func bind test" >:: test_ast expect_func_bind code_func_bind;
       ]

let () = run_test_tt_main tests
