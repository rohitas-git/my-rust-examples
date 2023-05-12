/* -------------------------------------------------------------------------- */
/*                           Matcher and Transcriber                          */
/* -------------------------------------------------------------------------- */

/// () => {} is the entry for a macro rule. 
/// We can have many rules to match for in a single macro.
/// 
/// () => {}  === Matcher & Transcriber
/// ($NAME: DESIGNATOR)
/// 
/// *Here’s a quick list of the designators available in Rust:
// item: an item, such as a function, a struct, a module, etc.
// block: a block (i.e. a block of statements and/or an expression, surrounded by braces)
// stmt: a statement
// pat: a pattern
// expr: an expression
// ty: a type
// ident: an identifier
// path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, ...)
// meta: a meta item; the things that go inside #[...] and #![...] attributes
// tt: a single token tree
/// 

macro_rules! say_hello_world {
    () => {
        println!("Hello World!")
    };

}

fn main(){
    say_hello_world!();
}