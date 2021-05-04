# naive-parsec

![Build Status](https://github.com/codgician/naive-parsec/actions/workflows/build.yml/badge.svg)

:frog: A naive parser combinator written while learning Rust.

## To-do

- 💖 **Core**
    - [x] `Parsable`: anything that could be parsed
    - [x] `ParseLogger`: logger for parser
    - [x] `Parser`: wrapper for parsable
- 🐣 **Primitives**
    - [x] `StrState`: parse state for `&str`
    - [x] `char`: consumes one char at a time from parse stream
    - [x] `satisfy`: consumes one char if given condition satisifies
    - [x] `literal`: consumes given literal string
    - [x] `regex`: consumes literal string that matches given regular expression
- 🍡 **Combinators**
    - [x] `map`: Maps the result of current parser to another value
    - [x] `map_opt`: `map`, but automatically unwraps `Option<T>` and `Result<T, E>`
    - [x] `pure`: injects value into an identity parser
    - [x] `compose`: compose one parser with another if applicable
    - [x] `empty`: a parser that always fails
    - [x] `fix`: fixed-point combinator for recursive syntax
    - [x] `and`: sequential combinator (pair)
    - [x] `bind`: monadic bind operator for context sensitive parsing
    - [x] `left`, `right`, `mid`: sequencial combinators (select left / right / middle)
    - [x] `or`: alternative combinators
    - [x] `some`, `many`: replicative combinators
    - [x] `info`, `warn`, `error`: log combinators
    - [ ] `pos`: returns current parsing position as result
- ✨ **Enhancements**
    - [x] Overload operators: `>>`, `<<`, `/`, `|`, `&`, `*`
    - [ ] ~~Support returning multiple results~~
    - [ ] Advanced error handling
- 🩺 **Tests**
    - [ ] Property tests **(WIP)**
    - [ ] Arthimetic calculator **(WIP)**
    - [ ] Markdown parser
- 📄 **Docs**
    - [ ] Core **(WIP)**
    - [ ] Combinators **(WIP)**
    - [ ] Wrappers **(WIP)**
  