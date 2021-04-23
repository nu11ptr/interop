import sys

from lark import Lark

from interop.ast import ToAST

_PARSER = "interop.lark"


def create_parser() -> Lark:
    with open(_PARSER, "r") as f:
        parser = Lark(f, start="program", debug=True, parser="lalr")

    return parser


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python compile.py <source.int>")
        sys.exit(1)

    with open(sys.argv[1], "r") as f:
        src = f.read()

    parser = create_parser()

    tree = parser.parse(src)
    print("Parse Tree: ", tree, "\n")

    t = ToAST()
    print("AST: ", t.transform(tree))
