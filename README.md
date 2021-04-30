# naive-parsec

![Build Status](https://github.com/codgician/naive-parsec/actions/workflows/build.yml/badge.svg)

:frog: A naive parser combinator written while learning Rust.

## To-do

- :o: Core
    - :heavy_check_mark: Parsable
    - :heavy_check_mark: ParseLogger
    - :heavy_check_mark: Primitives (`StrStream` and related lexers)
- :o: Combinators
    - :heavy_check_mark: `char`: Consumes one char at a time from parse stream
    - :heavy_check_mark: `satisfy`: Consumes one char if given condition satisifies
    - :heavy_check_mark: `literal`: Consumes given literal string
    - :heavy_check_mark: `pure`: A parser that consumes nothing and always succeeds
    - :heavy_check_mark: `empty`: A parser that always fails
    - :heavy_check_mark: `fix`: Fixed-point combinator to support recursive syntax
    - :heavy_check_mark: `map`: fmap for functor
    - :heavy_check_mark: `bind`: bind for monad
    - :heavy_check_mark: `and`: Sequential combinators
    - :heavy_check_mark: `or`: Alternative combinators
    - :heavy_check_mark: `some`, `many`: Replicative combinators
- :x: Tests
    - :x: Property tests
    - :x: Arthimetic calculator
    - :x: Markdown parser
- :o: Enhancements
    - :heavy_check_mark: Implement iterator-style interface
    - ~~:x: Support returning multiple results~~
    - :x: Customizable error messages
    - :o: Wrapper for parsers
