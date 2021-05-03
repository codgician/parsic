# naive-parsec

![Build Status](https://github.com/codgician/naive-parsec/actions/workflows/build.yml/badge.svg)

:frog: A naive parser combinator written while learning Rust.

## To-do

- 💖 **Core**
    - [x] `Parsable`: anything that could be parsed
    - [x] `ParseLogger`: logger for parser
    - [ ] `IntoParser`: anything that could be converted to a parser
    - [ ] `Parser`: wrapper for parsable
- 🐣 **Primitives**
    - [x] `StrState`: parse state for `&str`
    - [x] `char`: consumes one char at a time from parse stream
    - [x] `satisfy`: consumes one char if given condition satisifies
    - [x] `literal`: consumes given literal string
    - [ ] `reg_exp`: consumes literal according to regular expression 
- 🍡 **Basic combinators**
    - [x] `map`: functor fmap `<$>`
    - [x] `pure`: injects value into an identity parser
    - [x] `compose`: applicative composition `<*>`
    - [x] `empty`: a parser that always fails
    - [x] `bind`: bind for monad `>>=`
    - [x] `fix`: fixed-point combinator for recursive syntax
    - [x] `and`: sequential combinator (pair)
    - [x] `left`, `right`, `mid`: sequencial combinators (select left / right / middle)
    - [x] `or`: alternative combinators
    - [x] `some`, `many`: replicative combinators
    - [x] `info`, `warn`, `error`: log combinators
    - [ ] More to be added...
- ✨ **Enhancements**
    - [x] Overload operators: `>>`, `<<`, `/`, `|`, `&`, `*`
    - [ ] ~~Support returning multiple results~~
    - [ ] Customizable error messages
- 🩺 **Tests**
    - [ ] Property tests
    - [ ] Arthimetic calculator
    - [ ] Markdown parser
- 📄 **Docs**
    - [ ] Core 
    - [ ] Combinators
    - [ ] Wrappers
  