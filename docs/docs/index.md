# Introduction and Features

Thank you for taking the time to read about the Interop language. It will hopefully become apparent how absolutely passionate I am about this project, and why I think yet another langugage is needed. Below you will find the major language features that I think are worth highlighting at this point.

Before we get to that, why do I think a new language is needed? In summary, desired language features don't compose between languages. What do I mean by that? I mean if language A that you like has A, B, C features/syntax/elements and language B has D, E, and F features how do you use them together? Answer: You don't, you settle. This right here is why there is so many languages. You can't just fundementally merge two languages, you create a third. (And now we have three problems instead of two... 😀)

Enough of that, on to the language features!

## Feature Summary

- Imagine Go, Python, and Rust had a baby 😀
    - Not trying to break huge new ground, but refine existing ideas
- Simple language with a small feature set (Go), but that must compose well together
- Syntax Python-like, but not exactly
    - No significant whitespace
- A safe language
    - No null poiners, statically typed, etc. (but not as safe as Ada/Rust)
- Large focuse on interoperability with other languages by generating into those languages directly (Haxe/Nim)
    - Hence the name "Interop"
    - First target language will be Go
- Highly opinionated, not a lot of different ways to do things (Go)
- Slightly functional, but imperative (Rust)
    - Most things are an expression, first class functions, sum types, pattern matching, etc.
- Error handling using special type of try/catch block but not really exceptions (no stack unwinding - a little like Swift)
- Major focus on auto-formatting
    - There will be one "correct" way to format and the language will be designed for it
- Automatic memory management
    - Likely using tracing garbage collection for most language targets
    - Value types will minimize amount of garbage on some targets
- Parallelism must be first class
    - Still under consideration
    - Leaning towards async/await model
