// *Procedural macros accept some code as an input, operate on that code, and produce some code as an output 
// rather than matching against patterns and replacing the code with other code as declarative macros do.

// The three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion.

// *When creating procedural macros, the definitions must reside in their own crate with a special crate type.

/* ---------------- An example of defining a procedural macro --------------- */

// Filename: src/lib.rs
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
/* ----------------------------------- x ---------------------------------- */

// *The function that defines a procedural macro takes a TokenStream as an input and produces a TokenStream as an output.
// *The TokenStream type is defined by the proc_macro crate that is included with Rust and represents a sequence of tokens. 
// This is the core of the macro: 
// the source code that the macro is operating on makes up the input TokenStream, 
// and the code the macro produces is the output TokenStream. 
// 
// *The function also has an attribute attached to it that specifies which kind of procedural macro we’re creating. 
// We can have multiple kinds of procedural macros in the same crate.

/* -------------------------------------------------------------------------- */
/*                                    Info Derive Macro                       */
/* -------------------------------------------------------------------------- */
// *derive only works for structs and enums;

/* ------------------------------- proc_macro ------------------------------- */
// The proc_macro crate comes with Rust, so we didn’t need to add that to the dependencies in Cargo.toml. 
// *The proc_macro crate is the compiler’s API that allows us to read and manipulate Rust code from our code.

/* ----------------------------------- syn ---------------------------------- */
// The syn crate parses Rust code from a string into a data structure that we can perform operations on.
// The parse function in syn takes a TokenStream and returns a DeriveInput struct representing the parsed Rust code

// https://docs.rs/syn/1.0.109/syn/struct.DeriveInput.html

/* ---------------------------------- quote --------------------------------- */
// The quote crate turns syn data structures back into Rust code.

// The returned TokenStream is added to the code that our crate users write, 
// so when they compile their crate, they’ll get the extra functionality that we provide in the modified TokenStream.

// https://docs.rs/quote/latest/quote/

/* ------------------------------------  ----------------------------------- */
// *It’s necessary for our procedural macro to panic on errors because proc_macro_derive functions must return TokenStream rather than Result to conform to the procedural macro API.
// you should provide more specific error messages about what went wrong by using panic! or expect

// These crates make it much simpler to parse any sort of Rust code we might want to handle: writing a full parser for Rust code is no simple task.


/* -------------------------------------------------------------------------- */
/*                          Info abt Attribute Macro                          */
/* -------------------------------------------------------------------------- */

// *Attribute-like macros allow you to create new attributes.
// attributes can be applied to other items as well, such as functions.

// #[route(GET, "/")]
// fn index() {}
// 
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
// 
// Here, we have two parameters of type TokenStream. 
// The first is for the contents of the attribute: the GET, "/" part. 
// The second is the body of the item the attribute is attached to: in this case, fn index() {} and the rest of the function’s body.

// Other than that, attribute-like macros work the same way as custom derive macros: 
// *you create a crate with the proc-macro crate type and implement a function that generates the code you want! 

/* -------------------------------------------------------------------------- */
/*                          Info Function-like Macros                         */
/* -------------------------------------------------------------------------- */

// *Function-like macros define macros that look like function calls.
// 
// Similarly to macro_rules! macros, they’re more flexible than functions; 
// for example, they can take an unknown number of arguments.
// 
// *Function-like macros take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do.

// let sql = sql!(SELECT * FROM posts WHERE id=1);
// 
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}
// 
// *we receive the tokens that are inside the parentheses and return the code we wanted to generate.
