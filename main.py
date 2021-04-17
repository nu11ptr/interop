from lark import Lark


with open("interop.lark", "r") as f:
    parser = Lark(f, start="call", debug=True, parser="lalr")
