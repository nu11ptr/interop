# Introduction and Features

Interop is a simple programming language designed for general purpose programming. It does not generate native machine code, but instead compiles to other programming languages for simple FFI. Below is a quick highlight of the features Interop provides.

Before we get to that, why do I think a new language is needed? In summary, desired language features don't compose between languages. What do I mean by that? I mean if language A that you like has A, B, C features/syntax/elements and language B has D, E, and F features how do you use them together? Answer: You don't, you settle. This right here is why there is so many languages. You can't just fundementally merge two languages, you create a third.

Enough of that, on to the language features!

## Feature Summary

- Python inspired syntax and behavior
    - "magic methods"
    - `with` blocks
    - List/set/map/generator comprehensions
    - "Truthy" boolean expressions

- Opinionated auto-formatting
    - Only one correct way to format code - language designed for it
    - Whitespace not significant (uses `:` and `end` to deliminate blocks)

- Compiles to other languages (no direct native compilatoin)
    - Takes advantage of years of libraries and optimizations from other languages
    - Multiple backends:
        - Go = 1st
        - Future: Bytecode (for fast compiles, REPL, and testing only)
        - Future: C/C++
        - ???

- Sum types and pattern matching
    - First class enums

- Closures and first class functions

- No exceptions
    - Result and option enums
    - Try/catch blocks for auto-unwrapping (no stack unwinding)

- Limited or no subtyping (undecided)
    - First class interfaces play role of subtypes

- Static typing
    - Basic type inference - top level functions define types
    - Basic generics

- Value and reference types
    - Value types are immutable

- A safe language
    - No null poiners, statically typed, etc. (but not as safe as Ada/Rust)

- Highly opinionated, not a lot of different ways to do things (Go)

- Slightly functional, but imperative (Rust)
    - Most things are an expression, first class functions, sum types, pattern matching, etc.

- Automatic memory management
    - Memory cleanup strategy inherited by target language
    - Value types will minimize amount of garbage on some targets

- First class parallelism
    - Still under consideration
    - Leaning towards async/await model
