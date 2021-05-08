initSidebarItems({"fn":[["char","Combinator: `char`Consume the given char from the parse stream. `char(x)` is equivalent to `satisfy(|x: &char| *x == ch)`"],["literal","Combinator: `literal`Consume given literal string from the parse stream.test"],["regex","Combinator: `regex`Consume a literal string that matches given regular expression."],["satisfy","Combinator: `satisfy`Consume a single character if given function applied to the next character from the parse stream yields `true`."],["space","Combinator: `space`Consume a single whitespace character (` `, `\\n`, `\\r` or `\\t`). Equivalant to `char(' ').or(char('\\n')).or(char('\\r')).or(char('\\t'))`."],["trim","Combinator: `trim` (function ver.)Consume as many whitespace characters (` `, `\\n`, `\\r` or `\\t`) as possible surrounding given parser. `trim(p)` is equivalant to `mid(space().many(), p, space().many())`."]],"trait":[["PrimitiveExt","Implement `trim` method for `Parsable<CharStream>`:"]]});