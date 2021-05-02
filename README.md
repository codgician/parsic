# naive-parsec

![Build Status](https://github.com/codgician/naive-parsec/actions/workflows/build.yml/badge.svg)

:frog: A naive parser combinator written while learning Rust.

## To-do

- 💖 **Core**
    - [x] Parsable
    - [x] ParseLogger
    - [x] Primitives (`StrState` and related lexers)
- 🍡 **Basic combinators**
    - [x] `char`: Consumes one char at a time from parse stream
    - [x] `satisfy`: Consumes one char if given condition satisifies
    - [x] `literal`: Consumes given literal string
    - [x] `map`: Functor fmap `<$>`
    - [x] `pure`: Injects value into an identity parser
    - [x] `apply`: Applicative apply `<*>`
    - [x] `empty`: A parser that always fails
    - [x] `bind`: bind for monad `>>=`
    - [x] `fix`: Fixed-point combinator for recursive syntax
    - [x] `and`: Sequential combinator (pair)
    - [x] `left`, `right`, `mid`: Sequencial combinators (select left / right / middle)
    - [ ] `chain`: Sequential combinators (iterator)
    - [x] `or`: Alternative combinators
    - [x] `some`, `many`: Replicative combinators
    - [x] `info`, `warn`, `error`: Log combinators
    - [ ] More to be added...
- ✨ **Enhancements**
    - [x] Implement iterator-style interface
    - [ ] ~~Support returning multiple results~~
    - [ ] Customizable error messages
    - [ ] Wrapper for parsers
- 🩺 **Tests**
    - [ ] Property tests
    - [ ] Arthimetic calculator
    - [ ] Markdown parser
- 📄 **Docs**
    - [ ] Core 
    - [ ] Combinators
    - [ ] Wrappers
  