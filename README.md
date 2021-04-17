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
    - [ ] `digit`: Digit literal
    - [ ] `letter`: Letter literal
    - [x] `literal`: String literal
    - [x] `pure`: Monadic return
    - [ ] `fmap`: Monadic fmap
    - [ ] `bind`: Monadic bind
    - [ ] `and`
    - [ ] `or`
    - [ ] `left`
    - [ ] `right`
    - [ ] `some`
    - [ ] `many`
- [ ] Enhancements
    - [ ] Support returning multiple results
    - [ ] Customizable error messages
