var searchIndex = JSON.parse('{\
"parsic":{"doc":"A naive parser combinator written while learning Rust.","t":[0,5,5,8,11,5,5,8,11,8,11,11,11,11,11,3,5,5,5,5,8,11,11,11,3,5,5,8,11,5,5,5,8,11,11,11,5,5,5,5,8,11,11,11,11,0,3,4,13,13,13,3,12,12,3,12,3,8,16,16,10,11,11,0,0,5,5,5,5,5,5,8,11,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["combinators","empty","or","AlternativeExt","or","pure","compose","ApplicativeExt","compose","LogExt","info","warn","error","inspect","recover","Fix","fix","map","map_option","map_result","FunctorExt","map","map_option","map_result","Lazy","lazy","bind","MonadExt","bind","many","some","optional","ReplicativeExt","many","some","optional","and","left","right","mid","SequentialExt","and","left","right","mid","core","Pos","Msg","Info","Warn","Error","MsgBody","msg","pos","ParseLogger","stack","Parser","Parsable","Stream","Result","parse","exec","into_parser","primitives","combinators","satisfy","char","literal","regex","space","trim","PrimitiveExt","trim","stream","CharStream","new","as_str","pos","index","len","is_empty","from","into","to_owned","clone_into","borrow","borrow_mut","try_from","try_into","type_id","from","into","to_owned","clone_into","borrow","borrow_mut","try_from","try_into","type_id","from","into","to_owned","clone_into","borrow","borrow_mut","try_from","try_into","type_id","from","into","to_owned","clone_into","to_string","borrow","borrow_mut","try_from","try_into","type_id","from","into","to_owned","clone_into","borrow","borrow_mut","try_from","try_into","type_id","from","into","to_owned","clone_into","borrow","borrow_mut","try_from","try_into","type_id","from","into","to_owned","clone_into","borrow","borrow_mut","try_from","try_into","type_id","from","into","into_iter","to_owned","clone_into","borrow","borrow_mut","try_from","try_into","type_id","parse","parse","parse","into_iter","next","clone","clone","clone","clone","clone","clone","clone","clone","default","default","eq","ne","eq","ne","eq","ne","eq","ne","fmt","fmt","fmt","fmt","fmt","fmt","mul","bitand","bitor","shl","shr","or","compose","info","warn","error","inspect","recover","map","map_option","map_result","bind","many","some","optional","and","left","right","mid","new","add","row","col","new","add","clear","with","len","is_empty","new","exec","into_parser"],"q":["parsic","parsic::combinators","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","parsic","parsic::core","","","","","","","","","","","","","","","","","parsic","parsic::primitives","parsic::primitives::combinators","","","","","","","","parsic::primitives","parsic::primitives::stream","","","","","","","parsic::combinators","","","","","","","","","","","","","","","","","","parsic::core","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","parsic::primitives::stream","","","","","","","","","","parsic::combinators","","parsic::core","","parsic::primitives::stream","parsic::combinators","","parsic::core","","","","","parsic::primitives::stream","parsic::core","","","","","","","","","","","","","","parsic::primitives::stream","parsic::core","","","","","","parsic::combinators","","","","","","","","","","","","","","","","","","parsic::core","","","","","","","","","","","",""],"d":["Generic parser combinators.","Combinator: <code>empty</code>","Combinator: <code>or</code> (function ver.)","Implement <code>or</code> combinator for <code>Parsable<S></code>.","Combinator: <code>or</code> (function ver.)","Combinator: <code>pure</code>","Combinator: <code>compose</code> (function ver.)","Implement <code>compose</code> combinator for <code>Parsable<S></code>.","Combinator: <code>compose</code>","Implement error related combinators for <code>Parsable<S></code>.","Combinator: <code>info</code>","Combinator: <code>warn</code>","Combinator: <code>error</code>","Combinator: <code>inspect</code>","Combinator: <code>recover</code>","","Combinator: <code>fix</code>","Combinator: <code>map</code> (function ver.)","Combinator: <code>map_option</code> (function ver.)","Combinator: <code>map_result</code> (function ver.)","Implement <code>map</code> and related combinators for <code>Parsable</code>.","Combinator: <code>map</code>","Combinator: <code>map_option</code>","Combinator: <code>map_result</code>","Data structure for <code>lazy</code> combinator.","Combinator: <code>lazy</code>","Combinator: <code>bind</code> (function ver.)","Implement <code>bind</code> combinator for <code>Parsable<S></code>.","Combinator: <code>bind</code>","Combinator: <code>many</code> (function ver.)","Combinator: <code>some</code> (function ver.)","Combinator: <code>optional</code> (function ver.)","Implement replicative combinators for <code>Parsable<S></code>.","Combinator: <code>many</code>","Combinator: <code>some</code> (function ver.)","Combinator: <code>optional</code>","Combinator: <code>and</code> (function ver.)","Combinator: <code>left</code> (function ver.)","Combinator: <code>right</code> (function ver.)","Combinator: <code>mid</code> (function ver.)","Implement sequential combinators for <code>Parsable<S></code>.","Combinator: <code>and</code> (function ver.)","Combinator: <code>left</code>","Combinator: <code>right</code>","Combinator: <code>mid</code>","Definitions of a parser and its friends.","Struct <code>Pos</code>","Data structure for log messages.","","","","Struct <code>MsgBody</code>","","","Struct <code>ParseLogger</code>","","<code>Parser</code> struct","<code>Parsable</code> trait","","","Parse function","Wrapper for parse function","Convert into a Parser","<code>CharStream</code> and its primitive parser combinators.","","Combinator: <code>satisfy</code>","Combinator: <code>char</code>","Combinator: <code>literal</code>","Combinator: <code>regex</code>","Combinator: <code>space</code>","Combinator: <code>trim</code> (function ver.)","Implement <code>trim</code> method for <code>Parsable<CharStream></code>:","Combinator: <code>trim</code>","","Struct: <code>CharStream</code>","Create a new CharStream instance","Return the <code>&str</code> form of parse stream","Return current position of parsing","Return current index","Return length of parse string","Check if the stream is empty","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","fix f = f (fix f)","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Combinator: <code>or</code> (function ver.)","Combinator: <code>compose</code>","Combinator: <code>info</code>","Combinator: <code>warn</code>","Combinator: <code>error</code>","Combinator: <code>inspect</code>","Combinator: <code>recover</code>","Combinator: <code>map</code>","Combinator: <code>map_option</code>","Combinator: <code>map_result</code>","Combinator: <code>bind</code>","Combinator: <code>many</code>","Combinator: <code>some</code> (function ver.)","Combinator: <code>optional</code>","Combinator: <code>and</code> (function ver.)","Combinator: <code>left</code>","Combinator: <code>right</code>","Combinator: <code>mid</code>","","","","","","Insert a new log message","Clear all existing logs","Intialize a new instance with provided log message","Return number of logs","Check if logger is empty","","Wrapper for parse function","Convert into a Parser"],"i":[0,0,0,0,1,0,0,0,2,0,3,3,3,3,3,0,0,0,0,0,0,4,4,4,0,0,0,0,5,0,0,0,0,6,6,6,0,0,0,0,0,7,7,7,7,0,0,0,8,8,8,0,9,9,0,10,0,0,11,11,11,11,11,0,0,0,0,0,0,0,0,0,12,0,0,13,13,13,13,13,13,14,14,14,14,14,14,14,14,14,15,15,15,15,15,15,15,15,15,16,16,16,16,16,16,16,16,16,8,8,8,8,8,8,8,8,8,8,9,9,9,9,9,9,9,9,9,10,10,10,10,10,10,10,10,10,17,17,17,17,17,17,17,17,17,13,13,13,13,13,13,13,13,13,13,14,15,17,10,13,14,15,16,8,9,10,17,13,16,10,16,16,8,8,9,9,10,10,16,8,9,10,13,8,17,17,17,17,17,1,2,3,3,3,3,3,4,4,4,5,6,6,6,7,7,7,7,16,16,16,16,9,10,10,10,10,10,17,11,11],"f":[null,[[],["parser",3]],[[],[["parser",3],["clone",8]]],null,[[],["parser",3]],[[["clone",8]],[["parser",3],["clone",8]]],[[],[["clone",8],["parser",3]]],null,[[],["parser",3]],null,[[["str",15]],["parser",3]],[[["str",15]],["parser",3]],[[["str",15]],["parser",3]],[[],["parser",3]],[[],["parser",3]],null,[[],["parser",3]],[[],[["clone",8],["parser",3]]],[[],[["clone",8],["parser",3]]],[[],[["clone",8],["parser",3]]],null,[[],["parser",3]],[[],["parser",3]],[[],["parser",3]],null,[[],["parser",3]],[[],["parser",3]],null,[[],["parser",3]],[[],[["clone",8],["vec",3],["parser",3]]],[[],[["clone",8],["vec",3],["parser",3]]],[[],[["option",4],["clone",8],["parser",3]]],null,[[],[["vec",3],["parser",3]]],[[],[["vec",3],["parser",3]]],[[],[["option",4],["parser",3]]],[[],[["clone",8],["parser",3]]],[[],[["parser",3],["clone",8]]],[[],[["clone",8],["parser",3]]],[[],[["clone",8],["parser",3]]],null,[[],["parser",3]],[[],["parser",3]],[[],["parser",3]],[[],["parser",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["parselogger",3]],["option",4]],[[]],[[],["parser",3]],null,null,[[],[["parser",3],["char",15],["charstream",3]]],[[["char",15]],[["parser",3],["char",15],["charstream",3]]],[[["str",15]],[["str",15],["charstream",3],["parser",3]]],[[["str",15]],[["str",15],["charstream",3],["parser",3]]],[[],[["parser",3],["char",15],["charstream",3]]],[[],[["charstream",3],["parser",3]]],null,[[],[["charstream",3],["parser",3]]],null,null,[[["str",15]]],[[],["str",15]],[[],["pos",3]],[[],["usize",15]],[[],["usize",15]],[[],["bool",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["string",3]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["parselogger",3]],["option",4]],[[["parselogger",3]],["option",4]],[[["parselogger",3]],["option",4]],[[]],[[],["option",4]],[[]],[[],["lazy",3]],[[],["pos",3]],[[],["msg",4]],[[],["msgbody",3]],[[],["parselogger",3]],[[],["parser",3]],[[],["charstream",3]],[[],["pos",3]],[[],["parselogger",3]],[[["pos",3]],["bool",15]],[[["pos",3]],["bool",15]],[[["msg",4]],["bool",15]],[[["msg",4]],["bool",15]],[[["msgbody",3]],["bool",15]],[[["msgbody",3]],["bool",15]],[[["parselogger",3]],["bool",15]],[[["parselogger",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[],["parser",3]],[[],["parser",3]],[[["str",15]],["parser",3]],[[["str",15]],["parser",3]],[[["str",15]],["parser",3]],[[],["parser",3]],[[],["parser",3]],[[],["parser",3]],[[],["parser",3]],[[],["parser",3]],[[],["parser",3]],[[],[["vec",3],["parser",3]]],[[],[["vec",3],["parser",3]]],[[],[["option",4],["parser",3]]],[[],["parser",3]],[[],["parser",3]],[[],["parser",3]],[[],["parser",3]],[[["usize",15]]],[[["usize",15]]],[[],["usize",15]],[[],["usize",15]],[[["pos",3],["option",4],["str",15]]],[[["msg",4]]],[[]],[[["msg",4]]],[[],["usize",15]],[[],["bool",15]],[[]],[[]],[[],["parser",3]]],"p":[[8,"AlternativeExt"],[8,"ApplicativeExt"],[8,"LogExt"],[8,"FunctorExt"],[8,"MonadExt"],[8,"ReplicativeExt"],[8,"SequentialExt"],[4,"Msg"],[3,"MsgBody"],[3,"ParseLogger"],[8,"Parsable"],[8,"PrimitiveExt"],[3,"CharStream"],[3,"Fix"],[3,"Lazy"],[3,"Pos"],[3,"Parser"]]}\
}');
initSearch(searchIndex);