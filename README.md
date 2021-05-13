# Parsic

![Build Status](https://github.com/codgician/parsic/actions/workflows/build.yml/badge.svg)
![Doc status](https://github.com/codgician/parsic/actions/workflows/doc.yml/badge.svg)

🪄 A naive parser combinator written while learning Rust. **Pars**ing mag**ic** ~~tragic~~.

## To-do

- 💖 **Core**
    - [x] `Parser`: wrapper for parser function
    - [x] `ParseLogger`: logger for parser
    - [x] `Parsable`: anything that could be parsed
    - [x] `IntoParser`: anything that could be converted into `Parser`
- 🐣 **Primitives**
    - [x] `CharStream`: parse state for `&str`
    - [x] `char`: consumes one char at a time from parse stream
    - [x] `satisfy`: consumes one char if given condition satisifies
    - [x] `literal`: consumes given literal string
    - [x] `regex`: consumes literal string that matches given regular expression
    - [x] `trim`: constructs a parser that consumes whitespaces at both ends
- 🍡 **Combinators**
    - [x] `map`: Maps the result of current parser to another value
    - [x] `map_option`: `map`, but automatically unwraps `Option<T>`
    - [x] `map_result`: `map`, but automatically unwraps `Result<T, E>`
    - [x] `pure`: injects value into an identity parser
    - [x] `compose`: compose one parser with another if applicable
    - [x] `empty`: a parser that always fails
    - [x] `fix`: fixed-point combinator for recursive syntax
    - [x] `and`: sequential combinator (pair)
    - [x] `bind`: monadic bind operator for context sensitive parsing
    - [x] `left`, `right`, `mid`: sequencial combinators (select left / right / middle)
    - [x] `or`: alternative combinators
    - [x] `many`, `some`, `optional`: replicative combinators
    - [x] `info`, `warn`, `error`: log combinators
    - [x] `inspect`: returns parser result alongwith current parsing state
    - [x] `recover`: returns a fallback value is given parser fails
- ✨ **Enhancements**
    - [x] Overload operators: `>>`, `<<`, `/`, `&`, `*`
    - [ ] ~~Support returning multiple results~~
    - [ ] Advanced error handling **(Planning)**
- 🩺 **Tests**
    - [x] Tests for laws of Functor, Applicative and Monad.
    - [x] Arthimetic calculator
    - [ ] Some more real-world tests **(Planning)**
- 📄 **Docs**
    - [x] Core
    - [x] Primitives
    - [x] Combinators
