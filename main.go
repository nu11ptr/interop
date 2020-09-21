//go:generate java -Xmx500M -cp ".:$CLASSPATH" org.antlr.v4.Tool -Dlanguage=Go Interop.g4 -visitor -no-listener -o pkg/parser/
package main

import (
	"os"

	"github.com/antlr/antlr4/runtime/Go/antlr"
	"github.com/nu11ptr/interop/pkg/parser"
)

func main() {
	input, _ := antlr.NewFileStream(os.Args[1])
	lexer := parser.NewInteropLexer(input)
	stream := antlr.NewCommonTokenStream(lexer, 0)
	p := parser.NewInteropParser(stream)
	p.AddErrorListener(antlr.NewDiagnosticErrorListener(true))
	p.BuildParseTrees = true
	p.Program()
}
