# Introduction and Features

Thank you for taking the time to read about the Interop language. It will hopefully become apparent how absolutely passionate I am about this project, and why I think yet another langugage is needed. Below you will find the major language features that I think are worth highlighting at this point.

Before we get to that, why do I think a new language is needed? In summary, desired language features don't compose between languages. What do I mean by that? I mean if language A that you like has A, B, C features/syntax/elements and language B has D, E, and F features how do you use them together? Answer: You don't, you settle. This right here is why there is so many languages. You can't just fundementally merge two languages, you create a third. (And now we have three problems instead of two... 😀)

Enough of that, on to the language features!

## Short Version: TLDR

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


## Long Version:

### Simplicity

^^The language MUST be simple^^. To that end, each feature the language gets must be painstakingly researched and considered to see how it fits into the language as a whole. This will not be a language with several ways to do the same thing. I think Go really made strides here in showing the value of simplicity. The benefit of this is we will not be looking to constantly add the newest thing in programming language theory, but want to keep the language small, simple, and focused.

That said, we are not making a language for complete novices either (unlike Go). This must be a very expressive and productive language with few features, but that compose well together. To that end, it will have a semi-functional feel to it (more below) but ultimately be imperative. Picking up a new language in one or two days is great, but if you are immediately suffering due to lack of compositiion or expressiveness, you won't be as productive longer term.

`Goal`: Imagine Go being on the far left and a language like Rust on the far right. We are aiming somewhat left of center. Considerably less complex than Rust, but solidly more capable than Go.

### Syntax

TODO

### Safety

TODO

### Language Interoperability

TODO

### Highly Opinionated

TODO

### Functional Flair

TODO

### Error Handling

TODO

### Auto Formatting

TODO

### Memory Management

TODO

### First Class Parallalism

TODO
