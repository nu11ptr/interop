from dataclasses import dataclass, field
from typing import List, Optional

from lark import Token, Transformer


class Node:
    def __init__(self, srow: int, scol: int, erow: int, ecol: int):
        self.start_row = srow
        self.start_col = scol
        self.end_row = erow
        self.end_col = ecol


class Decl(Node):
    pass


class Stmt(Node):
    pass


class Expr(Stmt):
    pass


@dataclass
class Program(Node):
    decls: List[Decl] = field(default_factory=list)


@dataclass
class Integer(Expr):
    value: int


@dataclass
class Ident(Node):
    value: str


@dataclass
class String(Expr):
    value: str


@dataclass
class Boolean(Expr):
    value: bool


@dataclass
class Type(Node):
    name: str


@dataclass
class FuncArg(Node):
    names: List[str]
    type_: Type
    expr: Optional[Expr] = None


@dataclass
class Func(Decl):
    name: str
    args: List[FuncArg] = field(default_factory=list)
    ret_type: Optional[Type] = None
    stmts: List[Stmt] = field(default_factory=list)


@dataclass
class CallArg(Node):
    name: Optional[str]
    arg: Expr


@dataclass
class Call(Expr):
    func: str
    args: List[CallArg]


class ToAST(Transformer):
    def IDENT(self, token: Token) -> Ident:
        return Ident(token.value)

    def INTEGER(self, token: Token) -> Integer:
        return Integer(token.value)

    def STRING(self, token: Token) -> String:
        return String(token.value)

    def TRUE(self, token: Token) -> Boolean:
        return Boolean(True)

    def FALSE(self, token: Token) -> Boolean:
        return Boolean(False)

    def program(self, decls: List[Decl]) -> Program:
        return Program(decls)

    def type_(self, idents: List[Ident]) -> Type:
        return Type(idents[0].value)

    def func_arg(self, data: List[Node]) -> FuncArg:
        last = data[-1]
        if isinstance(last, Expr):
            names = [d.value for d in data[:-2]]
            return FuncArg(names, data[-2], last)
        elif isinstance(last, Type):
            names = [d.value for d in data[:-1]]
            return FuncArg(names, last)
        else:
            raise AssertionError("Malformed function args")

    def func(self, data: List[Node]) -> Func:
        name = data[0].value
        args = []
        ret_type: Optional[Type] = None
        stmts = []

        for d in data[1:]:
            if isinstance(d, FuncArg):
                args.append(d)
            elif isinstance(d, Type):
                ret_type = d
            elif isinstance(d, Stmt):
                stmts.append(d)
            else:
                raise AssertionError(f"Unknown type in function data (type: {type(d)})")

        return Func(name, args, ret_type, stmts)

    def call_arg(self, data: List[Node]) -> CallArg:
        if len(data) == 2:
            return CallArg(name=data[0].value, arg=data[1])  # type: ignore

        return CallArg(name=None, arg=data[0])  # type: ignore

    def call(self, data: List[Node]) -> Call:
        func_name, args = data[0], data[1:]
        return Call(func_name.value, args)  # type: ignore
