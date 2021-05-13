var searchIndex = JSON.parse('{\
"naive_parsec":{"doc":"A naive parser combinator written while learning Rust.","i":[[0,"combinators","naive_parsec","Generic parser combinators.",null,null],[5,"empty","naive_parsec::combinators","Combinator: <code>empty</code>A parser that consumes no item and …",null,[[],["parser",3]]],[5,"pure","","Combinator: <code>pure</code>Injects a value into an identity parser.",null,[[["clone",8]],[["parser",3],["clone",8]]]],[5,"bind","","Combinator: <code>bind</code> (function ver.)Monadic bind operator …",null,[[],["parser",3]]],[8,"BindExt","","Implement <code>bind</code> combinator for <code>Parsable<S></code>.",null,null],[11,"bind","","Combinator: <code>bind</code>Monadic bind operator <code>(>>=)</code> for context …",0,[[],["parser",3]]],[5,"compose","","Combinator: <code>compose</code> (function ver.)Functional composition …",null,[[],[["parser",3],["clone",8]]]],[8,"ComposeExt","","Implement <code>compose</code> combinator for <code>Parsable<S></code>.",null,null],[11,"compose","","Combinator: <code>compose</code>Functional composition between parsers.",1,[[],["parser",3]]],[8,"LogExt","","Implement error related combinators for <code>Parsable<S></code>.",null,null],[11,"info","","Combinator: <code>info</code>",2,[[["str",15]],["parser",3]]],[11,"warn","","Combinator: <code>warn</code>",2,[[["str",15]],["parser",3]]],[11,"error","","Combinator: <code>error</code>",2,[[["str",15]],["parser",3]]],[11,"inspect","","Combinator: <code>inspect</code>",2,[[],["parser",3]]],[11,"recover","","Combinator: <code>recover</code>",2,[[],["parser",3]]],[3,"Fix","","Data structure for <code>fix</code> combinator.",null,null],[5,"fix","","Combinator: <code>fix</code>In Rust, closures are anonymous functions, …",null,[[],["parser",3]]],[5,"map","","Combinator: <code>map</code> (function ver.)Maps the result of a …",null,[[],[["parser",3],["clone",8]]]],[5,"map_option","","Combinator: <code>map_option</code> (function ver.)Maps the result of …",null,[[],[["parser",3],["clone",8]]]],[5,"map_result","","Combinator: <code>map_result</code> (function ver.)Maps the result of …",null,[[],[["parser",3],["clone",8]]]],[8,"MapExt","","Implement <code>map</code> and related combinators for <code>Parsable</code>.",null,null],[11,"map","","Combinator: <code>map</code>Maps the result of current parser to …",3,[[],["parser",3]]],[11,"map_option","","Combinator: <code>map_option</code>Maps the result of a parser to …",3,[[],["parser",3]]],[11,"map_result","","Combinator: <code>map_result</code>Maps the result of a parser to …",3,[[],["parser",3]]],[5,"or","","Combinator: <code>or</code> (function ver.)Alternative combinator. …",null,[[],[["parser",3],["clone",8]]]],[8,"OrExt","","Implement <code>or</code> combinator for <code>Parsable<S></code>.",null,null],[11,"or","","Combinator: <code>or</code> (function ver.)Alternative combinator. …",4,[[],["parser",3]]],[5,"many","","Combinator: <code>many</code> (function ver.)Apply given parser as …",null,[[],[["vec",3],["clone",8],["parser",3]]]],[5,"some","","Combinator: <code>some</code> (function ver.)Apply given parser as …",null,[[],[["vec",3],["clone",8],["parser",3]]]],[5,"optional","","Combinator: <code>optional</code> (function ver.)Apply given parser <strong>at …",null,[[],[["parser",3],["option",4],["clone",8]]]],[8,"ReplicativeExt","","Implement replicative combinators for <code>Parsable<S></code>.",null,null],[11,"many","","Combinator: <code>many</code>Apply given parser as many times as …",5,[[],[["vec",3],["parser",3]]]],[11,"some","","Combinator: <code>some</code> (function ver.)Apply given parser as …",5,[[],[["vec",3],["parser",3]]]],[11,"optional","","Combinator: <code>optional</code>Apply given parser <strong>at most one time</strong>. …",5,[[],[["parser",3],["option",4]]]],[5,"and","","Combinator: <code>and</code> (function ver.)A sequential combinator …",null,[[],[["clone",8],["parser",3]]]],[5,"left","","Combinator: <code>left</code> (function ver.)A sequential combinator …",null,[[],[["parser",3],["clone",8]]]],[5,"right","","Combinator: <code>right</code> (function ver.)A sequential combinator …",null,[[],[["parser",3],["clone",8]]]],[5,"mid","","Combinator: <code>mid</code> (function ver.)A sequential combinator …",null,[[],[["parser",3],["clone",8]]]],[8,"SequentialExt","","Implement sequential combinators for <code>Parsable<S></code>.",null,null],[11,"and","","Combinator: <code>and</code> (function ver.)A sequential combinator …",6,[[],["parser",3]]],[11,"left","","Combinator: <code>left</code>A sequential combinator that applys the …",6,[[],["parser",3]]],[11,"right","","Combinator: <code>right</code>A sequential combinator that applys the …",6,[[],["parser",3]]],[11,"mid","","Combinator: <code>mid</code>A sequential combinator that applys three …",6,[[],["parser",3]]],[0,"core","naive_parsec","Definitions of a parser and its friends.",null,null],[3,"Lazy","naive_parsec::core","Trait: <code>Lazy</code>Wraps anything that implements <code>Parsable</code> to …",null,null],[5,"lazy","","",null,[[],["lazy",3]]],[3,"Pos","","Struct <code>Pos</code>Data structure for parsing position.",null,null],[4,"Msg","","Data structure for log messages.",null,null],[13,"Info","","",7,null],[13,"Warn","","",7,null],[13,"Error","","",7,null],[3,"MsgBody","","Struct <code>MsgBody</code>Data structure for error message body.",null,null],[12,"msg","","",8,null],[12,"pos","","",8,null],[3,"ParseLogger","","Struct <code>ParseLogger</code>An implementation of parse logger that …",null,null],[12,"stack","","",9,null],[3,"Parser","","<code>Parser</code> structWraps the parser function.",null,null],[8,"Parsable","","<code>Parsable</code> traitAnything that is parsable should implement …",null,null],[16,"Stream","","",10,null],[16,"Result","","",10,null],[10,"parse","","Parse function",10,[[["parselogger",3]],["option",4]]],[11,"exec","","Wrapper for parse function",10,[[]]],[0,"primitives","naive_parsec","<code>CharStream</code> and its primitive parser combinators.",null,null],[0,"combinators","naive_parsec::primitives","",null,null],[5,"satisfy","naive_parsec::primitives::combinators","Combinator: <code>satisfy</code>Consume a single character if given …",null,[[],[["charstream",3],["parser",3],["char",15]]]],[5,"char","","Combinator: <code>char</code>Consume the given char from the parse …",null,[[["char",15]],[["charstream",3],["parser",3],["char",15]]]],[5,"literal","","Combinator: <code>literal</code>Consume given literal string from the …",null,[[["str",15]],[["parser",3],["str",15],["charstream",3]]]],[5,"regex","","Combinator: <code>regex</code>Consume a literal string that matches …",null,[[["str",15]],[["parser",3],["str",15],["charstream",3]]]],[5,"space","","Combinator: <code>space</code>Consume a single whitespace character (<code> </code>…",null,[[],[["charstream",3],["parser",3],["char",15]]]],[5,"trim","","Combinator: <code>trim</code> (function ver.)Consume as many …",null,[[],[["parser",3],["charstream",3]]]],[8,"PrimitiveExt","","Implement <code>trim</code> method for <code>Parsable<CharStream></code>:",null,null],[11,"trim","","Combinator: <code>trim</code>Consume as many whitespace characters (<code> </code>, …",11,[[],[["parser",3],["charstream",3]]]],[0,"stream","naive_parsec::primitives","",null,null],[3,"CharStream","naive_parsec::primitives::stream","Struct: <code>CharStream</code>An implementation for parse stream …",null,null],[11,"new","","Create a new CharStream instance",12,[[["str",15]]]],[11,"as_str","","Return the <code>&str</code> form of parse stream",12,[[],["str",15]]],[11,"pos","","Return current position of parsing",12,[[],["pos",3]]],[11,"index","","Return current index",12,[[],["usize",15]]],[11,"len","","Return length of parse string",12,[[],["usize",15]]],[11,"is_empty","","Check if the stream is empty",12,[[],["bool",15]]],[11,"from","naive_parsec::combinators","",13,[[]]],[11,"into","","",13,[[]]],[11,"to_owned","","",13,[[]]],[11,"clone_into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"from","naive_parsec::core","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_owned","","",14,[[]]],[11,"clone_into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"to_owned","","",15,[[]]],[11,"clone_into","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"to_string","","",7,[[],["string",3]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_owned","","",9,[[]]],[11,"clone_into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_owned","","",16,[[]]],[11,"clone_into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"from","naive_parsec::primitives::stream","",12,[[]]],[11,"into","","",12,[[]]],[11,"into_iter","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"parse","naive_parsec::core","",14,[[["parselogger",3]],["option",4]]],[11,"parse","","",16,[[["parselogger",3]],["option",4]]],[11,"into_iter","","",9,[[]]],[11,"next","naive_parsec::primitives::stream","",12,[[],["option",4]]],[11,"clone","naive_parsec::combinators","",13,[[]]],[11,"clone","naive_parsec::core","",14,[[],["lazy",3]]],[11,"clone","","",15,[[],["pos",3]]],[11,"clone","","",7,[[],["msg",4]]],[11,"clone","","",8,[[],["msgbody",3]]],[11,"clone","","",9,[[],["parselogger",3]]],[11,"clone","","",16,[[],["parser",3]]],[11,"clone","naive_parsec::primitives::stream","",12,[[],["charstream",3]]],[11,"default","naive_parsec::core","",15,[[],["pos",3]]],[11,"default","","",9,[[],["parselogger",3]]],[11,"eq","","",15,[[["pos",3]],["bool",15]]],[11,"ne","","",15,[[["pos",3]],["bool",15]]],[11,"eq","","",7,[[["msg",4]],["bool",15]]],[11,"ne","","",7,[[["msg",4]],["bool",15]]],[11,"eq","","",8,[[["msgbody",3]],["bool",15]]],[11,"ne","","",8,[[["msgbody",3]],["bool",15]]],[11,"eq","","",9,[[["parselogger",3]],["bool",15]]],[11,"ne","","",9,[[["parselogger",3]],["bool",15]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",9,[[["formatter",3]],["result",6]]],[11,"fmt","naive_parsec::primitives::stream","",12,[[["formatter",3]],["result",6]]],[11,"fmt","naive_parsec::core","",7,[[["formatter",3]],["result",6]]],[11,"mul","","",16,[[]]],[11,"bitand","","",16,[[]]],[11,"bitor","","",16,[[]]],[11,"shl","","",16,[[]]],[11,"shr","","",16,[[]]],[11,"bind","naive_parsec::combinators","Combinator: <code>bind</code>Monadic bind operator <code>(>>=)</code> for context …",0,[[],["parser",3]]],[11,"compose","","Combinator: <code>compose</code>Functional composition between parsers.",1,[[],["parser",3]]],[11,"info","","Combinator: <code>info</code>",2,[[["str",15]],["parser",3]]],[11,"warn","","Combinator: <code>warn</code>",2,[[["str",15]],["parser",3]]],[11,"error","","Combinator: <code>error</code>",2,[[["str",15]],["parser",3]]],[11,"inspect","","Combinator: <code>inspect</code>",2,[[],["parser",3]]],[11,"recover","","Combinator: <code>recover</code>",2,[[],["parser",3]]],[11,"into_parser","","",13,[[],["parser",3]]],[11,"map","","Combinator: <code>map</code>Maps the result of current parser to …",3,[[],["parser",3]]],[11,"map_option","","Combinator: <code>map_option</code>Maps the result of a parser to …",3,[[],["parser",3]]],[11,"map_result","","Combinator: <code>map_result</code>Maps the result of a parser to …",3,[[],["parser",3]]],[11,"or","","Combinator: <code>or</code> (function ver.)Alternative combinator. …",4,[[],["parser",3]]],[11,"many","","Combinator: <code>many</code>Apply given parser as many times as …",5,[[],[["vec",3],["parser",3]]]],[11,"some","","Combinator: <code>some</code> (function ver.)Apply given parser as …",5,[[],[["vec",3],["parser",3]]]],[11,"optional","","Combinator: <code>optional</code>Apply given parser <strong>at most one time</strong>. …",5,[[],[["parser",3],["option",4]]]],[11,"and","","Combinator: <code>and</code> (function ver.)A sequential combinator …",6,[[],["parser",3]]],[11,"left","","Combinator: <code>left</code>A sequential combinator that applys the …",6,[[],["parser",3]]],[11,"right","","Combinator: <code>right</code>A sequential combinator that applys the …",6,[[],["parser",3]]],[11,"mid","","Combinator: <code>mid</code>A sequential combinator that applys three …",6,[[],["parser",3]]],[11,"new","naive_parsec::core","",15,[[["usize",15]]]],[11,"add","","",15,[[["usize",15]]]],[11,"row","","",15,[[],["usize",15]]],[11,"col","","",15,[[],["usize",15]]],[11,"new","","",8,[[["option",4],["pos",3],["str",15]]]],[11,"add","","Insert a new log message",9,[[["msg",4]]]],[11,"clear","","Clear all existing logs",9,[[]]],[11,"with","","Intialize a new instance with provided log message",9,[[["msg",4]]]],[11,"len","","Return number of logs",9,[[],["usize",15]]],[11,"is_empty","","Check if logger is empty",9,[[],["bool",15]]],[11,"new","","",16,[[]]],[11,"exec","","Wrapper for parse function",10,[[]]]],"p":[[8,"BindExt"],[8,"ComposeExt"],[8,"LogExt"],[8,"MapExt"],[8,"OrExt"],[8,"ReplicativeExt"],[8,"SequentialExt"],[4,"Msg"],[3,"MsgBody"],[3,"ParseLogger"],[8,"Parsable"],[8,"PrimitiveExt"],[3,"CharStream"],[3,"Fix"],[3,"Lazy"],[3,"Pos"],[3,"Parser"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);