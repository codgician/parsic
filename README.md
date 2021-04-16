# naive-parsec

:space_invader: A naive parser combinator, written purely for fun.

## To-do

- [-] Core
    - [x] Parser
    - [x] ParseState
    - [x] Logger
- [-] Combinators
    - [x] `char`: Consumes one char at a time from parse stream
    - [] `satisfy`: Consume one char if given consition satisifies
    - [] `digit`: Digit literal
    - [] `letter`: Letter literal
    - [] `string`: String literal
    - [ ] `fmap`: Monadic fmap
    - [ ] `bind`: Monadic bind
    - [ ] `pair`
    - [ ] `left`
    - [ ] `right`
    - [ ] `some`
    - [ ] `many`
- [ ] Refinements
    - [ ] Support returning multiple results