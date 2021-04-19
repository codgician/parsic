# naive-parsec

![Build Status](https://github.com/codgician/naive-parsec/actions/workflows/build.yml/badge.svg)

:space_invader: A naive parser combinator, written purely for fun.

## To-do

- [ ] Core
    - [x] Parser
    - [x] ParseState
    - [x] Logger
- [ ] Combinators
    - [x] `char`: Consumes one char at a time from parse stream
    - [x] `satisfy`: Consumes one char if given condition satisifies
    - [x] `literal`: Consumes given literal string
    - [x] `pure`: A parser that consumes nothing and always succeeds
    - [x] `empty`: A parser that always fails
    - [ ] `fix`: Fixed-point combinator to support recursive syntax
    - [ ] `fmap`: Monadic fmap
    - [ ] `bind`: Monadic bind
    - [ ] `and`: Sequential combinators
    - [x] `or`: Alternative combinators
    - [ ] `some`, `many`: Replicative combinators
- [ ] Enhancements
    - [ ] Support returning multiple results
    - [ ] Customizable error messages
