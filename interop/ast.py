from dataclasses import dataclass
from typing import List, Optional

from lark import Token, Transformer


class Node:
    pass


class Expr(Node):
    pass


@dataclass
class Integer(Expr):
    value: int


@dataclass
class Ident(Expr):
    value: str


@dataclass
class String(Expr):
    value: str


@dataclass
class CallArg(Node):
    name: Optional[Ident]
    arg: Expr


@dataclass
class Call(Expr):
    func: Ident
    args: List[CallArg]


class ToAST(Transformer):
    def IDENT(self, token: Token) -> Ident:
        return Ident(token.value)

    def INTEGER(self, token: Token) -> Integer:
        return Integer(token.value)

    def STRING(self, token: Token) -> String:
        return String(token.value)

    def call_arg(self, data: List[Expr]) -> CallArg:
        if len(data) == 2:
            return CallArg(name=data[0], arg=data[1])  # type: ignore

        return CallArg(name=None, arg=data[0])

    def call(self, data: List[Node]) -> Call:
        func_name, args = data[0], data[1:]
        return Call(func_name, args)  # type: ignore
